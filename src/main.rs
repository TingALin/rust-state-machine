mod balances;
mod system;

mod types {
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type AccountId = String;
	pub type Balance = u128;
}
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config  for Runtime {
    // type AccountId = types::AccountId;
    type Balance = types::Balance;
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "bob".to_string();

	runtime.balances.set_balance(&alice, 100);
	runtime.system.set_block_number();
	assert_eq!(runtime.system.get_block_number(), 1);

	runtime.system.inc_nonce(&alice);
	let _ = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| eprintln!("{}", e));
	runtime.system.inc_nonce(&alice);
	let _ = runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| eprintln!("{}", e));
}
