#![no_std]
#![feature(extern_prelude)]

static COUNTER_KEY: H256 = H256([
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

#[owasm_abi_derive::contract]
trait WasmCounter {
    fn constructor(&mut self) {
        owasm_ethereum::write(&COUNTER_KEY, &U256::zero().into());
    }

    #[constant]
    fn count(&mut self) -> U256 {
        U256::from_big_endian(&owasm_ethereum::read(&COUNTER_KEY))
    }

    fn increment(&mut self) {
        owasm_ethereum::write(&COUNTER_KEY, &(self.count() + 1).into());
    }
}
