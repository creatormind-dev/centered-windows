#include <stdlib.h>
#include <wchar.h>
#include <stdio.h>
#include <windows.h>

#include "AppWindow.h"
#include "WindowBlacklist.h"

#define PASS return TRUE
#define BLACKLIST_FILENAME L"blacklist.txt"


wchar_t blacklist[MAX_PATH][MAX_BLACKLIST_ENTRIES];
int blacklistEntries = 0;


// Callback function for EnumWindows. Gets called for every window in the system.
BOOL CALLBACK WindowEnumProc (HWND hWnd, LPARAM lParam) {
	// A lot of unknown processes that shouldn't be processed are passed by the EnumWindows function.
	if (IsValidAppWindow(hWnd) == FALSE)
		PASS;

	AppWindow window;
	
	if (GetAppWindow(hWnd, &window) == FALSE)
		PASS;

	wchar_t exePath[MAX_PATH];
	wchar_t exeName[64];
	unsigned int exePathLen = 0;
	unsigned int exeNameLen = 0;

	// Windows titles often change, making them unreliable for a blacklist.
	// The executable path is used for comparison instead.
	if (GetAppWindowExecutable(&window, exePath) == FALSE)
		PASS;

	exePathLen = wcslen(exePath);
	
	for (int i = (exePathLen - 1); i >= 0; i--) {
		if (exePath[i] != '\\')
			continue;

		// Slices the executable name from it's path, populating exeName with the '*.exe' part.
		wcsncpy_s(exeName,
			sizeof(exeName) / sizeof(wchar_t),
			(exePath + i + 1),
			exePathLen - (i + 1)
		);

		break;
	}

	exeNameLen = wcslen(exeName);

#ifdef _DEBUG
	wprintf_s(L"AppWnd: %ls\nAppExe: %ls (%ls)\n", window.title, exePath, exeName);
#endif

	for (int i = 0; i < blacklistEntries; i++) {
		if (wcsncmp(exeName, blacklist[i], exeNameLen) == 0) {
#ifdef _DEBUG
			wprintf_s(L"\"%ls\" found in blacklist, skipping.\n\n", exeName);
#endif

			PASS;
		}
	}

	if (CenterWindow(&window, TRUE) == FALSE) {
#ifdef _DEBUG
		wprintf_s(L"Err: Couldn't center window \"%ls\".\n\n", window.title);
#endif
	}

	PASS;
}


int main (int argc, char** argv) {
	blacklistEntries = ReadWindowBlacklist(BLACKLIST_FILENAME, blacklist, MAX_PATH);

#ifdef _DEBUG
	if (blacklistEntries == -1)
		wprintf_s(L"\"%ls\" not found. Blacklist will be omitted.\n\n", BLACKLIST_FILENAME);
	else
		wprintf_s(L"Read %d entries from \"%ls\".\n\n", blacklistEntries, BLACKLIST_FILENAME);
#endif

	EnumWindows(WindowEnumProc, 0);

#ifdef _DEBUG
	system("pause");
#endif

	return EXIT_SUCCESS;
}
