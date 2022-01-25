use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};

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
  adopter: Option<AccountId>,
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
    assert!(name.len() > 0, "Name is reqired.");
    assert!(picture.len() > 0, "Image link is required.");
    assert!(age > 0, "Age is reqired");
    assert!(breed.len() > 0, "Breed is reqired");
    assert!(location.len() > 0, "Location is reqired");
    let new_pet = Pet {
      name: name,
      picture: picture,
      age: age,
      breed: breed,
      location: location,
      adopter: None,
    };
    let id = self.pets.len();
    self.pets.insert(&id, &new_pet);
    true
  }

  pub fn adopt(&mut self, pet_id: u64) {
    assert!(self.pets.get(&pet_id).is_some(), "Pet with such id not found");
    let adopter_id = env::predecessor_account_id();
    let mut pet = self.pets.get(&pet_id).unwrap();
    assert!(&pet.adopter.is_none(), "The pet is aleray adopted");
    self.adopters.insert(&adopter_id, &pet_id);
    pet.adopter.get_or_insert(adopter_id);
  }

  //Getters

  pub fn get_pets(&self) -> Vec<(u64, Pet)> {
    self.pets.iter().collect()
  }

  pub fn get_adopters(&self) -> Vec<(String, u64)> {
    self.adopters.iter().collect()
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
  #[should_panic]
  fn name_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic]
  fn picture_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "name".to_string(),
      "".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic]
  fn age_is_0() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      0,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic]
  fn breed_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic]
  fn location_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShop::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "".to_string(),
    );
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

  #[test]
  #[should_panic]
  fn cannot_adopt_nonexisting_pet() {
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
    contract.adopt(10);
  }

  #[test]
  #[should_panic]
  fn cannot_adopt_if_already_adopted() {
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
    contract.adopt(0);
  }
}
