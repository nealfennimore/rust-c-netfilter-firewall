# Linux Userspace Firewall

Comparing the speed of a C vs Rust implementation using netfilter_queue. Load testing performed with [ethr](https://github.com/microsoft/ethr).

- [Research Paper](docs/paper.md)
- [Results](result/notebook.ipynb)

## Reproducing
### Build and Test

#### C
```sh
# Starting the server
ethr -s -ip 127.0.0.1 -port 9999

# Compile and setup iptables
make

# Starting the client
ethr -c localhost -port 9999 -p tcp -4 -d 1m
```
#### Rust
```sh
# Starting the server
ethr -s -ip 127.0.0.1 -port 9999

# Compile and setup iptables
make rust

# Starting the client
ethr -c localhost -port 9999 -p tcp -4 -d 1m
```

## Attribution

Code derived from [libnetfilter_queue](https://www.netfilter.org/projects/libnetfilter_queue/doxygen/html/group__LibrarySetup.html) and [nfqueue-rs](https://github.com/chifflier/nfqueue-rs).