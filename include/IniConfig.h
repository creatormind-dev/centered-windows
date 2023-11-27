#ifndef _INI_CONFIG_H_
#define _INI_CONFIG_H_

#ifndef _INC_STDLIB
#include <stdlib.h>
#endif // _INC_STDLIB

#ifndef _INC_WCHAR
#include <wchar.h>
#endif // _INC_WCHAR

#ifndef _INC_STDIO
#include <stdio.h>
#endif // _INC_STDIO

#define true 1
#define false 0


typedef unsigned char bool;


// The variables declared here are used to store the configuration values.
// They are declared as extern so they can be used in other files.

extern wchar_t* BlacklistFilename;
extern wchar_t* LogFilename;
extern bool DebugMode;
extern bool UseWorkArea;
extern bool UseWhitelist;


// The functions declared here are used to set the configuration values.

bool SetBlacklistFilename (const wchar_t* filename);
bool SetLogFilename (const wchar_t* filename);
bool SetDebugMode (const bool debug);
bool SetUseWorkArea (const bool useWorkArea);
bool SetUseWhitelist (const bool useWhitelist);

bool LoadConfig (const wchar_t* filename);
void FreeConfig (void);

bool GetConfigAttribute (const wchar_t* line, wchar_t* attribute, wchar_t* value, const unsigned int attrSize, const unsigned int valSize);


#endif // _INI_CONFIG_H_