const Counter = artifacts.require("Counter");
const ConfidentialCounter = artifacts.require("ConfidentialCounter");
const NestedCounter = artifacts.require("NestedCounter");
const WasmCounter = artifacts.require("WasmCounter");
const ConfidentialWasmCounter = artifacts.require("ConfidentialWasmCounter");
const NestedWasmCounter = artifacts.require("NestedWasmCounter");

module.exports = function(deployer) {
  // deploy solidity contracts
  deployer.deploy(Counter);
  deployer.deploy(ConfidentialCounter);
  deployer.deploy(NestedCounter);
  // deploy wasm contracts
  deployer.deploy(WasmCounter);
  deployer.deploy(ConfidentialWasmCounter);
  deployer.deploy(NestedWasmCounter);
}
