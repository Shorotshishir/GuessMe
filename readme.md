# GuessMe

A simple number guessing game implemented in Rust.

## Description

GuessMe is a command-line game where players try to guess a randomly generated number. This game is described in the [Official Rust Programming Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) as a command line game. This project adds a Ui to the game. [slint](https://slint.dev/) is used for the project.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Shorotshishir/GuessMe.git
cd GuessMe
```

2. Build the project:

```bash
cargo build
```

### Running the Game

```bash
cargo run
```

## How to Play

- The game generates a random number between 0 and 100.
- Enter your guess and receive feedback.
- Continue until you guess correctly.


## UI

This project uses Slint for its graphical user interface.

### Slint Setup

- Ensure you have the required Slint dependencies installed. Check the [Slint documentation](https://slint.dev/) for details.
- The UI is integrated with the game logic. Building the project via Cargo will compile the Slint UI components.

## Contributing

Feel free to fork the repository and submit pull requests.

## License

This project is licensed under the MIT License.
