# REGP CPU Emulator - Rust port

Idk, I'm learning Rust and wanted to port one of my projects to it.

## Running example

``` bash
# to assemble code and see the results
cargo run -- assemble examples/code/double.asm

# to assemble code and save it to a file
cargo run -- assemble examples/code/double.asm > path/to/save/the/result

# to assemble and run
cargo run -- assemble-and-run examples/code/double.asm


# to run code already assembled
cargo run -- run examples/compiled/double
```