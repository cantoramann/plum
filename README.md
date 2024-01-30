# Obsidian Embedding Storage System

## Introduction

This project is currently under active development and aims to provide a customizable embedding mechanism for Obsidian notes, and other plugins later on. The motivation behind Plum is to provide a plugin-based, fully local and customizable agent worklfow in the Rust ecosystem.

## Project Structure

The repository is organized as follows:

- `src/`: The core directory containing the source code.
  - `aggregator/`: This component is responsible for the collection and preparation of data.
  - `embeddings/`: Contains algorithms for generating and handling embeddings.
  - `obsidian/`: Implementation details specific to Obsidian note structures.
  - `main.rs`: The main executable for the Rust application.
- `target/`: The build directory where the compiled project is placed.
- `.gitignore`: Lists files and directories to be ignored by version control.
- `Cargo.lock`: A manifest that describes the exact versions of dependencies used in the project.
- `Cargo.toml`: Defines project metadata and dependencies.
- `embeddings_test.txt`: A file for testing the output of the embedding generation.
- `README.md`: Provides essential information about the project and serves as a guide for users and contributors.

## Installation and Usage

To utilize the system, you will need to have Rust and Cargo installed. The following steps will guide you through the setup:

1. Clone this repository to your local environment.
2. Change to the repository directory in your terminal.
3. Build the project with `cargo build --release` to compile the source code.
4. Run the application using `cargo run --release`.

Please note that the system is designed to process notes created in the Obsidian markdown editor. Ensure your notes are accessible to the application before running it.

## Contributing

This project welcomes contributions. We encourage you to contribute to the development of this system by submitting pull requests with detailed descriptions of your proposed changes and the benefits they bring.

## Status: Work in Progress

This project is a work in progress. Features and documentation are subject to change. We are working diligently to achieve a stable release and appreciate your patience and support.

## License

MIT.
