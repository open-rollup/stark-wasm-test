use log::debug;
use std::io::Write;

mod fib_miden_run;
mod fib_winter_run;

use fib_miden_run::run_miden;
use fib_winter_run::run_winter;

use winterfell::math::{fields::f64::BaseElement, FieldElement, StarkField};

pub fn compute_fib_term(n: usize) -> BaseElement {
    let mut t0 = BaseElement::ONE;
    let mut t1 = BaseElement::ONE;

    for _ in 0..(n - 1) {
        t1 = t0 + t1;
        core::mem::swap(&mut t0, &mut t1);
    }

    t1
}

fn main() {
    // configure logging
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log::LevelFilter::Debug)
        .init();

    let sequence_length = 2_usize.pow(16);
    let result = compute_fib_term(sequence_length);

    debug!("\n");
    debug!("============================================================");
    debug!(
        "Fibonacci sequence up to {}th term: {}",
        sequence_length, result
    );

    debug!("\n\ntest fib_winter >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n");
    run_winter(sequence_length, result);

    debug!("\n\ntest fib_miden >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n");
    run_miden(sequence_length, result.as_int().into());
}
