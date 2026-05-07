# Latency-Critical Event Processing Engine.

## Next step
- Record transactions

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

## Dump
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x518bbd42bd506517858d229a8680ee2e289ab690] | In (ETH): 49000000000000000
🦄 Token->ETH | Path: [0x04f12a892c3b14d916c6d5af6f0315f17974c80a, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 1132760017354139227788221200
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x518bbd42bd506517858d229a8680ee2e289ab690] | In (ETH): 23400000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 29300000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 43000000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x518bbd42bd506517858d229a8680ee2e289ab690] | In (ETH): 11920000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 6800000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 24000000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 11800000000000000
🦄 Token->ETH | Path: [0x26e550ac11b26f78a04489d5f20f24e3559f7dd9, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 1626741016860
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 33900000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 7000000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 35800000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 66610000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 42100000000000000
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 18400000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 25400000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 23500000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 21060000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 20200000000000000
ignore
2026-01-03T05:08:37.283782Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 22900000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 14600000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 132100000000000000
2026-01-03T05:09:14.251812Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 13100000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 35600000000000000
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 33900000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 60000000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 2600000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 45850000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 122200000000000000
ignore
ignore
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 60400000000000000
ignore
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 25680000000000000
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 15400000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 65000000000000000
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 49800000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 26500000000000000
2026-01-03T05:12:49.461957Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
2026-01-03T05:12:49.463587Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 12800000000000000
2026-01-03T05:13:01.781666Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
ignore
ignore
🦄 Token->Token | Path: [0xaaee1a9723aadb7afa2810263653a34ba2c21c7a, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 794565750420213800000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 30100000000000000
ignore
ignore
2026-01-03T05:13:37.374014Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
2026-01-03T05:13:37.374194Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 10480000000000000
ignore
ignore
2026-01-03T05:14:37.431193Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 24000000000000000
2026-01-03T05:15:01.616695Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 27700000000000000
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 18500000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 15300000000000000
2026-01-03T05:16:14.496637Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
ignore
ignore
🦄 Token->ETH | Path: [0x04f12a892c3b14d916c6d5af6f0315f17974c80a, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 1011738477328502951988529100
ignore
ignore
2026-01-03T05:17:17.853505Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 Token->ETH | Path: [0x518bbd42bd506517858d229a8680ee2e289ab690, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 56134933838761602938461551800
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x04f12a892c3b14d916c6d5af6f0315f17974c80a] | In (ETH): 50000000000000000
ignore
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xd6203889c22d9fe5e938a9200f50fdffe9dd8e02] | In (ETH): 1168300000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 13000000000000000
ignore
ignore
ignore
2026-01-03T05:19:36.945262Z  INFO rusty_mev: Inserted Pool { address: 0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc, token0: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, token1: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, reserve0: 11215945164177, reserve1: 3584726951634944360666 }
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 15000000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0xbea389cc2f222c3e47424968163c70abc5b581d7] | In (ETH): 43100000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 34000000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 24190000000000000
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 34000000000000000
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 18400000000000000
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 16600000000000000
ignore
🦄 Token->ETH | Path: [0x590813065947e2cbf857bfc5ac96017516893eb3, 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2] | In: 2513377007646612990694318100
ignore
ignore
🦄 ETH->Token | Path: [0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x590813065947e2cbf857bfc5ac96017516893eb3] | In (ETH): 19000000000000000
