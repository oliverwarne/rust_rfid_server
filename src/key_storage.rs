use std::collections::HashSet;
use rustc_serialize::json;

use string;

struct key_store {
    // Handy wrapper so I can impl to_bytes and other such things
    set : HashSet<String>,
}

impl key_store {
    pub fn new() -> key_store {
        key_store {
            set : HashSet::new(),
        }
    }
    
    pub fn new_from_vec(input: Vec<String>) -> key_store {
        let mut return_set = key_store::new();
        
        for key in input {
            return_set.set.insert(key);
        }

        return return_set;
    }

    pub fn contains(&self, value: String) -> bool {
        self.set.contains(&value)
    }

    pub fn remove(&mut self, value: String) -> bool {
        self.set.remove(&value)
    }

    
    pub fn to_bytes(self) -> Vec<u8> {
        let mut string_vec = Vec::new();
        
        for key in self.set {
            let k = *key;

            string_vec.push(&k);
        }
        
        //let mut str_vec: Vec<&str> = Vec::new();

        return string::prepare_string_vec(&str_vec);
    }

    pub fn from_bytes(input :Vec<u8>) -> key_store {
        key_store::new_from_vec(string::read_output_blob(&input))
    }
}
