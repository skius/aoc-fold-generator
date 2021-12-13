# Advent of Code 2021 - 13 - Fold Generator

## Usage
Put the desired text in `input.txt`. For a list of supported characters, see `main.rs`.

Run:
```shell
$ cargo run -- NUM_FOLDS
```

where `NUM_FOLDS` is the number of folds (well, unfolds) you want to generate.

The output will then be printed to STDOUT, if it's too long for your terminal's buffer, you can redirect it with
```shell
$ cargo run -- NUM_FOLDS > output.txt
```

See [this gist](https://gist.github.com/skius/d4c3cc4fcd8b199f1603c411714f220f) for a sample output.

## Example

![example](showcase.png)