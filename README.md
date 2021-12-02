# albw-randomizer

A library for randomzing The Legend of Zelda: A Link Between Worlds.

## Building from source

You must have the Rust programming language installed from <https://rustup.rs>. Once you have install Rust, enter the following commands into the terminal of your choice.

```cmd
git clone https://gitlab.com/marsolk/albw-randomizer
cargo build
```

## Testing

Some tests require a copy of the North American ROM in the root folder of the workspace as `test.3ds`. You can copy the file to this directory, or use a symbolic link on a Linux or Unix system. To run all tests, enter this command:

```cmd
cargo test
```

## Front-end

To build and run the command-line front-end, use the following commands:

```cmd
git clone https://gitlab.com/marsolk/albw-randomizer-cli
cargo run
```

## License

This program is licensed under the GNU General Public License v2.0.
