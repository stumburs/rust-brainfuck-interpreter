// GOOD LORD...
// MY UNSAFE C++ BRAIN CAN'T HANDLE THE SAFETY
// YOU CAN'T OVERFLOW VALUES SUCH AS INTEGERS?!?
// I FEEL SO TRAPPED
// HELP
// WHAT IS THIS???
// WHERE ARE MY BELOVED FOR LOOPS???
// warnings, errors, all causing me night terrors
// please, let me manage my own memory, at least a bit

const MEMORY_SIZE: usize = 30000;

fn interpret(program: &str) {
    let mut mem: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];
    let mut pointer: usize = 0;

    let mut i: usize = 0;
    while i < program.len() - 1 {
        // I have no clue why it's the way it is.
        let current_char = program.as_bytes()[i];
        if current_char == b'>' {
            pointer += 1;
            continue;
        }
        if current_char == b'<' {
            pointer -= 1;
            continue;
        }
        // This makes it not work :)
        if current_char == b'+' {
            // Can't wrap values by overflowing them, because it's Rust.
            mem[pointer] = mem[pointer].wrapping_add(1);
            continue;
        }
        // Don't even know about this.
        if current_char == b'-' {
            // Can't do -1, because it's unsigned and whatnot.
            mem[pointer] = mem[pointer].wrapping_sub(1);
            continue;
        }
        // ¯\_(ツ)_/¯
        if current_char == b'.' {
            print!("{}", mem[pointer]);
            continue;
        }
        if current_char == b',' {
            continue;
        }

        i += 1;
        // Funny C++ code i've yet to turn into Rust code.

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
