use std::collections::{HashSet};
use std::string::String;

use string;

#[derive(Debug)]
pub struct KeyStore {
    // Handy wrapper so I can impl to_bytes and other such things
    set : HashSet<String>,
}

#[derive(Debug)]
pub enum KeyStoreError {
    KeyNotPresent,
    KeyNotValid,
    KeyAlreadyPresent,
}

pub type KeyStoreResult = Result<bool,KeyStoreError>;

impl KeyStore {
    
    pub fn new() -> KeyStore {
        KeyStore {
            set : HashSet::new(),
        }
    }
    
    pub fn new_from_vec(input: Vec<String>) -> KeyStore {
        let mut return_set = KeyStore::new();
        
        for key in input {
            return_set.set.insert(key);
        }

        return return_set;
    }

    pub fn contains(&self, value: &String) -> KeyStoreResult {
        Ok(self.set.contains(value)) 
    }
    
    pub fn insert(&mut self, value: String) -> KeyStoreResult {
        // Returns false if value was already present
        let result = self.set.insert(value);
        if result {
            Ok(result)
        } else {
            Err(KeyStoreError::KeyAlreadyPresent)
        }
    }
    
    pub fn remove(&mut self, value: &String) -> KeyStoreResult {
        // Returns true if value was removed
        let result = self.set.remove(value);
        if result {
            Ok(result)
        } else {
            Err(KeyStoreError::KeyNotPresent)
        }
    }
    
    pub fn update(&mut self, value: String) -> KeyStoreResult {
        // Returns true if value was added, false if removed
        match self.contains(&value) {
            Err(why) => panic!(match why {
                    KeyStoreError::KeyNotValid
                            => "key for f(contain) is invalid!",
                    KeyStoreError::KeyNotPresent
                            => "Key is not inside!",
                    KeyStoreError::KeyAlreadyPresent
                            => "Key is already inside!"
                    }),
            
            Ok(val)  =>  match val {
                    true    => self.remove(&value),
                    false   => self.insert(value)
                }
        }
    }   
    
    pub fn to_bytes(self) -> Vec<u8> {
        let mut string_vec = Vec::new();
        
        for key in self.set {
            string_vec.push(key);
        }
        
        //let mut str_vec: Vec<&str> = Vec::new();

        return string::prepare_string_vec(&string_vec);
    }

    pub fn from_bytes(input :Vec<u8>) -> KeyStore {
        KeyStore::new_from_vec(string::read_output_blob(&input))
    }
}


pub fn main() {
    let bs: Vec<String> = vec!["i".to_string(),"c".to_string(),"u".to_string()];
    let list: KeyStore = KeyStore::new_from_vec(bs);
    println!("{:?}",&list);
    let byte_array: Vec<u8> = list.to_bytes();
    let list2: KeyStore = KeyStore::from_bytes(byte_array);
    println!("{:?}",&list2);
}
