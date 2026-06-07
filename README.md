# 🦀 Learning Rust

> Systems programming. Memory safety. Zero-cost abstractions.

This repository documents my journey learning Rust from scratch — focusing on systems thinking, ownership, and data structures.

---

## 🎯 Goals

- Understand Rust ownership & borrowing deeply  
- Write safe systems-level code  
- Implement DSA problems in Rust  
- Transition from C/C++ mindset to Rust mindset  
- Build towards real-world projects (backend, embedded, performance systems)

---

## 📚 What I’m Covering

### 🟢 Basics
- [x] Variables & mutability  
- [x] Data types  
- [x] Arrays, Strings, Slice 
- [x] Loops  
- [x] Functions  
- [ ] Vectors  

### 🧠 Core Rust Concepts
- [x] Ownership  
- [x] Borrowing  
- [ ] Lifetimes  
- [ ] Moves vs Copy  
- [ ] Stack vs Heap  
- [ ] Smart Pointers

## 🚀 Why Rust?

- Memory safety without garbage collection  
- Compile-time guarantees  
- Concurrency without data races  
- Modern tooling  
- Performance comparable to C++  

---

## 📁 Repository Structure

The repository is organized into distinct categories:

* **[basics/](file:///home/jemin/Projects/Learning-Rust/basics)** - Fundamental syntax and language basics.
  * [basics.rs](file:///home/jemin/Projects/Learning-Rust/basics/basics.rs): Variables, data types, loops, strings, and slice basics.
  * [CompoundDataTypes.rs](file:///home/jemin/Projects/Learning-Rust/basics/CompoundDataTypes.rs): Compound types (tuples, arrays).
  * [functions.rs](file:///home/jemin/Projects/Learning-Rust/basics/functions.rs): Functions and parameters.
* **[concepts/](file:///home/jemin/Projects/Learning-Rust/concepts)** - Core Rust features and memory model concepts.
  * [ownership.rs](file:///home/jemin/Projects/Learning-Rust/concepts/ownership.rs): Ownership rules, moves, and scopes.
  * [references-borrowing.rs](file:///home/jemin/Projects/Learning-Rust/concepts/references-borrowing.rs): Borrowing with references, mutable vs immutable rules.
* **[projects/](file:///home/jemin/Projects/Learning-Rust/projects)** - Hands-on mini-projects.
  * **[load_img/](file:///home/jemin/Projects/Learning-Rust/projects/load_img)**: Terminal ASCII Art Renderer.
  * **[video_player/](file:///home/jemin/Projects/Learning-Rust/projects/video_player)**: Terminal ASCII Video Player.

---

## 🛠️ Projects

### 🖼️ Terminal ASCII Art Renderer (`load_img`)

A CLI tool that reads a JPEG image (like `cat.jpg`), resizes it using nearest-neighbor scaling, and maps its grayscale intensity values to ASCII characters to render the image directly inside the terminal.

#### Run Instructions:

```bash
# From the repository root, run:
cargo run --manifest-path projects/load_img/Cargo.toml
```

### 🎥 Terminal ASCII Video Player (`video_player`)

A CLI tool that downloads a YouTube video using `yt-dlp`, extracts individual frames using `ffmpeg`, resizes them, and streams them smoothly in monochrome ASCII art directly inside the terminal. Temporary files are automatically cleaned up.

#### Run Instructions:

```bash
# From the repository root, run:
cargo run --manifest-path projects/video_player/Cargo.toml
```

