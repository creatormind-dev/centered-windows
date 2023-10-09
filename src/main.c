#include <stdlib.h>
#include <tchar.h>
#include <stdio.h>
#include <windows.h>

#include "AppWindow.h"
#include "WindowBlacklist.h"

#define PASS return TRUE
#define BLACKLIST_FILENAME "blacklist.txt"


TCHAR blacklist[MAX_PATH][MAX_BLACKLIST_ENTRIES];
int blacklistEntries = 0;


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

#ifdef _DEBUG
	_tprintf_s(_T("AppWnd: %s\nAppExe: %s\n"), window.title, exeName);
#endif

	for (int i = 0; i < blacklistEntries; i++) {
		if (_tcsncmp(exeName, blacklist[i], _tcslen(exeName)) == 0) {
#ifdef _DEBUG
			_tprintf_s(_T("\"%s\" found in blacklist, skipping.\n\n"), exeName);
#endif

			PASS;
		}
	}

	if (CenterWindow(&window, TRUE) == FALSE) {
#ifdef _DEBUG
		_tprintf_s(_T("Err: Couldn't center window \"%s\".\n\n"), window.title);
#endif
	}

	PASS;
}


int main (int argc, char** argv) {
	blacklistEntries = ReadWindowBlacklist(_T(BLACKLIST_FILENAME), blacklist, MAX_PATH);

#ifdef _DEBUG
	if (blacklistEntries == -1)
		_tprintf_s(_T("\"%s\" not found. Blacklist will be omitted.\n\n"), _T(BLACKLIST_FILENAME));
	else
		_tprintf_s(_T("Read %d entries from \"%s\".\n\n"), blacklistEntries, _T(BLACKLIST_FILENAME));
#endif

	EnumWindows(WindowEnumProc, 0);

#ifdef _DEBUG
	system("pause");
#endif

	return EXIT_SUCCESS;
}
