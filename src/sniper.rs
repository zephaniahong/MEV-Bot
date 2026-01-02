use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use alloy::{consensus::Transaction, primitives::Address, providers::Provider, sol_types::SolCall};
use anyhow::Result;
use futures_util::StreamExt;

use crate::{
    constants::{
        SWAP_EXACT_ETH_FOR_TOKENS, SWAP_EXACT_TOKENS_FOR_ETH, SWAP_EXACT_TOKENS_FOR_TOKENS,
        UNISWAP_V2_ROUTER,
    },
    types::{swapExactETHForTokensCall, swapExactTokensForETHCall, swapExactTokensForTokensCall},
    utils::calculate_pair_address,
};

/// Listens for pending tx and determines if there is a profitable opportunity
pub async fn start_sniper<P>(
    provider: P,
    cache: Arc<RwLock<HashMap<Address, (u128, u128)>>>,
) -> Result<()>
where
    P: Clone + Send + Sync + 'static,
    P: Provider,
{
    let sub = provider.subscribe_pending_transactions();
    let stream = sub.await?.into_stream();

    let mut buffered_stream = stream
        .map(
            async |tx_hash| match provider.get_transaction_by_hash(tx_hash).await {
                Ok(Some(tx)) if tx.to() == Some(UNISWAP_V2_ROUTER) => Some(tx),
                _ => None,
            },
        )
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
                            println!(
                                "🦄 Token->Token | Path: {:?} | In: {}",
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
                            println!(
                                "🦄 ETH->Token | Path: {:?} | In (ETH): {}",
                                decoded.path, amount_in
                            );

                            if decoded.path.len() >= 2 {
                                let token_in = decoded.path[0];
                                let token_out = decoded.path[1];
                                let pair_address = calculate_pair_address(token_in, token_out);
                                let reserves = {
                                    let reader = cache.read().await;
                                    reader.get(&pair_address).cloned()
                                };

                                if let Some((reserve0, reserve1)) = reserves {
                                    todo!()
                                }
                            }
                        }
                        Err(e) => tracing::warn!("Decode Error (ETH->Tokens): {}", e),
                    }
                }

                x if x == SWAP_EXACT_TOKENS_FOR_ETH => {
                    match swapExactTokensForETHCall::abi_decode(tx_data) {
                        Ok(decoded) => {
                            println!(
                                "🦄 Token->ETH | Path: {:?} | In: {}",
                                decoded.path, decoded.amountIn
                            );
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
