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
		WS_CHILD,
		WS_EX_APPWINDOW,
		WS_EX_TOOLWINDOW,
		WS_POPUP,
	}
};


/**
Represents information for an active application window in the operating system.
 */
pub struct AppWindowInfo {
	pub title: String,

	pub position: LogicalPosition<i32>,
	pub size: LogicalSize<u32>,

	#[cfg(target_os = "windows")]
	hwnd: HWND,
}


impl AppWindowInfo {
	/**
	Returns a tuple containing the coordinates of the window's upper-left and lower-right corners (left, top, right, bottom).
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

	let mut buffer = [0u16; 512];
	let length = GetWindowTextW(hwnd, &mut buffer);

	// Same as before. Most windows without a title are other type of processes.
	if length == 0 {
		return Err(());
	}

	Ok(AppWindowInfo {
		hwnd,
		position: LogicalPosition::new(rect.left, rect.top),
		size: LogicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
		title: String::from_utf16_lossy(&buffer[..length as usize]),
	})
}
