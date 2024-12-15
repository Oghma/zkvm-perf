#[cfg(feature = "nexus")]
use std::fs;

#[cfg(feature = "nexus")]
use nexus_sdk::{
    nova::seq::{Generate, Nova, PP},
    Prover, Verifiable,
};

#[cfg(feature = "nexus")]
use crate::utils::{get_elf, time_operation};

use crate::{EvalArgs, PerformanceReport};

pub struct NexusEvaluator;

impl NexusEvaluator {
    #[cfg(feature = "nexus")]
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        use crate::ProgramId;

        let elf_path = get_elf(args);
        let elf = fs::read(&elf_path).unwrap();

        let pp: PP = PP::generate().expect("failed to generate parameters");

        // Generate the prover. Arithmetization should be made at this step
        let (prover, execution_duration) =
            time_operation(|| Nova::new(&elf).expect("failed to load program"));

        // Generate the proof.
        let (proof, core_prove_duration) = match args.program {
            ProgramId::Fibonacci => time_operation(|| {
                prover
                    .prove_with_input::<u32>(
                        &pp,
                        &args.fibonacci_input.expect("missing fibonacci input"),
                    )
                    .expect("failed to prove program")
            }),
            _ => time_operation(|| prover.prove(&pp).expect("failed to prove program")),
        };

        // Verify the proof
        let (_, core_verify_duration) =
            time_operation(|| proof.verify(&pp).expect("failed to verify program"));

        let prove_duration = core_prove_duration;

        // Create the performance report
        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: "".to_string(),
            shards: 0,
            shard_size: 0,
            cycles: 0,
            speed: 0.0,
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: core_prove_duration.as_secs_f64(),
            core_verify_duration: core_verify_duration.as_secs_f64(),
            core_proof_size: 0,
            compress_proof_size: 0,
            compress_verify_duration: 0.0,
            compress_prove_duration: 0.0,
        }
    }

    #[cfg(not(feature = "nexus"))]
    pub fn eval(_args: &EvalArgs) -> PerformanceReport {
        panic!("Nexus feature is not enabled. Please compile with --features nexus");
    }
}
