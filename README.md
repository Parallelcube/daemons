# Description
This repository contains the implementation of a same service using different languages

```
┌─────────────┐   POSIX Queue (request)    ┌─────────────┐
│             ├───────────────────────────►│             │
│  Service A  │                            │  Service B  │
│             │◄───────────────────────────┤             │
└─────────────┘   POSIX Queue (response)   └─────────────┘
```

# Current languages
- C++
- Python
- Rust

# Features
- IPC communication using POSIX queues

# Scripts
We have multiple scripts to build and test the different services

## C++
- buildcpp.sh
- runcpp.sh

## Python
- runpython.sh

## Rust
- buildrust.sh
- runrust.sh