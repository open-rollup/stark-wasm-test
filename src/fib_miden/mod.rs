use crate::MidenExample;
use log::debug;
use miden::{Assembler, ProgramInputs, Script};

// EXAMPLE BUILDER
// ================================================================================================

pub fn get_example(n: usize, result: u64) -> MidenExample {
    // generate the program and expected results
    let program = generate_fibonacci_program(n);
    let expected_result = vec![result];
    debug!(
        "Generated a program to compute {}-th Fibonacci term; expected result: {}",
        n, expected_result[0]
    );

    MidenExample {
        program,
        inputs: ProgramInputs::from_stack_inputs(&[0, 1]).unwrap(),
        pub_inputs: vec![0, 1],
        expected_result,
        num_outputs: 1,
    }
}

/// Generates a program to compute the `n`-th term of Fibonacci sequence
fn generate_fibonacci_program(n: usize) -> Script {
    // the program is a simple repetition of 4 stack operations:
    // the first operation moves the 2nd stack item to the top,
    // the second operation duplicates the top 2 stack items,
    // the third operation removes the top item from the stack
    // the last operation pops top 2 stack items, adds them, and pushes
    // the result back onto the stack
    let program = format!(
        "begin 
            repeat.{}
                swap dup.1 add
            end
        end",
        n - 1
    );

    let assembler = Assembler::new();
    assembler.compile_script(&program).unwrap()
}
