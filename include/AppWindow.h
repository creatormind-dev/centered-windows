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

#ifndef MAX_TITLE_LEN
#define MAX_TITLE_LEN 256
#endif // MAX_TITLE


typedef struct {

	HWND handle;
	DWORD processId;

	DisplayMonitor monitor;

	TCHAR title[MAX_TITLE_LEN];

	UINT width;
	UINT height;
	UINT x;
	UINT y;

} AppWindow;


BOOL GetAppWindow (HWND, AppWindow*);
BOOL IsValidAppWindow (const AppWindow);
BOOL GetAppWindowExecutable (const AppWindow, PTCHAR, DWORD);


#endif // _APPWINDOW_H
