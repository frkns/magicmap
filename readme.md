# magicmap

Blazingly fast static hash maps for fixed sets of integer keys.

Given a list of unsigned 64-bit integers, it searches for a `magic` multiplier and
`shift` such that every key maps to a unique `index` in a flat table: `index = (key * magic) >> shift`.

## build
With `make`. (Requires Rust.) It will produce `./magicmap` at root.

## usage
Pass keys to stdin.
```sh
./magicmap < keys.txt
```

## input format
Provide one unsigned 64-bit integer per line to stdin. Blank lines are ignored.

`keys.txt`:
```
123
77777
42
67
```

## output
Something.
