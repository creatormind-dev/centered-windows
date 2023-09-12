#include "Monitor.h"


Monitor* new_monitor (HMONITOR handle, LPCRECT rect) {
	Monitor* monitor = (Monitor*) malloc(sizeof(Monitor));

	monitor->handle = handle;
	monitor->width = rect->right - rect->left;
	monitor->height = rect->bottom - rect->top;
	monitor->x = rect->left;
	monitor->y = rect->top;

	return monitor;
}
