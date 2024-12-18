use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 17");
    let data = input(17, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let mut computer = Computer::parse(input);
    computer.execute();
    println!("P1: {}", computer.output());
}

fn p2(input: &str) {}

#[derive(Debug, Default)]
struct Computer {
    ra: i64,
    rb: i64,
    rc: i64,
    ins: Vec<i64>,
    out: Vec<i64>,
    ip: usize,
}

impl Computer {
    fn execute(&mut self) {
        while self.ip < self.ins.len() {
            let mut jumped = false;

            let ins = self.ins[self.ip];
            let op = self.ins[self.ip + 1];

            match ins {
                0 => self.adv(op),
                1 => self.bxl(op),
                2 => self.bst(op),
                3 => jumped = self.jnz(op),
                4 => self.bxc(op),
                5 => self.out(op),
                6 => self.bdv(op),
                7 => self.cdv(op),
                _ => unreachable!(),
            }

            if !jumped {
                self.ip += 2;
            }
        }
    }

    fn output(&self) -> String {
        itertools::join(&self.out, ",")
    }

    fn adv(&mut self, op: i64) {
        self.ra /= 2i64.pow(self.combo(op) as u32);
    }

    fn bxl(&mut self, op: i64) {
        self.rb ^= op;
    }

    fn bst(&mut self, op: i64) {
        self.rb = self.combo(op) % 8;
    }

    fn jnz(&mut self, op: i64) -> bool {
        if self.ra == 0 {
            return false;
        }
        self.ip = op as usize;
        true
    }

    fn bxc(&mut self, _op: i64) {
        self.rb ^= self.rc;
    }

    fn out(&mut self, op: i64) {
        self.out.push(self.combo(op) % 8);
    }

    fn bdv(&mut self, op: i64) {
        self.rb = self.ra / 2i64.pow(self.combo(op) as u32);
    }

    fn cdv(&mut self, op: i64) {
        self.rc = self.ra / 2i64.pow(self.combo(op) as u32);
    }

    fn parse(input: &str) -> Self {
        input.lines().fold(Computer::default(), |mut comp, line| {
            if line.is_empty() {
                return comp;
            }
            let (label, value) = line.split_once(':').unwrap();
            match label {
                "Register A" => comp.ra = value.trim().parse().unwrap(),
                "Register B" => comp.rb = value.trim().parse().unwrap(),
                "Register C" => comp.rc = value.trim().parse().unwrap(),
                "Program" => {
                    comp.ins = value
                        .trim()
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect_vec()
                }
                _ => unreachable!(),
            }
            comp
        })
    }

    fn combo(&self, op: i64) -> i64 {
        match op {
            ..4 => op,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }
}
