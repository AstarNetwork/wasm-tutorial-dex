use crate::traits::pair::PairRef;
pub use crate::{
    impls::factory::*,
    traits::factory::*,
};
use ink::env::hash::Blake2x256;
use openbrush::traits::{
    AccountId,
    Storage,
    ZERO_ADDRESS,
};

impl<T: Storage<data::Data>> Factory for T {
    fn all_pair_length(&self) -> u64 {
        self.data::<data::Data>().all_pairs.len() as u64
    }

    fn create_pair(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
    ) -> Result<AccountId, FactoryError> {
        if token_a == token_b {
            return Err(FactoryError::IdenticalAddresses)
        }
        let token_pair = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        if token_pair.0 == ZERO_ADDRESS.into() {
            return Err(FactoryError::ZeroAddress)
        }

        let salt = Self::env().hash_encoded::<Blake2x256, _>(&token_pair);
        let pair_contract = self._instantiate_pair(salt.as_ref())?;

        PairRef::initialize(&pair_contract, token_pair.0, token_pair.1)?;

        self.data::<data::Data>()
            .get_pair
            .insert(&(token_pair.0, token_pair.1), &pair_contract);
        self.data::<data::Data>()
            .get_pair
            .insert(&(token_pair.1, token_pair.0), &pair_contract);
        self.data::<data::Data>().all_pairs.push(pair_contract);

        self._emit_create_pair_event(
            token_pair.0,
            token_pair.1,
            pair_contract,
            self.all_pair_length(),
        );

        Ok(pair_contract)
    }

    default fn _instantiate_pair(&mut self, _salt_bytes: &[u8]) -> Result<AccountId, FactoryError> {
        // need to be overridden in contract
        unimplemented!()
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

    default fn _emit_create_pair_event(
        &self,
        _token_0: AccountId,
        _token_1: AccountId,
        _pair: AccountId,
        _pair_len: u64,
    ) {
    }
}
