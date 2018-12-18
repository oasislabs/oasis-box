# oasis-box

A truffle box to support compiling, migrating, and testing both Solidity and Rust contracts with or without confidentiality in the Oasis cloud.

## Installation

Note: It's suggested to use the [Contract Kit](https://docs.oasiscloud.io/en/latest/contract-kit/) docker image so that you can skip installation and take advantage of complete Truffle integration with Oasis. Currently in development is support for a Truffle workflow with Rust contracts outside of Contract Kit. In the meantine, if you're using Truffle to develop Rust contracts, we'll assume you're using Contract Kit.

If you're using Solidity outside of Contract Kit, ensure you have truffle v5 installed. If you have another version of truffle installed you'll want to either uninstall it or ensure you're using v5 with this box.

`npm install -g truffle`

Now download the truffle box.

`truffle unbox oasislabs/oasis-box`

## Using oasis-box

Once installed, you can use truffle commands like you would any other truffle project. However, you'll want to make sure you're pointing at an Oasis gateway, for example, by specifying `--network oasis`. The supported commands are

- `truffle compile`
- `truffle migrate --network oasis`
- `truffle test --network oasis`.

To mark your contract confidential, prefix your contract's filename with either `confidential_` or `confidential-*`, for example, `confidential_MyContract.sol` for Solidity or `confidential-my-contract-crate` for Rust. When your contract compiles, with one of the above truffle commands, its bytecode will be prepended with `b'\0enc'`. On deploy, this bytecode prefix will notify the Oasis runtime that the contract is to be run inside a secure enclave, at which point one should use [web3c.js](https://github.com/oasislabs/web3c.js) to communicate with the contract through an encrypted channel.

To do this, simply retrieve the contract's deploy address from the artifact, and instantiate a web3c.js Contract object with it. For example,

```
const MyContract = artifacts.require("MyContract");
const contractInstance = web3c.confidential.Contract(MyContract.abi, MyContract.address, { ... });
```

For more details, see the [truffle](https://truffleframework.com/docs/truffle/overview) and [web3c.js](https://github.com/oasislabs/web3c.js) documentation.
