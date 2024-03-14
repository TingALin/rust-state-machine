mod balances;
mod proof_of_existence;
mod support;
mod system;
use crate::support::Dispatch;

mod types {
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type AccountId = String;
	pub type Balance = u128;

	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;

	pub type Content = &'static str;
}

pub enum RuntimeCall {
	// BalancesTransfer { to: types::AccountId, amount: types::Balance },
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
	type Balance = types::Balance;
}
impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.set_block_number();
		if self.system.get_block_number() != block.header.block_number {
			return Err("Wrong block number");
		}
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _ = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			// RuntimeCall::BalancesTransfer { to, amount } => {
			// 	self.balances.transfer(&caller, &to, amount)?;
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) => {
				self.proof_of_existence.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&alice, 100);
	// runtime.system.set_block_number();
	// assert_eq!(runtime.system.get_block_number(), 1);

	// runtime.system.inc_nonce(&alice);
	// let _ = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| eprintln!("{}", e));
	// runtime.system.inc_nonce(&alice);
	// let _ = runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| eprintln!("{}", e));

	// let block_1 = types::Block {
	// 	header: support::Header { block_number: 1 },
	// 	extrinsics: vec![
	// 		support::Extrinsic {
	// 			caller: alice.clone(),
	// 			// call: RuntimeCall::BalancesTransfer { to: bob, amount: 20 },
	// 			call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 20 }),
	// 		},
	// 		support::Extrinsic {
	// 			caller: alice,
	// 			// call: RuntimeCall::BalancesTransfer { to: charlie, amount: 20 },
	// 			call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
	// 		},
	// 	],
	// };
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: bob.clone(),
					amount: 20,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: &"Hello, world!",
				}),
			},
		],
	};

	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: &"Hello, world!",
				}),
			},
		],
	};
	
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");
	
	println!("{:#?}", runtime);
}
