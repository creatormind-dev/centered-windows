#ifndef _DISPLAYMONITOR_H
#define _DISPLAYMONITOR_H

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


#endif // _DISPLAYMONITOR_H
