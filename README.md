## Rproxy
**Rproxy** is a lightweight reverse proxy with cache support implemented in Rust.

## Features
- [x] gRPC client-Server Connection
- [x] Cache support

## Architecture
![](resources/architecture.png)

## Components
- Client: A lightweight client to communicate with the Reverse Proxy Server via gRPC.
- Server: The server is composed of a cache system and a HTTP client. It serves the requests received from the client.

## Usage
### Client
```
A lightweight client to communicate with the Reverse Proxy Server via gRPC

Usage: rproxy_client [OPTIONS] --url <URL>

Options:
      --server-ip <SERVER_IP>      IP of the Proxy Server [default: 0.0.0.0]
      --server-port <SERVER_PORT>  Port of the Proxy Server [default: 50051]
  -u, --url <URL>                  URL to query
  -h, --help                       Print help information
  -V, --version                    Print version information
```
### server
```
A simple reverse proxy with cache support

Usage: rproxy [OPTIONS]

Options:
  -i, --ip <IP>      IP of the Proxy Server [default: 127.0.0.1]
  -p, --port <PORT>  Port of the Proxy Server [default: 50051]
  -t, --ttl <TTL>    Time To Live for cache entries (in seconds) [default: 30]
  -h, --help         Print help information
  -V, --version      Print version information
```
### Example
```bash
## Run Server
$ rproxy

## Run Client
$ rproxy_client --url "https://blockstream.info/api/blocks/0" --server-ip "127.0.0.1"
[{"id":"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f","height":0,"version":1,"timestamp":1231006505,"tx_count":1,"size":285,"weight":816,"merkle_root":"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b","previousblockhash":null,"mediantime":1231006505,"nonce":2083236893,"bits":486604799,"difficulty":1}]
```
### Testing
```bash
$ cargo test
```

## Cache Support
The cache implemented here is an in-memory cache with a TTL based eviction policy. It uses a reader-writer lock in rust `RwLock`. An RwLock will allow any number of readers to acquire the lock as long as a writer is not holding the lock, thus improving the performance of the cache.

In addition, when a miss occurs (e.g when a requested value is not present in the cache), the cache will be unlocked (will not block while fetching the response from the original server) thus allowing other requests to be fullfilled.

There are different ways to implement a TTL based evijction policies. For example, when a key is requested, the corresponding entry is checked if expired, and is removed. Our implemenation uses this solution. Another way is entry is deleted as soon as expiry is hit.
 ### Possible improvements
 Some possible improvements could be made to improve the efficiency of the cache:
 - Add a mechanism to flush and delete expired entries through using thread will occasionally visit entries and remove the expired ones eagerly.
 - Alternatively, we could add a size for the cache. The cache may evict the expired entries lazily when it needs more space.

## TODO
- Improve Security (Authentication, Client-Server Encryption, DoS mitigation)
- URL parsing and verification (Client and Server side)
- Cache Stampede and Cache Key Collision Handling
- More unit and e2e tests