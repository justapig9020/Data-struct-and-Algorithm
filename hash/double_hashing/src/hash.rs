use std::fmt::Debug;
type Hash<T> = fn(T) -> usize;

#[derive(Debug)]
pub struct HashTable <T> {
    h1: Hash<T>,
    h2: Hash<T>,
    table: Vec<Option<T>>,
}

impl<T: Debug + Copy> HashTable<T> {
    pub fn new (h1:Hash<T>, h2: Hash<T>, size: usize) -> Self {
        Self {
            h1,
            h2,
            table: (0..size).map(|_| None).collect(),
        }
    }
    fn get_index(&self, key: T, i: usize) -> usize {
        let m = self.table.len();
        ((self.h1)(key) + (self.h2)(key) * i) % m
    }
    pub fn hash(&mut self, key: T) {
        let mut turn = 0;
        loop {
            let idx = self.get_index(key, turn);
            if let None = self.table[idx] {
                self.table[idx] = Some(key);
                return;
            }
            turn += 1;
        }
    }
    pub fn print(&self) {
        println!("/---T---\\");
        for (i, k) in self.table.iter().enumerate() {
            let k = k
                .map(|key| format!("{:?}", key))
                .unwrap_or("".to_string());
            println!("|{:3}|{:3}|", i, k);
            println!("|---+---|");
        }
    }
}
