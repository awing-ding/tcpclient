# Usage

Depending on your screen, you may have to edit the size of the buffer at app.rs:30.
To run the script, use the following command when in the root directory.

```bash
cargo run --release -- <ip:port> <pseudo>
```

you can also build the project with

```bash
cargo build --release
```

and run the binary directly with

```bash
./target/release/tcpland <ip:port> <pseudo>
```

# Commands

There's really few shortcut :

## Normal mode
- `<c>` Clears the console
- `<q>` exits
- `<i>` enter edit mode
- `<j> or <down>` scroll down
- `<k> or <up>` scroll up
## Edit mode
- `<Enter>` send message
- `<Escape>` return to normal mode
