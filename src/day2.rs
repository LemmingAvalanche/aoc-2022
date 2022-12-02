// Opponent:
// • A: Rock
// • B: Paper
// • C: Scissors

// Me:
// X: Rock
// Y: Paper
// Z: Scissors

// Part 2:
// X: Lost
// Y: Draw
// Z: Win

// Scores:
// Rock: 1
// Paper: 2
// Scissors: 3

// 0 for lost
// 3 for draw
// 6 for won
enum Outcome {
    Won,
    Lost,
    Draw
}

#[derive(Clone,Copy,PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

pub fn part1_score(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (opp, me) = Move::from_line(l);
            opp.play(me)
        })
        .sum()
}

pub fn part2_score(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (opp, me) = Move::from_line_part2(l);
            opp.play_part2(me)
        })
        .sum()
}

impl Outcome {
    // My input
    pub fn from_input(c: char) -> Self {
        use Outcome::*;
        match c {
            'X' => Lost,
            'Y' => Draw,
            'Z' => Won,
            _ => panic!("impossible (bad input)")
        }
    }
}

impl Move {
    /// return opponent and me
    pub fn from_line(line: &str) -> (Move, Move) {
        let mut line_c = line.chars();
        let opponent = Self::from_opponent(line_c.nth(0).unwrap());
        let me = Self::from_me(line_c.nth(1).unwrap());
        (opponent, me)
    }

    pub fn from_line_part2(line: &str) -> (Move, Outcome) {
        let mut line_c = line.chars();
        let opponent = Self::from_opponent(line_c.nth(0).unwrap());
        let outcome = Outcome::from_input(line_c.nth(1).unwrap());
        (opponent, outcome)
    }

    fn from_opponent(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("impossible (bad input)")
        }
    }

    fn from_me(c: char) -> Self {
        match c {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("impossible (bad input)")
        }
    }

    fn piece_point(self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn battle(self, me: Move) -> Outcome {
        use Outcome::*;
        match self {
            Move::Rock => match me {
                Move::Rock => Draw,
                Move::Paper => Won,
                Move::Scissors => Lost,
            }
            Move::Paper => match me {
                Move::Rock => Lost,
                Move::Paper => Draw,
                Move::Scissors => Won,
            },
            Move::Scissors => match me{
                Move::Rock => Won,
                Move::Paper => Lost,
                Move::Scissors => Draw,
            },
        }
    }

    fn move_for_outcome(self, me: Outcome) -> Move {
        use Outcome::*;
        match self {
            Move::Rock => match me {
                Draw => Move::Rock,
                Won => Move::Paper,
                Lost => Move::Scissors,
            }
            Move::Paper => match me {
                Lost => Move::Rock,
                Draw => Move::Paper,
                Won => Move::Scissors,
            },
            Move::Scissors => match me{
                Won => Move::Rock,
                Lost => Move::Paper,
                Draw => Move::Scissors,
            },
        }
    }

    /// Opponent plays against me
    /// return score
    pub fn play(self, me: Move) -> u64 {
        use Outcome::*;

        let piece_point = me.piece_point();
        let game_outcome = match self.battle(me) {
            Lost => 0,
            Draw => 3,
            Won => 6,
        };
        piece_point + game_outcome
    }

    /// Opponent plays against me
    /// return score
    pub fn play_part2(self, outcome: Outcome) -> u64 {
        use Outcome::*;
        let me = self.move_for_outcome(outcome);
        let piece_point = me.piece_point();
        let game_outcome = match self.battle(me) {
            Lost => 0,
            Draw => 3,
            Won => 6,
        };
        piece_point + game_outcome
    }
}
