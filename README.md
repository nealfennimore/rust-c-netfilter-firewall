# Linux Userspace Firewall
### Check Queues
```sh
sudo cat /proc/net/netfilter/nfnetlink_queue
```

```sh
sudo iptables -A INPUT -p tcp --dport 80 -j NFQUEUE --queue-num 1 # Append
sudo iptables -D INPUT -p tcp --dport 80 -j NFQUEUE --queue-num 1 # Delete
```

```sh
nc -zv localhost:80
```

## Scripts
### Build

```sh
make
```