# Rust Distributed Job Queue

## 📖 About

A correctness-first distributed job-queue prototype exploring reliable background work, worker ownership, retries, leases, acknowledgements, and failure semantics in Rust.

## ✨ Features

- Unique job IDs
- Worker leases
- Acknowledgement ownership
- Retry accounting
- Maximum-attempt policy
- Dead-letter handling
- Deterministic unit tests

## 🛠 Tech Stack

- Rust
- Cargo
- In-memory queue core

## 🏗 Architecture

```text
Producer
   ↓
Job queue
   ↓
Lease / ownership
   ↓
Worker
   ├── acknowledge → completed
   └── reject/fail → retry → dead letter
```

## 📁 Project Structure

```text
.
├── src/        # Queue implementation
├── tests/      # Correctness/failure tests
├── Cargo.toml
└── README.md
```

## 📋 Prerequisites

- Rust stable toolchain

## 🚀 Getting Started

```bash
git clone https://github.com/matinwgg/rust-distributed-job-queue.git
cd rust-distributed-job-queue
cargo test
```

## 💻 Usage

The current core is an in-memory library. Integrate a producer and worker through the queue API exposed under `src/`.

## 🧮 Mathematical / Systems Foundations

Important concepts include state machines, invariants, leases, partial failure, probability of duplicate execution, queueing behavior, idempotency, and consistency semantics.

## 🧪 Testing

Tests should cover worker crashes, duplicate delivery, expired leases, acknowledgement races, retry budgets, and dead-letter transitions.

## 🔐 Reliability & Security

A job being fetched does not imply durable ownership. Production use requires durable persistence, atomic claim operations, authentication, authorization, input validation, bounded payloads, and observability.

## 🚧 Future Work

- Durable persistence
- Lease expiry
- Atomic claims
- Network protocol
- Multiple workers/nodes
- Metrics/tracing
- Fault injection
- Idempotency support

## 🤝 Contributing

Document the failure semantics of every change and add deterministic tests for races and crash scenarios.

## 📄 License

See repository license information.

## 👨‍💻 Author

**Matin Odoom**
