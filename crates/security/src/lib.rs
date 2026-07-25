pub struct SecurityManager {
    enabled: bool,
}

impl SecurityManager {
    pub fn new(enabled: bool) -> Self {
        SecurityManager { enabled }
    }

    pub fn verify_boot(&self) -> bool {
        true
    }
}
