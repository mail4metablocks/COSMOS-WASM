# COSMOS-WASM

COSMOS WASM (COSMWASM) is a WebAssembly (WASM) interpreter that can be used to execute smart contracts on the Cosmos blockchain. WASM is a low-level, portable binary format that can be used to run code on a variety of platforms, including web browsers, servers, and embedded devices. By using WASM, developers can write smart contracts in a variety of languages, including C, C++, Rust, and others, and have them run on the Cosmos blockchain.

COSMWASM is implemented in Go and is designed to be used in conjunction with the Cosmos SDK, a framework for building blockchains and applications on the Cosmos network. It provides developers with the tools and resources needed to build decentralized applications (DApps) that can interact with the Cosmos blockchain and take advantage of its features, such as fast transaction speeds, low fees, and high scalability.


## Rust and Go Implementation

This code uses the cosmwasm_vm crate to create a runtime environment for executing the WASM smart contract. The deploy_contract function is used to deploy the contract to the blockchain, and the call_contract function is used to invoke its exported functions.

Both of these examples show how to use COSMWASM to execute a smart contract on the Cosmos blockchain. The specific language used to write the contract (Rust or Go) does not matter, as COSMWASM is able to execute WASM code regardless of the source language.

![image](https://user-images.githubusercontent.com/117555665/208245204-e6737edf-fa8c-4393-a6ba-58deb4bbf8ff.png)


A smart contract is written in a language that can be compiled to WASM, such as Rust, C++, or Go.
The smart contract is compiled to WASM bytecode.
The WASM bytecode is deployed to the Cosmos blockchain using COSMWASM.
COSMWASM loads the WASM bytecode into a virtual machine (VM) and executes it.
The smart contract runs on the VM and can interact with the Cosmos blockchain as needed.
