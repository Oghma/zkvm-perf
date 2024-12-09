#[cfg(feature = "jolt")]
use jolt_sdk::{Jolt, RV32IJoltVM, Serializable};

#[cfg(feature = "jolt")]
use fibonacci::{
    analyze_main as analyze_fibonacci, preprocess_main as preprocess_fibonacci,
    prove_main as prove_fibonacci,
};

use crate::{utils::time_operation, EvalArgs, PerformanceReport, ProgramId};

pub struct JoltEvaluator;

impl JoltEvaluator {
    #[cfg(feature = "jolt")]
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        let (analyze, preprocess, prove) = match args.program {
            ProgramId::Fibonacci => (analyze_fibonacci, preprocess_fibonacci, prove_fibonacci),
            _ => panic!("not implemented yet"),
        };

        // Get the total cycles of the program
        let summary = analyze();
        let instruction_count = summary.analyze::<jolt_sdk::F>();
        let total_cycles = instruction_count.iter().map(|(_, count)| count).sum::<usize>();

        // Generate the program and arithmetization
        let ((program, preprocessing), execution_duration) = time_operation(|| preprocess());

        // Generate the proof
        let ((_, proof), prove_duration) =
            time_operation(|| prove(program.clone(), preprocessing.clone()));

        // Get the proof size
        let proof_size = proof.size().expect("failed to get proof size");

        // Verify the proof
        let (_, verify_duration) = time_operation(|| {
            RV32IJoltVM::verify(preprocessing, proof.proof, proof.commitments, None)
        });

        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: "".to_string(),
            shard_size: 0,
            shards: 0,
            cycles: total_cycles as u64,
            speed: (total_cycles as f64) / prove_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: prove_duration.as_secs_f64(),
            core_verify_duration: verify_duration.as_secs_f64(),
            core_proof_size: proof_size,
            compress_prove_duration: 0.0,
            compress_verify_duration: 0.0,
            compress_proof_size: 0,
        }
    }

    #[cfg(not(feature = "jolt"))]
    pub fn eval(_args: &EvalArgs) -> PerformanceReport {
        panic!("Jolt feature is not enabled. Please compile with --features jolt")
    }
}
