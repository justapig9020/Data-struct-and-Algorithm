mod hash;

use hash::HashTable;

fn hash1(key: i32) -> usize {
    (key % 16) as usize
}

fn hash2(key: i32) -> usize {
    (key % 15) as usize + 1
}

fn main() {
    let mut h = HashTable::new(hash1, hash2, 16);
    let vals = [16, 3, 35, 67, 51, 1, 15, 31, 19, 17];
    for v in vals.iter() {
        h.hash(*v);
    }
    h.print();
}
