#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use openbrush::traits::{
        Storage,
        ZERO_ADDRESS,
    };
    use uniswap_v2::{
        impls::factory::*,
        traits::factory::*,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId, pair_code_hash: Hash) -> Self {
            let mut instance = Self::default();
            instance.factory.pair_contract_code_hash = pair_code_hash;
            instance.factory.fee_to_setter = fee_to_setter;
            instance.factory.fee_to = ZERO_ADDRESS.into();
            instance
        }
    }
}
