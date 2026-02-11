use crate::pool::{PoolStateV2, PoolStateV3};
use alloy::primitives::Address;
use dashmap::DashMap;
use std::sync::Arc;

/// Protocol type for pool identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    UniswapV2,
    UniswapV3,
}

/// Unified pool state enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolState {
    V2(PoolStateV2),
    V3(PoolStateV3),
}

pub struct StateManager {
    pools: Arc<DashMap<Address, PoolState>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(DashMap::new()),
        }
    }

    /// Insert or update a V2 pool state
    pub fn update_v2(&self, pool: PoolStateV2) {
        self.pools.insert(pool.address, PoolState::V2(pool));
    }

    /// Insert or update a V3 pool state
    pub fn update_v3(&self, pool: PoolStateV3) {
        self.pools.insert(pool.address, PoolState::V3(pool));
    }

    /// Get pool state by address (zero-copy read)
    pub fn get(&self, address: &Address) -> Option<PoolState> {
        self.pools.get(address).map(|entry| entry.clone())
    }

    /// Get V2 pool state specifically
    pub fn get_v2(&self, address: &Address) -> Option<PoolStateV2> {
        match self.get(address)? {
            PoolState::V2(pool) => Some(pool),
            _ => None,
        }
    }

    /// Get V3 pool state specifically
    pub fn get_v3(&self, address: &Address) -> Option<PoolStateV3> {
        match self.get(address)? {
            PoolState::V3(pool) => Some(pool),
            _ => None,
        }
    }

    /// Get current pool count (for monitoring)
    pub fn pool_count(&self) -> usize {
        self.pools.len()
    }

    /// Clear all cached state (useful for testing)
    pub fn clear(&self) {
        self.pools.clear();
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    #[test]
    fn test_state_manager_v2_operations() {
        let manager = StateManager::new();

        let pool_address = Address::from(rand::random::<[u8; 20]>());
        let pool = PoolStateV2 {
            address: pool_address,
            token0: Address::ZERO,
            token1: Address::ZERO,
            reserve0: U256::from(1000u128),
            reserve1: U256::from(2000u128),
        };

        // Insert
        manager.update_v2(pool);
        assert_eq!(manager.pool_count(), 1);

        // Retrieve
        let retrieved = manager.get_v2(&pool_address);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().reserve0, U256::from(1000u128));

        // Update
        let updated_pool = PoolStateV2 {
            reserve0: U256::from(1500u128),
            ..pool
        };
        manager.update_v2(updated_pool);
        assert_eq!(manager.pool_count(), 1);

        let retrieved = manager.get_v2(&pool_address).unwrap();
        assert_eq!(retrieved.reserve0, U256::from(1500u128));
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let manager = Arc::new(StateManager::new());
        let mut handles = vec![];

        // Spawn 10 threads writing different pools
        for i in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                let pool = PoolStateV2 {
                    address: Address::from(rand::random::<[u8; 20]>()),
                    token0: Address::ZERO,
                    token1: Address::ZERO,
                    reserve0: U256::from(i * 1000),
                    reserve1: U256::from(i * 2000),
                };
                manager_clone.update_v2(pool);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(manager.pool_count(), 10);
    }
}
