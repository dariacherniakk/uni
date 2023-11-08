#![deny(warnings)]
mod owner;
mod wasm;

use near_sdk::serde_json::{self};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    env, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise,
};
use serde_json::json;

const GAS_FOR_CREATE_UNI: Gas = Gas(50_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    proposed_owner_id: AccountId,
    uni: UnorderedSet<AccountId>, // Uni accounts
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Uni,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            proposed_owner_id: owner_id,
            uni: UnorderedSet::new(StorageKey::Uni),
        }
    }

    #[payable]
    pub fn add_uni(&mut self, uni_name: String, uni_owner_id: AccountId) -> Promise {
        self.assert_owner();

        let deposit = env::attached_deposit();
        let uni_account_id = format!("{}.{}", uni_name.to_lowercase(), env::current_account_id());
        println!("{}", uni_account_id);

        let uni_id = AccountId::try_from(uni_account_id).expect("Uni ID is invalid");
        assert_eq!(self.uni.contains(&uni_id), false);

        let ext_self = Self::ext(env::current_account_id());

        self.deploy_uni_account(&uni_id, deposit, &uni_owner_id)
            .then(ext_self.register_token(&uni_id))
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

    #[private]
    pub fn register_token(&mut self, uni_id: &AccountId) {
        self.uni.insert(uni_id);
    }

    /// This is NOOP implementation. KEEP IT if you haven't changed contract state.
    /// Should only be called by this contract on migration.
    /// This method is called from `upgrade()` method.
    /// For next version upgrades, change this function.
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let contract: Contract = env::state_read().expect("Contract is not initialized");
        contract
    }

    pub fn upgrade(&self) -> Promise {
        self.assert_owner();

        const UPDATE_GAS_LEFTOVER: Gas = Gas(10_000_000_000_000);
        const NO_ARGS: Vec<u8> = vec![];

        // Receive the code directly from the input to avoid the
        // GAS overhead of deserializing parameters
        let code = env::input().expect("Error: No input").to_vec();

        // Deploy the contract on self
        Promise::new(env::current_account_id())
            .deploy_contract(code)
            .function_call(
                "migrate".to_string(),
                NO_ARGS,
                0,
                env::prepaid_gas() - env::used_gas() - UPDATE_GAS_LEFTOVER,
            )
            .as_return()
    }

    fn assert_owner(&self) {
        if !self.is_owner(&env::predecessor_account_id()) {
            panic!("This method can be called only by owner")
        }
    }

    fn deploy_uni_account(
        &mut self,
        uni_id: &AccountId,
        deposit: Balance,
        uni_owner_id: &AccountId,
    ) -> Promise {
        let args = json!({ "owner_id": uni_owner_id }).to_string().into_bytes();

        Promise::new(uni_id.clone())
            .create_account()
            .transfer(deposit)
            .deploy_contract(wasm::wasm_code().to_vec())
            .function_call("new".to_string(), args, 0, GAS_FOR_CREATE_UNI)
    }

    pub fn get_unis(&self) -> Vec<AccountId> {
        self.uni.to_vec()
    }
}

#[cfg(test)]
mod test {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn change_ownership() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = Contract::new(accounts(1));
        contract.propose_new_owner(accounts(2));
        assert_eq!(contract.owner(), accounts(1));
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.accept_ownership();
        assert_eq!(contract.owner(), accounts(2));
    }

    #[test]
    #[should_panic(expected = "This method can be called only by owner")]
    fn change_owner() {
        let context = get_context(accounts(2));
        testing_env!(context.build());

        let mut contract = Contract::new(accounts(1));
        contract.propose_new_owner(accounts(2));
    }
}
