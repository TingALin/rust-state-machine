use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	// type AccountId: Ord + Clone;
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountId, value: T::Balance) {
		self.balances.insert(who.clone(), value);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		let balance_from = self.balance(from);
		let balance_to = self.balance(to);

		let new_balance_from = balance_from.checked_sub(&amount).ok_or("Not enough balance")?;
		let new_balance_to = balance_to.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(from, new_balance_from);
		self.set_balance(to, new_balance_to);

		Ok(())
	}
}

pub enum Call<T: Config> {
	Transfer { to: T::AccountId, amount: T::Balance },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			Call::Transfer { to, amount } => {
				self.transfer(&caller, &to, amount)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod balance_tests {
	use super::Pallet;

	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();

		assert_eq!(balances.balance(&alice), 0);
		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_ok() {
		let mut balances = Pallet::<TestConfig>::new();

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
