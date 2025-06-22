---
name: Bug report
about: Create a report to help us improve TuneIn
title: ''
labels: bug, enhancement
assignees: ''

---

# Bug Report

**Describe the bug**

A clear and concise description of what the bug is. *(Be specific. Instead of "The application crashes," say "TuneIn crashes when I try to switch to Manual Mode while connected to a specific serial port.")*

**To Reproduce**

Steps to reproduce the behavior:

1.  Go to '...' *(Be precise. Instead of "Go to the project," say "Launch TuneIn.")*
2.  Click on '....' *(Be specific about TUI interactions. Instead of "Click the button," say "Press 'p' to open the Serial Port Configuration popup.")*
3.  Scroll down to '....' *(If scrolling is involved, explain what to scroll to. For example, "Select COM3 from the list of available ports.")*
4.  See error *(Describe the error. Is it an application crash, a specific message in the TUI, or an unexpected behavior? What does it say?)*

**Expected behavior**

A clear and concise description of what you expected to happen. *(Be specific. Instead of "The application should work," say "TuneIn should connect to the selected serial port without crashing, and then allow me to switch modes.")*

**Screenshots**

If applicable, add screenshots to help explain your problem. *(Annotate screenshots to highlight the relevant parts of the TUI or error messages.)*

**Desktop (please complete the following information):**

* OS: [e.g. Windows 10, macOS Ventura, Ubuntu 22.04]
* TuneIn version: [e.g. 0.1.0 (from GitHub Releases page)]
* Rust version: [e.g. 1.70.0 (run `rustc --version`)]
* Terminal Emulator: [e.g. Windows Terminal, Alacritty, iTerm2, Kitty, tmux]
* Serial Device/Adapter: [e.g. USB-to-UART converter, specific MIDI interface model]
* Serial Port Name (as seen by OS): [e.g. COM1, /dev/ttyUSB0, /dev/tty.usbmodem12345]

**Additional context**

Add any other context about the problem here. *(This is important! Include things like:)*

* Are there any specific serial port settings you are using (e.g., baud rate, data bits, parity, stop bits)?
* Does the bug always occur or only sometimes? If sometimes, under what circumstances?
* Are there any error messages displayed in the terminal where TuneIn is running, or in your system logs (e.g., Event Viewer on Windows, `dmesg` or `journalctl` on Linux)?
* If possible, provide a minimal reproducible example (e.g., the exact MIDI packet sequence, a small script, or precise key presses that trigger the bug).