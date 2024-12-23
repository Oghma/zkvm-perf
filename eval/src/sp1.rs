#[cfg(feature = "sp1")]
use std::fs;

#[cfg(feature = "sp1")]
use crate::{
    utils::{get_elf, /* get_reth_input, */ time_operation},
    ProgramId,
};

#[cfg(feature = "cuda")]
use sp1_cuda::SP1CudaProver;

#[cfg(feature = "sp1")]
use sp1_prover::{components::DefaultProverComponents, utils::get_cycles};
#[cfg(feature = "sp1")]
use sp1_sdk::{provers::ProofOpts, utils::setup_logger, SP1Context, SP1Prover, SP1Stdin};

use crate::{EvalArgs, PerformanceReport};

pub struct SP1Evaluator;

impl SP1Evaluator {
    #[cfg(feature = "sp1")]
    pub fn eval(args: &EvalArgs) -> PerformanceReport {
        // Setup the logger.
        setup_logger();

        // Set enviroment variables to configure the prover.
        std::env::set_var("SHARD_SIZE", format!("{}", 1 << args.shard_size));
        if args.program == ProgramId::Reth {
            std::env::set_var("SHARD_CHUNKING_MULTIPLIER", "4");
        }

        // Get stdin.
        let stdin = match args.program {
            ProgramId::Reth => panic!("reth currently disabled"),
            // let input = get_reth_input(args);
            // let mut stdin = SP1Stdin::new();
            // stdin.write(&input);
            // stdin
            ProgramId::Fibonacci => {
                let mut stdin = SP1Stdin::new();
                stdin.write(&args.fibonacci_input.expect("missing fibonacci input"));
                stdin
            }
            _ => SP1Stdin::new(),
        };

        // Get the elf.
        let elf_path = get_elf(args);
        let elf = fs::read(elf_path).unwrap();
        let cycles = get_cycles(&elf, &stdin);

        let prover = SP1Prover::<DefaultProverComponents>::new();

        #[cfg(feature = "cuda")]
        let server = SP1CudaProver::new().expect("Failed to initialize CUDA prover");

        // Setup the program.
        let (pk, vk) = prover.setup(&elf);

        // Execute the program.
        let context = SP1Context::default();
        let (_, execution_duration) =
            time_operation(|| prover.execute(&elf, &stdin, context.clone()));

        // Setup the prover opionts.
        #[cfg(not(feature = "cuda"))]
        let opts = ProofOpts::default().sp1_prover_opts;

        // Generate the core proof (CPU).
        #[cfg(not(feature = "cuda"))]
        let (core_proof, prove_core_duration) =
            time_operation(|| prover.prove_core(&pk, &stdin, opts, context).unwrap());

        // Generate the core proof (CUDA).
        #[cfg(feature = "cuda")]
        let (core_proof, prove_core_duration) =
            time_operation(|| server.prove_core(&pk, &stdin).unwrap());

        let num_shards = core_proof.proof.0.len();

        // Verify the proof.
        let core_bytes = bincode::serialize(&core_proof).unwrap();
        let (_, verify_core_duration) = time_operation(|| {
            prover.verify(&core_proof.proof, &vk).expect("Proof verification failed")
        });

        #[cfg(not(feature = "cuda"))]
        let (compress_proof, compress_duration) =
            time_operation(|| prover.compress(&vk, core_proof, vec![], opts).unwrap());

        #[cfg(feature = "cuda")]
        let (compress_proof, compress_duration) =
            time_operation(|| server.compress(&vk, core_proof, vec![]).unwrap());

        let compress_bytes = bincode::serialize(&compress_proof).unwrap();
        println!("recursive proof size: {}", compress_bytes.len());

        let prove_duration = prove_core_duration + compress_duration;

        // Create the performance report.
        PerformanceReport {
            program: args.program.to_string(),
            prover: args.prover.to_string(),
            hashfn: args.hashfn.to_string(),
            shard_size: args.shard_size,
            shards: num_shards,
            cycles: cycles as u64,
            speed: (cycles as f64) / prove_core_duration.as_secs_f64(),
            execution_duration: execution_duration.as_secs_f64(),
            prove_duration: prove_duration.as_secs_f64(),
            core_prove_duration: prove_core_duration.as_secs_f64(),
            core_verify_duration: verify_core_duration.as_secs_f64(),
            core_proof_size: core_bytes.len(),
            compress_prove_duration: compress_duration.as_secs_f64(),
            compress_verify_duration: 0.0, // TODO: fill this in.
            compress_proof_size: compress_bytes.len(),
        }
    }

    #[cfg(not(feature = "sp1"))]
    pub fn eval(_args: &EvalArgs) -> PerformanceReport {
        panic!("SP1 feature is not enabled. Please compile with --features sp1");
    }
}
