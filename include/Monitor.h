#ifndef _MONITOR_H
#define _MONITOR_H

#ifndef _WINDOWS_
#include <windows.h>
#endif // _WINDOWS_


typedef struct {

	HMONITOR handle;
	unsigned int width;
	unsigned int height;
	unsigned int x;
	unsigned int y;

} Monitor;


Monitor* new_monitor (HMONITOR, LPCRECT);


#endif // _MONITOR_H
