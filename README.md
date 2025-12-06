# Latency-Critical Event Processing Engine.

## Phase 1: The Firehose (Ingestor & Decoder)
The Problem: The network sends ~30-50 transactions per second (bursts of 100+). We need to filter 99% of them (noise) to find the 1% (UniV2 Swaps) instantly.
Your Learning Outcome:
  - Async Rust: Handling streams, buffers, and backpressure (what happens if we parse slower than the network sends?).
  - Protocol Design: Understanding JSON-RPC and WebSocket implementation details.
  - Binary Parsing: Decoding raw ABI bytes manually (the "Compiler" front-end work).

## Phase 2: The World View (State Management)
The Goal: To know if a trade is profitable, we need to know the Reserves (How much ETH/USDT is in the pool?).
The Problem: Querying the blockchain node via network takes ~30ms. That is too slow. We need a Local Cache of the blockchain state in RAM.
Your Learning Outcome:
  - Multicore Architecture: Managing shared state (Arc<RwLock<HashMap>>) across threads.
  - Cache Coherency: Updating your local cache immediately when a block is mined so you don't trade on stale data.
  - Lock-Free Programming: Eventually replacing RwLock with lock-free structures (DashMap or atomics) to prevent CPU stalling.

## Phase 3: The Brain (Simulation & EVM)
The Goal: Calculate the exact profit.
The Problem: "Math" isn't enough. We need to actually execute the transaction logic to see if it reverts or succeeds.
The Solution: We embed a lightweight Virtual Machine (revm) inside your Rust binary.
Your Learning Outcome:
  - Virtual Machines: You will interact with the EVM internals (Stack, Memory, Storage).
  - Emulation: Running a "Fork" of the blockchain in memory.
  - OS Concepts: Snapshotting memory states (Copy-on-Write) to run thousands of simulations per second efficiently.

## Phase 4: The Hammer (Execution & Huff)
The Goal: Capture the money.
The Problem: Standard Solidity contracts are bloated. They check for overflows, they have metadata, they use too much Gas.
The Solution: You will write a custom Huff (Assembly) smart contract.
Your Learning Outcome:
  - Compilers/Assembly: You will manually manage the Stack (PUSH, POP, SWAP). You will write the "backend" code.
  - Bytecode Optimization: Shaving off individual bytes of code to save gas.
  - The "Bare Metal" of Crypto: No safety rails. Pure logic.

## Phase 5: The Polish (Benchmarking & Profiling)
The Goal: Go faster.
The Problem: "It works, but is it fast?"
The Solution: We hook up flamegraph and criterion.
Your Learning Outcome:
  - Performance Engineering: Identifying bottlenecks (e.g., "Why is deserialization taking 40% of CPU?").
  - Systems Tuning: Switching memory allocators (Jemalloc vs System), tuning Tokio thread pools, pinning threads to CPU cores.
