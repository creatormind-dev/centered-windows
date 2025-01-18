mod overlay;
pub use overlay::*;

use std::{fmt, fs};

use serde::Deserialize;
use std::error::Error;

use winit::dpi::{PhysicalPosition, PhysicalSize};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, RECT, TRUE},
    Graphics::{
        Dwm::{
            DwmGetWindowAttribute,
            DWMWA_EXTENDED_FRAME_BOUNDS,
        },
        Gdi::{
            GetMonitorInfoW,
            HMONITOR,
            MONITOR_DEFAULTTONEAREST,
            MonitorFromWindow,
            MONITORINFO
        }
    },
    UI::WindowsAndMessaging::{

        EnumWindows,
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


/// The location of the preferences file, relative to the program's directory.
const PREFERENCES_FILE: &str = "config.yml";

/// The global user-defined preferences. Accessible through the [`Preferences::get`] method.
static mut PREFERENCES: Option<Preferences> = None;


/// Initializes the logger.
pub fn init_logger() -> Result<(), flexi_logger::FlexiLoggerError> {
    let log_spec = flexi_logger::LogSpecBuilder::new()
        .default(log::LevelFilter::Warn)
        .module("centered_windows", log::LevelFilter::Debug)
        .build();

    let file_spec = flexi_logger::FileSpec::default()
        .directory("log")
        .use_timestamp(false);

    flexi_logger::Logger::with(log_spec)
        .log_to_file(file_spec)
        .format(|writer, now, record| {
            writer.write_fmt(format_args!(
                "[{} {}]: {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),      // Timestamp.
                record.level(),                             // Log Level.
                &record.args(),                             // Message.
            ))
        })
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::TimestampsDirect,
            flexi_logger::Cleanup::KeepLogFiles(7),
        )
        .start()?;

    Ok(())
}


/// The GenericError enum is a common way to express that a process for window or monitor handling
/// didn't go well.
///
/// This is more so just an indicator of an error without any actual weight.
#[derive(Debug, Clone)]
enum GenericError {
    InvalidData
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generic Data Error: {}", match self {
            Self::InvalidData => "invalid data",
        })
    }
}

impl Error for GenericError {}



/// Represents the bounding rectangle of a quad.
///
/// Not to be confused with the Windows API RECT struct.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// Constructs a new Rect from a position and a size.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Self {
            left: x,
            top: y,
            right: x + w as i32,
            bottom: y + h as i32,
        }
    }
    
    /// Given a `base` rect, a new rect is created with a transformation applied to the provided `rect`
    /// so it becomes relative to the `base` rect.
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
    
    /// Checks whether the given coordinate is contained by the bounding rect.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
    
    /// Returns a tuple containing the four attributes of a rect: (left, top, right, bottom) to allow
    /// for data manipulation of the bounding rect.
    pub fn raw(&self) -> (i32, i32, i32, i32) {
        (self.left, self.top, self.right, self.bottom)
    }
}


/// Represents information for an active application window in the operating system.
#[derive(Debug)]
pub struct WindowInfo {
    title: String,
    
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,

    monitor: MonitorInfo,

    #[cfg(target_os = "windows")]
    handle: HWND,
}

impl fmt::Display for WindowInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} ({}, {}) [{}x{}] {}",
            self.handle,
            self.position.x,
            self.position.y,
            self.size.width,
            self.size.height,
            self.title
        )
    }
}

// Windows OS specific implementations.
#[cfg(target_os = "windows")]
impl WindowInfo {
    /// Constructs a new window from a given handle.
    /// Multiple filters are applied to avoid returning invisible windows or os-specific processes.
    unsafe fn build(hwnd: HWND) -> Result<Self, Box<dyn Error>> {
        if hwnd.is_invalid() {
            return Err(GenericError::InvalidData.into());
        }

        if IsWindow(hwnd).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }

        let mut buffer = [0u16; 1024];
        let length = GetWindowTextW(hwnd, &mut buffer);

        // Most windows without a title are other type of processes.
        if length == 0 {
            return Err(GenericError::InvalidData.into());
        }

        let preferences = Preferences::get();

        let ex_ws_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;

        // Check if the window isn't a toolbar or other type of widget.
        if (ex_ws_style & WS_EX_TOOLWINDOW.0) != 0 && (ex_ws_style & WS_EX_APPWINDOW.0) == 0 {
            return Err(GenericError::InvalidData.into());
        }

        let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;

        // Check if the window has a parent window.
        if (style & WS_CHILD.0) != 0 && preferences.allow_child_ws == false {
            return Err(GenericError::InvalidData.into());
        }

        // Check if the window can be considered a popup.
        if (style & WS_POPUP.0) != 0 && preferences.allow_popup_ws == false {
            return Err(GenericError::InvalidData.into());
        }

        // A window should be visible, otherwise it could be an overlay or hidden process.
        if IsWindowVisible(hwnd).as_bool() == false {
            return Err(GenericError::InvalidData.into());
        }

        let mut rect = RECT::default();

        // The DwmGetWindowAttribute function is needed to obtain the RECT of the window without
        // the drop shadow that is present ever since Vista. You will be missed, GetWindowRect...
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut _,
            size_of::<RECT>() as u32,
        )?;

        let monitor_handle = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let monitor_info = MonitorInfo::build(monitor_handle)?;

        Ok(Self {
            handle: hwnd,
            monitor: monitor_info,
            title: String::from_utf16_lossy(&buffer[..length as usize]),
            position: PhysicalPosition::new(rect.left, rect.top),
            size: PhysicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
        })
    }
    
    unsafe fn is_maximized(&self) -> bool {
        IsZoomed(self.handle).as_bool()
    }
    
    unsafe fn is_minimized(&self) -> bool {
        IsIconic(self.handle).as_bool()
    }
}

impl WindowInfo {
    /// Check if the window is centered.
    pub fn is_centered(&self) -> bool {
        let monitor_position = self.monitor.position;
        let monitor_size = self.monitor.size;
        let x = monitor_position.x + ((monitor_size.width / 2) as i32) - ((self.size.width / 2) as i32);
        let y = monitor_position.y + ((monitor_size.height / 2) as i32) - ((self.size.height / 2) as i32);
        
        self.position.x == x && self.position.y == y
    }
    
    /// Tries to position the window to the center of it's corresponding monitor.
    pub fn center(&self) -> Result<(), Box<dyn Error>> {
        let monitor_position = self.monitor.position;
        let monitor_size = self.monitor.size;
        let x = monitor_position.x + ((monitor_size.width / 2) as i32) - ((self.size.width / 2) as i32);
        let y = monitor_position.y + ((monitor_size.height / 2) as i32) - ((self.size.height / 2) as i32);
        
        #[cfg(target_os = "windows")]
        unsafe {
            SetWindowPos(
                self.handle,
                None,
                x,
                y,
                0, // This gets ignored because of the no size flag.
                0, // This gets ignored because of the no size flag.
                SET_WINDOW_POS_FLAGS(SWP_NOSIZE.0 | SWP_NOZORDER.0 | SWP_NOACTIVATE.0),
            )?;
        }
        
        log::debug!("Repositioned window to the center: {self}");

        Ok(())
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


/// Represents information for a display screen (monitor) detected in the system.
#[derive(Debug, Copy, Clone)]
pub struct MonitorInfo {
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
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

        let preferences = Preferences::get();

        let rect = if preferences.use_absolute_area {
            monitor_info.rcMonitor
        } else {
            monitor_info.rcWork
        };

        Ok(Self {
            position: PhysicalPosition::new(rect.left, rect.top),
            size: PhysicalSize::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32
            ),
        })
    }
}


/// Returns a list of the currently active and visible application windows in the operating system.
pub fn get_windows() -> Result<Vec<WindowInfo>, Box<dyn Error>> {
    let mut windows: Vec<WindowInfo> = Vec::new();

    #[cfg(target_os = "windows")]
    unsafe {
        EnumWindows(
            Some(window_enum_proc),
            LPARAM(&mut windows as *mut _ as isize) // Casting to a mutable pointer.
        )?;
    }

    if windows.is_empty() {
        log::info!("No windows found.");
    }

    Ok(windows)
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn window_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let window = match WindowInfo::build(hwnd) {
        Ok(w) => w,
        Err(_) => {
            // Continues enumeration of windows, skipping the invalid window.
            // Returning false would instead end the enumeration.
            return TRUE;
        }
    };
    
    // There's no point in repositioning these windows.
    if window.is_maximized() || window.is_minimized() || window.is_centered() {
        return TRUE;
    }

    log::debug!("Collected window: {window}");

    // Casting of LPARAM pointer to a Vec.
    let window_list = &mut *(lparam.0 as *mut Vec<WindowInfo>);

    window_list.push(window);

    TRUE
}


/// Represents the user preferences, contained by the [`PREFERENCES_FILE`].
#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub allow_child_ws: bool,
    pub allow_popup_ws: bool,
    pub overlay_color: u32,
    pub overlay_opacity: f64,
    pub use_absolute_area: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            allow_child_ws: true,
            allow_popup_ws: false,
            overlay_color: 0,
            overlay_opacity: 0.6,
            use_absolute_area: false,
        }
    }
}

impl Preferences {
    /// Gets the global user-defined preferences. If the [`PREFERENCES`] variable hasn't been
    /// initialized, it tries to read them from the [`PREFERENCES_FILE`], otherwise it just returns
    /// the default preferences as defined by the program.
    #[allow(static_mut_refs)]
    pub fn get() -> &'static Self {
        // Global preferences has a value, return it.
        if unsafe { PREFERENCES.is_some() } {
            return unsafe { PREFERENCES.as_ref().unwrap() }
        }

        // Try to parse the preferences.
        let preferences = match Preferences::try_from_file(PREFERENCES_FILE) {
            Ok(prefs) => {
                log::info!("Loaded and parsed user preferences from \"{}\"", PREFERENCES_FILE);
                prefs
            }
    
            Err(error) => {
                log::error!("Failed to load user preferences from \"{}\": {}", PREFERENCES_FILE, error);
                Preferences::default()
            }
        };
    
        unsafe { PREFERENCES = Some(preferences) }
        unsafe { PREFERENCES.as_ref().unwrap() }
    }

    /// Tries to read the preferences from the given file path.
    /// Returns the preferences if successfull.
    pub fn try_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = fs::File::open(path)?;
        let data: Preferences = serde_yaml::from_reader(file)?;

        Ok(data)
    }
}
