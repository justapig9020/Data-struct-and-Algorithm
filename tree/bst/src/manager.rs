use interactive::Management;
use bst_lib::Bst;

pub struct Manager {
    bst: Bst,
}

impl Manager {
    pub fn new() -> Result<Self, ()> {
        Ok(Self {
            bst: Bst::new(),
        })
    }
}

fn do_insert_val(bst: &mut Bst, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    match args[0].parse::<i32>() {
        Ok(val) => {
            bst.insert(val);
            true
        },
        _ => false,
    }
}

fn do_remove_val(bst: &mut Bst, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    match args[0].parse::<i32>() {
        Ok(val) =>
            bst.remove_val(val).is_some(),
        _ => false,
    }
}

impl Management for Manager {
    fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool {
        let bst = &mut self.bst;
        match cmd {
            "iv" => do_insert_val(bst, args),
            "rv" => do_remove_val(bst, args),
            _ => false,
        }
    }
    fn gen_graph(&self) -> String {
        match self.bst.graph() {
            Ok(g) => g,
            Err(_) => String::from(""),
        }
    }
}
