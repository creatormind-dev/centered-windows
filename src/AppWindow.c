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
	GetWindowText(hWnd, window->title, MAX_TITLE_LEN);

	return TRUE;
}

BOOL IsValidAppWindow (const AppWindow* window) {
	if (window == NULL || window->handle == NULL)
		return FALSE;

	if (!IsWindow(window->handle))
		return FALSE;

	if (!IsWindowVisible(window->handle))
		return FALSE;

	if (GetWindowTextLength(window->handle) == 0)
		return FALSE;

	if (GetWindowLong(window->handle, GWL_STYLE) & WS_EX_TOOLWINDOW)
		return FALSE;

	return TRUE;
}

BOOL IsWindowMaximized (const AppWindow* window) {
	const DisplayMonitor* monitor = &window->monitor;

	if (window->x == monitor->workX &&
		window->y == monitor->workY &&
		window->width == monitor->workWidth &&
		window->height == monitor->workHeight)
		return TRUE;

	return FALSE;
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

BOOL GetAppWindowExecutable (const AppWindow* window, TCHAR exeName[], DWORD maxSize) {
	if (window == NULL || exeName == NULL)
		return FALSE;

	HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, window->processId);

	if (hProcess == NULL)
		return FALSE;

	BOOL success = QueryFullProcessImageName(hProcess, 0, exeName, &maxSize);

	CloseHandle(hProcess);

	return success;
}

BOOL CenterWindow (const AppWindow* window, const BOOL useWorkArea) {
	if (window == NULL)
		return FALSE;

	const DisplayMonitor* monitor = &window->monitor;

	if (IsWindowMaximized(window) || IsWindowFullScreen(window))
		return TRUE;

	UINT X, Y;

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
