# Send-Reqwest

Reads from stdin and sends for each line a get request. Uses the following parameters for the reqests:

- 2 seconds timeout
- 2 retries for retryable requests

## Installation Notes

### Requirements

- Rust _(edition 2021)_
- Cargo

### Rust

We suggest that you install Rust using the 'rustup' tool. Rustup will install
the latest version of Rust, Cargo, and the other development tools.

Follow the instructions at [Installing
Rust](https://www.rust-lang.org/tools/install).

For Mac users, Homebrew is also an option.  The Mac Homebrew command is `brew
install rustup` and then `rustup-init`. See [Mac
Setup](https://sourabhbajaj.com/mac-setup/Rust/) & [Installing
Rust](https://www.rust-lang.org/tools/install) for more details.

After installation, you should have `rustc`, `cargo`, & `rustup`. You should
also have `~/.cargo/bin` in your PATH environment variable.

## Usage

You can run the application through cargo:

    cargo run

## FAQ

### What are retryable requests?

Requests are considered retryable according to the rules implemented in

    reqwest-retry::Retryable::from_reqwest_response

## Contributing

### How to build the software

Cargo is used as the package manager and build system for `send-reqwest`.

    $ git clone https://github.com/Trust-NICKOL/send-reqwest.git
    $ cd send-reqwest
    $ cargo build --release

### How to test the software

The unit-tests of this repository can be used to test the functionality of this library.

    cargo test

### Known issues

There are currently no know issues with this tool.

### Getting help

Please you the issue tracker of the github repository if you have any problems using the library.

### Getting involved

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

For more information see [CONTRIBUTING](CONTRIBUTING.md).

### License

This software is released under the MIT License.
