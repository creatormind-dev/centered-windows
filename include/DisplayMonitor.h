#ifndef _DISPLAYMONITOR_H
#define _DISPLAYMONITOR_H

#ifndef _WINDOWS_
#include <windows.h>
#endif // _WINDOWS_


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
