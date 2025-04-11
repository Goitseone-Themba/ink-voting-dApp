#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::env::call::build_call;
use ink::env::DefaultEnvironment;
use ink::storage::Mapping;

#[ink::contract]
mod voting {
    use super::*;

    #[ink(storage)]
    pub struct Voting {
        votes: Mapping<u32, u32>,               // Proposal ID -> Vote count
        voter_registry: AccountId,              // Address of the VoterRegistry contract
        voted: Mapping<(AccountId, u32), bool>, // (Voter, Proposal ID) -> Has voted
    }

    impl Voting {
        #[ink(constructor)]
        pub fn new(voter_registry: AccountId) -> Self {
            Self {
                votes: Mapping::default(),
                voter_registry,
                voted: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) {
            let caller = self.env().caller();
            // Check if the caller is a registered voter
            let is_voter: bool = build_call::<DefaultEnvironment>()
                .call(self.voter_registry)
                .exec_input(
                    ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(
                        ink::selector_bytes!("is_voter"),
                    ))
                    .push_arg(&caller),
                )
                .returns::<bool>()
                .invoke();
            assert!(is_voter, "Caller is not a registered voter");

            // Check if the caller has already voted for this proposal
            let has_voted = self.voted.get((caller, proposal_id)).unwrap_or(false);
            assert!(!has_voted, "Caller has already voted");

            // Record the vote with checked arithmetic
            let current_votes = self.votes.get(proposal_id).unwrap_or(0);
            let new_votes = current_votes.checked_add(1).expect("Vote count overflow");
            self.votes.insert(proposal_id, &new_votes);
            self.voted.insert((caller, proposal_id), &true);
        }

        #[ink(message)]
        pub fn get_votes(&self, proposal_id: u32) -> u32 {
            self.votes.get(proposal_id).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn get_votes_initially_zero() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let voter_registry = accounts.bob; // Simulate the VoterRegistry contract address
            let contract = Voting::new(voter_registry);
            assert_eq!(contract.get_votes(1), 0); // Proposal ID 1 should have 0 votes initially
        }
    }
}
