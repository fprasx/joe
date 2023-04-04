# Who is Joe?

## Installation

You can install the `joe` binary by running the following command in your
terminal

```
$ cargo install joe
```
If you'd like, run `joe --version` to ensure it's installed correctly. For more
hardcore developers, you can clone this repository and use `Make` like so:
```
$ git clone https://github.com/fprasx/joe
$ make joe
```
Then once again run `joe --version` if you want to verify the software was
installed correctly.

The `joe` crate also exposes a flexible API which you can use in your Rust
software. To do so, add the following line to your `Cargo.toml`:
```
joe = '1.0.0'
```
or run `cargo add joe` in your terminal.

Note that the `joe` crate has reached `1.0.0` and has a stable API, so you can
use it in production code.

## License
This project is licensed under the GNU General Public License v3.0 or later
(GPLv3), because Joe Mama is for everyone.
