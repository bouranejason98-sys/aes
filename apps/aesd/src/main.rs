use aes_kernel::run_kernel;

fn main() {
    if let Err(e) = run_kernel() {
        eprintln!("Kernel Fatal Error: {}", e);
        std::process::exit(1);
    }
}
