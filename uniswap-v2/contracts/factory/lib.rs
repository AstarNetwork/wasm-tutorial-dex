#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        Storage,
        ZERO_ADDRESS,
    };
    use uniswap_v2::{
        impls::factory::*,
        traits::factory::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId, pair_code_hash: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.factory.pair_contract_code_hash = pair_code_hash;
                instance.factory.fee_to_setter = fee_to_setter;
                instance.factory.fee_to = ZERO_ADDRESS.into();
            })
        }
    }
}