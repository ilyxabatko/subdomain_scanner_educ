# Subdomain Scanner 
Scans the subdomains for a specified target (domain) for a publicly available server. After that, the program scans for publicly available services by establishing a TCP connection to check if the port is open (for simplicity, timeout is used).

To check all of the most popular ports per subdomain, we use Rayon which brings data-parallelism, parallel iterators in our case.

With Rayon we create a custom thread pool, customize it and process the scan inside the pool without sharing data between tasks.

## Run:
```
cargo run -- [TARGET DOMAIN]

// E.g.
cargo run -- kerkour.com
```

Possible output:

```
academy.kerkour.com:
     80
     443
     8080
     8443
social.kerkour.com:
     80
     443
     8080
     8443
kerkour.com:
     80
     443
     8080
     8443
www.kerkour.com:
     80
     443
     8080
     8443
```

*The project was written following the "Black Hat Rust" book.*