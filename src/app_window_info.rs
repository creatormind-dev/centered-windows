use crate::display_monitor_info::DisplayMonitorInfo;
use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(target_os = "windows")]
use windows::Win32::{
	Foundation::{BOOL, HWND, LPARAM, RECT, TRUE},
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
Represents information for an active application window in the operating system.
 */
#[derive(Debug)]
pub struct AppWindowInfo {
	pub title: String,

	pub position: LogicalPosition<i32>,
	pub size: LogicalSize<u32>,

	monitor: DisplayMonitorInfo,

	#[cfg(target_os = "windows")]
	handle: HWND,
}


impl AppWindowInfo {
	/**
	Returns a tuple containing the coordinates of the window's bounding rectangle (left, top, right, bottom).
	 */
	pub fn rect(&self) -> (i32, i32, i32, i32) {
		let position = self.position;
		let size = self.size;

		(
			position.x,
			position.y,
			position.x + (size.width as i32),
			position.y + (size.height as i32),
		)
	}

	pub fn center(&self, use_work_area: bool) -> Result<(), String> {
		let monitor_position = if use_work_area { self.monitor.work_position } else { self.monitor.position };
		let monitor_size = if use_work_area { self.monitor.work_size } else { self.monitor.size };
		let x = monitor_position.x + ((monitor_size.width / 2) as i32) - ((self.size.width / 2) as i32);
		let y = monitor_position.y + ((monitor_size.height / 2) as i32) - ((self.size.height / 2) as i32);

		let mut result: Result<(), String> = Err("Unknown error.".to_string());

		// TODO: Figure out a way to save the window's state so it doesn't revert back when restoring.
		
		if cfg!(target_os = "windows") {
			unsafe {
				result = SetWindowPos(
					self.handle,
					None,
					x,
					y,
					self.size.width as i32,
					self.size.height as i32,
					SET_WINDOW_POS_FLAGS(SWP_NOSIZE.0 | SWP_NOZORDER.0 | SWP_NOACTIVATE.0),
				).map_err(|err| { err.message() });
			}
		}

		result
	}
}


/**
Returns a list of the currently active and visible application windows in the operating system.
 */
pub fn get_windows() -> Vec<AppWindowInfo> {
	let mut windows: Vec<AppWindowInfo> = vec![];

	if cfg!(target_os = "windows") {
		unsafe {
			let _ = EnumWindows(Some(window_enum_proc), LPARAM(&mut windows as *mut _ as isize));
		}
	}

	return windows;
}


#[cfg(target_os = "windows")]
unsafe extern "system" fn window_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
	let window = match get_win32_window_info(hwnd) {
		Ok(w) => w,
		Err(_) => return TRUE,
	};

	let window_list = &mut *(lparam.0 as *mut Vec<AppWindowInfo>);

	window_list.push(window);

	TRUE
}

#[cfg(target_os = "windows")]
unsafe fn get_win32_window_info(hwnd: HWND) -> Result<AppWindowInfo, ()> {

	// Check if the window handle is a valid pointer.
	if hwnd.is_invalid() {
		return Err(());
	}

	// The handle should belong to a window.
	if IsWindow(hwnd).as_bool() == false {
		return Err(());
	}

	let ex_ws_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;

	// Check if the window isn't a toolbar or other type of widget.
	if (ex_ws_style & WS_EX_TOOLWINDOW.0) != 0 && (ex_ws_style & WS_EX_APPWINDOW.0) == 0 {
		return Err(());
	}

	let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;

	if (style & WS_CHILD.0) != 0 || (style & WS_POPUP.0) != 0 {
		return Err(());
	}

	// A window should be visible, otherwise it could be an overlay or hidden process.
	if IsWindowVisible(hwnd).as_bool() == false {
		return Err(());
	}

	// Minimized windows should be skipped to avoid causing problems with the system.
	// (Plus they can't be rendered in the centering overlay)
	// Same thing for maximized windows.
	if IsIconic(hwnd).as_bool() == true || IsZoomed(hwnd).as_bool() == true {
		return Err(());
	}

	let mut rect = RECT::default();

	if GetWindowRect(hwnd, &mut rect).is_err() {
		return Err(());
	}

	let monitor_info = match DisplayMonitorInfo::from_window(hwnd) {
		Ok(m) => m,
		Err(_) => return Err(()),
	};

	let mut buffer = [0u16; 512];
	let length = GetWindowTextW(hwnd, &mut buffer);

	// Same as before. Most windows without a title are other type of processes.
	if length == 0 {
		return Err(());
	}

	Ok(AppWindowInfo {
		handle: hwnd,
		monitor: monitor_info,
		position: LogicalPosition::new(rect.left, rect.top),
		size: LogicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
		title: String::from_utf16_lossy(&buffer[..length as usize]),
	})
}
