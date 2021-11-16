use bst_lib::Bst;

#[derive(Debug)]
struct Inode {
    key: i32,
    weight: usize,
}

struct Enode {
    weight: usize,
}

pub struct Obst {
    inode: Vec<Inode>,
    enode: Vec<Enode>,
}

impl Obst {
    pub fn new() -> Self {
        Self {
            inode: Vec::new(),
            enode: Vec::new(),
        }
    }
    pub fn insert_node(&mut self, key: i32, weight: usize) {
        self.inode.push(Inode {
            key,
            weight,
        });
    }
    pub fn insert_fail(&mut self, weight: usize) {
        self.enode.push(Enode {
            weight,
        })
    }
    fn _obst(&self) -> BstConstructor {
        let len = self.enode.len();
        let mut otree = vec![vec![SubTree::default() ;len]; len];
        let last = len - 1;

        // Generate first two level
        for i in 0..last {
            let leaf = &mut otree[i][i];
            leaf.weight = self.enode[i].weight;
            leaf.cost = 0;
            leaf.root = 0;
            let button = &mut otree[i][i+1];
            button.cost = self.inode[i].weight +
                                self.enode[i].weight +
                                self.enode[i+1].weight;
            button.weight = button.cost;
            button.root = i;
        }
        otree[last][last].weight = self.enode[last].weight;
        otree[last][last].cost = 0;
        otree[last][last].root = 0;

        for range in 2..len {
            for i in 0..len-range {
                let j = i + range;
                let weight = otree[i][j-1].weight +
                                self.inode[j-1].weight +
                                self.enode[j].weight;
                let root = optimal_root(&otree, i, j);
                otree[i][j].cost = otree[i][root].cost +
                                    otree[root+1][j].cost +
                                    weight;
                otree[i][j].weight = weight;
                otree[i][j].root = root;
            }
        }
        let key = self.inode
                        .iter()
                        .map(|n| n.key)
                        .collect();

        let mut cost_matrix = String::new();
        let mut root_matrix = String::new();
        let mut weight_matrix = String::new();
        for row in otree.iter() {
            for entry in row.iter() {
                cost_matrix.push_str(&format!("{}\t", entry.cost));
                root_matrix.push_str(&format!("{}\t", entry.root));
                weight_matrix.push_str(&format!("{}\t", entry.weight));
            }
            cost_matrix.push('\n');
            root_matrix.push('\n');
            weight_matrix.push('\n');
        }

        println!("Cost:");
        println!("{}", cost_matrix);
        println!("");

        println!("Root:");
        println!("{}", root_matrix);
        println!("");

        println!("Weight:");
        println!("{}", weight_matrix);
        println!("");

        BstConstructor {
            map: otree,
            key,
        }
    }
    pub fn obst(mut self) -> Result<BstConstructor, ()> {
        if self.enode.len() == self.inode.len() + 1 {
            self.inode
                .sort_by(|a, b|
                    a.key.cmp(&b.key));
            Ok(self._obst())
        } else {
            Err(())
        }
    }
}

fn optimal_root(tree: &Vec<Vec<SubTree>>, i: usize, j: usize) -> usize {
    let mut m = tree[i][i].cost + tree[i+1][j].cost;
    let mut r = i;
    for k in i+1..j {
        let curr = tree[i][k].cost + tree[k+1][j].cost;
        if m > curr {
            r = k;
            m = curr;
        }
    }
    r
}

#[derive(Default, Copy, Clone, Debug)]
struct SubTree {
    cost: usize,
    weight: usize,
    root: usize,
}

#[derive(Debug)]
pub struct BstConstructor {
    map: Vec<Vec<SubTree>>,
    key: Vec<i32>,
}

impl BstConstructor {
    pub fn construct(self) -> Bst {
        let mut bst = Bst::new();
        self._construct(0, self.map.len() - 1, &mut bst);
        bst
    }
    fn _construct(&self, i: usize, j: usize, tree: &mut Bst) {
        if i == j {
            return;
        }
        let root = self.map[i][j].root;
        tree.insert(self.key[root]);
        self._construct(i, root, tree);
        self._construct(root + 1, j, tree);
    }
}
