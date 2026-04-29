#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

// Public input: expected_score
// Private input: encoded_trades
//
// Encoding rule:
// - Each trade is stored in one 4-bit nibble.
// - nibble 8 means 0 half-R.
// - nibble 9 means +0.5R.
// - nibble 10 means +1.0R.
// - nibble 7 means -0.5R.
// - nibble 6 means -1.0R.
//
// This keeps the guest program tiny and no_std-friendly.
#[nexus_rt::main]
#[nexus_rt::public_input(expected_score)]
fn main(expected_score: u32, encoded_trades: u32) -> u32 {
    let mut net_half_r: i32 = 0;
    let mut wins: u32 = 0;
    let mut losses: u32 = 0;

    for i in 0..8 {
        let nibble = ((encoded_trades >> (i * 4)) & 0xF) as i32;
        let half_r = nibble - 8;

        if half_r > 0 {
            wins += 1;
        } else if half_r < 0 {
            losses += 1;
        }

        net_half_r += half_r;
    }

    let score = (net_half_r + 128) as u32;

    println!("Net half-R units: {}", net_half_r);
    println!("Wins: {}", wins);
    println!("Losses: {}", losses);
    println!("Computed score: {}", score);

    if score == expected_score {
        score
    } else {
        0
    }
}
