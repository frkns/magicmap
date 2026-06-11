# magicmap

A magic constant searcher for fast hash maps.

Given a list of 64-bit integers, it searches for a `magic` multiplier and
`shift` such that every key maps to a unique `index` in a flat table: `index = (key * magic) >> shift`.

## build

Run the build script. It installs Rust automatically if you don't have it, then
produces `./magicmap` at the repo root:
```sh
./build.sh
```

(Windows: run it under Git Bash or WSL.)

Alternatively, use `cargo run --release` to run directly.

## usage
Pass your integer keys to stdin. 
```sh
./magicmap < keys.txt
```

By default indexing is multiply-and-shift with `index = (key * magic) >> shift`.

Use `./magicmap --mod` flag to switch to modulus backend, i.e. `index = key % magic`.

## input format
Provide one 64-bit integer per line to stdin. 

Blank lines are ignored. Errors if passed keys are not distinct.

Example `keys.txt`:
```
123
77777
42
67
-1
```

Negative integers are casted to their unsigned representation assuming two's complement, so `-1` will be treated as `18446744073709551615`.

## output
Something.
