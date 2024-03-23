use std::io::Read;

#[derive(Debug)]
struct Band {
    right: Vec<u8>,
    null: u8,
    left: Vec<u8>,
    pos: i32,
    code_pos: i32,
}

impl Band {
    pub fn new() -> Self {
        Self {
            right: vec![],
            null: 0,
            left: vec![],
            pos: 0,
            code_pos: 0,
        }
    }

    fn get(&self) -> &u8 {
        match self.pos {
            pos if pos < 0 => self.left.get(pos.abs() as usize - 1).unwrap(),
            pos if pos == 0 => &self.null,
            pos if pos > 0 => self.right.get(pos as usize - 1).unwrap(),
            _ => panic!("unreachable"),
        }
    }

    fn set(&mut self, value: u8) {
        match self.pos {
            pos if self.pos < 0 => self.left[pos.abs() as usize - 1] = value,
            _ if self.pos == 0 => self.null = value,
            pos if self.pos > 0 => self.right[pos as usize - 1] = value,
            _ => panic!("unreachable"),
        }
    }

    fn inc(&mut self) {
        match self.pos {
            pos if pos < 0 => {
                self.left[pos.abs() as usize - 1] =
                    self.left[pos.abs() as usize - 1].wrapping_add(1)
            }
            pos if pos == 0 => self.null = self.null.wrapping_add(1),
            pos if pos > 0 => {
                self.right[pos as usize - 1] = self.right[pos as usize - 1].wrapping_add(1)
            }
            _ => panic!("unreachable"),
        }
    }

    fn dec(&mut self) {
        match self.pos {
            pos if pos < 0 => {
                self.left[pos as usize - 1] = self.left[pos as usize - 1].wrapping_sub(1)
            }
            pos if pos == 0 => self.null = self.null.wrapping_sub(1),
            pos if pos > 0 => {
                self.right[pos as usize - 1] = self.right[pos as usize - 1].wrapping_sub(1)
            }
            _ => panic!("unreachable"),
        }
    }

    fn move_left(&mut self) {
        self.pos -= 1;
        if self.left.len() < self.pos.abs() as usize {
            self.left.push(0);
        }
    }

    fn move_right(&mut self) {
        self.pos += 1;
        if self.right.len() < self.pos as usize {
            self.right.push(0);
        }
    }

    fn output(&self) {
        print!("{}", *self.get() as char);
    }

    fn input(&mut self) {
        self.set(
            std::io::stdin()
                .bytes()
                .next()
                .expect("Unexpected end of stdin")
                .expect(format!("Unable to read char at pos {}", "TODO").as_str()),
        );
    }

    fn get_pos_of_matching_bracket(&self, code: &[char]) -> Option<usize> {
        let mut open_count = 0;
        let mut close_count = 0;

        for (i, c) in code.iter().enumerate() {
            match c {
                '[' => open_count += 1,
                ']' => {
                    close_count += 1;
                    if open_count == close_count {
                        return Some(i);
                    }
                }
                _ => (),
            }
        }

        None
    }

    pub fn exec_loop(&mut self, input: &[char], is_main: bool) {
        loop {
            let mut it = input.iter().enumerate();
            while let Some((i, c)) = it.next() {
                match c {
                    '+' => self.inc(),
                    '-' => self.dec(),
                    '<' => self.move_left(),
                    '>' => self.move_right(),
                    '.' => self.output(),
                    ',' => self.input(),
                    '[' => {
                        let matching_pos = self
                            .get_pos_of_matching_bracket(&input[i..])
                            .expect("No matching bracket");

                        self.exec_loop(&input[i + 1..matching_pos + i], false);
                        it.nth(matching_pos - 1);
                    }
                    _ => panic!("not possible"),
                }
                self.code_pos += 1;
            }

            if is_main || self.get() == &0u8 {
                break;
            }
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = args.get(1).expect("missing arg (path to brainfuck file)");
    let input: Vec<char> = std::fs::read_to_string(path)
        .expect("Cant open file")
        .chars()
        .into_iter()
        .filter(|c| ['+', '-', '<', '>', '[', ']', '.', ','].contains(c))
        .collect();
    let mut band = Band::new();
    band.exec_loop(&input, true);
}
