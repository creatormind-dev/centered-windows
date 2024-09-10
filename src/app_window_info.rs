use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;


pub struct AppWindowInfo {
	pub title: String,

	pub position: LogicalPosition<i32>,
	pub size: LogicalSize<u32>,

	#[cfg(target_os = "windows")]
	hwnd: HWND,
}
