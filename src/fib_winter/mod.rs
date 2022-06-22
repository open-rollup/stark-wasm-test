// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::Example;
use log::debug;
#[cfg(feature = "std")]
use std::time::Instant;
#[cfg(feature = "std")]
use winterfell::math::log2;
use winterfell::{
    math::{fields::f64::BaseElement, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};

mod air;
use air::FibAir;

mod prover;
use prover::FibProver;

// ================================================================================================

const TRACE_WIDTH: usize = 2;

// FIBONACCI EXAMPLE
// ================================================================================================

pub struct FibExample {
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
}

impl FibExample {
    pub fn new(sequence_length: usize, options: ProofOptions, result: BaseElement) -> FibExample {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        FibExample {
            options,
            sequence_length,
            result,
        }
    }
}

// EXAMPLE IMPLEMENTATION
// ================================================================================================

impl Example for FibExample {
    fn prove(&self) -> StarkProof {
        debug!(
            "Generating proof for computing Fibonacci sequence (2 terms per step) up to {}th term\n\
            ---------------------",
            self.sequence_length
        );

        // create a prover
        let prover = FibProver::new(self.options.clone());

        // generate execution trace
        #[cfg(feature = "std")]
        let now = Instant::now();
        let trace = prover.build_trace(self.sequence_length);

        #[cfg(feature = "std")]
        let trace_width = trace.width();
        #[cfg(feature = "std")]
        let trace_length = trace.length();
        #[cfg(feature = "std")]
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            log2(trace_length),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<FibAir>(proof, self.result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<FibAir>(proof, self.result + BaseElement::ONE)
    }
}
