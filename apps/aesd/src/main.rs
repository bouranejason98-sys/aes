use aes_kernel::run_kernel;

fn main() {
    println!("========================================");
    println!("  AES Kernel v0.3.0 - Sprint 3 Build  ");
    println!("========================================\n");
    
    if let Err(e) = run_kernel() {
        eprintln!("Kernel Fatal Error: {}", e);
        std::process::exit(1);
    }
}
