## cargo profiler

```bash
cargo install -f cargo-profile
cargo profiler callgrind --release
cargo profiler cachegrind
valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.txt ./target/release/profiler-sandbox
```

## cpuprofiler
* http://athemathmo.github.io/2016/09/14/tools-for-profiling-rust.html

```bash
wget https://github.com/gperftools/gperftools/releases/download/gperftools-2.7/gperftools-2.7.tar.gz
tar xf gperftools-2.7.tar.gz
cd gperftools-2.7
./configure
make
make install
sudo ldconfig
```

```console
$ cargo run
$ cat my-prof.profile
'56012d05b000-56012d0f9000 r-xp 00000000 00:00 4242259     /home/legokichi/Github/rust-snipets/profiler-sandbox/target/debug/profiler-sandbox
56012d2f8000-56012d2fd000 r--p 0009d000 00:00 4242259     /home/legokichi/Github/rust-snipets/profiler-sandbox/target/debug/profiler-sandbox
56012d2fd000-56012d2fe000 rw-p 000a2000 00:00 4242259     
...
```


* https://github.com/google/pprof
* https://github.com/golang/go/wiki/Ubuntu
* http://golang.jp/install

```bash
sudo add-apt-repository ppa:gophers/archive
sudo apt-get update
sudo apt-get install golang-1.10-go
go get -u github.com/google/pprof
```

* http://goog-perftools.sourceforge.net/doc/cpu_profiler.html
* print call graph

```
sudo apt install gv
pprof --gv target/debug/profiler-sandbox my-prof.profile
```

