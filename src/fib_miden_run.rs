use log::debug;
use std::time::Instant;

use miden::{ProofOptions, StarkProof};
use stark_wasm::{get_miden_example, MidenExample};

pub fn run_miden(sequence_length: usize, result: u64) {
    let MidenExample {
        program,
        inputs,
        num_outputs,
        pub_inputs,
        expected_result,
    } = get_miden_example(sequence_length, result);
    debug!("--------------------------------");

    let options = ProofOptions::with_96_bit_security();
    // execute the program and generate the proof of execution
    let now = Instant::now();
    let (outputs, proof) = miden::execute(&program, &inputs, num_outputs, &options).unwrap();
    debug!("--------------------------------");

    debug!(
        "Executed program in {} ms",
        //hex::encode(program.hash()), // TODO: include into message
        now.elapsed().as_millis()
    );
    debug!("Program output: {:?}", outputs);
    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    // serialize the proof to see how big it is
    let proof_bytes = proof.to_bytes();
    debug!("Execution proof size: {} KB", proof_bytes.len() / 1024);
    debug!(
        "Execution proof security: {} bits",
        proof.security_level(true)
    );
    debug!("--------------------------------");

    // verify that executing a program with a given hash and given inputs
    // results in the expected output
    let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    let now = Instant::now();
    match miden::verify(program.hash(), &pub_inputs, &outputs, proof) {
        Ok(_) => debug!("Execution verified in {} ms", now.elapsed().as_millis()),
        Err(err) => debug!("Failed to verify execution: {}", err),
    }
}
