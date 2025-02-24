use host_utils::ProofMode::{Fake, Groth16};
use host_utils::Prover as Risc0Prover;
use risc0_zkvm::ExecutorEnv;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let mode = if std::env::var("RISC0_DEV_MODE") == Ok("1".to_string()) {
        Fake
    } else {
        Groth16
    };
    let env = ExecutorEnv::builder().build()?;
    Risc0Prover::try_new(mode)?.prove(env, guest_wrapper::RISC0_CALL_GUEST_ELF)?;

    Ok(())
}
