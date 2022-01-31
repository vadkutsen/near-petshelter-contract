use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
/// Contract structure is represented here
pub struct PetShelter {
  /// The contact must have pets collection
  pub pets: UnorderedMap<u64, Pet>,
  /// and donations amount
  pub donations: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
/// Pet structure is defined here
pub struct Pet {
  name: String,
  picture: String,
  age: u64,
  breed: String,
  location: String,
  adopter: Option<AccountId>,
}
/// Default implementation of the contract
impl Default for PetShelter {
  fn default() -> Self {
    Self {
      pets: UnorderedMap::new(b"a".to_vec()),
      donations: 0,
    }
  }
}

/// Implementation of the Pet structure
impl Pet {
  /// The function used for updating adopter on a Pet structure
  fn update_adopter(&mut self, adopter_id: AccountId) {
      self.adopter = Some(adopter_id);
  }
}

#[near_bindgen]
/// The contract implementation
impl PetShelter {
  /// Function for adding pets
  pub fn add_pet(&mut self, name: String, picture: String, age: u64, breed: String, location: String) -> bool {
    let signer_account_id = env::signer_account_id();
    let current_account_id = env::current_account_id();
    assert_eq!(signer_account_id, current_account_id, "Only owner can add pets");
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
    env::log(format!("Adding pet with id {} ", id).as_bytes());
    self.pets.insert(&id, &new_pet);
    true
  }
  /// Function for adopting a pet
  pub fn adopt(&mut self, pet_id: u64) -> bool {
    let adopter_id = env::predecessor_account_id();
    let mut pet = self.get_pet(pet_id);
    assert!(&pet.adopter.is_none(), "The pet is aleray adopted");
    env::log(format!("Adding @{} as the pet {} adopter", &adopter_id, &pet_id).as_bytes());
    pet.update_adopter(adopter_id);
    self.pets.insert(&pet_id, &pet);
    true
  }

  #[payable]
  /// Function for donation
  pub fn donate(&mut self) {
      let deposit = env::attached_deposit();
      let donator_account_id: String = env::predecessor_account_id();
      let current_account_id: String = env::current_account_id();
      assert!(deposit > 0, "The amount of donation should be greater than 0");
      assert_ne!(current_account_id, donator_account_id, "You cannot donate to yourself");
      self.donations += deposit;
      env::log(format!("@{} donated {} yNEAR", donator_account_id, deposit).as_bytes());
  }

  /// Getters
  ///
  /// Returns all pets
  pub fn get_pets(&self) -> Vec<(u64, Pet)> {
    self.pets.iter().collect()
  }

  /// Returns a pet by id
  pub fn get_pet(&self, pet_id: u64) -> Pet {
    self.pets.get(&pet_id).unwrap()
  }

  /// Returns donations amount
  pub fn get_donations(&self) -> u128 {
    self.donations
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
      current_account_id: "alice_near".to_string(),
      signer_account_id: "alice_near".to_string(),
      signer_account_pk: vec![0, 1, 2],
      predecessor_account_id: "alice_near".to_string(),
      input,
      block_index: 0,
      block_timestamp: 0,
      account_balance: 0,
      account_locked_balance: 0,
      storage_usage: 0,
      attached_deposit: 1_000_000_000_000_000_000_000,
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
    let mut contract = PetShelter::default();
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
  #[should_panic(expected="Only owner can add pets")]
  fn cannot_add_pet_if_not_contract_owner() {
    let mut contract = PetShelter::default();
    let mut context = get_context(vec![], false);
    context.signer_account_id = "bob_near".to_string();
    testing_env!(context);
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
  #[should_panic(expected="Name is reqired.")]
  fn name_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic(expected="Image link is required.")]
  fn picture_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "name".to_string(),
      "".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic(expected="Age is reqired")]
  fn age_is_0() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      0,
      "breed".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic(expected="Breed is reqired")]
  fn breed_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "".to_string(),
      "location".to_string(),
    );
  }

  #[test]
  #[should_panic(expected="Location is reqired")]
  fn location_is_empty() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "".to_string(),
    );
  }

  #[test]
  fn adopt_and_get_pet() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.add_pet(
      "name".to_string(),
      "picture".to_string(),
      3,
      "breed".to_string(),
      "location".to_string(),
    );
    let adopt = contract.adopt(0);
    assert!(adopt);
    let pet = contract.get_pet(0);
    assert_eq!("alice_near".to_string(), pet.adopter.unwrap());
  }

  #[test]
  #[should_panic(expected="The pet is aleray adopted")]
  fn cannot_adopt_if_already_adopted() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
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

  #[test]
  fn donate() {
    let mut context = get_context(vec![], false);
    context.predecessor_account_id = "bob_near".to_string();
    testing_env!(context);
    let mut contract = PetShelter::default();
    assert_eq!(contract.get_donations(), 0);
    contract.donate();
    assert_eq!(contract.get_donations(), 1_000_000_000_000_000_000_000);
  }

  #[test]
  #[should_panic(expected="You cannot donate to yourself")]
  fn cannot_donate_as_the_contract_owner() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = PetShelter::default();
    contract.donate();
  }
}
