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
  -t, --tests <TESTS>  
  -d, --download       
  -h, --help           Print help
```

`-t, --tests` -- number of tests.
`-d, --download` -- enable download test.

To build:
```
git clone https://github.com/SplatDem/SplatChec-Internet-Connection
cd SplatCheckInternetConnection
cargo build --release
cd target/release
sudo cp scic /bin
```
