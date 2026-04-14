# Git Phrase based explorer:
This is a simple tool which can be utilized to search all the changes in a specific file and check for commits which contain some certain phrases which the user wishes to identify and analyze.


# Installing [Rust](https://rust-lang.org/) on your system:

## Windows:
To install Rust on Windows, the easiest and recommended way is to use `rustup`, the official Rust toolchain installer. [rust-lang](https://rust-lang.org/learn/get-started/)

***

### Step 1: Download the Windows installer

1. Open a browser and go to the official Rust setup page:  
   `https://www.rust-lang.org/tools/install` [rust-lang](https://rust-lang.org/tools/install/)
2. On Windows, this page will automatically show you the **Rust‑Windows (64‑bit)** `.exe` installer.  
3. Click the link and download `rustup‑init.exe`. [rust-lang.github](https://rust-lang.github.io/rustup/installation/windows.html)

***

### Step 2: Run the installer

1. Open **Command Prompt** (or PowerShell) as a normal user.  
2. Navigate to your `Downloads` folder:  
   ```bash
   cd %USERPROFILE%\Downloads
   ```  
3. Run the installer:  
   ```bash
   rustup-init.exe
   ```  
   (If you prefer just double‑clicking, that also works and opens a terminal‑style wizard.) [doc.rust-lang](https://doc.rust-lang.org/book/ch01-01-installation.html)

4. Choose the default installation (option `1` usually), which will:
   - Install the latest stable Rust toolchain (`rustc`, `cargo`, `rustup`).  
   - Add the required environment‑path modifications. [rust-lang.github](https://rust-lang.github.io/rustup/installation/windows.html)

Wait for the installer to finish; it tells you when it’s done.

***

### Step 3: Verify installation

Close and reopen your terminal or PowerShell, then run:

```bash
rustc --version
```

and

```bash
cargo --version
```

If you see versions printed (for example `rustc 1.x.x` and `cargo 1.x.x`), Rust is correctly installed. [rust-lang](https://rust-lang.org/learn/get-started/)

***

### (Optional) First test project

To test, create a simple “Hello, world” project:

```cmd
cargo new hello_world
cd hello_world
cargo run
```

This creates a new Rust project and runs it; you should see `Hello, world!` printed. [doc.rust-lang](https://doc.rust-lang.org/book/ch01-01-installation.html)

If you tell me whether you’re on Windows 10 or 11 and want **MSVC‑based** (Visual Studio) or **GNU‑based** (MinGW) toolchain, I can give you the exact triplet choice in `rustup‑init`.

<br><br>

## Linux:
### Using rustup (Recommended):
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation:

Close and reopen your terminal, then run:

```bash
rustc --version
```

and

```bash
cargo --version
```

If you see versions printed (for example `rustc 1.x.x` and `cargo 1.x.x`), Rust is correctly installed.

### (Optional) First test project

To test, create a simple "Hello, world" project:

```bash
cargo new hello_world
cd hello_world
cargo run
```

This creates a new Rust project and runs it; you should see `Hello, world!` printed.



<br><br>

## Command Usage:

cargo run -- --repo <REPO_PATH> --file <FILE_PATH> --phrase <PHRASE> [--max-count <N>] [--verbose]


## Command Structure description:

The tool accepts the following command-line arguments:

- `-r, --repo <REPO_PATH>`: Path to the Git repository to analyze (required).
- `-f, --file <FILE_PATH>`: Path to the specific file within the repository to search for changes (required).
- `-p, --phrase <PHRASE>`: The phrase to search for in the file's diff lines (required).
- `-m, --max-count <N>`: Maximum number of matching commits to return (optional, default: 5).
- `-v, --verbose`: Enable verbose output (optional, default: false).


## Command example:
```bash
cargo run -- --repo "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test" --file "file1.txt" --phrase "UPDATED FILE IN branch2 changes" --max-count 1 -v
```

