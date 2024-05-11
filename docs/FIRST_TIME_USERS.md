# Quick Start for New Users

## I'm new to the terminal, but I want to use `johnnybgoode`! How do I get started?

This guide is designed to take you from zero to hero with johnnybgoode, even if you've never set foot in a terminal before. You'll be an expert before you know it.

### Terminal/shell basics

To start off, let's open a terminal. On Windows, it's Windows Terminal, on Mac, it's just called "Terminal", and if you're on Linux, i'm surprised you need to read this, but you might have Konsole, GNOME Terminal, or something else depending on your distro. But at the end of the day, which terminal you use doesn't matter at all.

Before we get to johnnybgoode, let's tackle a few basics about *shells*.

Shells are systems you use in a terminal to interact with your computer. Each operating system ships with its own shell.

| Operating System | Shell Name |
| ----------------| -----------|
| Windows | Command Prompt (`cmd`), or PowerShell* |
| MacOS | `zsh` (usually pronounced Z-shell) |
| Linux | `bash` |

A few other shells exist that you can choose from on some systems, such as `fish` or my personal favorite, `nushell`.

*Johnnybgoode is not officially supported for PowerShell. If you want to use it, you will need to do your own homework to get the jump command to work. `cmd` will work perfectly fine.

While I've mentioned a bunch of shells here, the important point here is that you know which one you're on.

### Your First Shell Command

When you launched your terminal, your shell likely started up in your home directory. Our first command is going to be `ls` (`dir` on Command Prompt), which lists the contents of the current directory. Type `ls`, press Enter, and your terminal will print out a list of everything that's in the folder. Great work!

We'll also cover one more command, to whet your terminal appetite. `cd` is short for Change Directory, and it lets you move from one directory to another. To try it out, type `cd Documents` to move to switch to the Documents folder that's most likely in the home directory. Some special characters to use with `cd` are a double period (`..`) to refer to the folder *above* your current location, and `~` to refer to your home directory (this does not work on Command Prompt).

## Downloading `johnnybgoode`

You can download the latest version of Johnnybgoode [here](https://github.com/SwissArmyWrench/johnnybgoode/releases/latest). Download the appropriate file to somewhere where you won't lose it.

Now, we need to let your system know where that file is, so we can call it with `johnny` commands in your shell. We can do this the proper way, or the easy  way.

### The proper way

Technically, the right way to deal with this problem is to add the binary to your system's PATH variable. This is a bit complicated, involving editing system environment variables. I suggest doing your research and learning as much as you can about how PATH works before messing with it - and since this guide is for beginners, we won't really touch on it here.

### The easy way

You can set up a terminal *alias* instead. This is much safer for a beginner. An alias is like a shortcut for a longer command.
