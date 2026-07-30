use aes_kernel::run_kernel;

fn main() {
    println!("========================================");
    println!("  AES Kernel v0.4.0 - Sprint 4 Build  ");
    println!("========================================");
    println!("");

    if let Err(e) = run_kernel() {
        eprintln!("Kernel Fatal Error: {}", e);
        std::process::exit(1);
    }
}
