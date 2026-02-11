## 🎯 Project Status

**Current Phase**: Foundation Layer (State Management) ✅  
**Next Phase**: REVM Integration + Mempool Monitor 🚧



## 🏗 Architecture

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   Mempool   │───▶│    State     │───▶│    REVM     │
│  Collector  │    │   Manager    │    │  Simulator  │
└─────────────┘    └──────────────┘    └─────────────┘
                         ▲                      │
                         │                      ▼
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Flashbots  │◀───│   Arbitrage  │◀───│   Solver    │
│   Bundler   │    │   Executor   │    │ (Optimizer) │
└─────────────┘    └──────────────┘    └─────────────┘
```

1. **Collector**: Filters Ethereum Mempool for relevant `Pending` transactions using an address-based Watchlist.
2. **State Manager**: Maintains a local, thread-safe cache of Liquidity and Reserves (DashMap).
3. **Simulation (REVM)**: Locally executes `Target` transactions to predict price impact without network overhead.
4. **Solver**: Executes mathematical optimization ($a_{optimal}$) on the post-simulation state.
5. **Executor**: Bundles the `Target` and `Arbitrage` transactions via Flashbots Relay.

## 🚧 Roadmap

### Phase 1: Foundation ✅ (Current)
- [x] State management architecture
- [x] V2/V3 pool state structures
- [x] Lock-free concurrent cache
- [x] Test suite + CI

### Phase 2: Simulation Engine 🔄 (In Progress)
- [ ] REVM integration for local execution
- [ ] Sub-millisecond simulation latency
- [ ] Gas estimation with EIP-1559
- [ ] Revert detection

### Phase 3: Mempool Monitor
- [ ] WebSocket dual-stream (pending + confirmed blocks)
- [ ] Address watchlist tx filtering
- [ ] Transaction priority queue

### Phase 4: Arbitrage Solver
- [ ] U256-based optimization (no floating point)
- [ ] Multi-hop path finding (V2 ↔ V3)
- [ ] Profitability threshold calculation

### Phase 5: Execution
- [ ] Flashbots bundle construction
- [ ] MEV-Share integration
- [ ] Bundle simulation + submission

