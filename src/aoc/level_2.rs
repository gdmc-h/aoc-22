use crate::helpers;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper,
    Scissor
}

enum Condition {
    Lost = 0,
    Draw = 3,
    Win = 6
}

impl Condition {
    fn get(inpt: &str) -> Self {
        match inpt {
            "X" => Self::Lost,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("No result found")
        }
    }

    fn win_condition(&self, shape: Shape) -> Shape {
        let match_condition = |win: Shape, lose: Shape| {
            match &self { // results are reverse as we should retrieve OUR shape, not the
                          // opponent's one. If they win, we obviously have a losing shape :)
                Condition::Win => lose,
                Condition::Draw => shape,
                Condition::Lost => win
            }
        };

        match shape {
            Shape::Rock => match_condition(Shape::Scissor, Shape::Paper),
            Shape::Scissor => match_condition(Shape::Paper, Shape::Rock),
            Shape::Paper =>  match_condition(Shape::Rock, Shape::Scissor)
        }
    }
}

impl Shape { 
    fn get(inpt: &str) -> Self {
       match inpt {
           "A" | "X" => Self::Rock,
           "B" | "Y" => Self::Paper,
           "C" | "Z" => Self::Scissor,
           _ => panic!("No shape found")
       } 
    }

    fn win_condition(&self, shape: Shape) -> Condition {
        match &self {
            Shape::Rock => match shape {
                Shape::Rock => Condition::Draw,
                Shape::Paper => Condition::Lost,
                Shape::Scissor => Condition::Win
            },
            Shape::Paper => match shape {
                Shape::Rock => Condition::Win,
                Shape::Paper => Condition::Draw,
                Shape::Scissor => Condition::Lost
            },
            Shape::Scissor => match shape {
                Shape::Rock => Condition::Lost,
                Shape::Paper => Condition::Win,
                Shape::Scissor => Condition::Draw
            },
        }
    }
}

pub fn solve() -> (i32, i32) {
    let level = helpers::load_input::select(2).unwrap();

    let l1: i32 = level.lines().map(|current_match| {     
            let mut shapes = current_match.split_whitespace();

            let opponent_shape = Shape::get(shapes.next().unwrap());
            let my_shape = Shape::get(shapes.next().unwrap());

            my_shape.win_condition(opponent_shape) as i32 + my_shape as i32
        })
        .sum();

    let l2: i32 = level.lines().map(|current_match| {
            let mut shapes = current_match.split_whitespace();

            let opponent_shape = Shape::get(shapes.next().unwrap());
            let match_result = Condition::get(shapes.next().unwrap());

            match_result.win_condition(opponent_shape) as i32 + match_result as i32
        })
        .sum();

    (l1.to_owned(), l2.to_owned())
}

