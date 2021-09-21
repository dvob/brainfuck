use std::error::Error;
use std::io::BufWriter;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum Command {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
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
            '>' => Command::MoveRight(1),
            '<' => Command::MoveLeft(1),
            '+' => Command::Increment(1),
            '-' => Command::Decrement(1),
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

fn optimize_commands(commands: Vec<Command>) -> Vec<Command> {
    let mut optimized_commands = Vec::new();
    if commands.len() == 0 {
        return optimized_commands;
    }
    let mut cmds = commands.into_iter();
    let mut prev = cmds.next().unwrap();
    for cmd in cmds {
        match (prev,cmd) {
            (Command::MoveRight(i), Command::MoveRight(j)) => prev = Command::MoveRight(i+j),
            (Command::MoveLeft(i), Command::MoveLeft(j)) => prev = Command::MoveLeft(i + j),
            (Command::Increment(i), Command::Increment(j)) => prev = Command::Increment(i + j),
            (Command::Decrement(i), Command::Decrement(j)) => prev = Command::Decrement(i+j),
            (p, Command::Loop(loop_cmds)) => {
                optimized_commands.push(p);
                prev = Command::Loop(optimize_commands(loop_cmds))
            },
            (p,cmd) => {
                optimized_commands.push(p);
                prev = cmd;
            }
        }
    }
    optimized_commands.push(prev);
    optimized_commands
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
                Command::Increment(i) => self.data[self.pos] = self.val().wrapping_add(*i),
                Command::Decrement(i) => self.data[self.pos] = self.val().wrapping_sub(*i),
                Command::MoveRight(i) => self.pos +=  i,
                Command::MoveLeft(i) => self.pos -= i,
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
fn test_opt_cmds() {
    let commands = vec![
        Command::Increment(1),
        Command::Increment(1),
        Command::MoveRight(1),
        Command::MoveRight(1),
        Command::Loop(vec![
            Command::Decrement(1),
            Command::Decrement(1),
        ])
    ];

    let commands = optimize_commands(commands);
    assert_eq!(commands, vec![
        Command::Increment(2),
        Command::MoveRight(2),
        Command::Loop(vec![
            Command::Decrement(2),
        ])
    ])
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let mut buf = BufWriter::new(Vec::new());
    let input = "+>++<-";
    let commands = parse_commands(&mut input.chars())?;
    assert_eq!(
        commands,
        vec![
            Command::Increment(1),
            Command::MoveRight(1),
            Command::Increment(1),
            Command::Increment(1),
            Command::MoveLeft(1),
            Command::Decrement(1),
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

fn count_instructions(commands: &Vec<Command>) -> u32 {
    let mut instructions = 0;
    for cmd in commands {
         instructions += match cmd {
            Command::Loop(loop_commands) => count_instructions(loop_commands),
            _ => 1,
        }
    }
    instructions
}

fn print_info(commands: Vec<Command>) {
    println!("instructions: {}", count_instructions(&commands));
    let commands = optimize_commands(commands);
    println!("optimized instructions: {}", count_instructions(&commands));
}

fn print_usage() {
    println!("Usage: brainfuck FILE");
    println!("");
    println!("Flags:");
    println!(" -d, --disable  Disable optimization of code");
    println!(" -i, --info     Print information about the brainfuck source file and exit");
    println!(" -h, --help     Show this help message");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut info = false;
    let mut optimize = true;
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        if arg.starts_with("-") {
            let arg = arg.trim_start_matches("-");
            match arg {
                "i" | "info" => info = true,
                "d" | "disable" => optimize = false,
                "h" | "help" => {
                    print_usage();
                    std::process::exit(0);
                },
                _ => {
                    write!(std::io::stderr(), "unknown option {}\n", arg)?;
                    std::process::exit(1);
                },
            }
        } else {
            args.push(arg)
        }
    }
    if args.len() == 0 {
        write!(std::io::stderr(), "missing source file\n")?;
        std::process::exit(1);
    }
    if args.len() > 1 {
        write!(std::io::stderr(), "only one source file supported\n")?;
        std::process::exit(1);
    }
    let input = std::fs::read_to_string(&args[0])?;
    let mut commands = parse_commands(&mut input.chars())?;
    if info {
        print_info(commands);
        return Ok(())
    }
    if optimize {
        commands = optimize_commands(commands);
    }
    let mut engine = Engine::new();
    engine.execute(&commands, &mut std::io::stdout())?;
    Ok(())
}
