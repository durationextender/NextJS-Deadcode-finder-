# dead-code-detector

A fast, Rust-based static analysis tool to find unused exports in Next.js projects.

## Why?
Standard dead-code symbols often miss framework-specific patterns in Next.js (like file-system routing) or struggle with the mapping between `default` exports and named imports.
I wanted to try and build this myself so i can learn how it works. This is not production ready code i know that. Its more abt the concepts i am trying to learn
## Features
- **Next.js Aware:** Automatically whitelists framework magic (page.tsx, layout.tsx, metadata, etc.).
- **Path Resolution:** Handles `@/` aliases and recursive directory scanning.
- **Fast:** Written in Rust for high-performance dependency graphing.

## Installation
Ensure you have the Rust toolchain installed.

git clone [https://github.com/YOUR_USERNAME/dead-code-detector.git](https://github.com/YOUR_USERNAME/dead-code-detector.git)
cd dead-code-detector
cargo build --release or cargo build -> cargo run 

## Usage
Run the binary against your project's root or source directory:

Bash
./target/release/dead-code-detector --input /path/to/your/next-app/src


## Limits
Does not currently parse export * from './file' (re-exports).

Comments inside import/export blocks may cause parsing noise.

Logic is limited to ES Modules (ESM).
