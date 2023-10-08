use std::fmt;
use std::io;
use std::io::Read;
use std::{collections::HashSet, fmt::Formatter};
use mess::{self, mess::{ProceduralMess, ProceduralRound}};

fn twgintw_o(c: &char) -> HashSet<char> {
    let s = c.clone();

    match s {
        '0' => vec!['0', '1'].into_iter().collect(),
        _ => HashSet::<char>::new()
    }
}

fn twgintw_iodecider(states:&HashSet<char>) -> &char {
    let stdin = io::stdin();
    let mut string = String::new();
    stdin.read_line(&mut string).expect("Input error");

    let char = string.chars().next().unwrap();

    if char == '0' {
        return &'0';
    }
    
    &'1'
}

fn main() {

   let game = ProceduralMess::new(
        '0', 
        twgintw_o,
        |n| {
            n.clone()
        }
    );
    
    let mut round = ProceduralRound::new(game, twgintw_iodecider);

    while round.next() {
        println!("{:?}",round.sequence);
    }

    println!("{:?}\n Game over!", round.sequence);
   

}
