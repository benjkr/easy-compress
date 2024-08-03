# Easy Compression

This is a simple compression algorithm that can be used to compress and decompress files.
The algorithm is very simple and can be used to compress any file.

## Compression

The compression algorithm is a very simple one:
Every sequential value will turn to `NC`. `N` being the number of times `C` accrued.

`AAAAAAABBBCD` -> `7A3B1C1D` (12 Bytes -> 8 Bytes)

## Problems

If the file does not contain a lot of duplicate bytes the compressed file will be at most 2x the original size. We need a way to store the original data without storing an extra byte for the `n` if its less than 3.

```
X a -> 1a (+1 byte)
X aa -> 2a (no change)
V aaa -> 3a (-1 byte)
```

## Command Usage

### Compress

```bash
ez <FILE-TO-COMPRESS>
```

### Decompress

```bash
ez -d <FILE-TO-DECOMPRESS>
```

### Help

```
Usage: ez [OPTIONS] <INPUT>

Arguments:
  <INPUT>

Options:
  -d, --decompress       Decompress a file
  -o, --output <OUTPUT>  Optional output file [default: output]
  -f, --force            Overwrite existing output file
  -h, --help             Print help
  -V, --version          Print versio
```
