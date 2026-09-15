//! C ABI over the WinGet COM API (`Microsoft.Management.Deployment`) for the
//! Haskell GUI. The contract lives in `include/winget_bridge.h`.

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports, clippy::all)]
mod bindings;
mod com;

use bindings::Microsoft::Management::Deployment::*;
use core::ffi::c_void;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use windows_collections::IVectorView;
use windows_core::{Error, HRESULT, HSTRING, Result};
use windows_future::AsyncOperationProgressHandler;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

const FIELD_SEP: char = '\u{1F}';
const RECORD_SEP: char = '\u{1E}';

const LIST_CHECK_UPDATES: u32 = 0x1;
const OP_SILENT: u32 = 0x1;
const OP_INTERACTIVE: u32 = 0x2;
const OP_FORCE: u32 = 0x4;
const OP_ACCEPT_AGREEMENTS: u32 = 0x8;

const STATE_QUEUED: u32 = 0;
const STATE_RUNNING: u32 = 1;
const STATE_POST: u32 = 2;
const STATE_FINISHED: u32 = 3;

const E_UNEXPECTED: HRESULT = HRESULT(0x8000FFFF_u32 as i32);
const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as i32);

pub type ProgressCb = Option<unsafe extern "C" fn(user: *mut c_void, state: u32, fraction: f64)>;

/// WinGet proxies live in the process MTA and are free-threaded, but the
/// generated types are not marked `Send`/`Sync`.
struct Agile<T>(T);
unsafe impl<T> Send for Agile<T> {}
unsafe impl<T> Sync for Agile<T> {}

type Snapshot = Arc<Vec<Agile<CatalogPackage>>>;
type Cancel = Arc<dyn Fn() + Send + Sync>;

static MANAGER: OnceLock<Agile<PackageManager>> = OnceLock::new();
static SNAPSHOTS: Mutex<Option<(u64, HashMap<u64, Snapshot>)>> = Mutex::new(None);
static RUNNING: Mutex<Option<HashMap<u64, Cancel>>> = Mutex::new(None);

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn manager() -> Result<&'static PackageManager> {
    if let Some(m) = MANAGER.get() {
        return Ok(&m.0);
    }
    let created = com::create::<PackageManager>(&com::CLSID_PACKAGE_MANAGER)?;
    Ok(&MANAGER.get_or_init(|| Agile(created)).0)
}

fn check(status_ok: bool, extended: Result<HRESULT>) -> Result<()> {
    if status_ok {
        return Ok(());
    }
    let hr = extended.unwrap_or(E_UNEXPECTED);
    Err(Error::from(if hr.is_ok() { E_UNEXPECTED } else { hr }))
}

// ---------------------------------------------------------------------------
// Listing
// ---------------------------------------------------------------------------

fn composite(pm: &PackageManager, skip_pending_agreements: bool) -> Result<PackageCatalogReference> {
    let options: CreateCompositePackageCatalogOptions = com::create(&com::CLSID_CREATE_COMPOSITE_PACKAGE_CATALOG_OPTIONS)?;
    let remotes = pm.GetPackageCatalogs()?;
    let catalogs = options.Catalogs()?;
    for i in 0..remotes.Size()? {
        let remote = remotes.GetAt(i)?;
        // Explicit sources are only searched when named, as `winget list` does.
        if remote.Info().and_then(|info| info.Explicit()).unwrap_or(false) {
            continue;
        }
        let pending = remote.SourceAgreements().and_then(|a| a.Size()).unwrap_or(0) > 0;
        if !(skip_pending_agreements && pending) {
            catalogs.Append(&remote)?;
        }
    }
    options.SetCompositeSearchBehavior(CompositeSearchBehavior::LocalCatalogs)?;
    pm.CreateCompositePackageCatalog(&options)
}

fn connect_installed(check_updates: bool) -> Result<PackageCatalog> {
    let pm = manager()?;
    if !check_updates {
        let connected = pm.GetLocalPackageCatalog(LocalPackageCatalog::InstalledPackages)?.Connect()?;
        check(connected.Status()? == ConnectResultStatus::Ok, connected.ExtendedErrorCode())?;
        return connected.PackageCatalog();
    }
    let mut connected = composite(pm, false)?.Connect()?;
    // A source whose agreement the user never accepted blocks the composite;
    // leave those sources out rather than accepting terms on the user's behalf.
    if connected.Status()? == ConnectResultStatus::SourceAgreementsNotAccepted {
        connected = composite(pm, true)?.Connect()?;
    }
    check(connected.Status()? == ConnectResultStatus::Ok, connected.ExtendedErrorCode())?;
    connected.PackageCatalog()
}

fn find_all(catalog: &PackageCatalog) -> Result<Vec<CatalogPackage>> {
    let options: FindPackagesOptions = com::create(&com::CLSID_FIND_PACKAGES_OPTIONS)?;
    let found = catalog.FindPackages(&options)?;
    check(found.Status()? == FindPackagesResultStatus::Ok, found.ExtendedErrorCode())?;
    let matches = found.Matches()?;
    let count = matches.Size()?;
    let mut packages = Vec::with_capacity(count as usize);
    for i in 0..count {
        if let Ok(package) = matches.GetAt(i).and_then(|m| m.CatalogPackage()) {
            packages.push(package);
        }
    }
    Ok(packages)
}

fn strings(view: Result<IVectorView<HSTRING>>) -> Vec<String> {
    let Ok(view) = view else { return Vec::new() };
    let count = view.Size().unwrap_or(0);
    (0..count).filter_map(|i| view.GetAt(i).ok()).map(|s| s.to_string_lossy()).collect()
}

fn text(value: Result<HSTRING>) -> String {
    value.map(|s| s.to_string_lossy()).unwrap_or_default()
}

/// What an Add/Remove Programs registry entry adds to WinGet's metadata.
#[derive(Default)]
struct ArpInfo {
    /// YYYY-MM-DD, or empty.
    date: String,
    /// Estimated size in bytes, or 0.
    size: u64,
    location: String,
    uninstall_command: String,
    icon: String,
}

/// Registry details from the Add/Remove Programs entry named by the package's
/// product codes (64-bit machine, 32-bit machine, then per-user hive).
fn arp_details(product_codes: &[String]) -> ArpInfo {
    const UNINSTALL: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";
    for code in product_codes {
        let path = format!(r"{UNINSTALL}\{code}");
        let key = LOCAL_MACHINE
            .options()
            .read()
            .wow64_64()
            .open(&path)
            .or_else(|_| LOCAL_MACHINE.options().read().wow64_32().open(&path))
            .or_else(|_| CURRENT_USER.options().read().open(&path));
        if let Ok(key) = key {
            let value = |name: &str| key.get_string(name).map(|s| s.trim().to_string()).unwrap_or_default();
            let date = Some(value("InstallDate"))
                .filter(|d| d.len() == 8 && d.bytes().all(|b| b.is_ascii_digit()))
                .map(|d| format!("{}-{}-{}", &d[0..4], &d[4..6], &d[6..8]))
                .unwrap_or_default();
            return ArpInfo {
                date,
                size: key.get_u32("EstimatedSize").map(|kb| kb as u64 * 1024).unwrap_or(0),
                location: value("InstallLocation"),
                uninstall_command: value("UninstallString"),
                icon: value("DisplayIcon"),
            };
        }
    }
    ArpInfo::default()
}

/// Expands a leading `%NAME%` (as in `%ProgramFiles%\App`).
fn expand_leading_env(s: &str) -> String {
    if let Some(rest) = s.strip_prefix('%') {
        if let Some(end) = rest.find('%') {
            if let Some(value) = std::env::var_os(&rest[..end]) {
                return format!("{}{}", value.to_string_lossy(), &rest[end + 1..]);
            }
        }
    }
    s.to_string()
}

/// The volume a path or command line points at: `C:` for a lettered drive,
/// `Network` for a UNC path, otherwise nothing (e.g. `MsiExec.exe /X{...}`).
fn drive_of(path_or_command: &str) -> Option<String> {
    let expanded = expand_leading_env(path_or_command.trim().trim_start_matches('"'));
    let path = expanded.strip_prefix(r"\\?\").unwrap_or(&expanded);
    let bytes = path.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        Some(format!("{}:", (bytes[0] as char).to_ascii_uppercase()))
    } else if path.starts_with(r"\\") {
        Some("Network".to_string())
    } else {
        None
    }
}

fn push_field(out: &mut String, value: &str) {
    out.extend(value.chars().map(|c| if c == FIELD_SEP || c == RECORD_SEP { ' ' } else { c }));
    out.push(FIELD_SEP);
}

fn write_record(out: &mut String, index: usize, package: &CatalogPackage, check_updates: bool) {
    let installed = package.InstalledVersion().ok();
    let meta = |field| installed.as_ref().map(|v| text(v.GetMetadata(field))).unwrap_or_default();
    let product_codes = installed.as_ref().map(|v| strings(v.ProductCodes())).unwrap_or_default();
    let families = installed.as_ref().map(|v| strings(v.PackageFamilyNames())).unwrap_or_default();
    let id = text(package.Id());

    let (source, available) = if check_updates {
        let default = package.DefaultInstallVersion().ok();
        let source = default
            .as_ref()
            .and_then(|v| v.PackageCatalog().ok())
            .and_then(|c| c.Info().ok())
            .map(|i| text(i.Name()))
            .unwrap_or_default();
        let available = match (&default, package.IsUpdateAvailable()) {
            (Some(v), Ok(true)) => text(v.Version()),
            _ => String::new(),
        };
        (source, available)
    } else {
        (String::new(), String::new())
    };

    let publisher = installed.as_ref().map(|v| text(v.Publisher())).unwrap_or_default();
    let publisher = if publisher.is_empty() { meta(PackageVersionMetadataField::PublisherDisplayName) } else { publisher };
    let uninstall = if !meta(PackageVersionMetadataField::SilentUninstallCommand).is_empty() {
        "S"
    } else if !meta(PackageVersionMetadataField::StandardUninstallCommand).is_empty() || !families.is_empty() {
        "U"
    } else {
        ""
    };
    // ARP ids look like ARP\Machine\X64\<key>; the third segment is the architecture.
    let arch = id.strip_prefix(r"ARP\").and_then(|rest| rest.split('\\').nth(1)).unwrap_or_default().to_string();
    let arp = arp_details(&product_codes);

    // Where it lives: WinGet's location, else the registry's, else the MSIX
    // package folder.
    let mut location = meta(PackageVersionMetadataField::InstalledLocation);
    if location.is_empty() {
        location = arp.location.clone();
    }
    if location.is_empty() {
        location = families
            .iter()
            .find_map(|family| com::newest_package(family).ok())
            .map(|(_, path)| path.to_string_lossy().into_owned())
            .unwrap_or_default();
    }
    // Without a location, the uninstaller usually sits in the app's folder.
    // MSI icons are cached under the system drive's Windows\Installer
    // wherever the app is, so those say nothing about the app's disk.
    let icon = Some(arp.icon.as_str()).filter(|icon| !icon.to_ascii_lowercase().contains(r"\windows\installer\"));
    let drive = drive_of(&location)
        .or_else(|| drive_of(&arp.uninstall_command))
        .or_else(|| icon.and_then(drive_of))
        .unwrap_or_default();

    push_field(out, &index.to_string());
    push_field(out, &id);
    push_field(out, &text(package.Name()));
    push_field(out, &installed.as_ref().map(|v| text(v.Version())).unwrap_or_default());
    push_field(out, &publisher);
    push_field(out, &source);
    push_field(out, &available);
    push_field(out, &meta(PackageVersionMetadataField::InstalledScope));
    push_field(out, &meta(PackageVersionMetadataField::InstallerType));
    push_field(out, &location);
    push_field(out, &arp.date);
    push_field(out, &arp.size.to_string());
    push_field(out, &product_codes.join(";"));
    push_field(out, &families.join(";"));
    push_field(out, uninstall);
    push_field(out, &arch);
    out.push_str(&drive);
    out.push(RECORD_SEP);
}

fn list(flags: u32) -> Result<(Vec<u8>, u64)> {
    let check_updates = flags & LIST_CHECK_UPDATES != 0;
    let started = std::time::Instant::now();
    let catalog = connect_installed(check_updates)?;
    let connected = started.elapsed();
    let packages = find_all(&catalog)?;
    let found = started.elapsed();
    // Each record is a dozen COM property reads plus registry lookups, and the
    // in-process objects are free-threaded, so build chunks in parallel and
    // join them in order.
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).clamp(1, 8);
    let chunk_len = packages.len().div_ceil(threads).max(1);
    let parts: Vec<String> = std::thread::scope(|scope| {
        let workers: Vec<_> = packages
            .chunks(chunk_len)
            .enumerate()
            .map(|(chunk, slice)| {
                let slice = Agile(slice);
                scope.spawn(move || {
                    let slice = slice;
                    let mut part = String::with_capacity(slice.0.len() * 256);
                    for (offset, package) in slice.0.iter().enumerate() {
                        write_record(&mut part, chunk * chunk_len + offset, package, check_updates);
                    }
                    part
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().unwrap_or_default()).collect()
    });
    let out = parts.concat();
    // WINGET_GUI_TIMING=1 prints where a listing spends its time.
    if std::env::var_os("WINGET_GUI_TIMING").is_some() {
        eprintln!(
            "winget-bridge: list(updates={check_updates}) connect {:?}, find {} packages {:?}, records {:?}",
            connected,
            packages.len(),
            found - connected,
            started.elapsed() - found
        );
    }
    let snapshot: Snapshot = Arc::new(packages.into_iter().map(Agile).collect());
    let mut guard = lock(&SNAPSHOTS);
    let (next, map) = guard.get_or_insert_with(|| (1, HashMap::new()));
    let id = *next;
    *next += 1;
    map.insert(id, snapshot);
    Ok((out.into_bytes(), id))
}

fn leak(bytes: Vec<u8>, out_len: *mut usize) -> *mut u8 {
    let boxed = bytes.into_boxed_slice();
    if !out_len.is_null() {
        unsafe { *out_len = boxed.len() };
    }
    Box::into_raw(boxed) as *mut u8
}

#[unsafe(no_mangle)]
pub extern "C" fn wg_init() -> i32 {
    match manager() {
        Ok(_) => 0,
        Err(e) => e.code().0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_list_installed(flags: u32, out_len: *mut usize, out_snapshot: *mut u64, out_hr: *mut i32) -> *mut u8 {
    let result = list(flags);
    unsafe {
        let (buffer, snapshot, hr) = match result {
            Ok((bytes, id)) => (leak(bytes, out_len), id, 0),
            Err(e) => {
                if !out_len.is_null() {
                    *out_len = 0;
                }
                (core::ptr::null_mut(), 0, e.code().0)
            }
        };
        if !out_snapshot.is_null() {
            *out_snapshot = snapshot;
        }
        if !out_hr.is_null() {
            *out_hr = hr;
        }
        buffer
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wg_release_snapshot(snapshot: u64) {
    if let Some((_, map)) = lock(&SNAPSHOTS).as_mut() {
        map.remove(&snapshot);
    }
}

// ---------------------------------------------------------------------------
// Uninstall / upgrade
// ---------------------------------------------------------------------------

struct OpOutcome {
    hr: HRESULT,
    status: u32,
    installer_code: u32,
    reboot: bool,
}

fn package_at(snapshot: u64, index: u32) -> Result<CatalogPackage> {
    let guard = lock(&SNAPSHOTS);
    let snap = guard.as_ref().and_then(|(_, m)| m.get(&snapshot)).ok_or_else(|| Error::from(E_INVALIDARG))?;
    snap.get(index as usize).map(|p| p.0.clone()).ok_or_else(|| Error::from(E_INVALIDARG))
}

/// The caller's progress callback, shared with WinGet's callback thread.
/// Closing it waits out any report in flight, so once an operation returns the
/// caller may free the callback without racing a late progress event.
#[derive(Clone)]
struct Reporter(Arc<Mutex<Option<(unsafe extern "C" fn(*mut c_void, u32, f64), usize)>>>);

impl Reporter {
    fn new(cb: ProgressCb, user: *mut c_void) -> Self {
        Reporter(Arc::new(Mutex::new(cb.map(|cb| (cb, user as usize)))))
    }

    fn is_set(&self) -> bool {
        lock(&self.0).is_some()
    }

    fn report(&self, state: u32, fraction: f64) {
        if let Some((cb, user)) = *lock(&self.0) {
            unsafe { cb(user as *mut c_void, state, fraction.clamp(0.0, 1.0)) }
        }
    }
}

/// Registers an operation for `wg_cancel` and closes its reporter on exit.
struct Running {
    token: u64,
    reporter: Reporter,
}

impl Running {
    fn new(token: u64, reporter: Reporter, cancel: Cancel) -> Self {
        lock(&RUNNING).get_or_insert_with(HashMap::new).insert(token, cancel);
        Running { token, reporter }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        if let Some(map) = lock(&RUNNING).as_mut() {
            map.remove(&self.token);
        }
        *lock(&self.reporter.0) = None;
    }
}

fn uninstall_mode(flags: u32) -> PackageUninstallMode {
    if flags & OP_SILENT != 0 {
        PackageUninstallMode::Silent
    } else if flags & OP_INTERACTIVE != 0 {
        PackageUninstallMode::Interactive
    } else {
        PackageUninstallMode::Default
    }
}

fn install_mode(flags: u32) -> PackageInstallMode {
    if flags & OP_SILENT != 0 {
        PackageInstallMode::Silent
    } else if flags & OP_INTERACTIVE != 0 {
        PackageInstallMode::Interactive
    } else {
        PackageInstallMode::Default
    }
}

fn uninstall(snapshot: u64, index: u32, flags: u32, token: u64, reporter: Reporter) -> Result<OpOutcome> {
    let package = package_at(snapshot, index)?;
    let options: UninstallOptions = com::create(&com::CLSID_UNINSTALL_OPTIONS)?;
    options.SetPackageUninstallMode(uninstall_mode(flags))?;
    options.SetForce(flags & OP_FORCE != 0)?;

    let operation = manager()?.UninstallPackageAsync(&package, &options)?;
    if reporter.is_set() {
        let r = reporter.clone();
        operation.SetProgress(&AsyncOperationProgressHandler::new(move |_, p: UninstallProgress| {
            let state = match p.State {
                PackageUninstallProgressState::Queued => STATE_QUEUED,
                PackageUninstallProgressState::Uninstalling => STATE_RUNNING,
                PackageUninstallProgressState::PostUninstall => STATE_POST,
                _ => STATE_FINISHED,
            };
            r.report(state, p.UninstallationProgress);
            Ok(())
        }))?;
    }
    let handle = Agile(operation.clone());
    let _running = Running::new(token, reporter, Arc::new(move || drop(handle.0.Cancel())));
    let result = operation.join()?;
    Ok(OpOutcome {
        hr: result.ExtendedErrorCode().unwrap_or(HRESULT(0)),
        status: result.Status()?.0 as u32,
        installer_code: result.UninstallerErrorCode().unwrap_or(0),
        reboot: result.RebootRequired().unwrap_or(false),
    })
}

fn upgrade(snapshot: u64, index: u32, flags: u32, token: u64, reporter: Reporter) -> Result<OpOutcome> {
    let package = package_at(snapshot, index)?;
    let options: InstallOptions = com::create(&com::CLSID_INSTALL_OPTIONS)?;
    options.SetPackageInstallMode(install_mode(flags))?;
    options.SetForce(flags & OP_FORCE != 0)?;
    options.SetAcceptPackageAgreements(flags & OP_ACCEPT_AGREEMENTS != 0)?;

    let operation = manager()?.UpgradePackageAsync(&package, &options)?;
    if reporter.is_set() {
        let r = reporter.clone();
        operation.SetProgress(&AsyncOperationProgressHandler::new(move |_, p: InstallProgress| {
            // Downloading fills the first half of the bar, installing the second.
            let (state, fraction) = match p.State {
                PackageInstallProgressState::Queued => (STATE_QUEUED, 0.0),
                PackageInstallProgressState::Downloading => (STATE_RUNNING, p.DownloadProgress * 0.5),
                PackageInstallProgressState::Installing => (STATE_RUNNING, 0.5 + p.InstallationProgress * 0.5),
                PackageInstallProgressState::PostInstall => (STATE_POST, 1.0),
                _ => (STATE_FINISHED, 1.0),
            };
            r.report(state, fraction);
            Ok(())
        }))?;
    }
    let handle = Agile(operation.clone());
    let _running = Running::new(token, reporter, Arc::new(move || drop(handle.0.Cancel())));
    let result = operation.join()?;
    Ok(OpOutcome {
        hr: result.ExtendedErrorCode().unwrap_or(HRESULT(0)),
        status: result.Status()?.0 as u32,
        installer_code: result.InstallerErrorCode().unwrap_or(0),
        reboot: result.RebootRequired().unwrap_or(false),
    })
}

unsafe fn finish_op(result: Result<OpOutcome>, out_status: *mut u32, out_installer_code: *mut u32, out_reboot: *mut u8) -> i32 {
    // Failures before WinGet produced a result report status 3 (InternalError).
    let outcome = result.unwrap_or_else(|e| OpOutcome { hr: e.code(), status: 3, installer_code: 0, reboot: false });
    unsafe {
        if !out_status.is_null() {
            *out_status = outcome.status;
        }
        if !out_installer_code.is_null() {
            *out_installer_code = outcome.installer_code;
        }
        if !out_reboot.is_null() {
            *out_reboot = outcome.reboot as u8;
        }
    }
    outcome.hr.0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_uninstall(
    snapshot: u64,
    index: u32,
    flags: u32,
    token: u64,
    cb: ProgressCb,
    user: *mut c_void,
    out_status: *mut u32,
    out_installer_code: *mut u32,
    out_reboot: *mut u8,
) -> i32 {
    let result = uninstall(snapshot, index, flags, token, Reporter::new(cb, user));
    unsafe { finish_op(result, out_status, out_installer_code, out_reboot) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_upgrade(
    snapshot: u64,
    index: u32,
    flags: u32,
    token: u64,
    cb: ProgressCb,
    user: *mut c_void,
    out_status: *mut u32,
    out_installer_code: *mut u32,
    out_reboot: *mut u8,
) -> i32 {
    let result = upgrade(snapshot, index, flags, token, Reporter::new(cb, user));
    unsafe { finish_op(result, out_status, out_installer_code, out_reboot) }
}

#[unsafe(no_mangle)]
pub extern "C" fn wg_cancel(token: u64) -> u8 {
    // Clone the handle out so the cross-process Cancel runs without the lock.
    let cancel = lock(&RUNNING).as_ref().and_then(|m| m.get(&token)).cloned();
    match cancel {
        Some(cancel) => {
            cancel();
            1
        }
        None => 0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_error_message(hr: i32, out_len: *mut usize) -> *mut u8 {
    let message = HRESULT(hr).message();
    let message = if message.trim().is_empty() { format!("HRESULT 0x{:08X}", hr as u32) } else { message };
    leak(message.into_bytes(), out_len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_last_error(out_len: *mut usize) -> *mut u8 {
    leak(com::last_error().into_bytes(), out_len)
}

#[unsafe(no_mangle)]
pub extern "C" fn wg_activation_mode() -> u32 {
    match com::mode() {
        None => 0,
        Some(com::Mode::OutOfProcess) => 1,
        Some(com::Mode::InProcess) => 2,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wg_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len))) };
    }
}
