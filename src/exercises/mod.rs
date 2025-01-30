use ::rand;
use ::rand::prelude::*;

pub trait Exercise {
    fn reset(&mut self);
}

#[derive(Debug, Clone)]
struct Diagram {
    result: String, // The values for the exercice
    x1: String,
    x2: String,
    dialog: String,    // The info box
    open_window: bool, // The reveal window to update value
    pop_up_buf: String,
    target: DiagramValue,
    nb_viewed: usize, // Answer stats
    nb_correct: usize,
    nb_wrong: usize,
    // nb_skipped: usize,
}

impl Diagram {
    fn check_result(&self) -> (bool, String) {
        let res = self.result.parse::<i32>();
        let x1_val = self.x1.parse::<i32>();
        let x2_val = self.x2.parse::<i32>();

        match (res, x1_val, x2_val) {
            (Ok(res), Ok(x1_val), Ok(x2_val)) => {
                if res == x1_val + x2_val {
                    (true, String::from("Correct!"))
                } else {
                    (false, String::from("Try again :)"))
                }
            }
            _ => (false, String::from("Invalid input!")),
        }
    }

    fn reset_counts(&mut self) {
        self.nb_viewed = 0;
        self.nb_correct = 0;
        self.nb_wrong = 0;
    }

    fn truncate(&mut self) {
        match self.target {
            DiagramValue::X1 => {
                let n = self.x1.len();
                if n > 0 {
                    self.x1.truncate(n - 1);
                }
            }
            DiagramValue::X2 => {
                let n = self.x2.len();
                if n > 0 {
                    self.x2.truncate(n - 1);
                }
            }
            DiagramValue::Result => {
                let n = self.result.len();
                if n > 0 {
                    self.result.truncate(n - 1);
                }
            }
        }
    }

    fn push(&mut self, c: char) {
        match self.target {
            DiagramValue::X1 => {
                self.x1.push(c);
            }
            DiagramValue::X2 => {
                self.x2.push(c);
            }
            DiagramValue::Result => {
                self.result.push(c);
            }
        }
    }

    fn random_addition(state: &mut Diagram) {
        let mut rng = rand::rng();
        let result: u32 = rng.random_range(0..=20);
        let x: i32 = rng.random_range(0..=result as i32);
        let xy = rng.random_range(0..=1);

        if xy == 0 {
            state.x1 = x.to_string();
            state.x2 = "".to_string();
            state.target = DiagramValue::X2;
        } else {
            state.x1 = "".to_string();
            state.x2 = x.to_string();
            state.target = DiagramValue::X1;
        }
        state.result = result.to_string();
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Self {
            result: String::from("12"),
            x1: String::from(""),
            x2: String::from("4"),
            dialog: String::from("Start the timer to play!"),
            open_window: false,
            pop_up_buf: String::from(""),
            target: DiagramValue::X1,
            nb_viewed: 0,
            nb_correct: 0,
            nb_wrong: 0,
            // nb_skipped: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DiagramValue {
    Result,
    X1,
    X2,
}

enum KeyAction {
    Truncate,
    Push(char),
    Check,
    Next,
}
