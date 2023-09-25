#include <stdlib.h>
#include <stdio.h>
#include <windows.h>

#include "AppWindow.h"


BOOL CALLBACK WindowEnumProc (HWND hWnd, LPARAM lParam) {
	AppWindow window;
	
	if (GetAppWindow(hWnd, &window) == FALSE)
		return TRUE;

	if (IsValidAppWindow(window) == FALSE)
		return TRUE;

	TCHAR exeName[MAX_PATH];

	if (GetAppWindowExecutable(window, exeName, MAX_PATH) == FALSE)
		return TRUE;

	printf_s("Wnd: %s\n", window.title);
	printf_s("Exe: %s\n\n", exeName);

	return TRUE;
}


int main (int argc, char** argv) {
	EnumWindows(WindowEnumProc, 0);

	return 0;
}
