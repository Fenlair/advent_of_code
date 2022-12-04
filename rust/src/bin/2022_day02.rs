fn main() {
    let input = include_str!("../../../inputs/2022_day02.txt");

    fn foo(round: &str) -> u32 {
        match round {
            "A X" => 1 + 3,  // rock, rock
            "A Y" => 2 + 6,  // rock, paper
            "A Z" => 3 + 0,  // rock, scissors
            "B X" => 1 + 0,  // paper, rock
            "B Y" => 2 + 3,  // paper, paper
            "B Z" => 3 + 6,  // paper, scissors
            "C X" => 1 + 6,  // scissors, rock
            "C Y" => 2 + 0,  // scissors, paper
            "C Z" => 3 + 3,  // scissors, scissors
            _ => panic!(),
        }
    }
    fn bar(round: &str) -> &str {
        match round {
            "A X" => "A Z",  // rock, lose
            "A Y" => "A X",  // rock, draw
            "A Z" => "A Y",  // rock, win
            "B X" => "B X",  // paper, lose
            "B Y" => "B Y",  // paper, draw
            "B Z" => "B Z",  // paper, win
            "C X" => "C Y",  // scissors, lose
            "C Y" => "C Z",  // scissors, draw
            "C Z" => "C X",  // scissors, win
            _ => panic!(),
        }
    }
    let strategy1: u32 =
        input.lines()
        .map(|round| foo(round))
        .sum();
    println!("{:?}", strategy1);

    let strategy2: u32 =
        input.lines()
        .map(|round| bar(round))
        .map(|round| foo(round))
        .sum();
    println!("{:?}", strategy2);

}
