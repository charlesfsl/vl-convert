# Reproducing segmentation fault locally

## Setup Rust environment
I added a `flake.nix` file to set this up for me, but do however you want.

1. Install [Nix](https://nixos.org/download.html)
1. Enable flakes by adding `experimental-features = nix-command flakes` to `~/.config/nix/nix.conf`
1. start nix dev environment:

    ```bash
    nix develop
    ```

1. Run with or without debugger
  - Without debugger:

    ```bash
    cd vl-convert-rs
    cargo run --example conversion_sequence
    ```

  - With debugger:

    ```bash
    rust-lldb target/debug/examples/conversion_sequence
    ```
