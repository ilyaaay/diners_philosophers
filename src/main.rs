use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Self {
        Self {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));

        let _right = table.forks[self.right].lock().unwrap();

        println!("{} start eat", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} finish eat", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosphers = vec![
        Philosopher::new("p1", 0, 1),
        Philosopher::new("p2", 1, 2),
        Philosopher::new("p3", 2, 3),
        Philosopher::new("p4", 3, 4),
        Philosopher::new("p5", 0, 4),
    ];

    let handles = philosphers
        .into_iter()
        .map(|x| {
            let table = table.clone();
            thread::spawn(move || {
                x.eat(&table);
            })
        })
        .collect::<Vec<_>>();

    for i in handles {
        i.join().unwrap();
    }
}
