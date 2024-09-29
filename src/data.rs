use std::fmt;

use std::error::Error;
use std::fmt::Formatter;
use winit::dpi::{PhysicalPosition, PhysicalSize};

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
        GetWindowTextLengthW,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidData => "invalid data"
        })
    }
}

impl Error for GenericError {}


/**
Represents the bounding rectangle of a quad.

Not to be confused with the Windows API RECT struct.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    /**
    Constructs a new Rect from a position and a size.
    */
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Self {
            left: x,
            top: y,
            right: x + w as i32,
            bottom: y + h as i32,
        }
    }
    
    /**
    Given a `base` rect, a new rect is created with a transformation applied to the provided `rect`
    so it becomes relative to the `base` rect.
    */
    pub fn adjust(rect: Rect, base: Rect) -> Rect {
        let left = rect.left - base.left;
        let top = rect.top - base.top;
        let right = left + (rect.right - rect.left);
        let bottom = top + (rect.bottom - rect.top);
        
        Rect {
            left,
            top,
            right,
            bottom
        }
    }
    
    /**
    Checks whether the given coordinate is contained by the bounding rect.
    */
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
    
    /**
    Returns a tuple containing the four attributes of a rect: (left, top, right, bottom) to allow
    for data manipulation of the bounding rect.
    */
    pub fn raw(&self) -> (i32, i32, i32, i32) {
        (self.left, self.top, self.right, self.bottom)
    }
}


/**
Represents information for an active application window in the operating system.
 */
#[derive(Debug, Copy, Clone)]
pub struct WindowInfo {
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,

    monitor: MonitorInfo,

    #[cfg(target_os = "windows")]
    handle: HWND,
}

#[cfg(target_os = "windows")]
impl WindowInfo {
    unsafe fn build(hwnd: HWND) -> Result<Self, Box<dyn Error>> {
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

        GetWindowRect(hwnd, &mut rect)?;

        let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let monitor_info = MonitorInfo::build(hmonitor)?;
        
        let length = GetWindowTextLengthW(hwnd);

        // Same as before. Most windows without a title are other type of processes.
        if length == 0 {
            return Err(GenericError::InvalidData.into());
        }

        Ok(Self {
            handle: hwnd,
            monitor: monitor_info,
            position: PhysicalPosition::new(rect.left, rect.top),
            size: PhysicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
        })
    }
}

impl WindowInfo {
    pub fn center(&self) -> Result<(), Box<dyn Error>> {
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
    
    pub fn position(&self) -> &PhysicalPosition<i32> {
        &self.position
    }
    
    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
    
    pub fn rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.size.width,
            self.size.height,
        )
    }
}


/**
Represents information for a display screen (monitor) detected in the system.
 */
#[derive(Debug, Copy, Clone)]
pub struct MonitorInfo {
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,

    #[cfg(target_os = "windows")]
    handle: HMONITOR,
}

#[cfg(target_os = "windows")]
impl MonitorInfo {
    pub unsafe fn build(handle: HMONITOR) -> Result<Self, Box<dyn Error>> {
        if handle.is_invalid() {
            return Err(GenericError::InvalidData.into());
        }

        let mut monitor_info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        
        if GetMonitorInfoW(handle, &mut monitor_info).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }

        let rect = monitor_info.rcMonitor;

        Ok(Self {
            handle,
            position: PhysicalPosition::new(rect.left, rect.top),
            size: PhysicalSize::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32
            ),
        })
    }
}


/**
Returns a list of the currently active and visible application windows in the operating system.
 */
pub fn get_windows() -> Result<Vec<WindowInfo>, Box<dyn Error>> {
    let mut windows: Vec<WindowInfo> = Vec::new();

    #[cfg(target_os = "windows")]
    unsafe {
        EnumWindows(Some(window_enum_proc), LPARAM(&mut windows as *mut _ as isize))?;
    }

    Ok(windows)
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn window_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let window = match WindowInfo::build(hwnd) {
        Ok(w) => w,
        Err(_) => {
            // Continues enumeration of windows, skipping the invalid window.
            return TRUE;
        }
    };

    let window_list = &mut *(lparam.0 as *mut Vec<WindowInfo>);

    window_list.push(window);

    TRUE
}
