#include "WindowBlacklist.h"

// Modifies the blacklist array with all the entries in the blacklist, one entry per line.
// Returns the amount of entries in the blacklist, or -1 if the blacklist isn't found.
int ReadWindowBlacklist (const TCHAR* filename, TCHAR blacklist[][MAX_BLACKLIST_ENTRIES], int maxEntrySize) {
	FILE* file = NULL;
	TCHAR line[MAX_LINE_LENGTH];

	_tfopen_s(&file, filename, _T("r"));

	if (file == NULL)
		return -1;

	unsigned int entries = 0;

	while (_fgetts(line, MAX_LINE_LENGTH, file) != NULL)
		_tcscpy_s(blacklist[entries++], maxEntrySize, line);

	fclose(file);

	return entries;
}
