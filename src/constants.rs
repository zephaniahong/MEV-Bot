use alloy::primitives::{Address, B256, address, b256};

pub const UNISWAP_V2_ROUTER: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

// 2. The Function Selectors (The "Verbs")
// standard: swapExactTokensForTokens(uint256,uint256,address[],address,uint256)
pub const SWAP_EXACT_TOKENS_FOR_TOKENS: [u8; 4] = [0x38, 0xed, 0x17, 0x39];

// standard: swapExactETHForTokens(uint256,address[],address,uint256)
pub const SWAP_EXACT_ETH_FOR_TOKENS: [u8; 4] = [0x7f, 0xf3, 0x6a, 0xb5];

// standard: swapExactTokensForETH(uint256,uint256,address[],address,uint256)
pub const SWAP_EXACT_TOKENS_FOR_ETH: [u8; 4] = [0x18, 0xcb, 0xaf, 0xe5];

// The Uniswap V2 Factory (The "Creator")
pub const UNISWAP_V2_FACTORY: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

// The Init Code Hash (The "DNA")
pub const UNISWAP_V2_INIT_CODE_HASH: B256 =
    b256!("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f");
