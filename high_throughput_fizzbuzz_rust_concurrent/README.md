```zsh
$ cargo build --release
$ ./target/release/high_throughput_fizzbuzz_rust_concurrent | pv > /dev/null # 6.5 GiB/s with Ryzen 5950X
$ cat /dev/zero | pv > /dev/null # 7.5 GiB/s
```
https://codegolf.stackexchange.com/questions/215216/high-throughput-fizz-buzz
