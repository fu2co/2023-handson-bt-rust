use std::fmt::{Debug, Display};

#[allow(dead_code)]
struct Process {
    active: bool,
    id: isize
}

impl Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").field("active", &self.active).field("id", &self.id).finish()
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.active {
            false => write!(f, "Process not active"),
            true => write!(f, "id: {}", self.id),
        }
    }
}

fn main() {
    let ap = Process {active: false, id: 100};
    println!("My process: {:?}", ap);
    println!("My process: {}", ap);
} 

// demo topics
// =============================
// [x] println macro
// [x] trait Debug / Display
// [ ] move variable -> shallow copy
// [ ] trait Clone -> deep copy
// [ ] trait Copy
// [ ] implement lifetimes
