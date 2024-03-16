# Templier: a simple templating tool

Templier reads the full standard input and replaces the placeholders with the given arguments.
The result is printed to the standard output.

## Usage

Simple usage:
```bash
$ templier knight sword
Hello {}, here is your {}
```
Output:
```bash
Hello knight, here is your sword
```

With indexed placeholders:
```bash
$ templier knight sword
Hello {1}, here is your {0}
```
Output:
```bash
Hello sword, here is your knight
```

## Build

```bash
$ cargo build
```

For minimal size:
```bash
$ export RUST_TARGET=$(rustc -vV | grep host | cut -d ' ' -f 2)
$ cargo +nightly build -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --target $RUST_TARGET --release
```
