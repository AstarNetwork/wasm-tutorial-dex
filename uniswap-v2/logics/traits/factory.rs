use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type FactoryRef = dyn Factory;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn fee_to(&self) -> AccountId;
}
