# Subdomain Scanner 
Scans the subdomains for a specified target (domain) for a publicly available server. After that, the program scans for publicly available services by establishing a TCP connection to check if the port is open (for simplicity, timeout is used).

To check all of the most popular ports per subdomain, we use Rayon which brings data-parallelism, parallel iterators in our case.

**TODO: complete the readme file.**


*The project was written following the "Black Hat Rust" book.*