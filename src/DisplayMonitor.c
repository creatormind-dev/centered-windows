#include "DisplayMonitor.h"


BOOL GetDisplayMonitor (HMONITOR hMonitor, DisplayMonitor* monitor) {
	if (hMonitor == NULL || monitor == NULL)
		return FALSE;

	monitor->handle = hMonitor;

	MONITORINFO mInfo;

	mInfo.cbSize = sizeof(MONITORINFO);

	if (GetMonitorInfo(hMonitor, &mInfo) == FALSE)
		return FALSE;

	const RECT baseRect = mInfo.rcMonitor;
	const RECT workRect = mInfo.rcWork;

	monitor->width = baseRect.right - baseRect.left;
	monitor->height = baseRect.bottom - baseRect.top;
	monitor->x = baseRect.left;
	monitor->y = baseRect.top;
	monitor->workWidth = workRect.right - workRect.left;
	monitor->workHeight = workRect.bottom - workRect.top;
	monitor->workX = workRect.left;
	monitor->workY = workRect.top;

	return TRUE;
}
