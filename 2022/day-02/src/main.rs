use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

enum OurResult {
    Win,
    Draw,
    Lose
}

#[derive(Debug, PartialEq)]
struct Game {
    opponent: Shape,
    our: Shape
}

fn match_shape(value: &str) -> Shape {
    match value {
        "A"|"X" => Shape::Rock,
        "B"|"Y" => Shape::Paper,
        "C"|"Z" => Shape::Scissors,
        _ => panic!("Unsupported value")
    }
}

fn map_to_game(opponent: &str, us: &str) -> Game
{
    let opponent_shape = match_shape(opponent);
    let our_shape = match_shape(us);

    Game { opponent: opponent_shape, our: our_shape}
}

fn match_result(value: &str) -> OurResult {
    match value {
        "X" => OurResult::Lose,
        "Y" => OurResult::Draw,
        "Z" => OurResult::Win,
        _ => panic!("Unsupported value")
    }
}

fn deduce_shape_based_on_result_and_opponent(result: OurResult, opponent: Shape) -> Shape {
    match (result, opponent) {
        (OurResult::Win, Shape::Rock) => Shape::Paper,
        (OurResult::Win, Shape::Paper) => Shape::Scissors,
        (OurResult::Win, Shape::Scissors) => Shape::Rock,

        (OurResult::Draw, Shape::Rock) => Shape::Rock,
        (OurResult::Draw, Shape::Paper) => Shape::Paper,
        (OurResult::Draw, Shape::Scissors) => Shape::Scissors,

        (OurResult::Lose, Shape::Rock) => Shape::Scissors,
        (OurResult::Lose, Shape::Paper) => Shape::Rock,
        (OurResult::Lose, Shape::Scissors) => Shape::Paper,
    }
}

fn map_to_game_part2(opponent: &str, result: &str) -> Game
{
    let opponent_shape = match_shape(opponent);
    let out_result = match_result(result);
    let our_shape = deduce_shape_based_on_result_and_opponent(out_result, opponent_shape);

    Game { opponent: opponent_shape, our: our_shape}
}

fn full_score(games: Vec<Game>) -> u32 {
   games.iter().map(|g| score_game(g)).sum()
}

fn score_game(game: &Game) -> u32 {
    score_shape(&game) + score_result(&game)
}

fn score_shape(game: &Game) -> u32 {
    match game.our {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

fn score_result(game: &Game) -> u32 {
    match (game.opponent, game.our) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Scissors) => 0,
        (Shape::Rock, Shape::Paper) => 6,

        (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Scissors, Shape::Rock) => 6,

        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Paper, Shape::Scissors) => 6,
    }
}

fn main() {
    let content = fs::read_to_string("input-02.txt").unwrap();
    let lines : Vec<Vec<char>> = content.split("\n").map(|l|
        vec![l.chars().nth(0).unwrap(),
             l.chars().nth(1).unwrap(),
             l.chars().nth(2).unwrap()]).collect();

    let games : Vec<Game> = lines.iter().map(|values|
            map_to_game(&values[0].to_string(), &values[2].to_string())
    ).collect();
    println!("Part1 - {:?}", full_score(games));

    let games2 : Vec<Game> = lines.iter().map(|values| {
        map_to_game_part2(&values[0].to_string(), &values[2].to_string())
    }).collect();
    println!("Part2 - {:?}", full_score(games2));
}
