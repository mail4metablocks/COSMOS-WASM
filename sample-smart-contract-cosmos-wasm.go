package main

import (
	"fmt"

	cosmwasm "github.com/cosmos/cosmwasm/go/cosmwasm"
)

func main() {
	vm, err := cosmwasm.NewVM(1024)
	if err != nil {
		panic(err)
	}

	contractAddr, err := vm.DeployContract([]byte("contract.wasm"))
	if err != nil {
		panic(err)
	}

	result, err := vm.CallContract(contractAddr, []byte("hello"), nil)
	if err != nil {
		panic(err)
	}

	fmt.Println("Result:", result)
}
