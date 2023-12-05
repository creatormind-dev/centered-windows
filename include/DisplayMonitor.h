#ifndef _DISPLAY_MONITOR_H_
#define _DISPLAY_MONITOR_H_

#ifndef _INC_WINDOWS
#include <windows.h>
#endif // _INC_WINDOWS


// Represents basic information of a detected screen display.
typedef struct {

	HMONITOR handle;

	int width;
	int height;
	int x;
	int y;

	int workWidth;
	int workHeight;
	int workX;
	int workY;

} DisplayMonitor;


BOOL GetDisplayMonitor (HMONITOR hMonitor, DisplayMonitor* monitor);


#endif // _DISPLAY_MONITOR_H_
