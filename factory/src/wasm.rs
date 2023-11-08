#[cfg(feature = "mainnet")]
pub(crate) const fn code() -> &'static [u8] {
    include_bytes!("../../target/wasm32-unknown-unknown/mainnet/token.wasm")
}

#[cfg(feature = "testnet")]
pub(crate) const fn wasm_code() -> &'static [u8] {
    include_bytes!("../../target/wasm32-unknown-unknown/testnet/token.wasm")
}

#[cfg(feature = "sandbox")]
pub(crate) const fn wasm_code() -> &'static [u8] {
    include_bytes!("../../target/wasm32-unknown-unknown/sandbox/token.wasm")
}

#[cfg(debug_assertions)]
pub(crate) const fn wasm_code() -> &'static [u8] {
    &[0; 0]
}
