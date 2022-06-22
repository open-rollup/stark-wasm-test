use log::debug;
use std::time::Instant;

use winterfell::math::fields::f64::BaseElement;
use winterfell::{FieldExtension, HashFunction, ProofOptions, StarkProof};
use stark_wasm::get_example;

pub fn run_winter(sequence_length: usize, result: BaseElement) {
    let options = ProofOptions::new(
        27, // number of queries
        8,  // blowup factor
        16, // grinding factor
        HashFunction::Blake3_192,
        FieldExtension::Quadratic,
        8,   // FRI folding factor
        256, // FRI max remainder length
    );

    let example = get_example(options, sequence_length, result);

    // generate proof
    let now = Instant::now();
    let proof = example.prove();
    debug!(
        "---------------------\nProof generated in {} ms",
        now.elapsed().as_millis()
    );

    let proof_bytes = proof.to_bytes();
    debug!("Proof size: {:.1} KB", proof_bytes.len() as f64 / 1024f64);
    debug!("Proof security: {} bits", proof.security_level(true));
    #[cfg(feature = "std")]
    debug!(
        "Proof hash: {}",
        hex::encode(blake3::hash(&proof_bytes).as_bytes())
    );

    // verify the proof
    debug!("---------------------");
    let parsed_proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    assert_eq!(proof, parsed_proof);
    let now = Instant::now();
    match example.verify(proof) {
        Ok(_) => debug!(
            "Proof verified in {:.1} ms",
            now.elapsed().as_micros() as f64 / 1000f64
        ),
        Err(msg) => debug!("Failed to verify proof: {}", msg),
    }
    debug!("============================================================");
}
