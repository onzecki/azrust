# azrust
Simple and an even *faster* file finder than [**turtureanu/az**](https://github.com/turtureanu/az) made in [**Rust**](https://www.rust-lang.org/).

[![Rust](https://github.com/onzecki/azrust/actions/workflows/rust.yml/badge.svg)](https://github.com/onzecki/azrust/actions/workflows/rust.yml)
## Usage

```man
A quick and easy-to-use file finder made with Rust.

Usage: azrust [OPTIONS] [PATTERN] [PATH]

Arguments:
  [PATTERN]  Pattern to search for
  [PATH]     Path to start the search from

Options:
  -d, --detail   Results return path, filename, size, date modified, and file type
  -j, --json     Output results in JSON format
      --hidden   Search hidden files and directories
  -h, --help     Print help
  -V, --version  Print version
```

## Building azrust

To build azrust, you can use [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager and build tool. Follow these simple steps:

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/onzecki/azrust.git
   ```

2. Navigate to the project directory:

   ```bash
   cd azrust
   ```

3. Build the project using Cargo:

   ```bash
   cargo build
   ```

4. Once the build process is complete, you can find the compiled binary in the target/debug/ directory:

   ```bash
   cd target/debug
   ./azrust -h
   ```
