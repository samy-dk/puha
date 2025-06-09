# puha

This workspace contains:

- **puha-lib**: a library providing puha's core functionality.
- **puha**: the command-line interface that wraps the library.

## CLI usage

Run `cargo run -p puha -- <COMMAND>` to manage your spaces. The data is stored
in `space.json` by default. Example commands:

```bash
# create a new root space
cargo run -p puha -- new-root "Home"

# add an item
cargo run -p puha -- add-item --space Home --item "Book" --description "Rust"

# show the entire tree
cargo run -p puha -- show-tree
```
