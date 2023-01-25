//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
    use crate::Web3;

    pub fn web3() -> Web3 {
        Web3::new("http://127.0.0.1:8545").unwrap()
    }

    pub fn get_contract() -> Vec<u8> {
        include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20.wasm").to_vec()
    }
}
