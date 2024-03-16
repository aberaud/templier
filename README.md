# Templier: a simple templating tool

Templier reads the full standard input and replaces the placeholders with the given arguments.
The result is printed to the standard output.

## Usage

Simple usage:
```bash
$ templier a b c
From {} to {} to {}
```
Output:
```bash
From a to b to c
```

With indexed placeholders:
```bash
$ templier a b c
From {2} to {1} to {2}
```
Output:
```bash
From c to b to c
```
