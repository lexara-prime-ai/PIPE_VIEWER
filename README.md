## * * * * Usage * * * *
Generate a file named, myfile with 128kbps of random data.

```bash
> dd if=/dev/urandom bs=1024 count=128 of=myfile
```

## Preview file info
> ls -lh

### Build a binary
> cargo build

## Cat a the file then pipe the bytes to pipe-viewer
> cat myfile | target/release/pipeviewer > myfile2