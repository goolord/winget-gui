/* C ABI between the Haskell GUI and the Rust WinGet bridge (winget_bridge.dll).
 *
 * All strings are UTF-8. Buffers returned by the bridge are owned by the
 * caller and must be released with wg_free. Every function is safe to call
 * from any thread that has not entered a single-threaded COM apartment.
 */
#ifndef WINGET_BRIDGE_H
#define WINGET_BRIDGE_H

#include <stddef.h>
#include <stdint.h>

#define WG_FIELD_SEP 0x1F /* separates fields within a package record */
#define WG_RECORD_SEP 0x1E /* terminates each package record */

/* wg_list_installed flags */
#define WG_LIST_CHECK_UPDATES 0x1 /* correlate with remote sources (slower, may use network).
                                     Sources with unaccepted agreements are skipped. */

/* wg_uninstall / wg_upgrade flags */
#define WG_OP_SILENT 0x1      /* no installer UI; otherwise the installer's default mode */
#define WG_OP_INTERACTIVE 0x2 /* force the installer's full interactive UI */
#define WG_OP_FORCE 0x4       /* skip WinGet's safety checks (e.g. running-app detection) */
#define WG_OP_ACCEPT_AGREEMENTS 0x8 /* upgrade only: accept package license agreements */

/* Operation states reported through wg_progress_cb */
#define WG_STATE_QUEUED 0
#define WG_STATE_RUNNING 1
#define WG_STATE_POST 2
#define WG_STATE_FINISHED 3

typedef void (*wg_progress_cb)(void *user, uint32_t state, double fraction);

/* Starts COM (process MTA) and connects to WinGet: its out-of-process server
 * when this process may use it, otherwise WinGet's in-process module. Windows
 * will not load that module from the WindowsApps folder into an unpackaged
 * process, so it is copied from the installed App Installer package to
 * %LOCALAPPDATA%\winget-gui\inproc\<package full name>\ and loaded from there.
 * Returns an HRESULT; see wg_last_error on failure. */
int32_t wg_init(void);

/* Details of the last activation failure (both paths), as a UTF-8 buffer. */
uint8_t *wg_last_error(size_t *out_len);

/* 0 = not connected, 1 = out-of-process server, 2 = in-process module. */
uint32_t wg_activation_mode(void);

/* Snapshots installed packages. Returns a record buffer (see WG_*_SEP) and
 * writes its length and a snapshot id that later operations refer to.
 * Record fields, in order:
 *   0 index        position within the snapshot
 *   1 id           WinGet package id (ARP\..., MSIX\..., or a catalog id)
 *   2 name
 *   3 version      installed version
 *   4 publisher
 *   5 source       catalog name of the correlated available package, or empty
 *   6 available    newer available version, or empty
 *   7 scope        "User", "Machine", or empty
 *   8 installer    installer type (msi, exe, msix, ...), or empty
 *   9 location     install location, or empty
 *  10 date         install date as YYYY-MM-DD, or empty
 *  11 size         estimated size in bytes, or 0
 *  12 productcodes ';'-separated ARP product codes
 *  13 families     ';'-separated MSIX package family names
 *  14 uninstall    "S" if a silent uninstall command is known, "U" if only a standard one, else empty
 *  15 arch         installed architecture, or empty
 *  16 drive        volume the app is installed on: "C:", "Network", or empty when unknown.
 *                  From the install location (WinGet, then the registry, then the
 *                  MSIX package folder), else the uninstaller or icon path.
 *                  Field 9 falls back the same way when WinGet has no location.
 * On failure returns NULL and writes the HRESULT to *out_hr. */
uint8_t *wg_list_installed(uint32_t flags, size_t *out_len, uint64_t *out_snapshot, int32_t *out_hr);

/* Releases a snapshot and the COM objects it holds. */
void wg_release_snapshot(uint64_t snapshot);

/* Uninstalls package `index` of `snapshot`, blocking until it finishes.
 * `token` names the operation for wg_cancel; `cb` may be NULL.
 * Returns the operation HRESULT and writes the WinGet result status, the
 * installer's own exit code, and whether a reboot is required. */
int32_t wg_uninstall(uint64_t snapshot, uint32_t index, uint32_t flags, uint64_t token,
                     wg_progress_cb cb, void *user,
                     uint32_t *out_status, uint32_t *out_installer_code, uint8_t *out_reboot);

/* Upgrades package `index` of `snapshot` to its default available version. Same contract as wg_uninstall. */
int32_t wg_upgrade(uint64_t snapshot, uint32_t index, uint32_t flags, uint64_t token,
                   wg_progress_cb cb, void *user,
                   uint32_t *out_status, uint32_t *out_installer_code, uint8_t *out_reboot);

/* Requests cancellation of a running operation. Returns 1 if the token was running. */
uint8_t wg_cancel(uint64_t token);

/* Human-readable message for an HRESULT, as a UTF-8 buffer. */
uint8_t *wg_error_message(int32_t hr, size_t *out_len);

void wg_free(uint8_t *ptr, size_t len);

#endif
