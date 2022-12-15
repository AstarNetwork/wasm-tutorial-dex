use openbrush::{
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};

#[openbrush::wrapper]
pub type PairRef = dyn Pair;

#[openbrush::trait_definition]
pub trait Pair {
    #[ink(message)]
    fn get_reserves(&self) -> (Balance, Balance, Timestamp);

    #[ink(message)]
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), PairError>;

    #[ink(message)]
    fn get_token_0(&self) -> AccountId;

    #[ink(message)]
    fn get_token_1(&self) -> AccountId;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PairError {
    Error
}
