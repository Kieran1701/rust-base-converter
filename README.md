# Introduction

I started teaching myself Rust, so this is just a quick project I wrote to do base conversion among binary, octal, decimal, and hex.  There's really nothing fancy here, and it was just a chance for me to learn the basics of the language.

# Build and Run

## Building

It's a very simple project.  Simply run:
```sh
cargo build
```
This will build the project.

## Running

```sh
./target/debug/base_converter -B|O|D|X|h -b|o|d|x <number>
```
The options coorespond to:
* -B/b: Output/input binary
* -O/o: Output/input octal
* -D/d: Output/input decimal
* -X/x: Output/input hex
* -h: Display help
