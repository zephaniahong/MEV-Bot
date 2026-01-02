use alloy::{
    primitives::{Address, U256, keccak256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol_types::SolCall,
};

use crate::{
    constants::{UNISWAP_V2_FACTORY, UNISWAP_V2_INIT_CODE_HASH},
    types::getReservesCall,
};

/// Fetches the reserves (`reserve0, `reserve1`) of a Uniswap V2 Pair
///
/// Performs a `eth_call` to the blockchain to query the contract state
///
/// # Arguments
///
/// * `provider` - The RPC provider used to send the request
/// * `pair_address` - The address of the Uniswap V2 Pair contract
///
/// # Returns
///
/// * `Result<(u128, u128)>` - A tuple containing `(reserve0, reserve1)` cast to `u128`.
pub async fn fetch_reserves(
    provider: &impl Provider,
    pair_address: Address,
) -> anyhow::Result<(u128, u128)> {
    let calldata = getReservesCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .to(pair_address)
        .input(calldata.into());

    let output = provider.call(tx).await?;

    let decoded_data = getReservesCall::abi_decode_returns(&output)?;

    return Ok((
        decoded_data.reserve0.to::<u128>(),
        decoded_data.reserve1.to::<u128>(),
    ));
}

/// Calculates the deterministic CREATE2 address for a Uniswap V2 Pair
///
/// This function sorts the tokens, computes the salt,
/// and derives the address using the Factory and InitCodeHash constants
///
/// # Arguments
/// * `token_a` - The address of the first token (order does not matter)
/// * `token_b` - The address of the second token
///
/// # Returns
///
/// * `Address` - The computed address of the pair
pub fn calculate_pair_address(token_a: Address, token_b: Address) -> Address {
    let (token0, token1) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    let mut salt_input = [0u8; 40];

    salt_input[0..20].copy_from_slice(token0.as_slice());
    salt_input[20..40].copy_from_slice(token1.as_slice());

    let salt = keccak256(salt_input);

    let mut packed = Vec::with_capacity(85);

    packed.push(0xff);
    packed.extend_from_slice(UNISWAP_V2_FACTORY.as_slice());
    packed.extend_from_slice(salt.as_slice());
    packed.extend_from_slice(UNISWAP_V2_INIT_CODE_HASH.as_slice());

    let hash = keccak256(packed);

    Address::from_slice(&hash[12..])
}

/// Calculates the output amount of a swap given an input amount and a pair reserves
///
/// Implements the standard Uniswap V2 Library math:
/// amount_out = (amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)
/// # Arguments
///
/// * `amount_in` - The amount of input tokens.
/// * `reserve_in` - The reserve of the input token in the pair.
/// * `reserve_out` - The reserve of the output token in the pair.
///
/// # Returns
///
/// * `U256` - The amount of output tokens the user will receive.
///
/// # Panics
///
/// Panics if `amount_in` is zero or if reserves are zero.
pub fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256) -> U256 {
    if amount_in == U256::ZERO || reserve_in == U256::ZERO || reserve_out == U256::ZERO {
        panic!("get_amount_out: Insufficient input or liquidity");
    }

    let amount_in_with_fee = amount_in.checked_mul(U256::from(997)).unwrap();
    let numerator = amount_in_with_fee.checked_mul(reserve_out).unwrap();

    let denominator_reserve = reserve_in.checked_mul(U256::from(1000)).unwrap();
    let denominator = denominator_reserve.checked_add(amount_in_with_fee).unwrap();

    numerator.checked_div(denominator).unwrap()
}
