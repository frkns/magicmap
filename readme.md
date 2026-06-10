# magicmap

Blazingly fast static hash maps for fixed sets of integer keys.

Given a list of 64-bit integers, it searches for a `magic` multiplier and
`shift` such that every key maps to a unique `index` in a flat table: `index = (key * magic) >> shift`.

## build
With `make`. (Requires Rust.) It will produce `./magicmap` at root.

## usage
Pass your integer keys to stdin. 
```sh
./magicmap < keys.txt
```

By default indexing is multiply-and-shift with `index = (key * magic) >> shift`.

Use `./magicmap --mod` flag to switch to modulus backend, i.e. `index = key % magic`.

## input format
Provide one 64-bit integer per line to stdin. Negative integers are casted to their unsigned representation assuming two's complement.

Blank lines are ignored. Errors if passed keys are not distinct.

`keys.txt`:
```
123
77777
42
67
```

## output
Something.
