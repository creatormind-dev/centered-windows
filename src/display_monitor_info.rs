use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(target_os = "windows")]
use windows::Win32::{
	Foundation::HWND,
	Graphics::Gdi::{
		GetMonitorInfoW,
		HMONITOR,
		MONITOR_DEFAULTTONEAREST,
		MonitorFromWindow,
		MONITORINFO
	},
};


#[derive(Debug)]
pub struct DisplayMonitorInfo {
	pub position: LogicalPosition<i32>,
	pub size: LogicalSize<u32>,
	pub work_position: LogicalPosition<i32>,
	pub work_size: LogicalSize<u32>,

	#[cfg(target_os = "windows")]
	handle: HMONITOR,
}


impl DisplayMonitorInfo {
	#[cfg(target_os = "windows")]
	pub unsafe fn from_window(hwnd: HWND) -> Result<Self, ()> {
		let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

		if hmonitor.is_invalid() {
			return Err(());
		}

		let mut monitor_info = MONITORINFO::default();

		monitor_info.cbSize = size_of::<MONITORINFO>() as u32;

		if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() == false {
			return Err(());
		}

		let rect = monitor_info.rcMonitor;
		let work_rect = monitor_info.rcWork;

		Ok(DisplayMonitorInfo {
			handle: hmonitor,
			position: LogicalPosition::new(rect.left, rect.top),
			size: LogicalSize::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32),
			work_position: LogicalPosition::new(work_rect.left, work_rect.top),
			work_size: LogicalSize::new((work_rect.right - work_rect.left) as u32, (work_rect.bottom - work_rect.top) as u32),
		})
	}

	/**
	Returns a tuple containing the coordinates of the monitor's bounding rectangle (left, top, right, bottom).
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

	/**
	Returns a tuple containing the coordinates of the monitor's working area bounding rectangle (left, top, right, bottom).
	 */
	pub fn work_rect(&self) -> (i32, i32, i32, i32) {
		let position = self.work_position;
		let size = self.work_size;

		(
			position.x,
			position.y,
			position.x + (size.width as i32),
			position.y + (size.height as i32),
		)
	}
}
