# CLIp

CLIp is a CLI (Command Line Interface) assistant inspired by the classic 90s office assistants. This project is a complete **rewrite** of the original [clip_assistant](https://github.com/Tasesho/clip_assistant), now built from scratch using **Rust**.

The goal is to provide a lightweight, cross-platform tool while learning Rust's memory safety and concurrency.

> **Note:** We are keeping a clean commit history. This is a fresh start!, For more information please visit: [CONTRIBUTING.md](./CONTRIBUTING.md)

## Main features!

The User can Interact with the Command Prompt. Following the next commands:

##### Hello
```CLIp salutes the User with a friendly message.```

##### Help
```Deploys a list with all the avaliable commands CLIp can do.```

##### GiveAdvice
```CLIp gives you an advice (in later updates there will be more)```

##### ShowHour
```show the actual date time.```


## Why CLIp?

CLIp was born as an educational tool to bridge the gap between users and the terminal. By interacting with a friendly CLI assistant, users can become more comfortable with command-line environments while learning how modern software is built.


## How it was developed

CLIp is a complete re-engineering of the original C++ version. While the first iteration relied on heavy Windows-specific APIs (`windows.h`) and manual memory management, this version follows a **Modern Systems Programming** approach.

We decided to rewrite it from scratch in **Rust** to achieve:
- **Memory Safety:** Eliminating segmentation faults and data races at compile time.
- **Cross-Platform Compatibility:** Moving away from hardcoded Windows paths to an agnostic architecture that runs natively on Linux, macOS, and Windows.
- **Modular Design:** Using Rust's module system to separate the "personality" (logic) from the core system interaction.

## Technologies Used

| Technology | Purpose |
| :--- | :--- |
| **Rust** | Core language for safety and performance. |
| **Cargo** | Package management and build system. |
| **Docker** | Containerization for a consistent development environment. |


## Links & Resources

* **Project History:** [Changelog](./CHANGELOG.md) - Stay updated with the latest changes.
* **Contributing:** [Contributing Guide](./CONTRIBUTING.md) - Learn how to join the team.
* **Legacy Project:** [Original C++ Version](https://github.com/Tasesho/clip_assistant) 
* **Learning Rust:** [The Rust Programming Language](https://doc.rust-lang.org/book/) 
* **Docker Docs:** [Docker Documentation](https://docs.docker.com/)