use std::str::FromStr;

const FILE: &str = "inputs/day24.txt";

type Value = i64;
type Program = Vec<Instruction>;

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(register: &str) -> Result<Self, Self::Err> {
        Ok(match register {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => return Err(format!("Invalid register: {}", register.to_string())),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Argument {
    Register(Register),
    Value(Value),
}

impl FromStr for Argument {
    type Err = String;

    fn from_str(argument: &str) -> Result<Self, Self::Err> {
        if let Ok(r) = argument.parse::<Register>() {
            Ok(Argument::Register(r))
        } else if let Ok(v) = argument.parse::<Value>() {
            Ok(Argument::Value(v))
        } else {
            Err(format!("Invalid argument: {}", argument.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(Register),
    Add(Register, Argument),
    Multiply(Register, Argument),
    Divide(Register, Argument),
    Modulo(Register, Argument),
    Equal(Register, Argument),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut iter = instruction.trim().split_whitespace();

        Ok(match (iter.next(), iter.next(), iter.next()) {
            (Some("inp"), Some(r), None) => Instruction::Input(r.parse()?),
            (Some("add"), Some(r), Some(a)) => Instruction::Add(r.parse()?, a.parse()?),
            (Some("mul"), Some(r), Some(a)) => Instruction::Multiply(r.parse()?, a.parse()?),
            (Some("div"), Some(r), Some(a)) => Instruction::Divide(r.parse()?, a.parse()?),
            (Some("mod"), Some(r), Some(a)) => Instruction::Modulo(r.parse()?, a.parse()?),
            (Some("eql"), Some(r), Some(a)) => Instruction::Equal(r.parse()?, a.parse()?),
            _ => return Err(format!("Invalid instruction: {}", instruction.to_string())),
        })
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
struct ALU<'a> {
    program: &'a Program,
    w: Value,
    x: Value,
    y: Value,
    z: Value,
}

impl<'a> ALU<'a> {
    #[allow(clippy::ptr_arg)]
    fn new(program: &'a Program) -> Self {
        ALU {
            program,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn run(&mut self, mut input: impl Iterator<Item = Value>) -> Value {
        for &instruction in self.program {
            match instruction {
                Instruction::Input(r) => *self.get_register_mut(r) = input.next().unwrap(),
                Instruction::Add(r, a) => *self.get_register_mut(r) += self.get_value(a),
                Instruction::Multiply(r, a) => *self.get_register_mut(r) *= self.get_value(a),
                Instruction::Divide(r, a) => *self.get_register_mut(r) /= self.get_value(a),
                Instruction::Modulo(r, a) => *self.get_register_mut(r) %= self.get_value(a),
                Instruction::Equal(r, a) => {
                    *self.get_register_mut(r) =
                        Value::from(self.get_register(r) == self.get_value(a))
                }
            }
        }

        self.z
    }

    fn get_value(&self, argument: Argument) -> Value {
        match argument {
            Argument::Register(r) => self.get_register(r),
            Argument::Value(v) => v,
        }
    }

    fn get_register(&self, register: Register) -> Value {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn get_register_mut(&mut self, register: Register) -> &mut Value {
        match register {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let program = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Program, String>>()?;
    let mut alu = ALU::new(&program);

    // To enable as many submarine features as possible, find the largest valid
    // fourteen-digit model number that contains no 0 digits. What is the
    // largest model number accepted by MONAD?
    // Essentially, we have fourteen steps, of which 7 will multiply z by 26 and
    // add (input + something), and 7 of which will divide by 26, when the input
    // is equal to an offset previous input. This offset will limit our choices,
    // but it's easy enough to do by hand.
    let largest_number = [9, 2, 9, 2, 8, 9, 1, 4, 9, 9, 9, 9, 9, 1];
    assert_eq!(0, alu.clone().run(largest_number.into_iter()));

    // What is the smallest model number accepted by MONAD?
    let smallest_number = [9, 1, 8, 1, 1, 2, 1, 1, 6, 1, 1, 9, 8, 1];
    assert_eq!(0, alu.run(smallest_number.into_iter()));

    Ok(())
}
