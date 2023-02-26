
# Rustfire
![open issues](https://img.shields.io/github/issues-raw/akarahdev/rustfire?style=plastic)
![rustfire version](https://img.shields.io/crates/v/rustfire?style=plastic)
![maintain status](https://img.shields.io/maintenance/yes/2023?style=plastic)
![docs status](https://img.shields.io/docsrs/rustfire/0.1.0?style=plastic)
![chat](https://img.shields.io/discord/1079430616082288680?style=plastic)

**Note:** This project is **not** ready for full usage. Please refrain from main usage until 1.0.0.

RustFire is a project coded in Rust. It allows users to create DiamondFire templates through the use of the Rust programming language. 

# Installation
You can either type the following into your command prompt:
```
cargo add rustfire
```
or add the following to your cargo.toml:
```toml
[dependencies]
rustfire = "0.1.1"
```
Dependencies of this will be installed automatically.

# Usage
Here is a basic program written in RustFire.
```rust
use rustfire::*;

#[rustfire_event] // This isn't implemented yet, but will be soon.
fn player_join() {
    player_event!(join);
    player_action!(send_message "Hello %default!");
    build!();
}
```
Check the examples directory for more examples.

# Contribution
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

# License
This project falls under the MIT license. Read it at `LICENSE.txt`

# Documentation
"ahhhhhhhhhhh" - hppeng  
*Wise words hppeng, wise words.*