https://www.freecodecamp.org/news/how-to-write-a-good-readme-file/

# Rustfire
Project Status: **Active**  

RustFire is a project coded in Rust. It allows users to create DiamondFire templates through the use of the Rust programming language.

# Installation
You can either type the following into your command prompt:
```
cargo add rustfire
```
or add the following to your cargo.toml:
```toml
[dependencies]
rustfire = "1.0.0"
```
Dependencies of this will be installed automatically.

# Usage
Here is a basic program written in RustFire.
```rust
use rustfire::*;

#[rustfire_event]
fn player_join() {
    player_event!(join);
    player_action!(send_message "Hello %default!");
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