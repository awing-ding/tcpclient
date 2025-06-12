# Usage

To run the script, use the following command when in the root directory.

```bash
cargo run --release
```

you can also specify the IP and port of the server you want to connect to, as well as a pseudo.

```bash
cargo run --release -- <ip:port> <pseudo>
```

you can also build the project with

```bash
cargo build --release
```

and run the binary directly with

```bash
./target/release/tcpland
```

# Commands

There are really few shortcut :

## Normal mode
- `<c>` Clears the console
- `<q>` exits
- `<i>` enter edit mode
- `<j> or <down>` scroll down
- `<k> or <up>` scroll up
## Edit mode
- `<Enter>` send message
- `<Escape>` return to normal mode
