#include "WindowBlacklist.h"

// Modifies the blacklist array with all the entries in the blacklist, one entry per line.
// Returns the amount of entries in the blacklist, or -1 if the blacklist isn't found.
int ReadWindowBlacklist (const wchar_t* filename, wchar_t*** blacklist) {
	FILE* file = NULL;
	wchar_t entry[MAX_ENTRY_SIZE];
	unsigned int entries = 0;

	if (_wfopen_s(&file, filename, L"r") != 0)
		return -1;

	while (fgetws(entry, MAX_ENTRY_SIZE, file) != NULL)
		entries++;

	*blacklist = (wchar_t**) malloc(sizeof(wchar_t*) * entries);

	rewind(file);

	for (int i = 0; i < entries; i++) {
		fgetws(entry, MAX_ENTRY_SIZE, file);

		(*blacklist)[i] = (wchar_t*) malloc(sizeof(wchar_t) * wcslen(entry));

		wcscpy((*blacklist)[i], entry);
	}

	fclose(file);

	return entries;
}

void FreeWindowBlacklist (wchar_t*** blacklist, int entries) {
	for (int i = 0; i < entries; i++)
		free((*blacklist)[i]);

	free(*blacklist);
}
