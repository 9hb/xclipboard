# xc
 
Simple tool for sharing commands between terminals. Saves a command to a file and runs it on demand.
 
![XC demo](.assets/showcase.gif)
 
## Installation
 
```bash
git clone https://github.com/9hb/xclipboard.git
cd xclipboard
cargo build --release
cp target/release/xc ~/.local/bin/
```
 
On Windows, copy `target\release\xc.exe` somewhere in your PATH.
 
## Usage
 
```bash
# save a command
xc ls -la ~/.config
 
# run the saved command
xc -p
```
 
The command is stored in `~/.xc_clipboard`. When run with `-p`, `xc` exits with the same exit code as the original command.
 
## Requirements
 
- Rust 1.85+
## License
 
MIT
 
