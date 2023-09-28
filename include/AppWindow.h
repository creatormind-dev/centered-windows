#ifndef _APPWINDOW_H
#define _APPWINDOW_H

#ifndef _STDIO_DEFINED
#include <stdio.h>
#endif // _STDIO_DEFINED

#ifndef _WINDOWS_
#include <windows.h>
#endif // _WINDOWS_

#ifndef _DISPLAYMONITOR_H
#include "DisplayMonitor.h"
#endif // _DISPLAY_MONITOR_H

#ifndef MAX_TITLE_LENGTH
#define MAX_TITLE_LENGTH 256
#endif // MAX_TITLE


// Represents a Windows process application with a visible rectangle.
typedef struct {

	HWND handle;
	DWORD processId;

	DisplayMonitor monitor;

	TCHAR title[MAX_TITLE_LENGTH];

	UINT width;
	UINT height;
	UINT x;
	UINT y;

} AppWindow;


BOOL GetAppWindow (HWND hWnd, AppWindow* window);
BOOL IsValidAppWindow (const HWND hWnd);
BOOL IsWindowMaximized (const AppWindow* window);
BOOL IsWindowFullScreen (const AppWindow* window);
BOOL GetAppWindowExecutable (const AppWindow* window, TCHAR exeName[], DWORD maxSize);
BOOL CenterWindow (const AppWindow* window, const BOOL useWorkArea);


#endif // _APPWINDOW_H
