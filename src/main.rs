use core::fmt;
use rand::Rng;
use std::{collections::HashMap, env::args, fmt::Formatter, vec};

fn main() {
    let arg: Vec<String> = args().collect();
    let (arg_rule, arg_n) = (&arg[1], &arg[2]);

    let rule = arg_rule.parse::<u8>().unwrap();
    let n = arg_n.parse::<u8>().unwrap();

    let mut minicat = Minicat::new(n, rule);

    println!(
        "\nStarting new rule-{} simulation with n = {}.\n",
        arg_rule, arg_n
    );

    while minicat.world.len() > 3 {
        minicat.next();
        println!("{minicat}");
    }

    println!(
        "\nSimulation stopped after {} generations\n",
        minicat.generation
    );
}

#[derive(Debug)]
struct Minicat {
    world: String,
    generation: u8,
    rule_converter: HashMap<u8, char>,
}

impl fmt::Display for Minicat {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = str::replace(&str::replace(&self.world, "1", "■"), "0", "□");

        println!("{}", text);
        Ok(())
    }
}

impl Minicat {
    #[allow(dead_code)]
    fn new_with_init_world(init_world: String, rule: u8) -> Self {
        Minicat {
            world: init_world,
            generation: 0,
            rule_converter: Minicat::generate_rule_hashmap(rule),
        }
    }

    fn new(size: u8, rule: u8) -> Self {
        Minicat {
            world: Minicat::random_init_world(size),
            generation: 0,
            rule_converter: Minicat::generate_rule_hashmap(rule),
        }
    }

    fn random_init_world(n: u8) -> String {
        let mut rng = rand::thread_rng();
        let world = (0..n)
            .map(|_| match rng.gen_range(0..2) {
                0 => "0".to_string(),
                1 => "1".to_string(),
                _ => panic!("Wat"),
            })
            .collect::<String>();
        println!("Init world: {}", world);
        world
    }

    fn next(&mut self) {
        let mut next_world: Vec<char> = vec![];
        for i in 0..(self.world.len() - 2) {
            let str_slice: &str = &self.world[i..i + 3];
            next_world.push(Minicat::to_char(self, str_slice));
        }
        self.world = next_world.iter().collect();
        self.generation += 1;
    }

    fn to_char(&self, str_slice: &str) -> char {
        let val = u8::from_str_radix(str_slice, 2).unwrap();
        match self.rule_converter.get(&val) {
            Some(&v) => return v,
            None => panic!("Could not find rule"),
        };
    }

    fn generate_rule_hashmap(rule: u8) -> HashMap<u8, char> {
        let mut map = HashMap::new();

        let rule_binary = format!("{rule:00000008b}");
        for (i, j) in (0..8).rev().enumerate() {
            map.insert(j as u8, rule_binary.as_bytes()[i] as char);
        }

        println!("Hashmap created: {:?}", map);

        map
    }
}
