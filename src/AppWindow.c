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

BOOL IsValidAppWindow (const AppWindow window) {
	if (window.handle == NULL)
		return FALSE;

	if (!IsWindow(window.handle))
		return FALSE;

	if (!IsWindowVisible(window.handle))
		return FALSE;

	if (GetWindowTextLength(window.handle) == 0)
		return FALSE;

	if (GetWindowLong(window.handle, GWL_STYLE) & WS_EX_TOOLWINDOW)
		return FALSE;

	return TRUE;
}

BOOL GetAppWindowExecutable (const AppWindow window, PTCHAR exeName, DWORD maxSize) {
	HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, window.processId);

	if (hProcess == NULL)
		return FALSE;

	BOOL err = QueryFullProcessImageName(hProcess, 0, exeName, &maxSize);

	CloseHandle(hProcess);

	return err;
}
