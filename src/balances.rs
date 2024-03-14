use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &AccountId, value: Balance) {
		self.balances.insert(who.clone(), value);
	}

	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &AccountId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		let balance_from = self.balance(from);
		let balance_to = self.balance(to);

		let new_balance_from = balance_from.checked_sub(&amount).ok_or("Not enough balance")?;
		let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(from, new_balance_from);
		self.set_balance(to, new_balance_to);

		Ok(())
	}
}

#[cfg(test)]
mod balance_tests {
	use super::Pallet;

	#[test]
	fn init_balances() {
		let mut balances = Pallet::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		assert_eq!(balances.balance(&alice), 0);
		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_ok() {
		let mut balances = Pallet::<String, u128>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();
		assert_eq!(balances.balance(&bob), 0);
		assert!(balances.transfer(&alice, &bob, 30).is_err());
		balances.set_balance(&alice, 100);
		assert_eq!(balances.transfer(&alice, &bob, 30), Ok(()));
		assert_eq!(balances.balance(&bob), 30);
		assert_eq!(balances.balance(&alice), 70);
	}
}
