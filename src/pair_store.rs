use std::collections::HashMap;

pub enum PairStore {
    Vec(Vec<(String, String)>),
    HashMap(HashMap<String, String>),
}

impl PairStore {
    pub fn add_pair(&mut self, k: String, v: String) {
        match *self {
            PairStore::Vec(ref mut vec) => vec.push((k, v)),
            PairStore::HashMap(ref mut map) => {map.insert(k, v);},
        }
    }

    pub fn sorted_pairs(self) -> Vec<(String, String)> {
        match self {
            PairStore::Vec(vec) => {
                let mut output = vec;
                output.sort();
                output
            },
            PairStore::HashMap(map) => {
                let mut output: Vec<(String, String)> = map.into_iter().collect();
                output.sort();
                output
            },
        }
    }
}
