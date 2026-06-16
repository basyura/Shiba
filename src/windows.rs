use crate::renderer::{Event, EventSender};
use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::mem;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use windows::core::{w, BOOL};
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE, HWND, LPARAM, LRESULT, WPARAM,
};
use windows::Win32::System::Console::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};
use windows::Win32::System::DataExchange::COPYDATASTRUCT;
use windows::Win32::System::Threading::{CreateMutexW, ReleaseMutex};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, DefWindowProcW, EnumWindows, GetPropW, IsIconic, RemovePropW, SendMessageW,
    SetForegroundWindow, SetPropW, SetWindowLongPtrW, ShowWindow, GWLP_WNDPROC, SW_RESTORE,
    WM_COPYDATA, WNDPROC,
};

const COPYDATA_OPEN_FILES: usize = 0x5348_4942;
const WINDOW_PROPERTY_VALUE: isize = 1;
const WINDOW_PROPERTY_NAME: windows::core::PCWSTR = w!("ShibaPreviewSingleInstanceWindow");
const WINDOW_HANDLER_PROPERTY_NAME: windows::core::PCWSTR = w!("ShibaPreviewSingleInstanceHandler");
const FIND_WINDOW_RETRIES: usize = 20;
const FIND_WINDOW_RETRY_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Serialize, Deserialize)]
struct OpenFilesPayload {
    paths: Vec<PathBuf>,
}

pub struct WindowsSingleInstance {
    mutex: HANDLE,
    primary: bool,
}

impl WindowsSingleInstance {
    pub fn new() -> Result<Self> {
        // SAFETY: The mutex name is a static null-terminated UTF-16 string. No security attributes are passed.
        let mutex = unsafe {
            CreateMutexW(None, true, w!("Local\\ShibaPreviewSingleInstanceMutex"))
                .context("Could not create single instance mutex")?
        };
        // SAFETY: GetLastError reads the thread-local last-error value set by CreateMutexW above.
        let already_exists = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
        Ok(Self { mutex, primary: !already_exists })
    }

    pub fn is_primary(&self) -> bool {
        self.primary
    }

    pub fn send_open_files(&self, paths: Vec<PathBuf>) -> Result<()> {
        let hwnd = find_shiba_window().context("Could not find running Shiba window")?;
        let payload = open_files_payload(paths)?;
        let copy_data = copy_data_struct(&payload)?;

        // SAFETY: hwnd is found from EnumWindows. copy_data points to payload, which lives until SendMessageW returns.
        unsafe {
            SendMessageW(
                hwnd,
                WM_COPYDATA,
                Some(WPARAM(0)),
                Some(LPARAM((&copy_data as *const COPYDATASTRUCT) as isize)),
            );
        }
        restore_window(hwnd);
        Ok(())
    }
}

fn open_files_payload(paths: Vec<PathBuf>) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(&OpenFilesPayload { paths })?)
}

fn copy_data_struct(payload: &[u8]) -> Result<COPYDATASTRUCT> {
    Ok(COPYDATASTRUCT {
        dwData: COPYDATA_OPEN_FILES,
        cbData: payload.len().try_into().context("Open file payload is too large")?,
        lpData: payload.as_ptr() as *mut c_void,
    })
}

impl Drop for WindowsSingleInstance {
    fn drop(&mut self) {
        if self.primary {
            // SAFETY: The mutex handle is owned by this guard and was initially acquired by CreateMutexW.
            if let Err(err) = unsafe { ReleaseMutex(self.mutex) } {
                log::error!("Failed to release single instance mutex: {err}");
            }
        }
        // SAFETY: The mutex handle is owned by this guard and is closed once here.
        if let Err(err) = unsafe { CloseHandle(self.mutex) } {
            log::error!("Failed to close single instance mutex: {err}");
        }
    }
}

pub struct WindowRegistration {
    hwnd: HWND,
    handler: *mut WindowMessageHandler,
    previous_wndproc: isize,
}

impl WindowRegistration {
    pub fn new(hwnd: *mut c_void, sender: impl EventSender) -> Result<Self> {
        let hwnd = HWND(hwnd);
        // SAFETY: hwnd is the live Shiba window handle returned by tao. The property name is static.
        unsafe {
            SetPropW(
                hwnd,
                WINDOW_PROPERTY_NAME,
                Some(HANDLE(WINDOW_PROPERTY_VALUE as *mut c_void)),
            )
            .context("Could not register Shiba window")?;
        }

        // SAFETY: hwnd is a live window handle. The returned value is the previous window procedure.
        let previous_wndproc =
            unsafe { SetWindowLongPtrW(hwnd, GWLP_WNDPROC, shiba_window_proc as usize as isize) };
        let handler = Box::into_raw(Box::new(WindowMessageHandler {
            sender: Box::new(sender),
            // SAFETY: SetWindowLongPtrW with GWLP_WNDPROC returns a window procedure pointer.
            previous_wndproc: unsafe { mem::transmute::<isize, WNDPROC>(previous_wndproc) },
        }));
        // SAFETY: hwnd is live and handler remains allocated until WindowRegistration is dropped.
        unsafe {
            SetPropW(hwnd, WINDOW_HANDLER_PROPERTY_NAME, Some(HANDLE(handler.cast())))
                .context("Could not register Shiba window message handler")?;
        }
        Ok(Self { hwnd, handler, previous_wndproc })
    }
}

impl Drop for WindowRegistration {
    fn drop(&mut self) {
        // SAFETY: hwnd is the window registered by WindowRegistration::new.
        unsafe {
            SetWindowLongPtrW(self.hwnd, GWLP_WNDPROC, self.previous_wndproc);
        }
        // SAFETY: The property was set on this window by WindowRegistration::new.
        if let Err(err) = unsafe { RemovePropW(self.hwnd, WINDOW_PROPERTY_NAME) } {
            log::error!("Failed to remove Shiba window property: {err}");
        }
        // SAFETY: The property was set on this window by WindowRegistration::new.
        if let Err(err) = unsafe { RemovePropW(self.hwnd, WINDOW_HANDLER_PROPERTY_NAME) } {
            log::error!("Failed to remove Shiba window message handler property: {err}");
        }
        if !self.handler.is_null() {
            // SAFETY: handler was allocated by Box::into_raw in WindowRegistration::new and is freed once here.
            unsafe {
                drop(Box::from_raw(self.handler));
            }
        }
    }
}

struct WindowMessageHandler {
    sender: Box<dyn EventSender>,
    previous_wndproc: WNDPROC,
}

unsafe extern "system" fn shiba_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    // SAFETY: hwnd is provided by the Windows message dispatcher. The property name is static.
    let handler = unsafe { GetPropW(hwnd, WINDOW_HANDLER_PROPERTY_NAME) };
    let handler = handler.0 as *mut WindowMessageHandler;
    if !handler.is_null() {
        // SAFETY: The pointer is owned by WindowRegistration and valid while the window procedure is installed.
        let handler = unsafe { &*handler };
        if let Some(paths) = open_files_from_copy_data(msg, lparam) {
            log::debug!("Received open-file request from another process: {:?}", paths);
            handler.sender.send(Event::OpenedFiles(paths));
            return LRESULT(1);
        }
        // SAFETY: previous_wndproc is the procedure returned by SetWindowLongPtrW for this hwnd.
        return unsafe { CallWindowProcW(handler.previous_wndproc, hwnd, msg, wparam, lparam) };
    }

    // SAFETY: No previous window procedure is registered yet. Use default processing.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn open_files_from_copy_data(msg: u32, lparam: LPARAM) -> Option<Vec<PathBuf>> {
    if msg != WM_COPYDATA {
        return None;
    }

    let copy_data = lparam.0 as *const COPYDATASTRUCT;
    if copy_data.is_null() {
        log::debug!("WM_COPYDATA had no payload");
        return None;
    }

    // SAFETY: WM_COPYDATA guarantees lParam points to COPYDATASTRUCT for the duration of message handling.
    let copy_data = unsafe { &*copy_data };
    if copy_data.dwData != COPYDATA_OPEN_FILES {
        log::debug!("Ignoring unknown WM_COPYDATA payload: {}", copy_data.dwData);
        return None;
    }

    let len = copy_data.cbData as usize;
    if len == 0 {
        return Some(Vec::new());
    }
    if copy_data.lpData.is_null() {
        log::debug!("WM_COPYDATA open-file payload was null");
        return None;
    }

    // SAFETY: WM_COPYDATA provides cbData bytes at lpData during SendMessageW handling.
    let bytes = unsafe { std::slice::from_raw_parts(copy_data.lpData as *const u8, len) };
    match serde_json::from_slice::<OpenFilesPayload>(bytes) {
        Ok(payload) => Some(payload.paths),
        Err(err) => {
            log::error!("Could not parse open-file payload: {err}");
            None
        }
    }
}

fn find_shiba_window() -> Option<HWND> {
    for _ in 0..FIND_WINDOW_RETRIES {
        if let Some(hwnd) = find_shiba_window_once() {
            return Some(hwnd);
        }
        thread::sleep(FIND_WINDOW_RETRY_INTERVAL);
    }
    None
}

fn find_shiba_window_once() -> Option<HWND> {
    let mut hwnd = HWND::default();
    // SAFETY: The callback only writes to hwnd while EnumWindows is synchronously executing.
    unsafe {
        let _ = EnumWindows(Some(enum_windows), LPARAM((&mut hwnd as *mut HWND) as isize));
    }
    (!hwnd.is_invalid()).then_some(hwnd)
}

unsafe extern "system" fn enum_windows(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // SAFETY: lparam is a valid pointer to HWND provided by find_shiba_window_once.
    let found = unsafe { &mut *(lparam.0 as *mut HWND) };
    // SAFETY: hwnd is supplied by EnumWindows. The property name is static.
    let prop = unsafe { GetPropW(hwnd, WINDOW_PROPERTY_NAME) };
    if prop.0 == WINDOW_PROPERTY_VALUE as *mut c_void {
        *found = hwnd;
        return false.into();
    }
    true.into()
}

fn restore_window(hwnd: HWND) {
    // SAFETY: hwnd is a top-level window found by EnumWindows.
    unsafe {
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }
        let _ = SetForegroundWindow(hwnd);
    }
}

pub struct WindowsConsole {
    attached: bool,
}

impl WindowsConsole {
    pub fn attach() -> Self {
        // SAFETY: Using Windows C API is always unsafe. I confirmed the usage in official document.
        // https://learn.microsoft.com/en-us/windows/console/attachconsole
        let attached = match unsafe { AttachConsole(ATTACH_PARENT_PROCESS) } {
            Ok(()) => true,
            Err(err) => {
                log::error!("Failed to attach to console: {err}");
                false
            }
        };
        Self { attached }
    }
}

impl Drop for WindowsConsole {
    fn drop(&mut self) {
        if self.attached {
            // SAFETY: Using Windows C API is always unsafe. I confirmed the usage in official document.
            // https://learn.microsoft.com/en-us/windows/console/freeconsole
            if let Err(err) = unsafe { FreeConsole() } {
                log::error!("Failed to free console: {err}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::UI::WindowsAndMessaging::WM_USER;

    fn lparam_from_copy_data(copy_data: &COPYDATASTRUCT) -> LPARAM {
        LPARAM((copy_data as *const COPYDATASTRUCT) as isize)
    }

    #[test]
    fn open_files_from_copy_data_restores_paths() {
        let paths = vec![PathBuf::from(r"C:\docs\A.md"), PathBuf::from(r"C:\docs\B.md")];
        let payload = open_files_payload(paths.clone()).unwrap();
        let copy_data = copy_data_struct(&payload).unwrap();

        let restored = open_files_from_copy_data(WM_COPYDATA, lparam_from_copy_data(&copy_data));

        assert_eq!(restored, Some(paths));
    }

    #[test]
    fn open_files_from_copy_data_ignores_other_message() {
        let payload = open_files_payload(vec![PathBuf::from(r"C:\docs\A.md")]).unwrap();
        let copy_data = copy_data_struct(&payload).unwrap();

        let restored = open_files_from_copy_data(WM_USER, lparam_from_copy_data(&copy_data));

        assert_eq!(restored, None);
    }

    #[test]
    fn open_files_from_copy_data_ignores_unknown_payload_kind() {
        let payload = open_files_payload(vec![PathBuf::from(r"C:\docs\A.md")]).unwrap();
        let mut copy_data = copy_data_struct(&payload).unwrap();
        copy_data.dwData = COPYDATA_OPEN_FILES + 1;

        let restored = open_files_from_copy_data(WM_COPYDATA, lparam_from_copy_data(&copy_data));

        assert_eq!(restored, None);
    }

    #[test]
    fn open_files_from_copy_data_ignores_invalid_payload() {
        let mut payload = b"not json".to_vec();
        let copy_data = COPYDATASTRUCT {
            dwData: COPYDATA_OPEN_FILES,
            cbData: payload.len().try_into().unwrap(),
            lpData: payload.as_mut_ptr().cast(),
        };

        let restored = open_files_from_copy_data(WM_COPYDATA, lparam_from_copy_data(&copy_data));

        assert_eq!(restored, None);
    }
}
