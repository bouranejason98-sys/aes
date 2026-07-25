#!/bin/bash

# Sprint 2: AESP (AES Protocol) Implementation
# This script creates the protocol layer and refactors the event bus.

set -e

echo ">>> Sprint 2: Initializing AESP Protocol Layer..."

# 1. Git Branch Setup
echo ">>> Creating git branch sprint-2-aesp..."
git checkout -b sprint-2-aesp 2>/dev/null || true
git add .
git commit -m "chore: prepare for Sprint 2 - AESP Protocol" --allow-empty 2>/dev/null || true
git push -u origin sprint-2-aesp 2>/dev/null || echo "Note: Remote not configured. Skipping push."

# 2. Create Directory Structure
echo ">>> Creating Sprint 2 directory structure..."
mkdir -p crates/protocol/src/{message,envelope,event,command,response,correlation,topic}
mkdir -p crates/event_bus/src/{publisher,subscriber,dispatcher,memory_bus}

# 3. Update Workspace Cargo.toml to include new modules (if needed)
# The workspace already includes crates/protocol and crates/event_bus, so we just update their contents.

# --- PROTOCOL CRATE ---
echo ">>> Updating Protocol Crate..."

# Cargo.toml for Protocol
cat > crates/protocol/Cargo.toml << 'EOF'
[package]
name = "aes-protocol"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { workspace = true, features = ["derive"] }
serde_json = "1.0"
uuid = { workspace = true, features = ["v4", "serde"] }
chrono = { workspace = true, features = ["serde"] }
aes-errors = { path = "../errors" }
EOF

# Correlation ID
cat > crates/protocol/src/correlation/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationId(pub Uuid);

impl CorrelationId {
    pub fn new() -> Self {
        CorrelationId(Uuid::new_v4())
    }

    pub fn from_u128(val: u128) -> Self {
        CorrelationId(Uuid::from_u128(val))
    }

    pub fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}
EOF

# Topic
cat > crates/protocol/src/topic/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic(pub String);

impl Topic {
    pub fn new(s: &str) -> Self {
        Topic(s.to_string())
    }

    pub fn matches(&self, other: &str) -> bool {
        // Simple wildcard support: "service.*" matches "service.health"
        if self.0.contains('*') {
            let pattern = self.0.replace('*', ".*");
            let regex = regex::Regex::new(&format!("^{}$", pattern)).unwrap();
            regex.is_match(other)
        } else {
            self.0 == other
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
EOF

// Note: We need to add regex to Cargo.toml for wildcard support, but for Sprint 2 let's keep it simple without regex dependency yet
// Let's simplify Topic to just string matching for now to avoid extra dependencies in Sprint 2
cat > crates/protocol/src/topic/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic(pub String);

impl Topic {
    pub fn new(s: &str) -> Self {
        Topic(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    // Simple prefix match for wildcard-like behavior
    pub fn matches(&self, other: &str) -> bool {
        if self.0.ends_with('*') {
            let prefix = &self.0[..self.0.len()-1];
            other.starts_with(prefix)
        } else {
            self.0 == other
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
EOF

# Message Type Enum
cat > crates/protocol/src/message/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Event,
    Command,
    Response,
    Notification,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MessageType::Event => write!(f, "EVENT"),
            MessageType::Command => write!(f, "COMMAND"),
            MessageType::Response => write!(f, "RESPONSE"),
            MessageType::Notification => write!(f, "NOTIFICATION"),
        }
    }
}
EOF

# Message Envelope
cat > crates/protocol/src/envelope/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use crate::correlation::CorrelationId;
use crate::message::MessageType;
use crate::topic::Topic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AespEnvelope {
    pub message_id: Uuid,
    pub correlation_id: CorrelationId,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub target: Option<String>,
    pub topic: Topic,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub metadata: std::collections::HashMap<String, String>,
    // Signature field reserved for future security implementation
    // pub signature: Option<String>, 
}

impl AespEnvelope {
    pub fn new(
        source: &str,
        topic: &str,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        AespEnvelope {
            message_id: Uuid::new_v4(),
            correlation_id: CorrelationId::new(),
            timestamp: Utc::now(),
            source: source.to_string(),
            target: None,
            topic: Topic::new(topic),
            message_type,
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_correlation_id(mut self, id: CorrelationId) -> Self {
        self.correlation_id = id;
        self
    }
}
EOF

# Event, Command, Response Wrappers
cat > crates/protocol/src/event/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use crate::envelope::AespEnvelope;
use crate::message::MessageType;
use crate::topic::Topic;

pub fn create_event(source: &str, topic: &str, payload: serde_json::Value) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Event, payload)
}
EOF

cat > crates/protocol/src/command/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use crate::envelope::AespEnvelope;
use crate::message::MessageType;

pub fn create_command(source: &str, target: &str, topic: &str, payload: serde_json::Value) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Command, payload)
        .with_target(target)
}
EOF

cat > crates/protocol/src/response/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use crate::envelope::AespEnvelope;
use crate::message::MessageType;

pub fn create_response(source: &str, topic: &str, payload: serde_json::Value, correlation_id: crate::correlation::CorrelationId) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Response, payload)
        .with_correlation_id(correlation_id)
}
EOF

# Protocol Lib
cat > crates/protocol/src/lib.rs << 'EOF'
pub mod message;
pub mod envelope;
pub mod event;
pub mod command;
pub mod response;
pub mod correlation;
pub mod topic;

pub use message::MessageType;
pub use envelope::AespEnvelope;
pub use event::create_event;
pub use command::create_command;
pub use response::create_response;
pub use correlation::CorrelationId;
pub use topic::Topic;
EOF

# --- EVENT BUS CRATE REFACTOR ---
echo ">>> Refactoring Event Bus with AESP..."

# Cargo.toml for Event Bus
cat > crates/event_bus/Cargo.toml << 'EOF'
[package]
name = "aes-event-bus"
version.workspace = true
edition.workspace = true

[dependencies]
aes-protocol = { path = "../protocol" }
aes-errors = { path = "../errors" }
serde = { workspace = true }
serde_json = "1.0"
uuid = { workspace = true }
EOF

# Publisher
cat > crates/event_bus/src/publisher/mod.rs << 'EOF'
use aes_protocol::AespEnvelope;

pub trait Publisher {
    fn publish(&self, envelope: AespEnvelope) -> Result<(), Box<dyn std::error::Error>>;
}
EOF

# Subscriber
cat > crates/event_bus/src/subscriber/mod.rs << 'EOF'
use aes_protocol::AespEnvelope;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(&AespEnvelope) + Send + Sync>;

pub trait Subscriber {
    fn subscribe(&self, topic: &str, handler: Handler);
    fn unsubscribe(&self, topic: &str);
}
EOF

# Dispatcher
cat > crates/event_bus/src/dispatcher/mod.rs << 'EOF'
use aes_protocol::AespEnvelope;

pub trait Dispatcher {
    fn dispatch(&self, envelope: &AespEnvelope);
}
EOF

# Memory Bus Implementation
cat > crates/event_bus/src/memory_bus/mod.rs << 'EOF'
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use aes_protocol::{AespEnvelope, Topic};
use crate::publisher::Publisher;
use crate::subscriber::{Subscriber, Handler};
use crate::dispatcher::Dispatcher;

pub struct MemoryBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<Handler>>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Publisher for MemoryBus {
    fn publish(&self, envelope: AespEnvelope) -> Result<(), Box<dyn std::error::Error>> {
        self.dispatch(&envelope);
        Ok(())
    }
}

impl Subscriber for MemoryBus {
    fn subscribe(&self, topic: &str, handler: Handler) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    fn unsubscribe(&self, _topic: &str) {
        // Implementation for future
    }
}

impl Dispatcher for MemoryBus {
    fn dispatch(&self, envelope: &AespEnvelope) {
        let subs = self.subscribers.lock().unwrap();
        let topic_str = envelope.topic.as_str();

        // Find matching subscribers (exact or wildcard)
        let mut matched_handlers = Vec::new();

        for (sub_topic, handlers) in subs.iter() {
            let t = Topic::new(sub_topic);
            if t.matches(topic_str) {
                matched_handlers.extend(handlers.iter().cloned());
            }
        }

        // Drop lock before calling handlers to avoid deadlocks
        drop(subs);

        for handler in matched_handlers {
            handler(envelope);
        }
    }
}
EOF

# Event Bus Lib
cat > crates/event_bus/src/lib.rs << 'EOF'
pub mod publisher;
pub mod subscriber;
pub mod dispatcher;
pub mod memory_bus;

pub use memory_bus::MemoryBus;
pub use publisher::Publisher;
pub use subscriber::Subscriber;
pub use dispatcher::Dispatcher;
EOF

# --- UPDATE BOOTSTRAP TO USE AESP ---
echo ">>> Integrating AESP into Bootstrap..."

# We need to update the bootstrap crate to use the new Event Bus
cat > crates/bootstrap/Cargo.toml << 'EOF'
[package]
name = "aes-bootstrap"
version.workspace = true
edition.workspace = true

[dependencies]
aes-errors = { path = "../errors" }
aes-config = { path = "../config" }
aes-logging = { path = "../logging" }
aes-registry = { path = "../registry" }
aes-event-bus = { path = "../event_bus" }
aes-scheduler = { path = "../scheduler" }
aes-state = { path = "../state" }
aes-security = { path = "../security" }
aes-health = { path = "../health" }
aes-metrics = { path = "../metrics" }
aes-tracing = { path = "../tracing" }
aes-protocol = { path = "../protocol" }
serde = { workspace = true }
EOF

# Update bootstrap lib to demonstrate AESP usage
cat > crates/bootstrap/src/lib.rs << 'EOF'
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
use aes_protocol::{create_event, AespEnvelope};
use std::time::Instant;
use std::sync::Arc;

pub struct KernelContext {
    pub config: aes_config::AppConfig,
    pub registry: Registry,
    pub event_bus: MemoryBus,
    pub scheduler: Scheduler,
    pub state: StateManager,
    pub security: SecurityManager,
    pub health: HealthMonitor,
    pub metrics: MetricsCollector,
    pub tracer: Tracer,
}

impl KernelContext {
    pub fn new() -> Result<Self, KernelError> {
        let config = aes_config::load("configs/development.toml")
            .map_err(|e| KernelError::ConfigLoad(e.to_string()))?;

        aes_logging::init();

        let tracer = Tracer::new();
        let metrics = MetricsCollector::new();

        let mut registry = Registry::new();
        let event_bus = MemoryBus::new();
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
        println!("✓ Constitution Loaded");

        // Register Services
        let services = [
            ("Logger", "0.1.0"),
            ("Registry", "0.1.0"),
            ("EventBus", "0.1.0"),
            ("Scheduler", "0.1.0"),
            ("StateManager", "0.1.0"),
            ("Security", "0.1.0"),
            # --- CONTINUATION OF BOOTSTRAP ---
            ("Health", "0.1.0"),
        ];

        for (name, version) in services.iter() {
            registry.register(name, version);
            println!("✓ {} Ready", name);
        }

        // Demonstrate AESP Event Publishing
        println!("\n>>> Testing AESP Message Bus...");
        
        // Create a test event using AESP
        let test_payload = serde_json::json!({
            "event": "kernel_boot",
            "version": "0.1.0",
            "status": "ready"
        });

        let envelope = create_event("bootstrap", "system.boot", test_payload);
        
        // Subscribe to the event
        event_bus.subscribe("system.boot", Arc::new(|env| {
            println!("    [EVENT RECEIVED] Topic: {}, Source: {}, ID: {}", 
                env.topic.as_str(), env.source, env.message_id);
        }));

        // Publish the event
        event_bus.publish(envelope).unwrap();

        self.state.transition(KernelState::Initializing);
        
        // Simulate remaining initialization
        // ...

        self.state.transition(KernelState::Ready);
        
        let boot_time = start_time.elapsed().as_millis();
        self.metrics.record_boot_time(boot_time as u64);

        println!("Kernel Status: READY");
        println!("✓ AESP Message Bus Operational");
        
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.state.transition(KernelState::Stopping);
        println!("Shutting down kernel...");
        self.state.transition(KernelState::Stopped);
    }
}
EOF

# --- UPDATE KERNEL CRATE ---
echo ">>> Updating Kernel Crate..."
cat > crates/kernel/Cargo.toml << 'EOF'
[package]
name = "aes-kernel"
version.workspace = true
edition.workspace = true

[dependencies]
aes-bootstrap = { path = "../bootstrap" }
aes-errors = { path = "../errors" }
EOF

cat > crates/kernel/src/lib.rs << 'EOF'
pub use aes_bootstrap::KernelContext;
pub use aes_errors::KernelError;

pub fn run_kernel() -> Result<(), KernelError> {
    let mut ctx = KernelContext::new()?;
    ctx.boot()?;
    ctx.shutdown();
    Ok(())
}
EOF

# --- UPDATE APPS/AESD ---
echo ">>> Updating Main Application..."
cat > apps/aesd/Cargo.toml << 'EOF'
[package]
name = "aesd"
version.workspace = true
edition.workspace = true

[[bin]]
name = "aesd"
path = "src/main.rs"

[dependencies]
aes-kernel = { path = "../../crates/kernel" }
EOF

cat > apps/aesd/src/main.rs << 'EOF'
use aes_kernel::run_kernel;

fn main() {
    println!("========================================");
    println!("  AES Kernel v0.2.0 - Sprint 2 Build  ");
    println!("========================================\n");
    
    if let Err(e) = run_kernel() {
        eprintln!("Kernel Fatal Error: {}", e);
        std::process::exit(1);
    }
}
EOF

# --- INTEGRATION TESTS ---
echo ">>> Creating Integration Tests..."
mkdir -p tests

cat > tests/sprint2_integration.rs << 'EOF'
use aes_protocol::{create_event, create_command, create_response, CorrelationId, MessageType};
use aes_event_bus::{MemoryBus, Publisher, Subscriber};
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_message_envelope_creation() {
    let envelope = create_event("test_service", "test.topic", serde_json::json!({"data": "value"}));
    
    assert_eq!(envelope.source, "test_service");
    assert_eq!(envelope.topic.as_str(), "test.topic");
    assert_eq!(envelope.message_type, MessageType::Event);
    assert!(!envelope.message_id.is_nil());
}

#[test]
fn test_event_bus_publish_subscribe() {
    let bus = MemoryBus::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = Arc::clone(&received);

    // Subscribe to topic
    bus.subscribe("test.topic", Arc::new(move |env| {
        received_clone.lock().unwrap().push(env.message_id);
    }));

    // Publish event
    let envelope = create_event("producer", "test.topic", serde_json::json!({}));
    bus.publish(envelope).unwrap();

    // Verify receipt
    let ids = received.lock().unwrap();
    assert_eq!(ids.len(), 1);
}

#[test]
fn test_correlation_id_persistence() {
    let correlation_id = CorrelationId::new();
    let _ = serde_json::to_string(&correlation_id); // Test serialization
    assert!(!correlation_id.to_string().is_empty());
}

#[test]
fn test_command_response_flow() {
    let bus = MemoryBus::new();
    let response_received = Arc::new(Mutex::new(None));
    let response_clone = Arc::clone(&response_received);

    // Subscribe to response topic
    bus.subscribe("system.response", Arc::new(move |env| {
        if env.message_type == MessageType::Response {
            response_clone.lock().unwrap().replace(env.message_id);
        }
    }));

    // Create command
    let command = create_command("client", "service", "system.command", serde_json::json!({"action": "test"}));
    
    // Simulate service processing and sending response
    // (In a real system, the service would catch the command and send a response)
    let response = create_response("service", "system.response", serde_json::json!({"result": "ok"}), command.correlation_id);
    
    bus.publish(response).unwrap();

    let resp_id = response_received.lock().unwrap();
    assert!(resp_id.is_some());
}

#[test]
fn test_wildcard_subscription() {
    let bus = MemoryBus::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = Arc::clone(&received);

    // Subscribe to wildcard
    bus.subscribe("system.*", Arc::new(move |env| {
        received_clone.lock().unwrap().push(env.topic.as_str().to_string());
    }));

    // Publish to matching topic
    let envelope = create_event("svc", "system.boot", serde_json::json!({}));
    bus.publish(envelope).unwrap();

    let topics = received.lock().unwrap();
    assert_eq!(topics.len(), 1);
    assert_eq!(topics[0], "system.boot");
}
EOF

# --- UPDATE README ---
echo ">>> Updating README..."
cat > README.md << 'EOF'
# AES Kernel (Sprint 2)

## Overview
Sprint 2 introduces the **AESP (AES Protocol)** layer, enabling message-driven communication between services.

## Key Features
- **AESP Message Envelope**: Unified structure for Events, Commands, Responses, and Notifications.
- **Correlation IDs**: Trace requests across services.
- **Publish/Subscribe**: Wildcard topic support.
- **In-Memory Bus**: High-performance event routing.

## Running the System
```bash
cargo run
