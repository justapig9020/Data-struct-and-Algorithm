pub struct Hash {
    table: HashTable,
}

pub enum InsertErr {
    Exist,
}

pub type InsertResult = Result<(), InsertErr>;

/// (key, level)
pub type SearchResult = Result<(u32, usize), ()>;

enum Slot {
    Item(u32),
    Table(HashTable),
    Empty,
}

struct HashTable {
    level: usize,
    slots: Vec<Slot>,
}

impl Hash {
    pub fn new(size: usize) -> Self {
        Self {
            table: HashTable::new(1, size), // Level start from 1
        }
    }
    pub fn insert(&mut self, key: u32) -> InsertResult {
        self.table.insert(key)
    }
    pub fn search(&self, key: u32) -> SearchResult {
        self.table.search(key)
    }
}

impl HashTable {
    fn new(level: usize, size: usize) -> Self {
        Self {
            level,
            slots: (0..size).map(|_| Slot::Empty).collect(),
        }
    }
    fn insert(&mut self, key: u32) -> InsertResult {
        let size = self.slots.len();
        let idx = hash(key, self.level, size);
        let slot_mut_ref = self.slots.iter_mut().nth(idx).unwrap();
        match slot_mut_ref {
            Slot::Empty => {
                self.slots[idx] = Slot::Item(key);
                Ok(())
            },
            Slot::Item(old) if *old == key=> { // Already exist
                Err(InsertErr::Exist)
            },
            Slot::Item(old) => { // Collision
                let old = *old;
                let mut new_table = HashTable::new(self.level + 1, size);
                new_table.insert(old)?;
                new_table.insert(key)?;
                *slot_mut_ref = Slot::Table(new_table);
                Ok(())
            },
            Slot::Table(next) => {
                next.insert(key)
            }
        }
    }
    fn search(&self, key: u32) -> SearchResult {
        let size = self.slots.len();
        let idx = hash(key, self.level, size);
        let slot_ref = self.slots.iter().nth(idx).unwrap();
        match slot_ref {
            Slot::Empty => {
                Err(())
            },
            Slot::Item(found) => {
                if *found == key {
                    Ok((key, self.level))
                } else {
                    Err(())
                }
            },
            Slot::Table(next) => {
                next.search(key)
            }
        }
    }
}

fn hash(key: u32, level: usize, size: usize) -> usize {
    let key = key as usize;
    let h = key % level;
    (key + h) % size
}
