use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};

const PACKAGE: &str = "guest";

// Demo trades in half-R units:
// +1.5R, -1.0R, +2.0R, -0.5R, +1.0R, 0R, -1.0R, +1.5R
// Nibble encoding: half_r + 8
// Low nibble first: B, 6, C, 7, A, 8, 6, B
const ENCODED_TRADES: u32 = 0xB68A7C6B;

// Net = +3.5R = 7 half-R units.
// Public score = 128 + 7 = 135.
const EXPECTED_SCORE: u32 = 135;

fn main() {
    println!("Compiling Nexus zkVM guest program...");

    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");
    let elf = prover.elf.clone();

    println!("Proving trading journal summary...");

    let (view, proof) = prover
        .prove_with_input::<u32, u32>(&ENCODED_TRADES, &EXPECTED_SCORE)
        .expect("failed to prove program");

    assert_eq!(
        view.exit_code().expect("failed to retrieve exit code"),
        nexus_sdk::KnownExitCodes::EXIT_SUCCESS as u32
    );

    let output: u32 = view
        .public_output::<u32>()
        .expect("failed to retrieve public output");

    assert_eq!(output, EXPECTED_SCORE);

    println!("Public output score: {}", output);
    println!(
        ">>>>> Guest logs\n{}<<<<<",
        view.logs()
            .expect("failed to retrieve debug logs")
            .join("")
    );

    println!("Verifying proof...");

    proof
        .verify_expected::<u32, u32>(
            &EXPECTED_SCORE,
            nexus_sdk::KnownExitCodes::EXIT_SUCCESS as u32,
            &EXPECTED_SCORE,
            &elf,
            &[],
        )
        .expect("failed to verify proof");

    println!("Succeeded: trading journal computation was proven and verified.");
}
