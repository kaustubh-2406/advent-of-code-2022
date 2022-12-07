use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct SlidingWindow {
    content: Vec<char>,
    size: usize,
}

impl SlidingWindow {
    fn new(size: usize) -> Self {
        SlidingWindow {
            content: vec![],
            size,
        }
    }

    fn handle_push(&mut self, val: char) {
        if self.content.len() == self.size {
            self.content.remove(0);
        }
        self.content.push(val);
    }

    fn clear(&mut self) {
        self.content.clear()
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

    let mut sliding_window_4 = SlidingWindow::new(4);
    let mut sliding_window_14 = SlidingWindow::new(14);

    sliding_window_4.clear();
    sliding_window_14.clear();

    println!("====== Sample input ======");
    println!(
        "Part 1: {}, Part 2: {}\n",
        solve(&mut sliding_window_4, sample.clone()),
        solve(&mut sliding_window_14, sample.clone())
    );

    sliding_window_4.clear();
    sliding_window_14.clear();

    println!("====== Input ======");
    println!(
        "Part 1: {}, Part 2: {}",
        solve(&mut sliding_window_4, input.clone()),
        solve(&mut sliding_window_14, input.clone())
    );

    sliding_window_4.clear();
    sliding_window_14.clear();

    fn solve(sliding_window: &mut SlidingWindow, s: String) -> usize {
        for (i, character) in s.char_indices() {
            sliding_window.handle_push(character);

            if sliding_window.content.len() == sliding_window.size {
                let cond = get_char_occurances(&sliding_window.content)
                    .iter()
                    .all(|(_char, usize)| *usize == 1);

                if cond {
                    return i + 1;
                }
            }
        }

        0
    }
}
