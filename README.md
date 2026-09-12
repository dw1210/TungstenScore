# TungstenScore
Windows Security Assessment Tool built with Rust

---

<img width="735" height="995" alt="image" src="https://github.com/user-attachments/assets/0a437db6-915c-4733-9619-68cb7369d063" />

## Quick start

### Prerequisites
* Rust / [cargo](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installation) (1.88.0 or later)
* Windows 10 (22H2+) or **Windows 11 (recommeㅜded)**

### Instructions

[Download the .exe file!](https://dw1210.github.io/TungstenScore-website/tungsten_score_v0-5-1.zip)

**or**

Download or clone the source code, navigate to the directory, and **run the following command**:

```powershell
# Download or clone the source code!
git clone https://github.com/dw1210/TungstenScore.git
# Navigate to the directory!
cd Tungstenscore
# Build and run!
cargo run
```

## Project plan
1. Expand security checks
   * Windows version verification
   * Additional checks as needed
2. Add an auto-fix feature
3. Add a graphical user interface (GUI)
4. Continue expanding the assessment scope

## Website
[https://dw1210.github.io/TungstenScore-website/](https://dw1210.github.io/TungstenScore-website/)

## How it works
* No installation required
* Works offline
* Reads the Windows Registry
* Some checks require administrator permissions
* Runs in the CLI
* This app doesn't include many checks yet, but you can add checks in `src/checks`. 


## This project is one of Hack Club Stardance Projects!
Visit [stardance.hackclub.com/projects/32022](https://stardance.hackclub.com/projects/32022)
