#include "AppWindow.h"


BOOL GetAppWindow (HWND hWnd, AppWindow* window) {
	if (hWnd == NULL || window == NULL)
		return FALSE;

	window->handle = hWnd;

	HMONITOR hMonitor = MonitorFromWindow(hWnd, MONITOR_DEFAULTTONEAREST);

	if (GetDisplayMonitor(hMonitor, &window->monitor) == FALSE)
		return FALSE;

	RECT rect;

	if (GetWindowRect(hWnd, &rect) == FALSE)
		return FALSE;

	window->width = rect.right - rect.left;
	window->height = rect.bottom - rect.top;
	window->x = rect.left;
	window->y = rect.top;

	GetWindowThreadProcessId(hWnd, &window->processId);
	GetWindowTextW(hWnd, window->title, MAX_TITLE_LENGTH);

	return TRUE;
}

// Verifies that the given Windows process falls under the AppWindow definition.
BOOL IsValidAppWindow (const HWND hWnd) {
	if (hWnd == NULL)
		return FALSE;

	if (!IsWindow(hWnd))
		return FALSE;

	if (!IsWindowVisible(hWnd))
		return FALSE;

	if (GetWindowTextLengthW(hWnd) == 0)
		return FALSE;

	if (GetWindowLongW(hWnd, GWL_STYLE) & WS_EX_TOOLWINDOW)
		return FALSE;

	return TRUE;
}

// Wrapper for the Windows API function.
BOOL IsWindowMinimized (const HWND hWnd) {
	if (hWnd == NULL)
		return FALSE;

	return IsIconic(hWnd);
}

// Wrapper for the Windows API function.
BOOL IsWindowMaximized (const HWND hWnd) {
	if (hWnd == NULL)
		return FALSE;

	return IsZoomed(hWnd);
}

BOOL IsWindowFullScreen (const AppWindow* window) {
	const DisplayMonitor* monitor = &window->monitor;

	if (window->x == monitor->x &&
		window->y == monitor->y &&
		window->width == monitor->width &&
		window->height == monitor->height)
		return TRUE;

	return FALSE;
}

BOOL IsWindowOutOfBounds (const AppWindow* window, unsigned int flags) {
	const DisplayMonitor* monitor = &window->monitor;

	if ((flags & APPWND_OOB_POSITION) &&
		(window->x > (monitor->x + monitor->width) ||
		window->y > (monitor->y + monitor->height) ||
		(window->x + window->width) < monitor->x ||
		(window->y + window->height) < monitor->y))
		return TRUE;

	if ((flags & APPWND_OOB_SIZE) &&
		(window->width > monitor->width ||
		window->height > monitor->height))
		return TRUE;

	return FALSE;
}

// Modifies fullPath with the full path to the app executable, if successful.
BOOL GetAppWindowExecutable (const AppWindow* window, wchar_t fullPath[MAX_PATH]) {
	if (window == NULL || fullPath == NULL)
		return FALSE;

	HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, window->processId);
	DWORD maxSize = MAX_PATH;

	if (hProcess == NULL)
		return FALSE;

	BOOL success = QueryFullProcessImageNameW(hProcess, 0, fullPath, &maxSize);

	CloseHandle(hProcess);

	return success;
}

// The work area contemplates the space used by the taskbar.
BOOL CenterWindow (const AppWindow* window, const BOOL useWorkArea) {
	if (window == NULL)
		return FALSE;

	const DisplayMonitor* monitor = &window->monitor;

	if (IsWindowMinimized(window->handle) ||
		IsWindowMaximized(window->handle) ||
		IsWindowFullScreen(window))
		return TRUE;

	if (IsWindowOutOfBounds(window, APPWND_OOB_POSITION | APPWND_OOB_SIZE))
		return FALSE;

	int X, Y;

	// In Windows 10 the taskbar can be at the bottom, left, right or top of the screen.
	// Changing the calculations is necessary because of this.
	if (useWorkArea) {
		X = monitor->workX + (monitor->workWidth / 2) - (window->width / 2);
		Y = monitor->workY + (monitor->workHeight / 2) - (window->height / 2);
	}
	else {
		X = monitor->x + (monitor->width / 2) - (window->width / 2);
		Y = monitor->y + (monitor->height / 2) - (window->height / 2);
	}

	return SetWindowPos(
		window->handle,
		NULL,
		X,
		Y,
		window->width,
		window->height,
		SWP_NOSIZE | SWP_NOZORDER
	);
}
