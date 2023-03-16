use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type FactoryRef = dyn Factory;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn all_pair_length(&self) -> u64;

    #[ink(message)]
    fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), FactoryError>;

    #[ink(message)]
    fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), FactoryError>;

    #[ink(message)]
    fn fee_to(&self) -> AccountId;

    #[ink(message)]
    fn fee_to_setter(&self) -> AccountId;

    #[ink(message)]
    fn get_pair(&self, token_a: AccountId, token_b: AccountId) -> Option<AccountId>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FactoryError {
    Error,
}
