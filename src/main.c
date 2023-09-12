#define STRICT
// #define DEBUG // Uncomment this to enable debug messages. Useful to get the window titles.
// #define WHITELIST // Uncomment this to switch the blacklist to a whitelist.
#define MAX_LINE_LENGTH 256

#include <stdlib.h>
#include <stdio.h>
#include <windows.h>

#include "Monitor.h"
#include "Window.h"


#pragma region Forward Declarations

void read_blacklist (void);
boolean is_valid_window (HWND);
BOOL CALLBACK window_callback (HWND, LPARAM);
BOOL CALLBACK monitor_callback (HMONITOR, HDC, LPRECT, LPARAM);

#pragma endregion
#pragma region Global Variables

const char BLACKLIST_FILENAME[] = "blacklist.txt";

Monitor** monitors = NULL;
char** blacklist = NULL;
unsigned short monitor_count = 0;
unsigned short blacklist_entries = 0;

#pragma endregion


int main (int argc, char* argv[]) {
	read_blacklist();

	// Iterates through all monitors and windows, calling their respective callbacks.
	EnumDisplayMonitors(NULL, NULL, monitor_callback, 0);
	EnumWindows(window_callback, 0);

	free(monitors);

#ifdef DEBUG
	system("pause");
#endif // DEBUG

	return 0;
}


#pragma region Function Definitions

/**
 * Reads the blacklist file (if any) and populates the blacklist array with the entries.
 * @param blacklist An array of strings to populate with the entries from the blacklist file.
 * @return The number of entries in the blacklist array (0 if the file doesn't exist).
*/
void read_blacklist (void) {
	FILE* file = NULL;
	char line[MAX_LINE_LENGTH];

	// Open the blacklist file.
	fopen_s(&file, BLACKLIST_FILENAME, "r");

	// File doesn't exist, bye bye.
	if (file == NULL) {
#ifdef DEBUG
		printf("Failed to open blacklist file: %s.\n", BLACKLIST_FILENAME);
#endif // DEBUG

		return;
	}

	// Count the number of lines in the file.
	while (fgets(line, MAX_LINE_LENGTH, file) != NULL)
		blacklist_entries++;

	// Allocate the memory for the blacklist array.
	blacklist = (char**) malloc(sizeof(char*) * blacklist_entries);

	if (blacklist == NULL) {
#ifdef DEBUG
		printf("Failed to allocate memory for blacklist array.\n");
#endif // DEBUG

		return;
	}

	// Reset the file pointer to the beginning of the file.
	fseek(file, 0, SEEK_SET);

	// Populate the blacklist array with the entries from the file.
	for (int i = 0; i < blacklist_entries; i++) {
		fgets(line, MAX_LINE_LENGTH, file);

		blacklist[i] = (char*) malloc(sizeof(char) * MAX_LINE_LENGTH);

		strcpy_s(blacklist[i], MAX_LINE_LENGTH, line);
	}

	// Close the file.
	fclose(file);
}

/**
 * Checks multiple conditions to determine if a window is valid to be centered.
 * @param window A pointer to the window struct to check.
 * @return TRUE if the window is valid, FALSE otherwise.
*/
boolean is_valid_window (Window* window) {
	// NULL pointer check, baby.
	if (window == NULL)
		return FALSE;

	HWND hwnd = window->handle;
	Monitor* monitor = window->monitor;

	// Window struct could possibly be uninitialized. No handle, no deal.
	if (hwnd == NULL)
		return FALSE;

	// Is it even possible to run windows without a display?
	if (monitor == NULL)
		return FALSE;

	// Ah yes, the handle of a window that ISN'T a window. Very cool.
	if (!IsWindow(hwnd))
		return FALSE;

	// I don't know how much this actually filters out, 'cause it doesn't seem to make a difference.
	// Like, there's a bunch of windows that are invisible (like, literally invisible, not just minimized) that still pass.
	if (!IsWindowVisible(hwnd))
		return FALSE;

	// To help with the filter above, if the window doesn't have a title, it's probably not a window (in the traditional sense).
	if (GetWindowTextLength(hwnd) == 0)
		return FALSE;

	// Don't really know what a tool window is, but I'm pretty sure it's not a window that should be centered anyway.
	if (GetWindowLong(hwnd, GWL_STYLE) & WS_EX_TOOLWINDOW)
		return FALSE;

	// The window is maximized... either that or it's just really big.
	if (window->width >= monitor->width && window->height >= monitor->height)
		return FALSE;

#ifdef WHITELIST
	// Check if the window's title is in the whitelist.
	for (int i = 0; i < blacklist_entries; i++) {
		if (strncmp(window->title, blacklist[i], strlen(window->title)) == 0)
			return TRUE;
	}

	return FALSE;
#else
	// Check if the window's title is in the blacklist.
	for (int i = 0; i < blacklist_entries; i++) {
		if (strncmp(window->title, blacklist[i], strlen(window->title)) == 0)
			return FALSE;
	}

	return TRUE;
#endif // WHITELIST
}

/**
 * Callback function for EnumDisplayMonitors. Creates a new monitor struct and adds it to the global array.
 * @param handle The handle of the monitor.
 * @param hdc The device context of the monitor.
 * @param rect The rectangle of the monitor.
 * @param lParam The LPARAM passed to EnumDisplayMonitors.
*/
BOOL CALLBACK monitor_callback (HMONITOR handle, HDC hdc, LPRECT rect, LPARAM lParam) {
	Monitor* monitor = NULL;
	MONITORINFO info;

	// Set the size of the MONITORINFO struct for the GetMonitorInfo function.
	info.cbSize = sizeof(MONITORINFO);

	if (GetMonitorInfo(handle, &info) == FALSE)
		return FALSE;

	// Use the work area of the monitor, not the entire monitor. Otherwise my OCD will kick in.
	monitor = new_monitor(handle, &info.rcWork);
	// Reallocate the memory for the array of monitors and add the new monitor to it.
	monitors = (Monitor**) realloc(monitors, sizeof(Monitor*) * (monitor_count + 1));
	monitors[monitor_count++] = monitor;

#ifdef DEBUG
	printf("Monitor: %d, %d, %d, %d\n", monitor->x, monitor->y, monitor->width, monitor->height);
#endif // DEBUG

	return TRUE;
}

/**
 * Callback function for EnumWindows.
*/
BOOL CALLBACK window_callback (HWND handle, LPARAM lParam) {
	Window* window = NULL;
	Monitor* monitor = NULL;
	RECT rect;

	// Get the monitor that the window is on.
	for (short i = 0; i < monitor_count; i++) {
		if (monitors[i]->handle != MonitorFromWindow(handle, MONITOR_DEFAULTTONEAREST))
			continue;

		monitor = monitors[i];
		break;
	}

	// I mean, I guess it's possible for a window to not be on a monitor, but I don't know how that would work.
	if (monitor == NULL)
		return TRUE;

	// Get the rectangle of the window.
	if (GetWindowRect(handle, &rect) == 0)
		return TRUE;

	window = new_window(handle, &rect, monitor);

	// If the window is valid, center it.
	if (is_valid_window(window)) {
#ifdef DEBUG
		printf("Wnd: %s\n", window->title);
#endif // DEBUG

		center_window(window);
	}

	free(window);

	return TRUE;
}

#pragma endregion
