# TuneIn

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

TuneIn is a cross-platform terminal user interface (TUI) application designed for testing MIDI synthesizers. It simplifies the process of connecting to a serial port and sending MIDI packets, replacing complex multi-software setups with a single, intuitive tool.

![TuneIn Banner](./docs/tunein_banner.png#center)

## Table of Contents

- [Introduction](#Introduction)
- [Features](#Features)
- [Installation](#Installation)
- [Initial Usage](#Initial-Usage)
- [Roadmap](ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
- [License](LICENSE)

## Introduction

Developing and testing MIDI synthesizers often involves a tedious and error-prone setup. During a course at the ZHAW School of Engineering in Switzerland, the necessity for a streamlined testing environment became evident, as the existing process required configuring three different pieces of software, leading to frustration and lost development time.

TuneIn started as a joke between 3 students. After going through the tedious setup process of the provided setup, the joke was no longer a joke and they started to implement their solution. TuneIn provides a robust, cross-platform solution to directly interact with MIDI devices via a serial port, abstracting away the complexities of multiple tools. Whether you're a fellow student at the ZHAW School of Engineering attending the same course (ET-PM2) building your synthesizer or a developer working on a project including MIDI, TuneIn aims to speed up your workflow by offering a dedicated and intuitive testing interface.

## Features

* **MIDI Synthesizer Testing**: Directly connect to serial ports and send MIDI packets to test your hardware.
* **Two Operation Modes**:
    * **Normal Mode**: Easily send MIDI packets for notes using keyboard inputs. Users can dynamically adjust the selected octave and note strength (velocity) or enable the random strength mode, to emulate a real world keyboard.
    * **Manual Mode**: Provides granular control, allowing users to manually fill in each field of a MIDI packet for custom testing scenarios.
* **Cross-Platform TUI**: A terminal user interface built with Rust, Ratatui, and Crossterm, ensuring a consistent experience across different operating systems.
* **Simplified Setup**: Replaces multiple, complex software tools with a single, easy-to-use application, reducing setup time and potential errors.

## Installation

TuneIn is built with Rust and distributed as a single binary. Thanks to our CI/CD setup, new features and bug fixes are integrated quickly. You can always find the latest pre-compiled binaries for various platforms on the [GitHub Releases page](https://github.com/BakxY/TuneIn/releases).

## Initial Usage

Upon starting TuneIn, the application will prompt you via an overlaid popup within the TUI to set up a serial communication port. After successfully configuring the port, you can begin testing your MIDI synthesizer. Take a look at [`KEYBINDS.md`](KEYBINDS.md) to learn what keybinds exist.

## Roadmap

Check out our [ROADMAP.md](ROADMAP.md) to see what we're planning for future releases!