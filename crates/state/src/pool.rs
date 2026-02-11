use alloy::primitives::{Address, U256};

/// Uniswap V2 pool state (constant product x * y = k)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStateV2 {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    pub reserve0: U256,
    pub reserve1: U256,
}

impl PoolStateV2 {
    /// Calculate current price: reserve1 / reserve0
    pub fn price(&self) -> f64 {
        if self.reserve0.is_zero() {
            return 0.0;
        }
        self.reserve1.to::<u128>() as f64 / self.reserve0.to::<u128>() as f64
    }

    /// Simulate swap impact: returns amount_out for given amount_in
    /// Uses constant product formula with 0.3% fee (γ = 0.997)
    pub fn simulate_swap(&self, amount_in: U256, zero_for_one: bool) -> U256 {
        const FEE_MULTIPLIER: u128 = 997;
        const FEE_DIVISOR: u128 = 1000;

        let (reserve_in, reserve_out) = if zero_for_one {
            (self.reserve0, self.reserve1)
        } else {
            (self.reserve1, self.reserve0)
        };

        let amount_in_with_fee = amount_in * U256::from(FEE_MULTIPLIER);
        let numerator = amount_in_with_fee * reserve_out;
        let denominator = reserve_in * U256::from(FEE_DIVISOR) + amount_in_with_fee;

        if denominator.is_zero() {
            return U256::ZERO;
        }

        numerator / denominator
    }
}

/// Uniswap V3 pool state (concentrated liquidity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolStateV3 {
    pub address: Address,
    pub token0: Address,
    pub token1: Address,
    /// Current price encoded as sqrt(price) * 2^96
    pub sqrt_price_x96: U256,
    /// Available liquidity at current tick
    pub liquidity: u128,
    /// Current tick (log_1.0001(price))
    pub tick: i32,
}

impl PoolStateV3 {
    /// Calculate human-readable price from sqrtPriceX96
    pub fn price(&self) -> f64 {
        let sqrt_price = self.sqrt_price_x96.to::<u128>() as f64 / (1u128 << 96) as f64;
        sqrt_price * sqrt_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_price_calculation() {
        let pool = PoolStateV2 {
            address: Address::ZERO,
            token0: Address::ZERO,
            token1: Address::ZERO,
            reserve0: U256::from(1_000_000),
            reserve1: U256::from(2_000_000),
        };

        let price = pool.price();
        assert!((price - 2.0).abs() < 0.001); // Price should be ~2.0
    }

    #[test]
    fn test_v2_swap_simulation() {
        let pool = PoolStateV2 {
            address: Address::ZERO,
            token0: Address::ZERO,
            token1: Address::ZERO,
            reserve0: U256::from(1_000_000_000_000_000_000u128),
            reserve1: U256::from(3000_000_000u128),
        };

        // Swap 0.1 ETH for USDC
        let amount_in = U256::from(100_000_000_000_000_000u128);
        let amount_out = pool.simulate_swap(amount_in, true);

        // Should get ~270 USDC (slippage + fee)
        assert!(amount_out > U256::from(250_000_000u128));
        assert!(amount_out < U256::from(300_000_000u128));
    }
}
