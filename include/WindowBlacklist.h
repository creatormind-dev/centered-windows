#ifndef _WINDOWBLACKLIST_H
#define _WINDOWBLACKLIST_H

#ifndef _TCHAR
#include <tchar.h>
#endif // _TCHAR

#ifndef _STDIO_DEFINED
#include <stdio.h>
#endif // _STDIO_DEFINED

#ifndef MAX_BLACKLIST_ENTRIES
#define MAX_BLACKLIST_ENTRIES 64
#endif // MAX_BLACKLIST_ENTRIES

#ifndef MAX_LINE_LENGTH
#define MAX_LINE_LENGTH 1024
#endif // MAX_LINE_LENGTH


int ReadWindowBlacklist (const TCHAR* filename, TCHAR blacklist[][MAX_BLACKLIST_ENTRIES], int maxEntrySize);


#endif // _WINDOWBLACKLIST_H
