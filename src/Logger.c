#include "Logger.h"


// Starts the logger by printing the current time to the log file.
void StartLogger (void) {
	time_t rawTime = time(NULL);
	wchar_t logTime[20];
	FILE* file = NULL;

	if (_wfopen_s(&file, LogFilename, L"a") != 0)
		return;

	// Formats the current time as YYYY-MM-DD HH:MM:SS.
	wcsftime(logTime, sizeof(logTime) / sizeof(wchar_t), L"%Y-%m-%d %H:%M:%S", localtime(&rawTime));

	fwprintf_s(file, L"%ls\n", LOG_DELIMITER);
	fwprintf_s(file, L"%ls\n", logTime);

	fclose(file);
}

// Writes a formatted string to the log file. Use like printf.
// Returns the number of characters written to the log file.
int WriteToLog (const LogType type, const wchar_t* format, ...) {
	va_list args; // The rest of the arguments are stored in this list.
	FILE* file = NULL;
	int result = 0;

	if (format == NULL)
		return -1;

	if (_wfopen_s(&file, LogFilename, L"a") != 0)
		return -1;

	va_start(args, format);

	switch (type) {
		case LOGTYPE_WARNING:
			result = fwprintf_s(file, L"[WARNING]: ");
			break;
		case LOGTYPE_ERROR:
			result = fwprintf_s(file, L"[ERROR]: ");
			break;
		case LOGTYPE_INFO:
		default:
			result = fwprintf_s(file, L"[INFO]: ");
			break;
	}

	result += vfwprintf_s(file, format, args);

	va_end(args);

	fclose(file);

	return result;
}
