# Centered Windows

Program that repositions application windows to the center of the screen.
Written in the C Programming Language, for the Windows OS.

## Downloading

You can download the 64 bit executable, or build the source code yourself (recommended).
Head over to the **Releases** section and either download the `.exe` file if you just want the program, or the `.zip` file if you want to build it yourself.

If you download the `.exe` file it is recommended to make a new folder and move the program inside the folder.

### Building the Source Code

This project was built and tested with the `g++` compiler, the one included in the GNU Compilers Collection. It is recommended to build the project with the same compiler, you can download and install it with [MSYS2 and MinGW](https://www.msys2.org/).

Settings related to how the compiler will build the program can be changed in the [Makefile](Makefile) configuration, although it's better left as is, unless you know exactly what you are doing.

To build the program follow these steps:
- Open a Linux subsystem terminal. The one provided by MinGW works fine.
- Navigate to the project directory.
- Run the `make` command.

If everything goes well, you should see the `Centered Windows.exe` file appear at the root of the project.

## Usage and Configuration

Out of the box, the program will center every window that is not maximized or in full-screen, you can change this behavior using a blacklist.

Simply create a `blacklist.txt` file in the same directory of the executable and write the absolute directory path to the application executable of the windows you want to exclude, one line per entry.

If you decided to download the source code or clone the repository, you can also change the name of the blacklist file. It should be defined as `BLACKLIST_FILENAME` in the **[main.c](src/main.c)** file.

## License

This project is licensed under the **Mozilla Public License Version 2.0** - see the [LICENSE](LICENSE) file for more details.
