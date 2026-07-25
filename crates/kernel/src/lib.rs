pub use aes_bootstrap::KernelContext;
pub use aes_errors::KernelError;

pub fn run_kernel() -> Result<(), KernelError> {
    let mut ctx = KernelContext::new()?;
    ctx.boot()?;
    
    // In a real system, we would enter the main event loop here
    // For Sprint 1, we just demonstrate the READY state and exit gracefully
    ctx.shutdown();
    
    Ok(())
}
