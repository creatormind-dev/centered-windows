#ifndef _DISPLAYMONITOR_H
#define _DISPLAYMONITOR_H

#ifndef _WINDOWS_
#include <windows.h>
#endif // _WINDOWS_


typedef struct {

	HMONITOR handle;

	UINT width;
	UINT height;
	UINT x;
	UINT y;

	UINT workWidth;
	UINT workHeight;
	UINT workX;
	UINT workY;

} DisplayMonitor;


BOOL GetDisplayMonitor (HMONITOR hMonitor, DisplayMonitor* monitor);


#endif // _DISPLAYMONITOR_H
