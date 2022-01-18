# Send-Reqwest

Reads from stdin and sends for each line a get request. Uses the following parameters for the reqests:

- 2 seconds timeout
- 2 retries for retryable requests

## Installation Notes

### Rust

We suggest that you install Rust using the 'rustup' tool. Rustup will install
the latest version of Rust, Cargo, and the other binaries used by Solana.

Follow the instructions at [Installing
Rust](https://www.rust-lang.org/tools/install).

For Mac users, Homebrew is also an option.  The Mac Homebrew command is `brew
install rustup` and then `rustup-init`. See [Mac
Setup](https://sourabhbajaj.com/mac-setup/Rust/) & [Installing
Rust](https://www.rust-lang.org/tools/install) for more details.

After installation, you should have `rustc`, `cargo`, & `rustup`. You should
also have `~/.cargo/bin` in your PATH environment variable.

### Git Repository

Clone the 'send-reqwest' repository into your development machine:
```bash
$ cd /path/to/your/work/folder/
$ git clone https://github.com/Trust-NICKOL/send-reqwest.git
$ cd send-reqwest
```

### Run the application

You can run the application through cargo:

    cargo run


## FAQ

### What are retryable requests?

Requests are considered retryable according to the rules implemented in

    reqwest-retry::Retryable::from_reqwest_response
