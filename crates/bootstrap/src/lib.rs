use aes_errors::KernelError;
use aes_config;
use aes_logging;
use aes_registry::Registry;
use aes_event_bus::EventBus;
use aes_scheduler::Scheduler;
use aes_state::{StateManager, KernelState};
use aes_security::SecurityManager;
use aes_health::HealthMonitor;
use aes_metrics::MetricsCollector;
use aes_tracing::Tracer;
use std::time::Instant;

pub struct KernelContext {
    pub config: aes_config::AppConfig,
    pub registry: Registry,
    pub event_bus: EventBus,
    pub scheduler: Scheduler,
    pub state: StateManager,
    pub security: SecurityManager,
    pub health: HealthMonitor,
    pub metrics: MetricsCollector,
    pub tracer: Tracer,
}

impl KernelContext {
    pub fn new() -> Result<Self, KernelError> {
        // 1. Load Configuration
        let config = aes_config::load("configs/development.toml")
            .map_err(|e| KernelError::ConfigLoad(e.to_string()))?;

        // 2. Initialize Logger
        aes_logging::init();

        // 3. Initialize Tracing & Metrics
        let tracer = Tracer::new();
        let metrics = MetricsCollector::new();

        // 4. Initialize Core Subsystems
        let registry = Registry::new();
        let event_bus = EventBus::new();
        let scheduler = Scheduler::new();
        let state = StateManager::new();
        let security = SecurityManager::new(config.security.encryption_enabled);
        let health = HealthMonitor::new();

        Ok(KernelContext {
            config,
            registry,
            event_bus,
            scheduler,
            state,
            security,
            health,
            metrics,
            tracer,
        })
    }

    pub fn boot(&mut self) -> Result<(), KernelError> {
        let start_time = Instant::now();
        self.state.transition(KernelState::Booting);

        println!("AES Kernel v{}", self.config.app.version);
        println!("✓ Configuration Loaded");

        // Load Constitution (Simulated)
        println!("✓ Constitution Loaded");

        // Register Services
        let services = [
            ("Logger", "0.1.0"),
            ("Registry", "0.1.0"),
            ("EventBus", "0.1.0"),
            ("Scheduler", "0.1.0"),
            ("StateManager", "0.1.0"),
            ("Security", "0.1.0"),
            ("Health", "0.1.0"),
        ];

        for (name, version) in services.iter() {
            self.registry.register(name, version);
            println!("✓ {} Ready", name);
        }

        self.state.transition(KernelState::Initializing);
        
        // Simulate initialization logic for all services
        // In a real system, this would call .initialize() on each Service trait impl
        
        self.state.transition(KernelState::Ready);
        
        let boot_time = start_time.elapsed().as_millis();
        self.metrics.record_boot_time(boot_time as u64);

        println!("Kernel Status: READY");
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.state.transition(KernelState::Stopping);
        println!("Shutting down kernel...");
        self.state.transition(KernelState::Stopped);
    }
}
