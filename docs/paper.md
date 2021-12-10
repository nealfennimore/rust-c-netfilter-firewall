# Rustful Linux Network Packet Analysis
###### Neal Fennimore
## Abstract
Using Rust as low level systems language can provide more safety and performance to modern day networks. In a simple experiment, we've determined that even by using native C libraries for packet filtering, Rust is close to its counterpart C implementation. This highlights the potential gains that can come from using Rust to do network packet filtering on Linux.

## Introduction
As our networks expand and connections speeds increase, we are at greater risk to denial of service attacks. Our firewalls will be bombarded more and more with malicious traffic as new botnets keep forming. Since most servers are Linux, I'll be discussing how packet filtering works on that operating system.

On Linux, the packet filtering and overall network stack is called netfilter, which is written in the C programming language. The C programming language has greater performance but it also is infamous for having difficult memory management[^10]. What if we were able to combine the performance of C with another language that provides memory safety? There's another modern day language called Rust, where we have memory safety and performance - the proverbial cake which we can eat too.

The structure of this paper will first provide some context into the Rust programming language. We'll then discuss some of the interfaces that are used on the Linux operating system that provide network and packet filtering. Afterwards, an experiment utilizing both a C and Rust programming language implementation will test the throughput on these same Linux operating system interfaces.

### What is Rust?
Rust is a newer systems language that has memory safety built-in without compromising on performance[^2]. It has a different paradigm of memory efficiency, as it doesn't rely on any sort of garbage collector to clean up unused memory. It is a typed language which uses a compiler called `rustc`. We'll be using Rust to link to the C implementation of `libnetfilter_queue` using a Foreign Function Interface. To first understand what `libnetfilter_queue` and the Foreign Function Interface is, we'll first need to understand how the Linux kernel filters packets.

### What is Netfilter?
Netfilter is a Linux library that allows for packet filtering within the Linux kernel. It has been in the Linux kernel since v2.4 and has several modules which can be used for different packet filtering needs[^4]. Netfilter provides an API that other applications can hook into. It is used at the kernel level and is often built on top of with modules like iptables[^3].

#### What are iptables?
Iptables is state management tool built on top of netfilter which is used to make decisions on packets[^9]. It designates rules that a packet must follow (e.g. a chain) and then eventually hooks back into netfilter once a decision is made[^1]. It can be used for NAT translation, routing, and modifying packets within a chain. Primarily in this paper we'll be using it to add packets that match a destination network, where they are then enqueued onto a libnetfilter_queue queue. 

#### Libnetfilter_queue
Libnetfilter_queue allows for packets to come in, be queued, and then programmatically acted on to as to whether the packet can make it to it's next destination. This is known as making a verdict on the packet, where verdicts can any action like accepting the packet, queuing it for another queue, repeating it, or dropping it entirely. The benefit of libnetfilter_queue is that it happens at userspace, so there's no need to create a kernel module for it. Our Rust and C code will be written to use libnetfilter_queue. For Rust, we need to use the previously mentioned Foreign Function Interface. 

#### How is libnetfilter_queue used with the Foreign Function Interface?
The Foreign Function Interface (FFI) allows for using native C libraries in Rust. To use an external library we use `extern` declaration to use the C library function, and an `unsafe` declaration to use the external library call[^8]. In the following snippet we're dynamically linking the system library `netfilter_queue` without including it in our compiled code.

```rust
// Dynamically linking to the system netfilter_queue library
#[link(name = "netfilter_queue")]
extern "C" {
	pub fn nfq_open() -> NfHandle;
}

// We can now call `nfq_open` in Rust
let value = unsafe { nfq_open() };
```

## Methodology
### Testing network performance

For the experiment I used [ethr](https://github.com/microsoft/ethr) to perform throughput testing on two implementations of the same code - one in C and one in Rust. Ethr is a Go based network performance tool that can be used for measuring TCP, UDP, and ICMP[^6].

### How was it setup?
![State Diagram](https://raw.githubusercontent.com/nealfennimore/rust-c-netfilter-firewall/main/docs/diagram.png)

First, a firewall rule (iptables) is activated that allows TCP packets going to `127.0.0.1` port `9999` to be queued onto a netfilter queue.

Second, a C or Rust program will create the netfilter queue and wait for incoming packets. A handler in the program would then receive packets with a non-blocking socket using the `MSG_DONTWAIT` flag. For this experiment, all packets were accepted by default in the handler.

Then finally the ethr server is run, followed by the ethr client being activated, and which marks the beginning of the test. The ethr server will test the throughput of incoming TCP packets listening on `127.0.0.1` port `9999`. An ethr client that will send TCP IPv4 packets to the ethr server (listening on `127.0.0.1` port `9999`). The client will perform the throughput testing for the duration of a minute.

```sh
# Starting the server
ethr -s -ip 127.0.0.1 -port 9999

# Starting the client
ethr -c localhost -port 9999 -p tcp -4 -d 1m
```

## Findings
### Initial Mistakes
Initially, the buffer was too low ($2^{16}$ bits) for the amount of traffic that ethr was sending. After updating the buffer size to $2^{32}$ an error would throw in both the C and Rust implementation of the code. This was due to the buffer being much larger than which the stack could handle. 

To fix that I used `calloc` in C and `Vector` in Rust to put the buffer on to the heap. This resolved the issues and allow the tests to continue. Ideally, the buffer would have been cleared after using `recv` and it could have remained on the stack, but it was hard to determine what exactly was going wrong. 

### Results
![Line Plot Comparison](https://raw.githubusercontent.com/nealfennimore/rust-c-netfilter-firewall/main/result/output.png)

On average it looks as though the throughput of [C implementation](https://github.com/nealfennimore/linux-userspace-firewall/blob/main/result/c-server.csv) is faster - though not by much. The [Rust implementation](https://github.com/nealfennimore/linux-userspace-firewall/blob/main/result/rust-server.csv) was approximately 0.06GB behind it's C counterpart.

```
Rust: 3.84 GB per second
C: 3.9 GB per second
```
## Conclusion
The Rust implementation is incredibly close to the C implementation in throughput when it comes to packet filtering with netfilter. There is a caveat with using a C library in Rust though, and that is that by introducing an external library we are now dealing with the side effects and memory issues that Rust doesn't guarantee safety for. If however, the native libraries were written in Rust this would be a non-issue. Overall, it points to Rust being a performant low, level systems language that can hold it's own against C, while also having guarantees for memory and thread safety. 

[^1]: Q.-X. Wu, “The Research and Application of Firewall based on Netfilter,” Physics Procedia, vol. 25, pp. 1231–1235, Jan. 2012, doi: 10.1016/j.phpro.2012.03.225.
[^2]: “Rust Programming Language.” https://www.rust-lang.org/ (accessed Nov. 13, 2021).
[^3]: K. Accardi, T. Bock, F. Hady, and J. Krueger, “Network processor acceleration for a Linux* netfilter firewall,” in 2005 Symposium on Architectures for Networking and Communications Systems (ANCS), Oct. 2005, pp. 115–123. doi: 10.1145/1095890.1095906.
[^4]: “netfilter/iptables project homepage - The netfilter.org ‘libnetfilter_queue’ project.” https://www.netfilter.org/projects/libnetfilter_queue/index.html (accessed Nov. 13, 2021).
[^5]: “iptables(8) - Linux man page.” https://linux.die.net/man/8/iptables (accessed Nov. 13, 2021).
[^6]: Ethr. Microsoft, 2021. Accessed: Nov. 13, 2021. [Online]. Available: https://github.com/microsoft/ethr
[^7]: B. Wang, K. Lu, and P. Chang, “Design and implementation of Linux firewall based on the frame of Netfilter/IPtable,” in 2016 11th International Conference on Computer Science Education (ICCSE), Aug. 2016, pp. 949–953. doi: 10.1109/ICCSE.2016.7581711.
[^8]: “FFI - The Rustonomicon.” [https://doc.rust-lang.org/nomicon/ffi.html](https://doc.rust-lang.org/nomicon/ffi.html) (accessed Nov. 13, 2021).
[^9]: T. Underwood, “Netfilter and iptables: Stateful firewalling for Linux,” ZDNet. https://www.zdnet.com/article/netfilter-and-iptables-stateful-firewalling-for-linux/ (accessed Nov. 13, 2021).
[^10]: R. Jain, R. Agrawal, R. Gupta, R. K. Jain, N. Kapil, and A. Saxena, “Detection of Memory Leaks in C/C++,” in 2020 IEEE International Students’ Conference on Electrical,Electronics and Computer Science (SCEECS), Feb. 2020, pp. 1–6. doi: 10.1109/SCEECS48394.2020.32.


