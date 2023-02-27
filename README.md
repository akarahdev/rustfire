# Rustfire 0.3.0
![open issues](https://img.shields.io/github/issues-raw/akarahdev/rustfire?style=plastic)
![rustfire version](https://img.shields.io/crates/v/rustfire?style=plastic)
![maintain status](https://img.shields.io/maintenance/yes/2023?style=plastic)
![docs status](https://img.shields.io/docsrs/rustfire/0.2.0?style=plastic)
![chat](https://img.shields.io/discord/1079430616082288680?style=plastic)

**Note:** This project is **not** ready for full usage. Please refrain from usage on public plots until 1.0.0. This is because syntax is constantly changing and new systems are being added and removed. Also, this project is meant for use on a Minecraft server known as DiamondFire. This won't be very useful to you if you don't play on it.

RustFire is a project coded in Rust. It allows users to create DiamondFire code templates through the use of the Rust programming language. 



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
// Import Boilerplate for RustFire. Recommended in all projects to minimize boilerplate.
use rustfire::*;
use rustfire::build::build::{BuildFlags, rf_build_flags};
use rustfire::types::item::Item;
use rustfire::build::build::build;

// Code to be ran. This is usually split into multiple functions.
fn main() {
    // build flags so the RustFire compiler knows things
    rf_build_flags(BuildFlags {
        template_name: "Join with Parameters"
    });
    player_event!(Join); // join event
    target!(AllPlayers); // change targeting to all players
    player_action!(SendMessage Item::Text("&a%default &7has joined!")); // send message to all players
    target!(Default); // change targeting to default / none
    function!(CallParams "PVPKit", Item::Text("StandardKit")); // call function with parameters
    build(); // build function
}
```
Check the examples directory for more examples.

# Contribution
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License
This project falls under the MIT license. Read it at `LICENSE.txt`

# Documentation
coming soonTM