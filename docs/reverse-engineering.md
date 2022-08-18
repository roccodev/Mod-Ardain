# Reverse-engineering Xenoblade executables

Working on Xenoblade ROMs is somewhat easy once you've built the right setup.

When dumping your game files, (I'd recommend dumping both `RomFS` and `ExeFS`, but you can get away with just the executable) make sure to get both the base executable and the one with all updates applied. (DLC should only affect `RomFS`)

All three games are written in C++, so expect C++ patterns and syntax. Iin other words, get ready to identify vtables and constructors/destructors.

Useful tools:
* [Ghidra SRE](https://ghidra-sre.org/) or [Cutter](https://github.com/rizinorg/cutter)/radare2 as reverse-engineering suites.
   * There is a Ghidra loader for Switch executables: https://github.com/Adubbz/Ghidra-Switch-Loader
* Xenoblade data tables: https://xenoblade.github.io/
* AArch64 reference manual: https://developer.arm.com/documentation/
* Ryujinx emulator: https://ryujinx.org/


## Definitive Edition
*Last updated as of XCDE v. `1.1.2`*

The Definitive Edition executable retains function and module names. Debug strings with file names and code references are also present.

This is probably the easiest game to reverse-engineer. There are also symbols for libraries like ImGui, though I haven't done research on the rendering part.

## Xenoblade 2
*Last updated as of XC2 v. `2.1.0`*

The latest executable has no symbols, but debug strings are left intact.  
Earlier executables have function names, so if you have the cartridge version you can dump the executable without update data.

I advise you to make research on the base version, then search for matching instructions/patterns to find offsets in the latest version.

## Xenoblade 3
*Last updated as of XC3 v. `1.1.0`*

Both the 1.0.0 and 1.1.0 executables are stripped from function names and debug symbols. Additionally, it looks like this game was built with a higher optimization level.

Reverse-engineering is severely hindered by this change, but it is still possible. Several other changes were made to the data format, e.g. you won't find BDAT keys in the program text anymore. (BDATv4 research is still ongoing)

The game bears similarities with Definitive Edition, particularly in save data management and sound systems.