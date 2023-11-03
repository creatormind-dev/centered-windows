#include "WindowBlacklist.h"

// Modifies the blacklist array with all the entries in the blacklist, one entry per line.
// Returns the amount of entries in the blacklist, or -1 if the blacklist isn't found.
int ReadWindowBlacklist (const wchar_t* filename, wchar_t blacklist[][MAX_BLACKLIST_ENTRIES], int maxEntrySize) {
	FILE* file = NULL;
	wchar_t line[MAX_LINE_LENGTH];

	_wfopen_s(&file, filename, L"r");

	if (file == NULL)
		return -1;

	unsigned int entries = 0;

	while (fgetws(line, MAX_LINE_LENGTH, file) != NULL)
		wcscpy_s(blacklist[entries++], maxEntrySize, line);

	fclose(file);

	return entries;
}
