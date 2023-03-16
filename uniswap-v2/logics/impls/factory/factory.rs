pub use crate::{
    impls::factory::*,
    traits::factory::*,
};
use openbrush::traits::{
    AccountId,
    Storage,
};

impl<T: Storage<data::Data>> Factory for T {
    fn all_pair_length(&self) -> u64 {
        self.data::<data::Data>().all_pairs.len() as u64
    }

    fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), FactoryError> {
        self.data::<data::Data>().fee_to = fee_to;
        Ok(())
    }

    fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), FactoryError> {
        self.data::<data::Data>().fee_to_setter = fee_to_setter;
        Ok(())
    }

    fn fee_to(&self) -> AccountId {
        self.data::<data::Data>().fee_to
    }

    fn fee_to_setter(&self) -> AccountId {
        self.data::<data::Data>().fee_to_setter
    }

    fn get_pair(&self, token_a: AccountId, token_b: AccountId) -> Option<AccountId> {
        self.data::<data::Data>().get_pair.get(&(token_a, token_b))
    }
}
