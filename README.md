# SplatCheck-Internet-Connection
Check Internet speed

Default usage (just type scic) -- 5 tests:
```
Uploading 10 MB of data
Test 1: Upload speed: 2.02 MB/s
Test 2: Upload speed: 3.24 MB/s
Test 3: Upload speed: 2.94 MB/s
Test 4: Upload speed: 2.79 MB/s
Test 5: Upload speed: 2.53 MB/s
Avg upload speed: 2.70 MB/s

It Will Do
```

You can also set parameters:
```
Usage: scic [OPTIONS]

Options:
  -t, --tests <NUMBER>  Number of tests
  -d, --download <URL>  Enable/Disable download test
  -s, --size <SIZE>     Size of data to upload [default: 10]
  -c, --check           Check the avaibility of the connection
  -h, --help            Print help
```

## Available in AUR
```
https://aur.archlinux.org/packages/scic
```
```
yay -Sy scic
```

To build:
```
git clone https://github.com/SplatDem/SplatChec-Internet-Connection
cd SplatCheckInternetConnection
cargo build --release
cd target/release
sudo cp scic /bin
```
