# checkemail

[![Rust](https://github.com/royge/checkemail/actions/workflows/rust.yml/badge.svg)](https://github.com/royge/checkemail/actions/workflows/rust.yml)

## Basic Email Validation Checker

This tool is created to help my wife in validating the email(s) provided by
their client.

__DISCLAIMER__

I used ChatGPT (Mar 23, 2023 release) to the start this program and improve it
because it won't compile back then. :-)

## How To Build

   ```bash
   cargo build --release
   ```

   **Build Windows Executable from Mac M1**

   _Source:_ [Stack Overflow](https://stackoverflow.com/questions/67061283/compile-a-rust-program-to-an-exe-using-an-m1-mac)

   ```bash
   brew install mingw-w64
   rustup target add x86_64-pc-windows-gnu
   rustup toolchain install stable-x86_64-pc-windows-gnu
   cargo build --release --target=x86_64-pc-windows-gnu
   ```

## How To Use

   **On Unix**

   ```bash
   ./checkemail user@email.com
   ```

   **On Windows**

   ```bash
   checkemail.exe user@email.com
   ```
