use std::fs;

#[derive(Debug, Clone, Copy)]
enum Command {
    Addx(isize),
    Noop,
}

#[derive(Debug)]
struct CRT {
    x: isize,
    cycle: usize,
    instr_indx: usize,
    instr_active: bool,
    instructions: Vec<Command>,
}

fn parse_instructions(input: String) -> Vec<Command> {
    input.trim().lines().fold(vec![], |mut acc, x| {
        let words = x.split_whitespace().collect::<Vec<&str>>();
        let cmd = *words.first().unwrap();

        match cmd {
            "addx" => {
                let num = words.get(1).unwrap().parse::<isize>().unwrap();
                acc.push(Command::Addx(num));
            }
            "noop" => acc.push(Command::Noop),
            _ => {}
        }

        acc
    })
}

impl CRT {
    fn new(instructions: Vec<Command>) -> Self {
        Self {
            x: 1,
            cycle: 1,
            instr_indx: 0,
            instr_active: false,
            instructions,
        }
    }

    fn step(&mut self) {
        if let Some(instr) = self.instructions.get(self.instr_indx) {
            match instr {
                Command::Addx(x) => {
                    if self.instr_active {
                        self.x += x;
                        self.instr_active = false;
                        self.instr_indx += 1;
                    } else {
                        self.instr_active = true;
                    }
                }
                Command::Noop => {
                    self.instr_indx += 1;
                }
            }
        }

        self.cycle += 1;
    }
}

fn main() {
    let input_file = "./data/input.txt";
    let input = fs::read_to_string(input_file).expect("input.txt file not found");

    // let sample_file = "./data/sample.txt";
    // let sample = fs::read_to_string(sample_file).expect("sample.txt file not found");

    let instructions = parse_instructions(input);
    let mut crt = CRT::new(instructions);
    let mut signals: Vec<isize> = vec![];

    for _ in 0..230 {
        crt.step();

        if vec![20, 60, 100, 140, 180, 220].contains(&crt.cycle) {
            let x = crt.x;
            signals.push(x * (crt.cycle as isize));
        }
    }

    println!("{:?}", signals.iter().sum::<isize>())
}
