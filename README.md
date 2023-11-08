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

## Cat a the file then pipe the bytes to pipe-viewer
```shell
> cat myfile | target/release/pipeviewer > myfile2
```
