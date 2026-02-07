## 🏗 System Architecture (V2 Quantum)


1. **Collector**: Filters Ethereum Mempool for relevant `Pending` transactions using an address-based Watchlist.
2. **State Manager**: Maintains a local, thread-safe cache of Liquidity and Reserves (DashMap).
3. **Simulation (REVM)**: Locally executes `Target` transactions to predict price impact without network overhead.
4. **Solver**: Executes mathematical optimization ($a_{optimal}$) on the post-simulation state.
5. **Executor**: Bundles the `Target` and `Arbitrage` transactions via Flashbots Relay.
