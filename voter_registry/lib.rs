#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::Mapping;

#[ink::contract]
mod voter_registry {
    use super::*;

    #[ink(storage)]
    pub struct VoterRegistry {
        voters: Mapping<AccountId, bool>,
    }

    impl Default for VoterRegistry {
        fn default() -> Self {
            Self::new()
        }
    }

    impl VoterRegistry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                voters: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn register_voter(&mut self, voter: AccountId) {
            self.voters.insert(voter, &true);
        }

        #[ink(message)]
        pub fn is_voter(&self, voter: AccountId) -> bool {
            self.voters.get(voter).unwrap_or(false) // Removed the `&` before `voter`
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn register_and_check_voter() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = VoterRegistry::new();
            let voter = accounts.alice;
            contract.register_voter(voter);
            assert!(contract.is_voter(voter));
        }
    }
}
