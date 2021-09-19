use std::error::Error;

#[derive(Debug,PartialEq)]
enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    Loop(Vec<Command>),
}

fn parse_commands<I>(input: &mut I) -> Result<Vec<Command>, Box<dyn Error>> 
where I: Iterator<Item=char>,
{
    let mut commands = Vec::new();
    while let Some(c) = input.next() {
        let command = match c {
            '>' => Command::MoveRight,
            '<' => Command::MoveLeft,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            '.' => Command::Print,
            ',' => Command::Read,
            '[' => Command::Loop(parse_commands(input)?),
            ']' => break,
            _ => continue,
        };
        commands.push(command);
    }
    Ok(commands)
}

const MAX_POSITIONS: usize = 30000;

struct Engine {
    data: [u8; MAX_POSITIONS],
    pos: usize,
}

impl Engine {
    fn new() -> Self {
        Self {
            data: [0; MAX_POSITIONS],
            pos: 0,
        }
    }

    fn execute(&mut self, commands: &Vec<Command>) -> Result<String, Box<dyn Error>> {
        let mut output = String::new();
        for command in commands {
            match command {
                Command::Increment => self.data[self.pos] += 1,
                Command::Decrement => self.data[self.pos] -= 1,
                Command::MoveRight => {
                    if self.pos == MAX_POSITIONS {
                        self.pos = 0 
                    } else {
                        self.pos += 1
                    }
                },
                Command::MoveLeft => {
                    if self.pos == 0 {
                        self.pos = MAX_POSITIONS
                    } else {
                        self.pos -= 1
                    }
                },
                Command::Print => output.push(self.data[self.pos].into()),
                Command::Read => todo!(),
                Command::Loop(loop_commands) => {
                    while self.data[self.pos] != 0 {
                        self.execute(&loop_commands)?;
                    }
                }
            }
        }
        Ok(output)
    }
}

#[test]
fn test() -> Result<(),Box<dyn Error>> {
    let input = "+>++<-";
    let commands = parse_commands(&mut input.chars())?;
    assert_eq!(commands, vec![
        Command::Increment,
        Command::MoveRight,
        Command::Increment,
        Command::Increment,
        Command::MoveLeft,
        Command::Decrement,
    ]);
    let mut engine = Engine::new();
    let _ = engine.execute(&commands)?;
    assert_eq!(engine.data[0], 0);
    assert_eq!(engine.data[1], 2);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name: String = std::env::args().skip(1).next().unwrap();
    let input = std::fs::read_to_string(file_name)?;
    let commands = parse_commands(&mut input.chars())?;
    let mut engine = Engine::new();
    let output = engine.execute(&commands)?;
    println!("{}", output);
    Ok(())
}
