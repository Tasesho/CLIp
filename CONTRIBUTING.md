# Contributing to CLIp Assistant

Thank you for your interest in contributing! This guide will help you set up your environment and understand our workflow.

## Why Rust?
This project is a migration from an original C++ version. We chose Rust for:
* **Memory Safety:** Secure memory management without performance loss.
* **Modern Tooling:** A centralized package manager with **Cargo** and **rustup**, replacing external tools like CMake.

## Our Goals
* Refactor the legacy C++ logic into idiomatic Rust.
* Ensure true **Cross-Platform** support (Windows, Linux, macOS).
* Provide a consistent environment using **Docker**.

---

## 1. Environment Setup (Docker)
To ensure everyone uses the same development environment and avoid "it works on my machine" issues, we use Docker.

### Prerequisites
- Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) (Windows/Mac) or Docker Engine (Linux).

### Usage
1. **Build the container:** Downloads the Rust image and compiles the environment.
   > `docker-compose build`
2. **Run CLIp:** Starts the assistant in interactive mode.
   > `docker-compose run app`

---

## 2. How to Contribute

### Step 1: Clone and Branch
Clone the repository and create a new branch. **Do not work directly on main**.
```
git clone https://github.com/Tasesho/clip_cloudshare
```
```
git checkout -b branch-name/your-feature-name
```

### Step 2: Commit Standards
We follow a simplified version of Conventional Commits. All commit messages must be in English and use one of the following prefixes:

| Prefix | Description |
| :--- | :--- |
| **Feat** | A new functionality (e.g., Feat: add random greeting) |
| **Fix** | A bug fix |
| **Docs** | Changes to documentation only |
| **Refactor** | Code changes that neither fix a bug nor add a feature |
| **Chore** | Updates to build tasks, Docker, or Git configs |
| **Build** | Changes to Cargo.toml or external dependencies |

---

## 3. Project Roadmap (Phase 1)

The following features are planned for our first release:

- [ ] **Greeting:** Reimplement "Hello" and the random greeting system.
- [ ] **Command Parser:** Create the core engine to handle user input using Rust's `match`.
- [ ] **Time/Date:** Implement `ShowHour` using the `chrono` crate.

---

## Guidelines

To ensure code quality and consistency, please follow these guidelines:

* **English Only:** All code comments, commits, and docs must be in English.
* **Keep it Simple:** Write clean, readable code.
* **Ask for Help:** If the **Borrow Checker** is giving you a hard time, open an Issue or ask in our communication channel.