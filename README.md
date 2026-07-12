# TungstenScore
Windows Security Assessment Tool built with Rust

---

<img width="708" height="721" alt="스크린샷 2026-07-12 134220" src="https://github.com/user-attachments/assets/4213f82d-d6ec-440f-a497-53f79cd7fbc2" />

## Quick start

### Prerequisites
* Rust / [cargo](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installation) (1.88.0 or later)
* Windows PowerShell

### Instructions
**Download or clone** the source code, navigate to the directory, and **run the following command**:

```
cargo run
```

## Supported OS
* Windows 10 (**22H2 or later**)
* Windows 11

## Features
* **User Account Control (UAC) Verification**: Instantly checks and reports the current UAC status and elevation policies on Windows.
* **Lightweight & Standalone**: Built entirely in Rust, requiring no heavy external runtimes or dependencies.
* **Fast Execution**: Blazing fast security assessment optimized for PowerShell environments.

## Project plan
1. Expand Security Checks

1. Firewall configurations

1. Windows Update and version verification

1. Auto-Fix Feature: Automatically resolve discovered security vulnerabilities.

1. Graphical User Interface (GUI): Modern desktop UI for non-technical users.

## How it works (Why Rust?)
Unlike traditional scripts that rely on external interpreters, **TungstenScore** is compiled into a standalone native binary using Rust. 

* **Zero Dependencies**: The tool runs instantly as a standalone binary without requiring any pre-installed runtime or environment.
