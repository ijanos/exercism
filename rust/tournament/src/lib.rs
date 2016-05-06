use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

enum GameResult {
    Win,
    Loss,
    Draw,
}

struct Game {
    team1: String,
    team2: String,
    result: GameResult,
}

struct TeamData {
    played: usize,
    won: usize,
    draw: usize,
    lost: usize,
    points: usize,
}

impl TeamData {
    fn new() -> TeamData {
        TeamData {
            played: 0,
            won: 0,
            draw: 0,
            lost: 0,
            points: 0,
        }
    }
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let input = input.trim(); // remove trailing & leading whitespace
        let input: Vec<&str> = input.split(";").collect();
        if input.len() != 3 {
            return Err("Invalid line");
        }
        let outcame = match input[2] {
            "win" => GameResult::Win,
            "loss" => GameResult::Loss,
            "draw" => GameResult::Draw,
            _ => return Err("invalid outcame"),
        };
        Ok(Game {
            team1: input[0].into(),
            team2: input[1].into(),
            result: outcame,
        })
    }
}

fn read_input(input: &Path) -> Vec<Game> {
    let f = File::open(input).unwrap();
    let file = BufReader::new(f);
    let mut games = Vec::new();
    for line in file.lines() {
        if let Ok(game) = line.unwrap().parse::<Game>() {
            games.push(game)
        }
    }
    games
}

fn calc_data(games: &Vec<Game>) -> HashMap<String, TeamData> {
    let mut teams: HashMap<String, TeamData> = HashMap::new();
    for game in games {
        let ((w1, l1, d1, p1), (w2, l2, d2, p2)) = match game.result {
            GameResult::Win => ((1, 0, 0, 3), (0, 1, 0, 0)),
            GameResult::Loss => ((0, 1, 0, 0), (1, 0, 0, 3)),
            GameResult::Draw => ((0, 0, 1, 1), (0, 0, 1, 1)),
        };
        {
            // new scope to satisfy the borrow checker
            let team1 = teams.entry(game.team1.clone()).or_insert(TeamData::new());
            team1.played += 1;
            team1.won += w1;
            team1.lost += l1;
            team1.draw += d1;
            team1.points += p1;
        }
        let team2 = teams.entry(game.team2.clone()).or_insert(TeamData::new());
        team2.played += 1;
        team2.won += w2;
        team2.lost += l2;
        team2.draw += d2;
        team2.points += p2;
    }
    teams
}

fn write_output(content: &str, output: &Path) {
    let mut buffer = File::create(output).unwrap();
    buffer.write_all(content.as_bytes()).ok();
}

fn custom_sort(team1: &(&String, &TeamData), team2: &(&String, &TeamData)) -> Ordering {
    match team1.1.points.cmp(&team2.1.points) {
        Ordering::Equal => {
            match team1.1.won.cmp(&team2.1.won) {
                Ordering::Equal => team2.0.cmp(&team1.0),
                ans @ _ => ans,
            }
        }
        ans @ _ => ans,
    }
}

fn pretty_print_results(results: &HashMap<String, TeamData>) -> String {
    let mut sorted_list: Vec<_> = results.iter().collect();
    sorted_list.sort_by(custom_sort);
    sorted_list.reverse();
    let mut results = String::new();
    results.push_str("Team                           | MP |  W |  D |  L |  P\n");
    for (ref team, ref data) in sorted_list {
        let line = format!("{:<30} |  {} |  {} |  {} |  {} |  {}\n",
                           team,
                           data.played,
                           data.won,
                           data.draw,
                           data.lost,
                           data.points);
        results.push_str(&line);
    }
    results
}

pub fn tally(input: &Path, output: &Path) -> Option<usize> {
    let games: Vec<Game> = read_input(input);
    let results = calc_data(&games);
    let content = pretty_print_results(&results);
    write_output(&content, output);
    Some(games.len())
}
