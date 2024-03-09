# MacLeon

![MacLeon logo](macleon_logoe.png)

## Overview

MacLeon is a command-line utility designed to clean up [AppleDouble files](https://en.wikipedia.org/wiki/AppleSingle_and_AppleDouble_formats) (`._*`) created by macOS when files are transferred to non-HFS file systems. Inspired by the meticulous "cleaner" from Luc Besson's film "Léon" and a wordplay combining Mac and the immortal Highlander McLeod, MacLeon helps maintain cleanliness in your directories by removing unnecessary metadata files that clutter shared volumes or external storage devices.

## Features

- **Recursive Scanning:** Effortlessly scans directories and their subdirectories for AppleDouble files.
- **Cross-platform Compatibility:** Designed in Rust, MacLeon can be built and run on various operating systems, offering wide usability.
- **Simple CLI:** Easy-to-use command-line interface, making it suitable for both beginners and advanced users.

### wish-list

- **Safe Deletion:** Safely removes `._*` files, ensuring that only the unnecessary metadata files are deleted.
- **Dry Run Option:** Preview which files would be deleted without actually removing them, perfect for validation before cleanup.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. You can check your Rust installation by running:

```bash
rustc --version
```

If Rust is not installed, follow the instructions on the official Rust website to set it up.

### Installation

Clone the repository:

```bash
git clone https://github.com/tomboulier/macleon.git
```

Navigate to the project directory:

```bash
cd macleon
```

Build the project:

```bash
cargo build --release
```

The executable will be located in ./target/release/macleon. Make it accesible to your `PATH`:

```bash
sudo mv target/release/macleon /usr/local/bin
sudo chmod +x /usr/local/bin/macleon
```

Verify the installation:
```bash
which macleon
```

### Usage

To clean a directory and its subdirectories of AppleDouble files, simply run:

```bash
macleon /path/to/directory
```