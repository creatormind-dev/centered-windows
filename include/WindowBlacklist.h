#ifndef _WINDOW_BLACKLIST_H_
#define _WINDOW_BLACKLIST_H_

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif // _INC_STDLIB

#ifndef _INC_WCHAR
#include <wchar.h>
#endif // _INC_WCHAR

#ifndef _INC_STDIO
#include <stdio.h>
#endif // _INC_STDIO

#ifndef MAX_ENTRY_SIZE
#define MAX_ENTRY_SIZE 64
#endif // MAX_ENTRY_SIZE


int ReadWindowBlacklist (const wchar_t* filename, wchar_t*** blacklist);
void FreeWindowBlacklist (wchar_t***, int entries);


#endif // _WINDOW_BLACKLIST_H_
