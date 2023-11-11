## * * * * Usage * * * *
Generate a file named, myfile with 128kbps of random data.

```shell
> dd if=/dev/urandom bs=1024 count=128 of=myfile
```

## Preview file info
```shell
> ls -lh
```

### Build a binary
```shell
> cargo build
```

## Cat the file then pipe the bytes to pipe-viewer
```shell
> cat myfile | target/release/pipeviewer > myfile2
```

## Using the 'yes' utility to test broken pipe error handling
```shell
> yes | cargo run | head -n 1 > /dev/null
```

## Preview console output
```shell
> yes | cargo run | head -n 100000000 > /dev/null
```

## Running the binary with cargo run
```shell
> cargo run -- -h
```

## Testing commandline arguments
```shell
> cargo run -- somefile --outfile file.out -s
```

## Testing environment variable with commandline arguments
```shell
> PV_SILENT=1 cargo run -- somefile --outfile file.out -s
```