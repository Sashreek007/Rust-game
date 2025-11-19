# The Rusty Dungeon

A CLI-based dungeon crawler game built to practice and demonstrate the core concepts of the Rust programming language.

## Project Goal
The primary goal of this project was to move beyond theory and implement Rust's unique memory management system in a practical scenario. I specifically focused on:
- **Ownership**: Managing the lifespan of game entities.
- **Borrowing**: Safely passing data between modules without taking ownership.
- **References**: Understanding the difference between Shared (`&`) and Mutable (`&mut`) references.

## Tech Stack
- **Language**: Rust
- **Build Tool**: Cargo
- **Interface**: Command Line Interface (CLI)

## Key Concepts Applied

### 1. Ownership & Modules
The project is split into three modules (`main`, `player`, `map`).
- `main.rs` holds **ownership** of the `Player` and

**Run directly:**
```bash
cargo run
```
