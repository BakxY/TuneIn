# Contributing to TuneIn

We welcome contributions to the TuneIn project! Whether you're reporting a bug, suggesting a new feature, or submitting code, your help is greatly appreciated.

## How to Contribute

If you want to create an issue for any reason, please use one of the provided templates and fill out all information. If you don't have a required information, please mark it as such and don't make up information, as this can make troubleshooting much harder.

1.  **Reporting Bugs**: If you find a bug, please open an issue on our [GitHub Issues page](https://github.com/BakxY/TuneIn/issues). Provide a clear description of the bug, steps to reproduce it, and your environment details.
2.  **Feature Requests**: Have an idea for a new feature? Open an issue on GitHub to discuss your idea.
3.  **Code Contributions**:
    * Fork the repository.
    * Create a new branch for your feature or bug fix.
    * Make your changes.
    * Ensure your code follows the existing style and passes all tests.
    * Submit a pull request with a clear description of your changes.

## Development Setup

To set up your development environment:

1.  **Prerequisites**: Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.
2.  **Clone the repository**: `git clone https://github.com/BakxY/TuneIn.git`
3.  **Navigate to the project directory**: `cd TuneIn`
4.  **Build the project**: `cargo build --release` (for a release build) or `cargo build` (for a debug build)
5.  **Run TuneIn**: The executable will be located in `target/release/tunein` (or `target/release/tunein.exe` on Windows).
6.  **Debugger**: You can then use your favorite debugger to pinpoint bugs and test new features.

## Coding Style and Guidelines

Please follow these guidelines when contributing code:

* **Language:** The project is written in Rust.
* **Style Guide:** Focus on writing clean, readable code. Avoid unnecessary abstraction or over-optimization. Don't hesitate to create new functions and files when necessary. Aim for functions with single, well-defined purposes. Design code for reusability—if a piece of code could be reused, make it a function.
* **Documentation:** When adding a new feature, please update the [usage documentation](USAGE.md) and the [feature list in the README](README.md). Add comments to your code where needed to clarify complex logic or explain what you're doing. Comments should be concise and focus on the "why" rather than the "what".