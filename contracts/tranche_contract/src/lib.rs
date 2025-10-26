#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AddContract;

#[contractimpl]
impl AddContract {
    pub fn add(env: Env, left: u64, right: u64) -> u64 {
        left + right
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn it_works() {
        let env = Env::default();
        let contract_id = env.register(AddContract, ());
        let client = AddContractClient::new(&env, &contract_id);
        let res = client.add(&symbol_short!("…")); // 如果你用了 Symbol

        assert_eq!(res, 4u64);
    }
}
