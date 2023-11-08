#![deny(warnings)]
mod owner;

use std::collections::{HashMap, HashSet};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    proposed_owner_id: AccountId,
    disciplines: UnorderedSet<String>,
    matches: HashMap<String, String>, // Match -> discipline
    reverse_matches: LookupMap<String, HashSet<String>>, // Discipline -> Set<matches>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Disciplines,
    Matches,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            proposed_owner_id: owner_id,
            disciplines: UnorderedSet::new(StorageKey::Disciplines),
            matches: HashMap::new(),
            reverse_matches: LookupMap::new(StorageKey::Matches),
        }
    }

    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn assert_owner(&self) {
        if !self.is_owner(&env::predecessor_account_id()) {
            panic!("This method can be called only by owner")
        }
    }

    pub fn propose_new_owner(&mut self, proposed_owner_id: AccountId) {
        self.assert_owner();
        self.inner_propose_new_owner(proposed_owner_id);
    }

    pub fn accept_ownership(&mut self) {
        assert_eq!(&env::predecessor_account_id(), self.proposed_owner_id(),);
        self.inner_accept_ownership();
    }

    pub fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_disciplines(&self) -> Vec<String> {
        self.disciplines.to_vec()
    }

    pub fn get_all_matches(&self) -> HashMap<String, String> {
        self.matches.clone()
    }

    pub fn get_discipline_matches(&self, discipline: String) -> Option<HashSet<String>> {
        self.reverse_matches.get(&discipline)
    }

    pub fn get_match(&self, discipline_match: String) -> Option<&String> {
        self.matches.get(&discipline_match)
    }

    pub fn add_discipline(&mut self, discipline: String) {
        self.assert_owner();
        assert_eq!(
            self.disciplines.contains(&discipline),
            false,
            "This discipline already exists"
        );

        self.disciplines.insert(&discipline);
        self.reverse_matches.insert(&discipline, &HashSet::new());
    }

    pub fn delete_discipline(&mut self, discipline: String) {
        self.assert_owner();
        assert_eq!(
            self.disciplines.contains(&discipline),
            true,
            "No such discipline"
        );

        let matches = self.reverse_matches.get(&discipline).unwrap();
        for discipline_match in matches {
            self.matches.remove(&discipline_match);
        }
        self.reverse_matches.remove(&discipline);
        self.disciplines.remove(&discipline);
    }

    pub fn add_match(&mut self, discipline_match: String, discipline: String) {
        self.assert_owner();
        assert_eq!(
            self.disciplines.contains(&discipline),
            true,
            "No such discipline"
        );
        assert!(
            self.matches.get(&discipline_match).is_none(),
            "Such match already exists"
        );

        let mut matches = self.reverse_matches.get(&discipline).unwrap();
        matches.insert(discipline_match.clone());

        self.reverse_matches.insert(&discipline, &matches);
        self.matches.insert(discipline_match, discipline);
    }

    pub fn delete_match(&mut self, discipline_match: String) {
        self.assert_owner();

        let discipline = self
            .matches
            .get(&discipline_match)
            .expect("Such match doesn't exist");
        let mut matches = self.reverse_matches.get(&discipline).unwrap();
        matches.remove(&discipline_match);

        self.reverse_matches.insert(&discipline, &matches);
        self.matches.remove(&discipline_match);
    }
}
