# showargsw - Show args in a Windows message box

This program shows its command-line arguments in a Windows message box.

## Details

Even though message boxes are graphical, this builds as a command-line application and receives its arguments through the usual Windows parsing rules automatically. (Unlike other systems, on Windows command-line arguments are passed as a single string that has to be parsed, though this often happens behind the scenes, e.g., in a C program it happens before `main` is called.)

This is meant as a diagnostic tool and its uses overlap with the very simple program [`showargs`](https://github.com/EliahKagan/showargs). If you're on Windows, and you don't have a console or it's not convenient to look at console output, or you want to be immediately informed that your application has called an executable at a particular path, this may be preferable.

For simplicity this currently only shows a message box, but that is not ideal because message boxes do not allow text selection. They do, however, allow their entire contents to be copied by pressing <kbd>Ctrl</kbd>+<kbd>C</kbd> while the message box has focus. (This is a feature of Windows message boxes, not specifically of this little program.)

## License

[0BSD](LICENSE)
