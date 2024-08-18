# Centered Windows

Program that repositions application windows to the center of the screen.
Written in the C Programming Language, for the Windows OS.

## Downloading

You can download the program by going to the [latest release](https://github.com/creatormind-dev/centered-windows/releases/latest) and downloading the [`CenteredWindows.zip`](https://github.com/creatormind-dev/centered-windows/releases/latest/CenteredWindows.zip) file. Once downloaded, extract the contents of the `.zip` file into the folder you'd like to hold the program.

### Building the Source Code

This project was built and tested with the `gcc` compiler, the one included in the GNU Compilers Collection. It is recommended to build the project with the same compiler, you can download and install it with [MSYS2 and MinGW](https://www.msys2.org/).

Additionally, you are required to install either the [MSYS2 MinGW Toolchain](https://www.msys2.org/wiki/MSYS2-introduction/#subsystems) or the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/). In reality, only the Windows SDK is needed, but getting the toolchain will also give you additional libraries to develop with C/C++. Alternatively, you can download [Visual Studio 2022 or Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/downloads/) and select only to install the Windows SDK.

Settings related to how the compiler will build the program can be changed in the [Makefile](Makefile) configuration, although it's better left as is, unless you know exactly what you are doing.

To build the program follow these steps:
- Open a Linux subsystem terminal. The one provided by MSYS2 works fine (UCRT/MinGW).
- Navigate to the project directory.
- Run the `make` command.

If everything goes well, you should see the `Centered Windows.exe` file appear at the root of the project. If building fails, make sure there is a `/bin` directory; if the build still fails, please report the problem using the [issue tracker](https://github.com/creatormind-dev/centered-windows/issues).

## Usage and Configuration

### The Basics

The program will automatically center every window that it detects. However, some of these detected "windows" may be overlays that you would probably want to avoid moving.

To exclude a window/application from being centered you may use the blacklist. Simply put the name of the application (the one with the `.exe` termination) into the `blacklist.txt` file.
You may create and name the blacklist to your liking by editing the `BlacklistFilename` option in the `Settings.ini` file.

To find the name of an application you can open the Task Manager and go to the `Details` view.

### Work Area and Absolute Area

By default, the `UseWorkArea` option in the `Settings.ini` file is set to `true`. This means that the available space to center the applications **will** exclude the Taskbar and the Copilot sidebar.
If you want to use the absolute area of your monitor to center the applications, simply set the previously mentioned option to `false`.

### Extra Options

If you prefer having a whitelist instead of a blacklist you can change the `UseWhitelist` option to `true` in the `Settings.ini` file. This will treat the defined `BlacklistFilename` file as a whitelist.

Every time you run the program it will try to write information into a `log.txt` file. You can change the name of this file by editing the `LogFilename` option in the `Settings.ini` file to the one you want to use.

If you want to see what the program is doing before it ends and closes itself, you can enable debug mode by changing the `DebugMode` option to `true` in the `Settings.ini` file.

## License

This project is licensed under the **Mozilla Public License Version 2.0** - see the [LICENSE](LICENSE) file for more details.
