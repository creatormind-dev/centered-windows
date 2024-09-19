use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, RECT, TRUE},
    Graphics::Gdi::{
        GetMonitorInfoW,
        HMONITOR,
        MONITOR_DEFAULTTONEAREST,
        MonitorFromWindow,
        MONITORINFO
    },
    UI::WindowsAndMessaging::{
        EnumWindows,
        GetWindowRect,
        GetWindowTextW,
        GWL_EXSTYLE,
        GWL_STYLE,
        GetWindowLongPtrW,
        IsIconic,
        IsWindow,
        IsWindowVisible,
        IsZoomed,
        SET_WINDOW_POS_FLAGS,
        SetWindowPos,
        SWP_NOACTIVATE,
        SWP_NOSIZE,
        SWP_NOZORDER,
        WS_CHILD,
        WS_EX_APPWINDOW,
        WS_EX_TOOLWINDOW,
        WS_POPUP,
    }
};


/**
Returns a list of the currently active and visible application windows in the operating system.
 */
pub fn get_windows() -> Vec<WindowInfo> {
    let mut windows: Vec<WindowInfo> = vec![];

    if cfg!(target_os = "windows") {
        unsafe {
            let _ = EnumWindows(Some(window_enum_proc), LPARAM(&mut windows as *mut _ as isize));
        }
    }

    windows
}


/**
Represents information for an active application window in the operating system.
 */
#[derive(Debug)]
pub struct WindowInfo {
    title: String,

    position: LogicalPosition<i32>,
    size: LogicalSize<u32>,

    monitor: MonitorInfo,

    #[cfg(target_os = "windows")]
    handle: HWND,
}


impl WindowInfo {
    #[cfg(target_os = "windows")]
    unsafe fn from_win32(hwnd: HWND) -> anyhow::Result<Self> {
        use anyhow::bail;

        if hwnd.is_invalid() {
            bail!("The window handle is not valid: {:?}", hwnd);
        }

        if IsWindow(hwnd).as_bool() == false {
            bail!("The handle associated to the process is not a window: {:?}", hwnd);
        }

        let ex_ws_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;

        // Check if the window isn't a toolbar or other type of widget.
        if (ex_ws_style & WS_EX_TOOLWINDOW.0) != 0 && (ex_ws_style & WS_EX_APPWINDOW.0) == 0 {
            bail!("The window process is presumed to be a widget, toolbar or other invalid process: {:?}", hwnd);
        }

        let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;

        if (style & WS_CHILD.0) != 0 || (style & WS_POPUP.0) != 0 {
            bail!("The window is a child process, a popup or other invalid form: {:?}", hwnd);
        }

        // A window should be visible, otherwise it could be an overlay or hidden process.
        if IsWindowVisible(hwnd).as_bool() == false {
            bail!("The window is not visible: {:?}", hwnd);
        }
        
        if IsIconic(hwnd).as_bool() == true || IsZoomed(hwnd).as_bool() == true {
            bail!("The window is not visible in the display area: {:?}", hwnd);
        }

        let mut rect = RECT::default();

        if GetWindowRect(hwnd, &mut rect).is_err() {
            bail!("The window has no bounding rect: {:?}", hwnd);
        }

        let monitor_info = MonitorInfo::from_window(hwnd)?;

        let mut buffer = [0u16; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);

        // Same as before. Most windows without a title are other type of processes.
        if length == 0 {
            bail!("The window does not have a valid title (required): {:?}", hwnd);
        }

        Ok(Self {
            handle: hwnd,
            monitor: monitor_info,
            position: LogicalPosition::new(rect.left, rect.top),
            size: LogicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
            title: String::from_utf16_lossy(&buffer[..length as usize]),
        })
    }
}

impl WindowInfo {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let left = self.position.x;
        let top = self.position.y;
        let right = self.position.x + (self.size.width as i32);
        let bottom = self.position.y + (self.size.height as i32);

        x >= left && x <= right && y >= top && y <= bottom
    }

    pub fn center(&self, use_work_area: bool) -> anyhow::Result<()> {
        let monitor_position = if use_work_area { self.monitor.work_position } else { self.monitor.position };
        let monitor_size = if use_work_area { self.monitor.work_size } else { self.monitor.size };
        let x = monitor_position.x + ((monitor_size.width / 2) as i32) - ((self.size.width / 2) as i32);
        let y = monitor_position.y + ((monitor_size.height / 2) as i32) - ((self.size.height / 2) as i32);

        // TODO: Figure out a way to save the window's state so it doesn't revert back when restoring.

        if cfg!(target_os = "windows") {
            unsafe {
                SetWindowPos(
                    self.handle,
                    None,
                    x,
                    y,
                    self.size.width as i32,
                    self.size.height as i32,
                    SET_WINDOW_POS_FLAGS(SWP_NOSIZE.0 | SWP_NOZORDER.0 | SWP_NOACTIVATE.0),
                )?;
            }
        }

        Ok(())
    }
}


#[cfg(target_os = "windows")]
unsafe extern "system" fn window_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let window = match WindowInfo::from_win32(hwnd) {
        Ok(w) => w,
        Err(e) => {
            log::log!(log::Level::Debug, "{}", e);
            return TRUE;
        }
    };

    let window_list = &mut *(lparam.0 as *mut Vec<WindowInfo>);

    window_list.push(window);

    TRUE
}


/**
Represents information for a display screen (monitor) detected in the system.
 */
#[derive(Debug)]
pub struct MonitorInfo {
    position: LogicalPosition<i32>,
    size: LogicalSize<u32>,
    work_position: LogicalPosition<i32>,
    work_size: LogicalSize<u32>,

    #[cfg(target_os = "windows")]
    handle: HMONITOR,
}


// Implementation for static methods.
impl MonitorInfo {
    #[cfg(target_os = "windows")]
    pub unsafe fn from_window(hwnd: HWND) -> anyhow::Result<Self> {
        use anyhow::bail;

        let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

        if hmonitor.is_invalid() {
            bail!("The monitor handle is not valid: {:?}", hmonitor);
        }

        let mut monitor_info = MONITORINFO::default();

        monitor_info.cbSize = size_of::<MONITORINFO>() as u32;

        if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() == false {
            bail!("There is no monitor information associated with the window: {:?}", hwnd);
        }

        let rect = monitor_info.rcMonitor;
        let work_rect = monitor_info.rcWork;

        Ok(Self {
            handle: hmonitor,
            position: LogicalPosition::new(rect.left, rect.top),
            size: LogicalSize::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32
            ),
            work_position: LogicalPosition::new(work_rect.left, work_rect.top),
            work_size: LogicalSize::new(
                (work_rect.right - work_rect.left) as u32,
                (work_rect.bottom - work_rect.top) as u32
            ),
        })
    }
}

// Implementation for instance methods.
impl MonitorInfo {
    pub fn position(&self) -> &LogicalPosition<i32> {
        &self.position
    }

    pub fn size(&self) -> &LogicalSize<u32> {
        &self.size
    }

    pub fn work_position(&self) -> &LogicalPosition<i32> {
        &self.work_position
    }

    pub fn work_size(&self) -> &LogicalSize<u32> {
        &self.work_size
    }
}
