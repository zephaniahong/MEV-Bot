use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy::primitives::Address;
use alloy::{consensus::Transaction, sol_types::SolCall};
use tracing::{debug, info, warn};

use crate::{
    Pool,
    constants::{
        SWAP_EXACT_ETH_FOR_TOKENS, SWAP_EXACT_TOKENS_FOR_ETH, SWAP_EXACT_TOKENS_FOR_TOKENS,
    },
    types::{swapExactETHForTokensCall, swapExactTokensForETHCall, swapExactTokensForTokensCall},
    utils::{calculate_pair_address, get_amount_out},
};

pub struct Strategy {
    pub cache: Arc<RwLock<HashMap<Address, Pool>>>,
}

impl Strategy {
    pub fn new(cache: Arc<RwLock<HashMap<Address, Pool>>>) -> Self {
        Self { cache }
    }
    pub fn process(&self, tx: &dyn Transaction) {
        let tx_data = tx.input();
        if tx_data.len() >= 4 {
            let selector = &tx_data[0..4];

            match selector {
                x if x == SWAP_EXACT_TOKENS_FOR_TOKENS => {
                    match swapExactTokensForTokensCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            info!(
                                "Token->Token swap | Path: {:?} | In: {}",
                                decoded.path, decoded.amountIn
                            );
                        }
                        Err(e) => tracing::warn!("Decode Error (Tokens->Tokens): {}", e),
                    }
                }

                x if x == SWAP_EXACT_ETH_FOR_TOKENS => {
                    match swapExactETHForTokensCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            let amount_in = tx.value();
                            info!(
                                "ETH->Token swap | Path: {:?} | In (ETH): {}",
                                decoded.path, amount_in
                            );

                            if decoded.path.len() >= 2 {
                                let token_in = decoded.path[0];
                                let token_out = decoded.path[1];
                                let pair_address = calculate_pair_address(token_in, token_out);
                                let pool = {
                                    let reader = self.cache.read().unwrap();
                                    reader.get(&pair_address).cloned()
                                };

                                if let Some(pool) = pool {
                                    let reserve0 = pool.reserve0;
                                    let reserve1 = pool.reserve1;
                                    // (reserve_in, reserve_out)
                                    let reserves = if pool.token0 == token_in {
                                        (reserve0, reserve1)
                                    } else {
                                        (reserve1, reserve0)
                                    };
                                    let amount_out =
                                        get_amount_out(amount_in, reserves.0, reserves.1);

                                    let price_before =
                                        f64::from(reserves.0) / f64::from(reserves.1);

                                    let price_after = ((f64::from(reserves.0)
                                        + f64::from(amount_in))
                                        * f64::from(997)
                                        / f64::from(1000))
                                        / (f64::from(reserves.1) - f64::from(amount_out));

                                    let impact = (price_after - price_before) / price_before;
                                    info!("🚨HIGH IMPACT: {impact}");
                                }
                            }
                        }
                        Err(e) => warn!("Decode Error (ETH->Tokens): {}", e),
                    }
                }

                x if x == SWAP_EXACT_TOKENS_FOR_ETH => {
                    match swapExactTokensForETHCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            info!(
                                "Token->ETH swap | Path: {:?} | In: {}",
                                decoded.path, decoded.amountIn
                            );
                        }
                        Err(e) => tracing::warn!("Decode Error (Tokens->ETH): {}", e),
                    }
                }
                _ => {
                    debug!("ignore");
                }
            }
        }
    }
}
