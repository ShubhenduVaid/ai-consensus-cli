use std::io::{self, Write};

pub fn show_progress_start(count: usize) {
    print!("🤖 Running {} solver(s)... ", count);
    io::stdout().flush().unwrap();
}

pub fn show_success() {
    print!("✅");
    io::stdout().flush().unwrap();
}

pub fn show_failure() {
    print!("❌");
    io::stdout().flush().unwrap();
}

pub fn show_timing(seconds: f32) {
    println!(" ({:.1}s)", seconds);
}

pub fn show_consensus_start() {
    print!("🧠 Getting consensus... ");
    io::stdout().flush().unwrap();
}

pub fn show_consensus_complete(seconds: f32) {
    println!("✅ ({:.1}s)\n", seconds);
}
