# CLI: Password Generator with Rust
### Author:  
Arnau Muñoz Barrera

A simple yet illustrative Rust program that generates a password based on user-provided keywords.  
The project focuses on interaction through the terminal, hashing, randomness, and core Rust concepts.

---

## 🔧 Prerequisites  
- **Rust** (v1.60 or higher recommended)  
- **Cargo** (bundled with Rust)  
- Any Rust-friendly IDE such as VS Code, IntelliJ IDEA with Rust plugin, or CLion.

---

# Rust Terminal Interaction Example  
This project demonstrates how to read and process user input at runtime using Rust’s standard library.

The program continuously collects keywords from the user, interpreting commands like **Exit** or **Quit**, and then generates a password using hashing and pseudo-random ordering.  
It highlights Rust’s ownership model, safe input handling, and basic CLI user experience.

---

# Password Generation Logic  
This project includes a simple password generator that uses:

- **Hashing:** Keywords are hashed using Rust's built-in `DefaultHasher`.  
- **Randomization:** The hashed values are shuffled using `SliceRandom` and `thread_rng`.  
- **Composition:** A final password is created by concatenating the first bytes of each hashed keyword.

This approach demonstrates how low-level hashing and random utilities can be combined in Rust to create deterministic-yet-random results.

---

# CI Usage  
You can integrate Continuous Integration using GitHub Actions for:  
- Running all tests on each push  
- Enforcing `rustfmt` and `clippy` checks  
- Ensuring the project builds without warnings  
- Preventing merges when tests fail  

---

## ⚙️ Setup  

**Clone the repository:**  
```sh
git clone <repository-url>
cd <repository-directory>
