//! Activation of WinGet's runtime classes.
//!
//! WinGet's classes are not registered for `RoActivateInstance` outside the
//! AppInstaller package, so there are two ways in:
//!
//! * **Out of process**: `CoCreateInstance` against the CLSIDs declared in the
//!   AppInstaller `AppxManifest.xml`. Current WinGet builds reject this from
//!   unpackaged callers with `APPMODEL_ERROR_NO_PACKAGE` (0x80073D54).
//! * **In process**: WinGet's in-proc module, `WindowsPackageManager.dll`,
//!   handing out activation factories by runtime class name. Operations then
//!   run inside this process. Windows refuses to map images out of the
//!   `WindowsApps` folder into an unpackaged process, so the DLL is first
//!   copied from the installed App Installer package into a per-user cache
//!   (the same thing the Microsoft.WinGet.Client PowerShell module does by
//!   bundling it).
//!
//! The first successful activation picks a mode and every later object uses
//! the same one, since objects from the two servers cannot be mixed.

use core::ffi::c_void;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use windows_core::{Error, GUID, HRESULT, HSTRING, Interface, Result, RuntimeName};

windows_link::link!("ole32.dll" "system" fn CoCreateInstance(rclsid: *const GUID, outer: *mut c_void, context: u32, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIncrementMTAUsage(cookie: *mut *mut c_void) -> HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(family: *const u16, count: *mut u32, names: *mut *mut u16, buffer_len: *mut u32, buffer: *mut u16) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetPackagePathByFullName(full_name: *const u16, path_len: *mut u32, path: *mut u16) -> i32);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExW(file: *const u16, reserved: *mut c_void, flags: u32) -> *mut c_void);
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void);

const CLSCTX_LOCAL_SERVER: u32 = 0x4;
// Packaged COM registrations count as lower trust; unpackaged callers must opt in.
const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: u32 = 0x0400_0000;
const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 0x8;
const ERROR_INSUFFICIENT_BUFFER: i32 = 122;
const ERROR_NOT_FOUND: u32 = 1168;
const APP_INSTALLER_FAMILY: &str = "Microsoft.DesktopAppInstaller_8wekyb3d8bbwe";
const IN_PROC_DLL: &str = "WindowsPackageManager.dll";

pub const CLSID_PACKAGE_MANAGER: GUID = GUID::from_u128(0xC53A4F16_787E_42A4_B304_29EFFB4BF597);
pub const CLSID_FIND_PACKAGES_OPTIONS: GUID = GUID::from_u128(0x572DED96_9C60_4526_8F92_EE7D91D38C1A);
pub const CLSID_CREATE_COMPOSITE_PACKAGE_CATALOG_OPTIONS: GUID = GUID::from_u128(0x526534B8_7E46_47C8_8416_B1685C327D37);
pub const CLSID_INSTALL_OPTIONS: GUID = GUID::from_u128(0x1095F097_EB96_453B_B4E6_1613637F3B14);
pub const CLSID_UNINSTALL_OPTIONS: GUID = GUID::from_u128(0xE1D9A11E_9F85_4D87_9C17_2B93143ADB8D);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    OutOfProcess,
    InProcess,
}

/// A failed activation step and its HRESULT.
type StepError = (&'static str, HRESULT);

type GetActivationFactory = unsafe extern "system" fn(class_id: *mut c_void, factory: *mut *mut c_void) -> HRESULT;

struct InProc {
    get_factory: GetActivationFactory,
}

static MODE: OnceLock<Mode> = OnceLock::new();
static IN_PROC: OnceLock<std::result::Result<InProc, StepError>> = OnceLock::new();
static LAST_ERROR: Mutex<String> = Mutex::new(String::new());

/// Keeps the process MTA alive so any thread without an explicit apartment
/// (every Haskell RTS worker) can use the agile WinGet objects.
pub fn ensure_mta() -> Result<()> {
    static MTA: OnceLock<HRESULT> = OnceLock::new();
    let hr = *MTA.get_or_init(|| {
        let mut cookie = core::ptr::null_mut();
        unsafe { CoIncrementMTAUsage(&mut cookie) }
    });
    hr.ok()
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn wide_path(p: &Path) -> Vec<u16> {
    p.as_os_str().encode_wide().chain(Some(0)).collect()
}

/// HRESULT_FROM_WIN32.
fn win32(code: u32) -> HRESULT {
    if code == 0 { HRESULT(0) } else { HRESULT(((code & 0xFFFF) | 0x8007_0000) as i32) }
}

fn io_hresult(e: &std::io::Error) -> HRESULT {
    e.raw_os_error().map(|c| win32(c as u32)).unwrap_or(HRESULT(0x80004005_u32 as i32))
}

fn describe(hr: HRESULT) -> String {
    format!("{} (0x{:08X})", hr.message().trim(), hr.0 as u32)
}

fn out_of_process<T: Interface>(clsid: &GUID) -> Result<T> {
    let mut ptr = core::ptr::null_mut();
    unsafe {
        CoCreateInstance(
            clsid,
            core::ptr::null_mut(),
            CLSCTX_LOCAL_SERVER | CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION,
            &T::IID,
            &mut ptr,
        )
        .ok()?;
        Ok(T::from_raw(ptr))
    }
}

fn from_wide_z(buf: &[u16]) -> OsString {
    OsString::from_wide(&buf[..buf.iter().position(|&c| c == 0).unwrap_or(buf.len())])
}

/// Full name and install directory of the newest registered App Installer package.
fn app_installer_package() -> std::result::Result<(String, PathBuf), HRESULT> {
    newest_package(APP_INSTALLER_FAMILY)
}

/// Full name and install directory of the newest package registered for this
/// user in a package family.
pub fn newest_package(family: &str) -> std::result::Result<(String, PathBuf), HRESULT> {
    let family = wide(family);
    let (mut count, mut buffer_len) = (0u32, 0u32);
    let rc = unsafe { GetPackagesByPackageFamily(family.as_ptr(), &mut count, core::ptr::null_mut(), &mut buffer_len, core::ptr::null_mut()) };
    if rc != ERROR_INSUFFICIENT_BUFFER || count == 0 {
        return Err(win32(if rc == 0 { ERROR_NOT_FOUND } else { rc as u32 }));
    }
    let mut names = vec![core::ptr::null_mut::<u16>(); count as usize];
    let mut buffer = vec![0u16; buffer_len as usize];
    let rc = unsafe { GetPackagesByPackageFamily(family.as_ptr(), &mut count, names.as_mut_ptr(), &mut buffer_len, buffer.as_mut_ptr()) };
    if rc != 0 {
        return Err(win32(rc as u32));
    }
    // Full names embed the version; the last registered one is the one in use.
    let full_name_ptr = *names.last().ok_or(win32(ERROR_NOT_FOUND))?;
    let full_name = {
        let start = (full_name_ptr as usize - buffer.as_ptr() as usize) / 2;
        from_wide_z(&buffer[start..]).to_string_lossy().into_owned()
    };
    let mut path_len = 0u32;
    let rc = unsafe { GetPackagePathByFullName(full_name_ptr, &mut path_len, core::ptr::null_mut()) };
    if rc != ERROR_INSUFFICIENT_BUFFER {
        return Err(win32(rc as u32));
    }
    let mut path = vec![0u16; path_len as usize];
    let rc = unsafe { GetPackagePathByFullName(full_name_ptr, &mut path_len, path.as_mut_ptr()) };
    if rc != 0 {
        return Err(win32(rc as u32));
    }
    Ok((full_name, PathBuf::from(from_wide_z(&path))))
}

/// Copy the in-proc DLL out of `WindowsApps` into
/// `%LOCALAPPDATA%\winget-gui\inproc\<package full name>\`, refreshing it when
/// the installed copy differs. Keyed by full name, an App Installer update
/// lands in a new directory.
fn cached_in_proc_dll(full_name: &str, package_dir: &Path) -> std::result::Result<PathBuf, StepError> {
    let source = package_dir.join(IN_PROC_DLL);
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let dir = base.join("winget-gui").join("inproc").join(full_name);
    let target = dir.join(IN_PROC_DLL);
    let source_len = std::fs::metadata(&source).map_err(|e| ("reading WinGet's in-process module", io_hresult(&e)))?.len();
    let current = std::fs::metadata(&target).map(|m| m.len()).ok();
    if current != Some(source_len) {
        std::fs::create_dir_all(&dir).map_err(|e| ("creating the in-process module cache", io_hresult(&e)))?;
        // Copy beside the target and rename, so a crash never leaves a torn DLL.
        let partial = dir.join(format!("{IN_PROC_DLL}.{}.partial", std::process::id()));
        std::fs::copy(&source, &partial).map_err(|e| ("copying WinGet's in-process module", io_hresult(&e)))?;
        if let Err(e) = std::fs::rename(&partial, &target) {
            let _ = std::fs::remove_file(&partial);
            // Another process may have loaded an identical copy already.
            if std::fs::metadata(&target).map(|m| m.len()).ok() != Some(source_len) {
                return Err(("installing WinGet's in-process module", io_hresult(&e)));
            }
        }
    }
    Ok(target)
}

fn load_in_proc() -> std::result::Result<InProc, StepError> {
    let (full_name, package_dir) = app_installer_package().map_err(|hr| ("locating the App Installer package", hr))?;
    let dll = cached_in_proc_dll(&full_name, &package_dir)?;
    unsafe {
        let module = LoadLibraryExW(wide_path(&dll).as_ptr(), core::ptr::null_mut(), LOAD_WITH_ALTERED_SEARCH_PATH);
        if module.is_null() {
            return Err(("loading WinGet's in-process module", HRESULT::from_thread()));
        }
        let initialize = GetProcAddress(module, c"WindowsPackageManagerInProcModuleInitialize".as_ptr().cast());
        let get_factory = GetProcAddress(module, c"WindowsPackageManagerInProcModuleGetActivationFactory".as_ptr().cast());
        if initialize.is_null() || get_factory.is_null() {
            return Err(("finding the in-process module exports", HRESULT::from_thread()));
        }
        let initialize: unsafe extern "system" fn() -> HRESULT = core::mem::transmute(initialize);
        let hr = initialize();
        if hr.is_err() {
            return Err(("initializing the in-process module", hr));
        }
        Ok(InProc { get_factory: core::mem::transmute::<*mut c_void, GetActivationFactory>(get_factory) })
    }
}

fn in_process<T: Interface + RuntimeName>() -> std::result::Result<T, StepError> {
    let module = IN_PROC.get_or_init(load_in_proc).as_ref().map_err(|e| *e)?;
    let name = HSTRING::from(T::NAME);
    let mut raw = core::ptr::null_mut();
    unsafe {
        let hr = (module.get_factory)(core::mem::transmute_copy(&name), &mut raw);
        if hr.is_err() || raw.is_null() {
            return Err(("getting the activation factory", if hr.is_err() { hr } else { win32(ERROR_NOT_FOUND) }));
        }
        let factory = windows_core::imp::IGenericFactory::from_raw(raw);
        factory.ActivateInstance::<T>().map_err(|e| ("activating the object", e.code()))
    }
}

/// Which server this process talks to, once any object has been created.
pub fn mode() -> Option<Mode> {
    MODE.get().copied()
}

/// Why the last failed activation failed, with both paths' errors.
pub fn last_error() -> String {
    LAST_ERROR.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

pub fn create<T: Interface + RuntimeName>(clsid: &GUID) -> Result<T> {
    ensure_mta()?;
    match MODE.get() {
        Some(Mode::OutOfProcess) => return out_of_process(clsid),
        Some(Mode::InProcess) => return in_process().map_err(|(_, hr)| Error::from(hr)),
        None => {}
    }
    // The first successful activation decides the mode for the life of the
    // process. Failures are not remembered, so a later call can retry.
    let out_err = match out_of_process::<T>(clsid) {
        Ok(object) => {
            let _ = MODE.set(Mode::OutOfProcess);
            return Ok(object);
        }
        Err(e) => e.code(),
    };
    match in_process::<T>() {
        Ok(object) => {
            let _ = MODE.set(Mode::InProcess);
            Ok(object)
        }
        Err((step, hr)) => {
            *LAST_ERROR.lock().unwrap_or_else(|e| e.into_inner()) = format!(
                "WinGet's COM server refused this process: {}. Loading WinGet in-process failed while {}: {}.",
                describe(out_err),
                step,
                describe(hr)
            );
            Err(Error::from(hr))
        }
    }
}
