use std::fmt;

use std::error::Error;
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


#[derive(Debug, Clone)]
enum GenericError {
    InvalidData
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidData => "Invalid data"
        })
    }
}

impl Error for GenericError {}


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
    unsafe fn from_win32(hwnd: HWND) -> Result<Self, Box<dyn Error>> {
        if hwnd.is_invalid() {
            return Err(GenericError::InvalidData.into());
        }

        if IsWindow(hwnd).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }

        let ex_ws_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;

        // Check if the window isn't a toolbar or other type of widget.
        if (ex_ws_style & WS_EX_TOOLWINDOW.0) != 0 && (ex_ws_style & WS_EX_APPWINDOW.0) == 0 {
            return Err(GenericError::InvalidData.into());
        }

        let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;

        if (style & WS_CHILD.0) != 0 || (style & WS_POPUP.0) != 0 {
            return Err(GenericError::InvalidData.into());
        }

        // A window should be visible, otherwise it could be an overlay or hidden process.
        if IsWindowVisible(hwnd).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }
        
        if IsIconic(hwnd).as_bool() == true || IsZoomed(hwnd).as_bool() == true {
            return Err(GenericError::InvalidData.into());
        }

        let mut rect = RECT::default();

        if GetWindowRect(hwnd, &mut rect).is_err() {
            return Err(GenericError::InvalidData.into());
        }

        let monitor_info = MonitorInfo::from_win32_window(hwnd)?;

        let mut buffer = [0u16; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);

        // Same as before. Most windows without a title are other type of processes.
        if length == 0 {
            return Err(GenericError::InvalidData.into());
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

    pub fn center(&self, use_work_area: bool) -> Result<(), Box<dyn Error>> {
        let monitor_position = self.monitor.position;
        let monitor_size = self.monitor.size;
        let x = monitor_position.x + ((monitor_size.width / 2) as i32) - ((self.size.width / 2) as i32);
        let y = monitor_position.y + ((monitor_size.height / 2) as i32) - ((self.size.height / 2) as i32);
        
        #[cfg(target_os = "windows")]
        unsafe {
            // TODO: Figure out a way to save the window's state so it doesn't revert back when restoring.
            
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

        Ok(())
    }
}


#[cfg(target_os = "windows")]
unsafe extern "system" fn window_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let window = match WindowInfo::from_win32(hwnd) {
        Ok(w) => w,
        Err(e) => {
            log::debug!("{e}");
            
            // Continues enumeration of windows without exiting.
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

    #[cfg(target_os = "windows")]
    handle: HMONITOR,
}


// Implementation for static methods.
impl MonitorInfo {
    #[cfg(target_os = "windows")]
    pub unsafe fn from_win32_window(hwnd: HWND) -> Result<Self, Box<dyn Error>> {
        let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

        if hmonitor.is_invalid() {
            return Err(GenericError::InvalidData.into());
        }

        let mut monitor_info = MONITORINFO::default();

        monitor_info.cbSize = size_of::<MONITORINFO>() as u32;

        if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }

        let rect = monitor_info.rcMonitor;

        Ok(Self {
            handle: hmonitor,
            position: LogicalPosition::new(rect.left, rect.top),
            size: LogicalSize::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32
            ),
        })
    }
}
