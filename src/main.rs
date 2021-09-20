use std::error::Error;
use std::io::BufWriter;
use std::io::Write;

#[derive(Debug, PartialEq)]
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
where
    I: Iterator<Item = char>,
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

    fn val(&self) -> u8 {
        self.data[self.pos]
    }

    fn execute<W: Write>(
        &mut self,
        commands: &[Command],
        out: &mut W,
    ) -> Result<(), Box<dyn Error>> {
        for command in commands {
            match command {
                Command::Increment => self.data[self.pos] = self.val().wrapping_add(1),
                Command::Decrement => self.data[self.pos] = self.val().wrapping_sub(1),
                Command::MoveRight => self.pos +=  1,
                Command::MoveLeft => self.pos -= 1,
                Command::Print => out.write_all(&[self.val()])?,
                Command::Read => todo!(),
                Command::Loop(loop_commands) => {
                    while self.val() != 0 {
                        self.execute(loop_commands, out)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_hello_world() -> Result<(), Box<dyn Error>> {
    let mut buf = BufWriter::new(Vec::new());
    let input = "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.";
    let commands = parse_commands(&mut input.chars())?;
    let mut engine = Engine::new();
    engine.execute(&commands, &mut buf)?;
    let output = String::from_utf8(buf.into_inner()?)?;
    assert_eq!(output, "hello world");
    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let mut buf = BufWriter::new(Vec::new());
    let input = "+>++<-";
    let commands = parse_commands(&mut input.chars())?;
    assert_eq!(
        commands,
        vec![
            Command::Increment,
            Command::MoveRight,
            Command::Increment,
            Command::Increment,
            Command::MoveLeft,
            Command::Decrement,
        ]
    );
    let mut engine = Engine::new();
    engine.execute(&commands, &mut buf)?;
    let output = String::from_utf8(buf.into_inner()?)?;
    assert_eq!(engine.data[0], 0);
    assert_eq!(engine.data[1], 2);
    assert_eq!(output, "");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name: String = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(file_name)?;
    let commands = parse_commands(&mut input.chars())?;
    let mut engine = Engine::new();
    engine.execute(&commands, &mut std::io::stdout())?;
    Ok(())
}
