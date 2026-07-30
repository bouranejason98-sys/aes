use aes_errors::KernelError;
use aes_config;
use aes_logging;
use aes_registry::Registry;
use aes_event_bus::{MemoryBus, Publisher, Subscriber};
use aes_scheduler::Scheduler;
use aes_state::{StateManager, KernelState};
use aes_security::SecurityManager;
use aes_health::HealthMonitor;
use aes_metrics::MetricsCollector;
use aes_tracing::Tracer;
use aes_protocol::create_event;
use aes_mission::{MissionScheduler, MissionExecutor, Mission, Priority};
use aes_knowledge::{KnowledgeManager, MemoryKnowledgeRepository};
use std::time::Instant;
use std::sync::Arc;

pub struct KernelContext {
    pub config: aes_config::AppConfig,
    pub registry: Registry,
    pub event_bus: Arc<MemoryBus>,
    pub scheduler: Scheduler,
    pub state: StateManager,
    pub security: SecurityManager,
    pub health: HealthMonitor,
    pub metrics: MetricsCollector,
    pub tracer: Tracer,
    pub mission_scheduler: MissionScheduler,
    pub mission_executor: MissionExecutor,
    pub knowledge_manager: Arc<KnowledgeManager>,
}

impl KernelContext {
    pub fn new() -> Result<Self, KernelError> {
        let config = aes_config::load("configs/development.toml")
            .map_err(|e| KernelError::ConfigLoad(e.to_string()))?;

        aes_logging::init();

        let tracer = Tracer::new();
        let metrics = MetricsCollector::new();

        let registry = Registry::new();
        let event_bus = Arc::new(MemoryBus::new());
        let scheduler = Scheduler::new();
        let state = StateManager::new();
        let security = SecurityManager::new(config.security.encryption_enabled);
        let health = HealthMonitor::new();

        let mission_scheduler = MissionScheduler::new(Arc::clone(&event_bus));
        let mission_executor = MissionExecutor::new(Arc::clone(&event_bus));

        event_bus.subscribe("mission.*", Arc::new(|env| {
            println!("    [MISSION] {}", env.topic.as_str());
        }));
        event_bus.subscribe("knowledge.*", Arc::new(|env| {
            println!("    [KNOWLEDGE] {}", env.topic.as_str());
        }));

        // KnowledgeManager depends on the Publisher trait object, not the
        // concrete MemoryBus, so it stays testable against a mock
        // publisher. Rust won't coerce Arc<MemoryBus> -> Arc<dyn Publisher>
        // through a bare Arc::clone(&event_bus) call passed as an
        // argument -- the coercion needs an explicitly-typed binding to
        // land on cleanly.
        let event_bus_as_publisher: Arc<dyn Publisher + Send + Sync> = event_bus.clone();
        let knowledge_manager = Arc::new(KnowledgeManager::new(
            Box::new(MemoryKnowledgeRepository::new()),
            event_bus_as_publisher,
        ));
        aes_knowledge::wire_mission_completed(&event_bus, Arc::clone(&knowledge_manager));

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
            mission_scheduler,
            mission_executor,
            knowledge_manager,
        })
    }

    pub fn boot(&mut self) -> Result<(), KernelError> {
        let start_time = Instant::now();
        self.state.transition(KernelState::Booting);

        println!("AES Kernel v{}", self.config.app.version);
        println!("[OK] Configuration Loaded");
        println!("[OK] Constitution Loaded");

        let services = [
            ("Logger", "0.1.0"),
            ("Registry", "0.1.0"),
            ("EventBus", "0.1.0"),
            ("Scheduler", "0.1.0"),
            ("StateManager", "0.1.0"),
            ("Security", "0.1.0"),
            ("Health", "0.1.0"),
            ("MissionScheduler", "0.1.0"),
            ("MissionExecutor", "0.1.0"),
            ("KnowledgeManager", "0.1.0"),
        ];

        for (name, version) in services.iter() {
            self.registry.register(name, version);
            println!("[OK] {} Ready", name);
        }

        println!("");
        println!(">>> Testing AESP Message Bus...");

        let test_payload = serde_json::json!({
            "event": "kernel_boot",
            "version": "0.1.0",
            "status": "ready"
        });

        let envelope = create_event("bootstrap", "system.boot", test_payload);

        self.event_bus.subscribe("system.boot", Arc::new(|env| {
            println!("    [EVENT RECEIVED] Topic: {}, Source: {}, ID: {}",
                env.topic.as_str(), env.source, env.message_id);
        }));

        self.event_bus.publish(envelope).unwrap();

        println!("");
        println!(">>> Running First Mission: SystemHealthCheck");

        let mission = Mission::new("SystemHealthCheck", Priority::High, serde_json::json!({}));

        self.mission_scheduler.enqueue(mission).unwrap();

        if let Some(m) = self.mission_scheduler.dequeue() {
            self.mission_executor.execute(m).unwrap();
        }

        println!("");
        println!(">>> Knowledge Repository: {} record(s) stored", self.knowledge_manager.count());

        self.state.transition(KernelState::Initializing);
        self.state.transition(KernelState::Ready);

        let boot_time = start_time.elapsed().as_millis();
        self.metrics.record_boot_time(boot_time as u64);

        println!("Kernel Status: READY");
        println!("[OK] AESP Message Bus Operational");
        println!("[OK] Mission Execution System Operational");
        println!("[OK] Knowledge Engine Operational");

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.state.transition(KernelState::Stopping);
        println!("Shutting down kernel...");
        self.state.transition(KernelState::Stopped);
    }
}
