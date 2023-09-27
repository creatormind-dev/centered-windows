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
	AppWindow window;
	
	if (GetAppWindow(hWnd, &window) == FALSE)
		PASS;

	if (IsValidAppWindow(&window) == FALSE)
		PASS;

	TCHAR exeName[MAX_PATH];

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

	return 0;
}
