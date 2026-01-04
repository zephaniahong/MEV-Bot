use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use alloy::{
    consensus::Transaction,
    primitives::{Address, U256},
    providers::Provider,
    sol_types::SolCall,
};
use anyhow::Result;
use futures_util::StreamExt;
use tracing::{error, info, warn};

use crate::{
    Pool,
    constants::{
        SWAP_EXACT_ETH_FOR_TOKENS, SWAP_EXACT_TOKENS_FOR_ETH, SWAP_EXACT_TOKENS_FOR_TOKENS,
        UNISWAP_V2_ROUTER,
    },
    types::{swapExactETHForTokensCall, swapExactTokensForETHCall, swapExactTokensForTokensCall},
    utils::{calculate_pair_address, get_amount_out},
};

/// Listens for pending tx and determines if there is a profitable opportunity
pub async fn start_sniper<P>(provider: P, cache: Arc<RwLock<HashMap<Address, Pool>>>) -> Result<()>
where
    P: Clone + Send + Sync + 'static,
    P: Provider,
{
    let sub = provider.subscribe_pending_transactions();
    let stream = sub.await?.into_stream();

    let mut buffered_stream = stream
        .map(|tx_hash| {
            let provider = provider.clone();
            async move {
                match provider.get_transaction_by_hash(tx_hash).await {
                    Ok(Some(tx)) if tx.to() == Some(UNISWAP_V2_ROUTER) => Some(tx),
                    _ => None,
                }
            }
        })
        .buffer_unordered(10)
        .filter_map(|res| std::future::ready(res));

    while let Some(tx) = buffered_stream.next().await {
        let tx_data = tx.input();
        if tx_data.len() >= 4 {
            let selector = &tx_data[0..4];

            match selector {
                x if x == SWAP_EXACT_TOKENS_FOR_TOKENS => {
                    match swapExactTokensForTokensCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            // println!(
                            //     "🦄 Token->Token | Path: {:?} | In: {}",
                            //     decoded.path, decoded.amountIn
                            // );
                            println!("ignore")
                        }
                        Err(e) => tracing::warn!("Decode Error (Tokens->Tokens): {}", e),
                    }
                }

                x if x == SWAP_EXACT_ETH_FOR_TOKENS => {
                    match swapExactETHForTokensCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            let amount_in = tx.value();
                            println!(
                                "🦄 ETH->Token | Path: {:?} | In (ETH): {}",
                                decoded.path, amount_in
                            );

                            if decoded.path.len() >= 2 {
                                let token_in = decoded.path[0];
                                let token_out = decoded.path[1];
                                let pair_address = calculate_pair_address(token_in, token_out);
                                let pool = {
                                    let reader = cache.read().await;
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
                                    if impact >= 0.05 {
                                        info!("🚨HIGH IMPACT: {impact}");
                                    }
                                }
                            }
                        }
                        Err(e) => warn!("Decode Error (ETH->Tokens): {}", e),
                    }
                }

                x if x == SWAP_EXACT_TOKENS_FOR_ETH => {
                    match swapExactTokensForETHCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            // println!(
                            //     "🦄 Token->ETH | Path: {:?} | In: {}",
                            //     decoded.path, decoded.amountIn
                            // );
                            println!("ignore");
                        }
                        Err(e) => tracing::warn!("Decode Error (Tokens->ETH): {}", e),
                    }
                }
                _ => {
                    println!("ignore");
                }
            }
        }
    }
    Ok(())
}
