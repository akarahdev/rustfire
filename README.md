# Rustfire
![open issues](https://img.shields.io/github/issues-raw/akarahdev/rustfire?style=plastic)
![rustfire version](https://img.shields.io/crates/v/rustfire?style=plastic)
![maintain status](https://img.shields.io/maintenance/yes/2023?style=plastic)
![docs status](https://img.shields.io/docsrs/rustfire/0.2.0?style=plastic)
![chat](https://img.shields.io/discord/1079430616082288680?style=plastic)

**Note:** This project is **not** ready for full usage. Please refrain from main usage until 1.0.0. This is because syntax is constantly changing and new systems are being added/removed.

RustFire is a project coded in Rust. It allows users to create DiamondFire templates through the use of the Rust programming language. 

# Installation
You can either type the following into your command prompt:
```
cargo add rustfire
```
or add the following to your cargo.toml:
```toml
[dependencies]
rustfire = "0.2.0"
```
Dependencies of this will be installed automatically.

After this, you will want to install Python 3 (if you don't have it already) and add the following to the root directory of your project. This file should be called `send.py`:
```py
import socket
import sys
import json

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

try:
    # Connect to server and send data
    sock.connect(("localhost", 31372))
    sock.sendall(bytes(sys.argv[1],encoding="utf-8"))
finally:
    sock.close()
```
This code opens a socket on 31372 for RustFire to send templates through. This will later be implemented as base into RustFire.
# Usage
Here is a basic program written in RustFire.
```rust
use rustfire::*;

// Code to be ran. This is usually split into multiple functions.
fn main() {
    build_set!("Player Join Event"); // Template name. System will soon be reworked.
    player_event!(join); // Player Join Event. 
    select!(all_players); // Select all players. By default, all codeblocks have the Selection target. This system will be reworked soon.
    player_action!(send_message "text::&a&lJOINED! &e%default &7has joined!"); // Send the player a message
    build!(); // Send code to Recode.
}
```
Check the examples directory for more examples.

# Contribution
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

# License
This project falls under the MIT license. Read it at `LICENSE.txt`

# Documentation
coming soonTM