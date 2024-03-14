use std::collections::BTreeMap;


pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, value: u128) {
        self.balances.insert(who.to_string(), value);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: u128) -> Result<(), &'static str>{
        let balance_from = self.balance(from);
        let balance_to = self.balance(to);

        let new_balance_from = balance_from.checked_sub(amount).ok_or("Not enough balance")?;
        let new_balance_to = balance_to.checked_add(amount).ok_or("Overflow")?;

        self.set_balance(from, new_balance_from);
        self.set_balance(to, new_balance_to);

        Ok(())
    }
}

#[cfg(test)]
mod balance_tests{
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
        let mut balances = Pallet::new();

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