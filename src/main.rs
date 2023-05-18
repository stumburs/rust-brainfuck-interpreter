// GOOD LORD...
// MY UNSAFE C++ BRAIN CAN'T HANDLE THE SAFETY
// YOU CAN'T OVERFLOW VALUES SUCH AS INTEGERS?!?
// I FEEL SO TRAPPED
// HELP
// WHAT IS THIS???
// WHERE ARE MY BELOVED FOR LOOPS???
// warnings, errors, all causing me night terrors
// please, let me manage my own memory, at least a bit

use std::io::stdin;

const MEMORY_SIZE: usize = 30000;

fn interpret(program: &str) {
    let stdin = std::io::stdin();

    let mut mem: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];
    let mut pointer: usize = 0;

    let i: usize = 0;
    while i < program.len() {
        let current_char = program.as_bytes()[i];
        if current_char == b'>' {
            pointer += 1;
            continue;
        }
        if current_char == b'<' {
            pointer -= 1;
            continue;
        }
        if current_char == b'+' {
            mem[pointer] += 1;
            continue;
        }
        if current_char == b'-' {
            mem[pointer] -= 1;
            continue;
        }
        if current_char == b'.' {
            print!("{}", mem[pointer]);
            continue;
        }
        if current_char == b',' {
            continue;
        }
        // case '[':
        //     if (*ptr == 0)
        //     {
        //         // Skip loop body
        //         int nested_loops = 1;
        //         while (nested_loops > 0)
        //         {
        //             ++i;
        //             if (program[i] == '[')
        //                 ++nested_loops;
        //             else if (program[i] == ']')
        //                 --nested_loops;
        //         }
        //     }
        //     else
        //         loop_stack.push(i);
        //     break;
        // case ']':
        //     if (*ptr != 0)
        //         i = loop_stack.top();
        //     else
        //         loop_stack.pop();
        //     break;
        // default:
        //     break;
        // }
        // ++i;
        // if (loop_stack.size() == 0 && i >= program.length())
        //     break; // exit loop if all loops have been exited
    }
}

fn main() {
    // Hello World code
    let code: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    interpret(code);
}
