use near_sdk::AccountId;

use crate::Contract;

impl Contract {
    pub fn inner_propose_new_owner(&mut self, proposed_owner_id: AccountId) {
        assert_ne!(self.owner_id, proposed_owner_id);
        self.proposed_owner_id = proposed_owner_id;
    }

    pub fn inner_accept_ownership(&mut self) {
        assert_ne!(self.owner_id, self.proposed_owner_id);
        self.owner_id = self.proposed_owner_id.clone();
    }

    pub fn owner_id(&self) -> &AccountId {
        &self.owner_id
    }

    pub fn is_owner(&self, account_id: &AccountId) -> bool {
        account_id == &self.owner_id
    }

    pub fn proposed_owner_id(&self) -> &AccountId {
        &self.proposed_owner_id
    }
}

