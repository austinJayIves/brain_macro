# brainiac_22ccb748d608358e

brainiac\_22ccb748d608358e is an implementation of the Brainfuck esoteric computer language in Rust's macro\_rules. 

It is named brainiac\_22ccb748d608358e to ensure that if there is a crate with a more apt need for the "brainiac" name, that this
"fun" crate doesn't take its place.

## Implementation

Brainfuck is a relatively simple language.

A program is initialized with a tape of 30,000 bytes, starting at index 0.

There are 8 commands available:
- `>`: Increment the index on the tape by 1
- `<`: Decrement the index on the tape by 1
- `+`: Increment the value at the current tape index by 1
- `-`: Decrement the value at the current tape index by 1
- `.`: Output the value at the current tape index to stdout
- `,`: Assign the tape at the current index a  u8 value off of stdin
- `[`: If the value at the current index of the tape is 0, move to the first command after a matching ']' character
- `]`: Jump back to the matching '[' character

## Limitations
There are several limitations due to how Rust interprets tokens in a macro:
- `>>` must be `> >`
- `->` must be `- >`
- `<<` must be `< <`
- `<-` must be `< -`
- `..` must be `. .`

This ensures that there aren't any symbols in invalid locations.

## Usage

Usage should be as easy as adding the package to your `Cargo.toml` manifest and
importing the brainfuck macro:

```rust
use brainiac_22ccb748d608358e::brainfuck;

fn main() {
    brainfuck! { [This does absolutely nothing except make the compiler do extra work.] }
}
```
