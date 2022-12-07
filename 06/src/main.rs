use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct SlidingWindow {
    content: Vec<char>,
}

impl SlidingWindow {
    fn new() -> Self {
        SlidingWindow { content: vec![] }
    }

    fn handle_push(&mut self, val: char) {
        if self.content.len() == 4 {
            self.content.remove(0);
        }
        self.content.push(val);
    }
}

fn get_char_occurances(xs: &Vec<char>) -> HashMap<char, usize> {
    let mut hs: HashMap<char, usize> = HashMap::new();
    for x in xs {
        let curr = hs.get(&x);
        let newvalue = match curr {
            Some(x) => x + 1,
            None => 1,
        };
        hs.insert(*x, newvalue);
    }

    return hs;
}

fn main() {
    let input_file = "./data/input.txt";
    let sample_file = "./data/sample.txt";

    let input = fs::read_to_string(input_file).expect("Input file reading failed!");
    let sample = fs::read_to_string(sample_file).expect("Sample input file reading failed!");

    let mut sliding_window = SlidingWindow::new();

    for (i, character) in input.char_indices() {
        sliding_window.handle_push(character);

        if sliding_window.content.len() == 4 {
            let cond = get_char_occurances(&sliding_window.content)
                .iter()
                .all(|(_char, usize)| *usize == 1);

            if cond {
                println!("part 1 .... {}", i + 1);
                break;
            }
        }
    }
}
