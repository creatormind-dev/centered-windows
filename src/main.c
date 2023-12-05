#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>
#include <wchar.h>
#include <windows.h>

#include "IniConfig.h"
#include "Logger.h"
#include "AppWindow.h"
#include "WindowBlacklist.h"

#define CONFIG_FILENAME L"Settings.ini"
#define PASS return TRUE


wchar_t** blacklist = NULL;
int blacklistEntries = 0;


// Logs a formatted string to the console and the log file. Use like printf.
VOID Log (const LogType type, const wchar_t* format, ...) {
	// IDK what kind of witchcraft C does to have variable arguments, but it's pretty cool.
	va_list args;
	wchar_t buffer[1024];

	va_start(args, format);

	vswprintf_s(buffer, sizeof(buffer) / sizeof(wchar_t), format, args);

	va_end(args);

	if (DebugMode == TRUE)
		wprintf_s(L"%ls", buffer);

	WriteToLog(type, L"%ls", buffer);
}

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

	if (DebugMode == TRUE)
		wprintf_s(L"AppWnd: %ls\nAppExe: %ls (%ls)\n", window.title, exePath, exeName);

	// Checks if the executable name is in the blacklist.
	for (int i = 0; i < blacklistEntries; i++) {
		if (wcsncmp(exeName, blacklist[i], exeNameLen) == 0) {
			Log(LOGTYPE_INFO, L"\"%ls\" found in blacklist, skipping.\n", exeName);

			PASS;
		}
	}

	if (CenterWindow(&window, UseWorkArea) == TRUE)
		Log(LOGTYPE_INFO, L"Window \"%ls\" (%ls) centered.\n", window.title, exeName);	
	else
		Log(LOGTYPE_WARNING, L"Couldn't center window \"%ls\" (%ls).\n", window.title, exeName);

	PASS;
}


int main (void) {
	if (LoadConfig(CONFIG_FILENAME) == TRUE) {
		StartLogger();
		Log(LOGTYPE_INFO, L"Configuration from %ls parsed and loaded.\n", CONFIG_FILENAME);
	}

	blacklistEntries = ReadWindowBlacklist(BlacklistFilename, &blacklist);

	if (blacklistEntries == -1)
		Log(LOGTYPE_WARNING, L"\"%ls\" not found. Blacklist will be omitted.\n", BlacklistFilename);
	else
		Log(LOGTYPE_INFO, L"Read %d entries from \"%ls\".\n", blacklistEntries, BlacklistFilename);

	EnumWindows(WindowEnumProc, 0);

	FreeWindowBlacklist(&blacklist, blacklistEntries);
	FreeConfig();

	Log(LOGTYPE_INFO, L"Exiting program...\n\n");

	if (DebugMode == TRUE)
		system("pause");

	return EXIT_SUCCESS;
}
