extern crate cosmwasm_vm;

use cosmwasm_vm::{error::Error, runtime};

fn main() -> Result<(), Error> {
    let mut runtime = runtime::Builder::new()
        .with_memory(1024)
        .with_trait(Box::new(MyTraitImpl))
        .build()?;

    let wasm = include_bytes!("contract.wasm");
    let contract_address = runtime.deploy_contract(wasm)?;

    let result = runtime.call_contract(contract_address, b"hello", &[])?;
    println!("Result: {:?}", result);

    Ok(())
}
