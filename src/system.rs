use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self{
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn get_block_number(&self) -> u32 {
        self.block_number
    }

    pub fn set_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &String){
        let new_nonce = self.nonce.get(who).unwrap_or(&0).checked_add(1).unwrap_or(0);
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod system_test{
    use super::Pallet;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();
        let alice = "alice".to_string();

        assert_eq!(system.get_block_number(), 0);
        system.set_block_number();
        assert_eq!(system.get_block_number(), 1);

        assert_eq!(system.nonce.get(&alice), None);
        system.inc_nonce(&alice);
        assert_eq!(system.nonce.get(&alice), Some(&1));
    }
}