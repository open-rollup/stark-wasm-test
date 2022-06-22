#![cfg_attr(not(feature = "std"), no_std)]

use miden::{ProgramInputs, Script};
use winterfell::math::fields::f64::BaseElement;
use winterfell::{ProofOptions, StarkProof, VerifierError};

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

mod fib_winter;
use fib_winter::FibExample;

mod fib_miden;
pub use fib_miden::get_example as get_miden_example;

pub struct MidenExample {
    pub program: Script,
    pub inputs: ProgramInputs,
    pub pub_inputs: Vec<u64>,
    pub num_outputs: usize,
    pub expected_result: Vec<u64>,
}

pub trait Example {
    fn prove(&self) -> StarkProof;
    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError>;
    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError>;
}

pub fn get_example(
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
) -> Box<dyn Example> {
    Box::new(FibExample::new(sequence_length, options, result))
}
