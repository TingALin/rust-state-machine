use std::collections::BTreeMap;
use num::traits::{One, Zero};
use std::ops::AddAssign;

pub trait Config {
    type BlockNumber: Zero + One + AddAssign + Copy;
	type AccountId: Ord + Clone;
	type Nonce:Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>
{
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn get_block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn set_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod system_test {
	use super::Pallet;

    struct TestConfig;
    impl super::Config for TestConfig {
        type Nonce = u32;
	    type BlockNumber = u32;
	    type AccountId = String;
    }

	#[test]
	fn init_system() {
		let mut system = Pallet::<TestConfig>::new();
		let alice = "alice".to_string();

		assert_eq!(system.get_block_number(), 0);
		system.set_block_number();
		assert_eq!(system.get_block_number(), 1);

		assert_eq!(system.nonce.get(&alice), None);
		system.inc_nonce(&alice);
		assert_eq!(system.nonce.get(&alice), Some(&1));
	}
}
