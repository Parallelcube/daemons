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

**Python**
- **runpython.sh** : Start service as host

**C++**
- **buildcpp.sh** : Compile Cpp code
- **runcpp.sh** : Start service as host

**Rust**
- **buildrust.sh** : Compile Rust code
- **runrust.sh** : Start service as host
