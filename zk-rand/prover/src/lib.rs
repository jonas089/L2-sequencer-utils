use rand::Rng;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use zk_logic::{
    random_bytes_to_int,
    types::{CircuitInputs, CircuitOutputs},
};
use zk_methods::{ZK_RAND_ELF, ZK_RAND_ID};

pub fn generate_random_number(public_key: Vec<u8>, nonce: Vec<u8>) -> Receipt {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen();
    let seed: Vec<u8> = random_float.to_be_bytes().to_vec();
    let input: CircuitInputs = CircuitInputs {
        public_key,
        // some int .to_bytes()
        nonce,
        // something of sufficient randomness
        seed,
    };
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let receipt = prover.prove(env, ZK_RAND_ELF).unwrap();
    let output: CircuitOutputs = receipt.journal.decode().unwrap();
    println!(
        "ZK random number: {:?}",
        random_bytes_to_int(&output.random_bytes)
    );
    receipt.verify(ZK_RAND_ID).unwrap();
    receipt
}