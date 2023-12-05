#include "Logger.h"

// Writes a formatted string to the log file. Use like printf.
int WriteLog (const wchar_t* filename, const wchar_t* format, ...) {
	va_list args;
	FILE* file = NULL;
	int result = 0;

	if (filename == NULL || format == NULL)
		return -1;

	if (_wfopen_s(&file, filename, L"a") != 0)
		return -1;

	va_start(args, format);

	result = vfwprintf_s(file, format, args);

	fclose(file);
	va_end(args);

	return result;
}
