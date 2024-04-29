
import fs from 'node:fs';

function main() {
	let inputFile = process.argv[2];

	let data = fs.readFileSync(inputFile, 'utf8');

	let code = compile(data);

	run(code)
}

const LEFT = 0
const RIGHT = 1
const INC = 2
const DEC = 3
const JUMP_IF_ZERO = 4
const JUMP_IF_NOT_ZERO = 5
const PRINT = 6

function compile(input) {
	let program = [];
	let jumpStack = [];

	for (let i = 0; i < input.length; i++) {
		switch (input[i]) {
			case '<':
				program.push(LEFT);
				break;
			case '>':
				program.push(RIGHT);
				break;
			case '+':
				program.push(INC);
				break;
			case '-':
				program.push(DEC);
				break;
			case '[':
				jumpStack.push(program.length);
				program.push(JUMP_IF_ZERO);
				program.push(0);
				break;
			case ']':
				let jumpIndex = jumpStack.pop()
				program.push(JUMP_IF_NOT_ZERO);
				program.push(jumpIndex + 2);
				// update back reference
				program[jumpIndex + 1] = program.length
				break;
			case '.':
				program.push(PRINT);
				break;
			default:
				break;
		}
	}
	if (jumpStack.length != 0) {
		throw Exception("invalid program missing closing bracket")
	}
	return program;
}

function run(code) {
	//console.log(code.join(" "))
	let mem = new Uint8Array(30000);
	let ptr = 0;
	let pc = 0;

	while ( pc < code.length ) {
		switch (code[pc]) {
			case LEFT:
				ptr -= 1;
				break;
			case RIGHT:
				ptr += 1;
				break;
			case INC:
				mem[ptr]++;
				break;
			case DEC:
				mem[ptr]--;
				break;
			case JUMP_IF_ZERO:
				if (mem[ptr] == 0) {
					// -1 becuase below we're adding one
					pc = code[pc+1] - 1
				} else {
					pc++;
				}
				break;
			case JUMP_IF_NOT_ZERO:
				if (mem[ptr] != 0) {
					// -1 becuase below we're adding one
					pc = code[pc+1] - 1
				} else {
					//console.log("do not jump to")
					pc++;
				}
				break;
			case PRINT:
				process.stdout.write(String.fromCharCode(mem[ptr]));
				break;
		}
		pc++;
	}
}

main();
