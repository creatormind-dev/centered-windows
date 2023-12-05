#ifndef _LOGGER_H_
#define _LOGGER_H_

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif // _INC_STDLIB

#ifndef _INC_STDARG
#include <stdarg.h>
#endif // _INC_STDARG

#ifndef _INC_WCHAR
#include <wchar.h>
#endif // _INC_WCHAR

#ifndef _INC_STDIO
#include <stdio.h>
#endif // _INC_STDIO


int WriteLog (const wchar_t* filename, const wchar_t* format, ...);


#endif // _LOGGER_H_
