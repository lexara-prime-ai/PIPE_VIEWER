## * * * * Usage * * * *
Generate a file named, myfile with 128kbps of random data.

```shell
dd if=/dev/urandom bs=1024 count=128 of=myfile
```

## Preview file info
```shell
ls -lh
```

### Build a binary
```shell
cargo build
```

## Cat the file then pipe the bytes to pipe-viewer
```shell
cat myfile | target/release/pipeviewer > myfile2
```

## Using the 'yes' utility to test broken pipe error handling
```shell
yes | cargo run | head -n 1 > /dev/null
```

## Preview console output
```shell
yes | cargo run | head -n 100000000 > /dev/null
```

## Running the binary with cargo run
```shell
cargo run -- -h
```

## Testing commandline arguments
```shell
cargo run -- somefile --outfile file.out -s
```

## Testing environment variable with command line arguments
```shell
PV_SILENT=1 cargo run -- somefile --outfile file.out -s
```

## Testing reader and writer
```shell
echo "some input" | cargo run --
```
### Reroute output to /dev/null
```shell
echo "some input" | cargo run -- > /dev/null
```

### Reroute output to outfile, -o /dev/null
```shell
echo "some input" | cargo run -- -o /dev/null
```

### Generate output to outfile, -o output.txt
```shell
echo "some input" | cargo run -- -o output.txt
```

### Reading from a file, output.txt -o /dev/null
```shell
echo "some input" | cargo run -- output.txt -o /dev/null
```

### Reading from a file, output.txt -o /dev/null in silent
```shell
echo "some input" | cargo run -- output.txt --silent / -s
```

### Write to a file with batch input
```shell
yes | cargo run -- -o yes.txt
```

### Read from large input file
```shell
cargo run -- yes.txt -o /dev/null
```

### Testing BufReader & BufWriter
```shell
cargo run -- yes.txt -o yes2.txt
```


### Read from file and redirect output to specified output file
```shell
cargo run -- yes2.txt -- > yes3.txt
```

## Piping output to head and redirect output to /dev/null
```shell
yes | cargo run -- | head -n 100000000 > /dev/null
```