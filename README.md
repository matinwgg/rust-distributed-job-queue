# Rust Distributed Job Queue

A correctness-first distributed job queue prototype exploring reliable background work in Rust.

## Implemented

- unique job IDs
- worker leases
- acknowledgement ownership
- retry accounting
- configurable maximum attempts
- dead-letter handling
- deterministic unit tests

## Reliability model

A worker does not own a job merely because it fetched it. The queue records the lease owner and only that worker can acknowledge or reject the lease. Failed jobs are requeued until their attempt budget is exhausted, after which they enter the dead-letter queue.

The current core is deliberately in-memory. Next stages are durable persistence, lease expiry, atomic claim operations, a network protocol, multiple workers, metrics/tracing and fault-injection tests.

## Run

```bash
cargo test
```

## Engineering questions

1. What failure semantics can be guaranteed when a worker crashes after completing work but before acknowledgement?
2. How should lease expiry interact with duplicate delivery and idempotent handlers?
3. What throughput/latency trade-offs arise from durable state and stronger consistency?
