# TuneIn Project Roadmap

This document outlines the future plans and development goals for the TuneIn project. Please note that this roadmap is subject to change based on feedback, development resources, and project priorities.

## Upcoming Features (Short-term)

* **Builds Using CI/CD**: Implement comprehensive Continuous Integration/Continuous Deployment (CI/CD) pipelines to automate testing across various platforms and streamline the release process for new versions. This ensures rapid availability of new features and bug fixes.
* **Refactoring**: Conduct a significant refactoring effort to generalize the codebase, removing any specific hardcoded elements or remnants from the initial two-developer setup and try to fuse those two styles. The goal is to make functions and structures more universal and reusable, enhancing maintainability and extensibility.
* **Documentation**: Prioritize adding comprehensive documentation to the code itself (e.g., inline comments, Rust doc comments for modules, functions, and structs) to improve developer understanding. Simultaneously, expand user-facing documentation with detailed guides and tutorials on how to effectively use all aspects of the tool.

## Mid-term Goals

* **Testing**: Establish a robust testing framework including unit tests for individual components, integration tests to verify interactions between modules, and potentially automated UI tests to ensure the TUI behaves as expected. This will significantly improve software reliability.
* **Kitty Keyboard Protocol**: Integrate support for the Kitty Keyboard Protocol. This will enable more precise and comprehensive keyboard event handling within the TUI, allowing for advanced key combinations, modifier states, and generally a more responsive and capable user experience for complex interactions.

## Long-term Vision

* **Scriptable Test Sequences**: Allow users to define custom test sequences using a configuration file.

## How to Influence the Roadmap

Your feedback is crucial! If you have suggestions for new features or wish to prioritize existing ones, please open an issue on our [GitHub Issues page](https://github.com/BakxY/TuneIn/issues) or participate in discussions.