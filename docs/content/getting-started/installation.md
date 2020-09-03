---
title: "Installation"
date: 2020-09-02T23:40:31-06:00
draft: true
weight: 1
---

### Installation
Ahhh... The best part of using any software: the installation. Hydrogen is made with Rust, a cross-platform language.
#### Rustup & Rust & Cargo
To start, first install [rustup](https://rustup.rs):
```bash
# Mac OS X / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
echo Click the link and follow the on screen instructions\!
```

#### Building
Simply invoke ```cargo build --release``` in the directory that you cloned Hydrogen into.

#### Installing
Invoke ```cargo install``` to install Hydrogen (or the ```hy``` executable) to your path for future use.
