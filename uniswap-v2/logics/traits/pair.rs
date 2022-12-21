use openbrush::{
    contracts::{
        traits::{
            psp22::PSP22Error,
        },
    },
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
    fn mint(&mut self, to: AccountId) -> Result<Balance, PairError>;

    fn _mint_fee(&mut self, reserve_0: Balance, reserve_1: Balance) -> Result<bool, PairError>;

    fn _update(
        &mut self,
        balance_0: Balance,
        balance_1: Balance,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<(), PairError>;

    #[ink(message)]
    fn get_token_0(&self) -> AccountId;

    #[ink(message)]
    fn get_token_1(&self) -> AccountId;

    fn _emit_mint_event(&self, _sender: AccountId, _amount_0: Balance, _amount_1: Balance);

    fn _emit_sync_event(&self, reserve_0: Balance, reserve_1: Balance);
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PairError {
    PSP22Error(PSP22Error),
    InsufficientLiquidityMinted,
    Overflow,
    SubUnderFlow1,
    SubUnderFlow2,
    SubUnderFlow3,
    SubUnderFlow14,
    MulOverFlow1,
    MulOverFlow2,
    MulOverFlow3,
    MulOverFlow4,
    MulOverFlow5,
    MulOverFlow14,
    MulOverFlow15,
    DivByZero1,
    DivByZero2,
    DivByZero5,
    AddOverflow1,
}

impl From<PSP22Error> for PairError {
    fn from(error: PSP22Error) -> Self {
        PairError::PSP22Error(error)
    }
}
