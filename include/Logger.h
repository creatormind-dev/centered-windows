#ifndef _LOGGER_H_
#define _LOGGER_H_

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif // _INC_STDLIB

#ifndef _INC_STDIO
#include <stdio.h>
#endif // _INC_STDIO

#ifndef _INC_STDARG
#include <stdarg.h>
#endif // _INC_STDARG

#ifndef _TIME_H_
#include <time.h>
#endif // _TIME_H_

#ifndef _INC_WCHAR
#include <wchar.h>
#endif // _INC_WCHAR

#ifndef _INI_CONFIG_H_
#include "IniConfig.h"
#endif // _INI_CONFIG_H_

#define LOG_DELIMITER L"----------------------------------------\n"


// Indicates the type of a log message.
typedef enum {
	LOGTYPE_INFO,
	LOGTYPE_WARNING,
	LOGTYPE_ERROR
} LogType;


void StartLogger (void);
int WriteToLog (const LogType type, const wchar_t* format, ...);


#endif // _LOGGER_H_
