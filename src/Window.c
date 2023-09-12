#include "Window.h"


Window* new_window (HWND handle, LPCRECT rect, Monitor* monitor) {
	Window* window = (Window*) malloc(sizeof(Window));

	window->handle = handle;
	window->monitor = monitor;
	window->width = rect->right - rect->left;
	window->height = rect->bottom - rect->top;
	window->x = rect->left;
	window->y = rect->top;

	GetWindowText(handle, window->title, sizeof(window->title));

	return window;
}

void center_window (Window* window) {
	if (window == NULL)
		return;

	Monitor* monitor = window->monitor;

	unsigned int x = monitor->x + (monitor->width / 2) - (window->width / 2);
	unsigned int y = monitor->y + (monitor->height / 2) - (window->height / 2);

	SetWindowPos(window->handle, NULL, x, y, window->width, window->height, SWP_NOSIZE);
}
