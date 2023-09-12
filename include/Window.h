#ifndef _WINDOW_H
#define _WINDOW_H

#ifndef _WINDOWS_
#include <windows.h>
#endif // _WINDOWS_

#ifndef _MONITOR_H
#include "Monitor.h"
#endif // _MONITOR_H


typedef struct {

	HWND handle;
	Monitor* monitor;
	char title[256];
	unsigned int width;
	unsigned int height;
	unsigned int x;
	unsigned int y;

} Window;


Window* new_window (HWND, LPCRECT, Monitor*);
void center_window (Window*);


#endif // _WINDOW_H
