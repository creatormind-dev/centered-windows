# Centered Windows

Script that repositions application windows to the center of the screen they are on.
Written in C, for the Windows OS.

## Downloading

Since this thingamajig is written in C, there is no executable to download.
Instead, you must build the source code to get the executable.

You can directly download the source code by going to the [latest release](releases).
Alternatively, you can clone the repository.

### Building the Source Code

Before starting, you must have a C/C++ compiler installed.

The majority of this project was built and tested with the GCC compiler, so I recommend using it.
You can download and install it with [MSYS2 and MinGW64](https://www.msys2.org/).
It may also be possible to use the MSVC compiler, however, I'd recommend creating a Visual Studio project and building the code from there.

For your convenience, I've included a very useful tool that you might know as a Makefile. If you installed MinGW64 you can use the `make` command in the project folder to build the source code without doing much more. Here's how to do it:
- Open a MinGW64 terminal.
- Change to the project directory.
- Run the `make` command.

_You may get an error when compiling, it most likely is due to a missing_ `bin/` _directory. Just create the folder and try again._

## Usage & Configuration

Out of the box, the script will center every window that is not maximized, in full-screen or bigger than it's parent display; you can change this behavior using a blacklist. Simply create a `blacklist.txt` file in the same directory of the executable and write the title of the windows you'd like to leave out, separated by a new line.

Alternatively, you can make this same file work as a whitelist instead. To achieve this, go to the [main.c](src/main.c) and uncomment the line that starts with `#define WHITELIST`. Now only the windows listed will be centered.

And as an extra, you can change the name of the blacklist/whitelist file, just change the string of the `BLACKLIST_FILENAME` constant in the [main.c](src/main.c) file to the name you want your file to be looked up as.

## License

This project is licensed under the **Mozilla Public License Version 2.0** - see the [LICENSE](LICENSE) file for details.
