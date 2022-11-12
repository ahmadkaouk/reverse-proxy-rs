## Rproxy
**Rproxy** is a lightweight reverse proxy with cache support implemented in Rust.

**Features**
- [x] URL Parsing and verification
- [x] gRPC communication
- [x] Cache support

## Components
- Client
- Server

## Usage
- server
```
rproxy --help
A simple reverse proxy with cache support

Usage: rproxy [OPTIONS]

Options:
  -i, --ip <IP>      IP of the Proxy Server [default: 127.0.0.1]
  -p, --port <PORT>  Port of the Proxy Server [default: 50051]
  -h, --help         Print help information
  -V, --version      Print version information
```

## TODO
- Authentication
- Encrypted Communication between Client and Server
- Test Scalability (Memory Cache Efficiency)
- URL parsing and verification