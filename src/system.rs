use std::collections::BTreeMap;
use num::traits::{One, Zero};
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
	BlockNumber: Zero + One + AddAssign + Copy,
	AccountId: Ord + Clone,
	Nonce:Zero + One + Copy,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn get_block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn set_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = *self.nonce.get(&who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod system_test {
	use super::Pallet;

	#[test]
	fn init_system() {
		let mut system = Pallet::<u32, String, u32>::new();
		let alice = "alice".to_string();

		assert_eq!(system.get_block_number(), 0);
		system.set_block_number();
		assert_eq!(system.get_block_number(), 1);

		assert_eq!(system.nonce.get(&alice), None);
		system.inc_nonce(&alice);
		assert_eq!(system.nonce.get(&alice), Some(&1));
	}
}
