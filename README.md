# ⚡ JuDo-cli

<div align="center">

[![GitHub stars](https://img.shields.io/github/stars/NotJuu/JuDo-cli?style=for-the-badge)](https://github.com/NotJuu/JuDo-cli/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/NotJuu/JuDo-cli?style=for-the-badge)](https://github.com/NotJuu/JuDo-cli/network)
[![GitHub issues](https://img.shields.io/github/issues/NotJuu/JuDo-cli?style=for-the-badge)](https://github.com/NotJuu/JuDo-cli/issues)
[![GitHub license](https://img.shields.io/github/license/NotJuu/JuDo-cli?style=for-the-badge)](LICENSE) <!-- TODO: Add a LICENSE file -->

**An asynchronous, minimal, and fast CLI Todo App built with Rust.**

</div>

## 📖 Overview

JuDo-cli is a powerful yet simple command-line interface application designed to help you manage your daily tasks efficiently. Built with Rust, it leverages asynchronous programming to provide a fast and responsive user experience. It's an ideal tool for developers and users who prefer to manage their todos directly from the terminal, focusing on speed and minimal overhead. This project also serves as a practical learning exercise in Rust's asynchronous capabilities and CLI application development.

## ✨ Features

-   🎯 **Add Todos**: Quickly add new tasks to your list.
-   📝 **List Todos**: View all your pending and completed tasks.
-   ✅ **Mark as Done**: Easily mark tasks as completed.
-   🗑️ **Remove Todos**: Delete tasks you no longer need.
-   ⚡ **Asynchronous Operations**: Experience fast and responsive command execution.
-   💾 **Local Persistence**: All your tasks are saved locally for continuity.

## 🛠️ Tech Stack

**Runtime:**
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

**Libraries:**
![Clap](https://img.shields.io/badge/Clap-F56000?style=for-the-badge)
![Tokio](https://img.shields.io/badge/Tokio-0077B6?style=for-the-badge)
![Serde](https://img.shields.io/badge/Serde-303030?style=for-the-badge)
![Serde_json](https://img.shields.io/badge/Serde_json-303030?style=for-the-badge)
![Anyhow](https://img.shields.io/badge/Anyhow-512BD4?style=for-the-badge)

## 🚀 Quick Start

### Prerequisites
-   **Rust Toolchain**: You need to have Rust and Cargo installed. If not, follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1.  **Clone the repository**
    ```bash
    git clone https://github.com/NotJuu/JuDo-cli.git
    cd JuDo-cli
    ```

2.  **Build and install the CLI tool**
    You can install JuDo-cli globally to make it available in your path:
    ```bash
    cargo install --path .
    ```
    Alternatively, you can build a release binary and place it in your path manually:
    ```bash
    cargo build --release
    # The executable will be found at target/release/judo-cli
    # You might want to move it to a directory in your PATH, e.g., /usr/local/bin
    # mv target/release/judo-cli /usr/local/bin/
    ```

### Usage

Once installed, you can start managing your todos but only if I coded that:

```bash
# Get help information
judo --help

# Add a new todo
judo add "Buy groceries"

# List all todos
judo list

# Mark a todo as done (replace <ID> with the todo's numerical ID from 'judo list')
judo done 1

# Remove a todo
judo rm 2
```

## 📁 Project Structure

```
JuDo-cli/
├── src/                # Source code directory
│   ├── main.rs         # Main entry point for the CLI application
│   └── (other modules) # Internal modules for task logic, data handling, etc.
├── .gitignore          # Git ignore file
├── Cargo.toml          # Rust package manifest and dependency declarations
└── Cargo.lock          # Cargo's lock file for exact dependency versions
```

## ⚙️ Configuration

JuDo-cli stores your todo list data in a local file. By default, this file is located in your user's configuration directory (e.g., `~/.config/judo-cli/tasks.json` on Linux/macOS, `C:\Users\<User>\AppData\Roaming\judo-cli\tasks.json` on Windows).

### Environment Variables
No specific environment variables are required or directly configured by JuDo-cli for its core functionality.

## 🔧 Development

### Available Commands
-   `cargo build`: Compiles the project in debug mode.
-   `cargo build --release`: Compiles the project in release mode (optimized).
-   `cargo install --path .`: Installs the current project's binary locally.
-   `cargo run`: Runs the project directly from source.

### Development Workflow
1.  **Clone the repository**.
2.  Navigate to the project root.
3.  Use `cargo run -- <COMMAND> [OPTIONS]` to test changes during development without installing globally.
    *   Example: `cargo run -- add "Develop new feature"`

## 🧪 Testing

(No explicit test files or commands detected, but for a Rust project, `cargo test` is the standard.)

```bash
# Run unit and integration tests
cargo test
```

## 🤝 Contributing

We welcome contributions to JuDo-cli! If you have suggestions or want to improve the tool, please feel free to fork the repository and submit a pull request.

### Development Setup for Contributors
1.  Fork the repository.
2.  Clone your forked repository: `git clone https://github.com/YOUR_USERNAME/JuDo-cli.git`
3.  Set up your development environment using the instructions in the [Development](#-development) section.
4.  Commit your changes following conventional commit guidelines.
5.  Push your changes and open a pull request.

## 📄 License

This project is currently unlicensed. <!-- TODO: Add a LICENSE file (e.g., MIT, Apache 2.0) -->

## 🙏 Acknowledgments

-   The Rust Programming Language team for creating such an amazing language and ecosystem.
-   [clap](https://crates.io/crates/clap) for robust command-line argument parsing.
-   [tokio](https://crates.io/crates/tokio) for the asynchronous runtime.
-   [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) for efficient data serialization.
-   [anyhow](https://crates.io/crates/anyhow) for simplified error handling.
-   [home](https://crates.io/crates/home) for finding user home directories.

## 📞 Support & Contact

-   🐛 Issues: [GitHub Issues](https://github.com/NotJuu/JuDo-cli/issues)

---

<div align="center">

**⭐ Star this repo if you find it helpful!**

Made with ❤️ by [NotJuu](https://github.com/NotJuu)

</div>
