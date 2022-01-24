use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PetShop {
  pub pets: UnorderedMap<u64, Pet>,
  pub adopters: UnorderedMap<String, u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pet {
  name: String,
  picture: String,
  age: u64,
  breed: String,
  location: String,
}

impl Default for PetShop {
  fn default() -> Self {
    Self {
      pets: UnorderedMap::new(b"a".to_vec()),
      adopters: UnorderedMap::new(b"a".to_vec()),
    }
  }
}

#[near_bindgen]
impl PetShop {
  pub fn add_pet(
    &mut self,
    name: String,
    picture: String,
    age: u64,
    breed: String,
    location: String,
  ) -> bool {
    let new_pet = Pet {
      name: name,
      picture: picture,
      age: age,
      breed: breed,
      location: location,
    };
    let id = self.pets.len();
    self.pets.insert(&id, &new_pet);
    true
  }

  pub fn adopt(&mut self, pet_id: u64) {
    let adopter_id = env::predecessor_account_id();
    self.adopters.insert(&adopter_id, &pet_id);
    env::log(format!("{} adopted a pet (id: {})", adopter_id, pet_id).as_bytes());
  }

  pub fn get_adopters(&self) -> Vec<(String, u64)> {
    self.adopters.iter().collect()
  }

  pub fn get_pets(&self) -> Vec<(u64, Pet)> {
    self.pets.iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  // mock the context for testing, notice "signer_account_id" that was accessed above from env::
  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
      current_account_id: "alice_near".to_string(),
      signer_account_id: "bob_near".to_string(),
      signer_account_pk: vec![0, 1, 2],
      predecessor_account_id: "carol_near".to_string(),
      input,
      block_index: 0,
      block_timestamp: 0,
      account_balance: 0,
      account_locked_balance: 0,
      storage_usage: 0,
      attached_deposit: 0,
      prepaid_gas: 10u64.pow(18),
      random_seed: vec![0, 1, 2],
      is_view,
      output_data_receivers: vec![],
      epoch_height: 19,
    }
  }

  #[test]
  fn add_then_get_pets() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    let add = contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
    let pets = contract.get_pets();
    assert!(add);
    assert_eq!(1, pets.len());
  }

  #[test]
  fn adopt_and_get_adopters() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
    contract.adopt(0);
    let adopters = contract.get_adopters();
    assert_eq!(1, adopters.len());
    assert_eq!("carol_near".to_string(), adopters[0].0);
    assert_eq!(0, adopters[0].1);
  }
}
