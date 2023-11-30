#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod atbash_voting {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct AtbashVoting {
        votes_received: ink::storage::Mapping<String, u128>,
        candidate_list: Vec<String>,
    }

    impl AtbashVoting {
        #[ink(constructor)]
        pub fn new(candidate_names: Vec<String>) -> Self {
            let votes_received = Mapping::default();
            Self {
                candidate_list: candidate_names,
                votes_received,
            }
        }

        #[ink(message)]
        pub fn valid_candidate(&self, candidate: String) -> bool {
            for i in 0..self.candidate_list.len() {
                if self.candidate_list[i] == candidate {
                    return true;
                };
            }
            return false;
        }

        #[ink(message)]
        pub fn total_votes_for(&self, candidate: String) -> u128 {
            assert!(
                !self.valid_candidate(candidate.clone()),
                "Not a valid candidate"
            );
            self.votes_received
                .get(candidate.clone())
                .unwrap_or_default()
        }

        #[ink(message)]
        pub fn vote_for_candidate(&mut self, candidate: String) {
            assert!(
                !self.valid_candidate(candidate.clone()),
                "Not a valid candidate"
            );
            self.votes_received.insert(
                &candidate,
                &(self.votes_received.get(&candidate).unwrap_or(0) + 1),
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::atbash_voting::AtbashVoting;

    use super::*;

    #[test]
    fn atbash_voting_test() {
        let contract = AtbashVoting::new(vec![
            "peter".to_string(),
            "ippo".to_string(),
            "phuong".to_string(),
        ]);

        assert!(contract.valid_candidate("peter".to_string()));
    }
}
