#include <stdlib.h>
#include <tchar.h>
#include <stdio.h>
#include <windows.h>

#include "AppWindow.h"
#include "WindowBlacklist.h"

#define PASS return TRUE
#define BLACKLIST_FILENAME "blacklist.txt"


TCHAR blacklist[MAX_PATH][MAX_BLACKLIST_ENTRIES];
UINT blacklistEntries = 0;


BOOL CALLBACK WindowEnumProc (HWND hWnd, LPARAM lParam) {
	// A lot of unknown processes that shouldn't be processed are passed by the EnumWindows function.
	if (IsValidAppWindow(hWnd) == FALSE)
		PASS;

	AppWindow window;
	
	if (GetAppWindow(hWnd, &window) == FALSE)
		PASS;

	TCHAR exeName[MAX_PATH];

	// Windows titles often change, making them unreliable for a blacklist.
	// The executable path is used for comparison instead.
	if (GetAppWindowExecutable(&window, exeName, MAX_PATH) == FALSE)
		PASS;

	for (int i = 0; i < blacklistEntries; i++) {
		if (_tcsncmp(exeName, blacklist[i], _tcslen(exeName)) == 0)
			PASS;
	}

	CenterWindow(&window, TRUE);

	PASS;
}


int main (int argc, char** argv) {
	blacklistEntries = ReadWindowBlacklist(_T(BLACKLIST_FILENAME), blacklist, MAX_PATH);

	EnumWindows(WindowEnumProc, 0);

	return EXIT_SUCCESS;
}
