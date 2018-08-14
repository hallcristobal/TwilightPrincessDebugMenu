# Twilight Princess Debug Menu / Practice Tool

## Compiling
Clone this repo and do a `git submodule update --init` to pull down the correct
TP Bindings.

Copy your copy of Twilight Princess (Currently only NTSC-U) to the root folder
for this poject, and rename it to `gz2e01.iso`.

You can choose not rename it if you would prefer, just make sure that the lines in
Romhack.toml match the iso path relative to the toml file.
```Toml
...
[src]
iso = "<iso-path/file-name>"
...
```

### Setup the compiler

You will need rust to compile the libraries and iso, you can download it from
[https://rustup.rs](https://rustup.rs).

Set the compiler to work on the nightly build with
```
rustup override set nightly
```

Add the `powerpc-unknown-linux-gnu` toolchain with
```
rustup target add powerpc-unknown-linux-gnu
```

And then do a install the Romhack Compiler with
```
cargo install --git https://github.com/CryZe/romhack-compiler
```

