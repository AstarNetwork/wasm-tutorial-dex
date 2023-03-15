use crate::traits::{
    factory::FactoryRef
};
pub use crate::{
    impls::pair::*,
    traits::pair::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        psp22::*,
        traits::psp22::PSP22Ref,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        Timestamp,
        ZERO_ADDRESS,
    },
};

pub const MINIMUM_LIQUIDITY: u128 = 1000;

impl<T: Storage<data::Data> + Storage<psp22::Data>> Pair for T {
    fn get_reserves(&self) -> (Balance, Balance, Timestamp) {
        (
            self.data::<data::Data>().reserve_0,
            self.data::<data::Data>().reserve_1,
            self.data::<data::Data>().block_timestamp_last,
        )
    }

    fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), PairError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        Ok(())
    }

    fn mint(&mut self, to: AccountId) -> Result<Balance, PairError> {
        let reserves = self.get_reserves();
        let contract = Self::env().account_id();
        let balance_0 = PSP22Ref::balance_of(&self.data::<data::Data>().token_0, contract);
        let balance_1 = PSP22Ref::balance_of(&self.data::<data::Data>().token_1, contract);
        let amount_0 = balance_0
            .checked_sub(reserves.0)
            .ok_or(PairError::SubUnderFlow1)?;
        let amount_1 = balance_1
            .checked_sub(reserves.1)
            .ok_or(PairError::SubUnderFlow2)?;

        let fee_on = self._mint_fee(reserves.0, reserves.1)?;
        let total_supply = self.data::<psp22::Data>().supply;

        let liquidity;
        if total_supply == 0 {
            let liq = amount_0
                .checked_mul(amount_1)
                .ok_or(PairError::MulOverFlow1)?;
            liquidity = sqrt(liq)
                .checked_sub(MINIMUM_LIQUIDITY)
                .ok_or(PairError::SubUnderFlow3)?;
            self._mint_to(ZERO_ADDRESS.into(), MINIMUM_LIQUIDITY)?;
        } else {
            let liquidity_1 = amount_0
                .checked_mul(total_supply)
                .ok_or(PairError::MulOverFlow2)?
                .checked_div(reserves.0)
                .ok_or(PairError::DivByZero1)?;
            let liquidity_2 = amount_1
                .checked_mul(total_supply)
                .ok_or(PairError::MulOverFlow3)?
                .checked_div(reserves.1)
                .ok_or(PairError::DivByZero2)?;
            liquidity = min(liquidity_1, liquidity_2);
        }

        if liquidity == 0 {
            return Err(PairError::InsufficientLiquidityMinted)
        }

        self._mint_to(to, liquidity)?;

        self._update(balance_0, balance_1, reserves.0, reserves.1)?;

        if fee_on {
            let k = reserves
                .0
                .checked_mul(reserves.1)
                .ok_or(PairError::MulOverFlow5)?;
            self.data::<data::Data>().k_last = k;
        }

        self._emit_mint_event(Self::env().caller(), amount_0, amount_1);

        Ok(liquidity)
    }

    fn burn(&mut self, to: AccountId) -> Result<(Balance, Balance), PairError> {
        let reserves = self.get_reserves();
        let contract = Self::env().account_id();
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        let mut balance_0 = PSP22Ref::balance_of(&token_0, contract);
        let mut balance_1 = PSP22Ref::balance_of(&token_1, contract);
        let liquidity = self._balance_of(&contract);

        let fee_on = self._mint_fee(reserves.0, reserves.1)?;
        let total_supply = self.data::<psp22::Data>().supply;
        let amount_0 = liquidity
            .checked_mul(balance_0)
            .ok_or(PairError::MulOverFlow6)?
            .checked_div(total_supply)
            .ok_or(PairError::DivByZero3)?;
        let amount_1 = liquidity
            .checked_mul(balance_1)
            .ok_or(PairError::MulOverFlow7)?
            .checked_div(total_supply)
            .ok_or(PairError::DivByZero4)?;

        if amount_0 == 0 || amount_1 == 0 {
            return Err(PairError::InsufficientLiquidityBurned)
        }

        self._burn_from(contract, liquidity)?;

        self._safe_transfer(token_0, to, amount_0)?;
        self._safe_transfer(token_1, to, amount_1)?;

        balance_0 = PSP22Ref::balance_of(&token_0, contract);
        balance_1 = PSP22Ref::balance_of(&token_1, contract);

        self._update(balance_0, balance_1, reserves.0, reserves.1)?;

        if fee_on {
            let k = reserves
                .0
                .checked_mul(reserves.1)
                .ok_or(PairError::MulOverFlow5)?;
            self.data::<data::Data>().k_last = k;
        }

        self._emit_burn_event(Self::env().caller(), amount_0, amount_1, to);

        Ok((amount_0, amount_1))
    }

    fn _mint_fee(&mut self, reserve_0: Balance, reserve_1: Balance) -> Result<bool, PairError> {
        let fee_to = FactoryRef::fee_to(&self.data::<data::Data>().factory);
        let fee_on = fee_to != ZERO_ADDRESS.into();
        let k_last = self.data::<data::Data>().k_last;
        if fee_on {
            if k_last != 0 {
                let root_k = sqrt(
                    reserve_0
                        .checked_mul(reserve_1)
                        .ok_or(PairError::MulOverFlow14)?,
                );
                let root_k_last = sqrt(k_last);
                if root_k > root_k_last {
                    let total_supply = self.data::<psp22::Data>().supply;
                    let numerator = total_supply
                        .checked_mul(
                            root_k
                                .checked_sub(root_k_last)
                                .ok_or(PairError::SubUnderFlow14)?,
                        )
                        .ok_or(PairError::MulOverFlow15)?;
                    let denominator = root_k
                        .checked_mul(5)
                        .ok_or(PairError::MulOverFlow15)?
                        .checked_add(root_k_last)
                        .ok_or(PairError::AddOverflow1)?;
                    let liquidity = numerator
                        .checked_div(denominator)
                        .ok_or(PairError::DivByZero5)?;
                    if liquidity > 0 {
                        self._mint_to(fee_to, liquidity)?;
                    }
                }
            }
        } else if k_last != 0 {
            self.data::<data::Data>().k_last = 0;
        }
        Ok(fee_on)
    }

    fn _update(
        &mut self,
        balance_0: Balance,
        balance_1: Balance,
        reserve_0: Balance,
        reserve_1: Balance,
    ) -> Result<(), PairError> {
        if balance_0 == u128::MAX || balance_1 == u128::MAX {
            return Err(PairError::Overflow)
        }
        let now = Self::env().block_timestamp();
        let time_elapsed = now - self.data::<data::Data>().block_timestamp_last;
        if time_elapsed > 0 && reserve_0 != 0 && reserve_1 != 0 {
            let price_cumulative_last_0 = (reserve_1 / reserve_0)
                .checked_mul(time_elapsed as u128)
                .ok_or(PairError::MulOverFlow4)?;
            let price_cumulative_last_1 = (reserve_0 / reserve_1)
                .checked_mul(time_elapsed as u128)
                .ok_or(PairError::MulOverFlow4)?;
            self.data::<data::Data>().price_0_cumulative_last += price_cumulative_last_0;
            self.data::<data::Data>().price_1_cumulative_last += price_cumulative_last_1;
        }
        self.data::<data::Data>().reserve_0 = balance_0;
        self.data::<data::Data>().reserve_1 = balance_1;
        self.data::<data::Data>().block_timestamp_last = now;

        self._emit_sync_event(reserve_0, reserve_1);
        Ok(())
    }

    fn _safe_transfer(
        &mut self,
        token: AccountId,
        to: AccountId,
        value: Balance,
    ) -> Result<(), PairError> {
        PSP22Ref::transfer(&token, to, value, Vec::new())?;
        Ok(())
    }

    fn get_token_0(&self) -> AccountId {
        self.data::<data::Data>().token_0
    }

    fn get_token_1(&self) -> AccountId {
        self.data::<data::Data>().token_1
    }

    default fn _emit_mint_event(&self, _sender: AccountId, _amount_0: Balance, _amount_1: Balance) {}

    default fn _emit_sync_event(&self, _reserve_0: Balance, _reserve_1: Balance) {}

    default fn _emit_burn_event(
        &self,
        _sender: AccountId,
        _amount_0: Balance,
        _amount_1: Balance,
        _to: AccountId,
    ) {
    }
}

fn min(x: u128, y: u128) -> u128 {
    if x < y {
        return x
    }
    y
}

fn sqrt(y: u128) -> u128 {
    let mut z = 1;
    if y > 3 {
        z = y;
        let mut x = y / 2 + 1;
        while x < z {
            z = x;
            x = (y / x + x) / 2;
        }
    }
    z
}
