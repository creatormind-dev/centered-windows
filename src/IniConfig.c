#include "IniConfig.h"


wchar_t* BlacklistFilename = NULL;
wchar_t* LogFilename = NULL;
bool DebugMode = false;
bool UseWorkArea = true;
bool UseWhitelist = false;


// Attempts to load the configuration file and parse its contents.
// If successful, the configuration values are stored in the global variables.
bool LoadConfig (const wchar_t* filename) {
	FILE* file = NULL;
	wchar_t line[256];
	wchar_t attribute[64];
	wchar_t value[64];

	if (filename == NULL)
		return false;

	if (_wfopen_s(&file, filename, L"r") != 0)
		return false;

	while (fgetws(line, 256, file) != NULL) {
		if (GetConfigAttribute(line, attribute, value, 64, 64) == false)
			continue;

		if (wcscmp(attribute, L"BlacklistFilename") == 0)
			SetBlacklistFilename(value);
		else if (wcscmp(attribute, L"LogFilename") == 0)
			SetLogFilename(value);
		else if (wcscmp(attribute, L"DebugMode") == 0)
			SetDebugMode(wcscmp(value, L"true") == 0);
		else if (wcscmp(attribute, L"UseWorkArea") == 0)
			SetUseWorkArea(wcscmp(value, L"true") == 0);
		else if (wcscmp(attribute, L"UseWhitelist") == 0)
			SetUseWhitelist(wcscmp(value, L"true") == 0);
	}

	fclose(file);

	return true;
}

// Reads both attribute and value from a line in the configuration file.
// Returns false if the line is invalid.
bool GetConfigAttribute (const wchar_t* line, wchar_t* attribute, wchar_t* value, const unsigned int attrSize, const unsigned int valSize) {
	unsigned int i = 0;
	unsigned int k = 0;
	wchar_t attr[attrSize];
	wchar_t val[valSize];

	if (line == NULL || attribute == NULL || value == NULL)
		return false;

	// Copies the attribute name.
	while (line[i] != L'=' && line[i] != L'\0') {
		attr[i] = line[i];

		i++;
	}

	if (line[i] == L'\0')
		return false;

	attr[i] = L'\0'; // Adds the null terminator.

	i++;

	// Copies the attribute value.
	while (line[i] != L'\n') {
		val[k] = line[i];

		i++;
		k++;
	}

	val[k] = L'\0'; // Adds the null terminator.

	wcscpy_s(attribute, attrSize, attr);
	wcscpy_s(value, valSize, val);

	return true;
}

bool SetBlacklistFilename (const wchar_t* filename) {
	if (filename == NULL)
		return false;

	BlacklistFilename = (wchar_t*) malloc(sizeof(wchar_t) * (wcslen(filename) + 1));

	if (BlacklistFilename == NULL)
		return false;

	wcscpy_s(BlacklistFilename, wcslen(filename) + 1, filename);

	return true;
}

bool SetLogFilename (const wchar_t* filename) {
	if (filename == NULL)
		return false;

	LogFilename = (wchar_t*) malloc(sizeof(wchar_t) * (wcslen(filename) + 1));

	if (LogFilename == NULL)
		return false;

	wcscpy_s(LogFilename, wcslen(filename) + 1, filename);

	return true;
}

bool SetDebugMode (const bool debug) {
	DebugMode = debug;

	return true;
}

bool SetUseWorkArea (const bool useWorkArea) {
	UseWorkArea = useWorkArea;

	return true;
}

bool SetUseWhitelist (const bool useWhitelist) {
	UseWhitelist = useWhitelist;

	return true;
}

void FreeConfig (void) {
	free(BlacklistFilename);
	free(LogFilename);
}
