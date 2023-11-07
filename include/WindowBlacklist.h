#ifndef _WINDOWBLACKLIST_H
#define _WINDOWBLACKLIST_H

#ifndef _WCHAR_T_DEFINED
#include <wchar.h>
#endif // _WCHAR_T_DEFINED

#ifndef _STDIO_DEFINED
#include <stdio.h>
#endif // _STDIO_DEFINED

#ifndef MAX_ENTRY_SIZE
#define MAX_ENTRY_SIZE 64
#endif // MAX_ENTRY_SIZE


int ReadWindowBlacklist (const wchar_t* filename, wchar_t*** blacklist);
void FreeWindowBlacklist (wchar_t***, int entries);


#endif // _WINDOWBLACKLIST_H
