# Rust Guessing Game

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)

A simple, interactive number guessing game built with Rust. This project serves as a great introduction to core Rust concepts.

## 🌟 Features

-   **Interactive Gameplay**: Guess a number between 1 and 100.
-   **User Input**: Demonstrates reading from standard input.
-   **Random Number Generation**: Uses the `rand` crate to create a new secret number every game.
-   **Error Handling**: Gracefully handles non-numeric input from the user.
-   **Control Flow**: Utilizes loops (`loop`) and pattern matching (`match`) for game logic.

## 📋 Prerequisites

Before you begin, ensure you have the Rust toolchain installed on your system. If you don't, you can install it using `rustup`.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation.

## 🚀 Getting Started

Follow these steps to get a local copy up and running.

### 1. Clone the Repository

```sh
git clone https://github.com/your-username/guessing_game.git
cd guessing_game
```

### 2. Build and Run

You can run the project directly using Cargo, which will handle compiling and executing the binary.

```sh
cargo run
```

The game will start, and you can begin guessing!

### 3. Build for Release (Optional)

To create an optimized binary for distribution:

```sh
cargo build --release
```

The executable will be located at `./target/release/guessing_game`.

## 🛠️ How It Works

The application logic is contained entirely within `src/main.rs`.

1.  The `rand` crate is used to generate a secret integer between 1 and 100.
2.  The program enters an infinite `loop` to continuously prompt the user for input.
3.  The user's input is read from standard input into a mutable `String`.
4.  The input string is parsed into a 32-bit unsigned integer (`u32`). If parsing fails (e.g., the user enters "hello"), an error message is shown, and the loop continues.
5.  The `match` statement compares the user's guess to the secret number.
6.  Hints ("Too small!" or "Too big!") are provided to the user.
7.  If the guess is correct, a "You win!" message is printed, and the `break` keyword exits the loop, ending the program.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

Please ensure your code is formatted with `cargo fmt` and passes `cargo clippy` checks.

## 📄 License

This project is distributed under the MIT License. See the `LICENSE` file for more information.
