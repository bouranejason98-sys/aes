// Sprint 2 Final Acceptance Tests
// Validates: 1 Pub/3 Sub, Crash Isolation, Unknown Topics, 1000 Events, Correlation IDs

use aes_protocol::{create_event, CorrelationId};
use aes_event_bus::{MemoryBus, Publisher, Subscriber};
use std::sync::{Arc, Mutex};
use std::thread;

// --- Test 1: One Publisher, Three Subscribers ---
#[test]
fn test_one_publisher_three_subscribers() {
    println!("\n>>> Running Test 1: One Publisher, Three Subscribers");
    
    let bus = MemoryBus::new();
    
    let received_a = Arc::new(Mutex::new(Vec::new()));
    let received_b = Arc::new(Mutex::new(Vec::new()));
    let received_c = Arc::new(Mutex::new(Vec::new()));
    
    let recv_a_clone = Arc::clone(&received_a);
    let recv_b_clone = Arc::clone(&received_b);
    let recv_c_clone = Arc::clone(&received_c);

    bus.subscribe("system.test", Arc::new(move |env| {
        recv_a_clone.lock().unwrap().push(env.message_id);
    }));
    bus.subscribe("system.test", Arc::new(move |env| {
        recv_b_clone.lock().unwrap().push(env.message_id);
    }));
    bus.subscribe("system.test", Arc::new(move |env| {
        recv_c_clone.lock().unwrap().push(env.message_id);
    }));

    let envelope = create_event("publisher", "system.test", serde_json::json!({"data": "test"}));
    bus.publish(envelope).unwrap();

    let a = received_a.lock().unwrap();
    let b = received_b.lock().unwrap();
    let c = received_c.lock().unwrap();

    assert_eq!(a.len(), 1);
    assert_eq!(b.len(), 1);
    assert_eq!(c.len(), 1);
    assert_eq!(a[0], b[0]);
    assert_eq!(b[0], c[0]);

    println!("    ✓ All 3 subscribers received the SAME event.");
}

// --- Test 2: Subscriber Crash Isolation ---
#[test]
fn test_subscriber_crash_isolation() {
    println!("\n>>> Running Test 2: Subscriber Crash Isolation");
    
    let bus = MemoryBus::new();
    let received_good = Arc::new(Mutex::new(Vec::new()));
    let recv_good_clone = Arc::clone(&received_good);

    // Subscriber A: Panics
    bus.subscribe("system.crisis", Arc::new(move |_env| {
        panic!("Subscriber A crashed!");
    }));

    // Subscriber B: Healthy
    bus.subscribe("system.crisis", Arc::new(move |env| {
        recv_good_clone.lock().unwrap().push(env.message_id);
    }));

    let envelope = create_event("publisher", "system.crisis", serde_json::json!({}));
    
    // Run in thread to catch panic
    let handle = thread::spawn(move || {
        bus.publish(envelope).unwrap();
    });
    
    let _ = handle.join(); // Wait for panic

    let good = received_good.lock().unwrap();
    assert_eq!(good.len(), 1, "Subscriber B failed after A crashed");

    println!("    ✓ Bus remained operational despite Subscriber A crash.");
}

// --- Test 3: Unknown Topic Handling ---
#[test]
fn test_unknown_topic_graceful() {
    println!("\n>>> Running Test 3: Unknown Topic Graceful Handling");
    
    let bus = MemoryBus::new();
    bus.subscribe("system.known", Arc::new(|_env| {}));

    let envelope = create_event("publisher", "system.unknown", serde_json::json!({}));
    let result = bus.publish(envelope);
    
    assert!(result.is_ok());
    println!("    ✓ No panic on unknown topic; event dropped gracefully.");
}

// --- Test 4: 1000 Events Stress Test ---
#[test]
fn test_1000_events_stress() {
    println!("\n>>> Running Test 4: 1000 Events Stress Test");
    
    let bus = MemoryBus::new();
    let received_count = Arc::new(Mutex::new(0));
    let recv_clone = Arc::clone(&received_count);

    bus.subscribe("stress.*", Arc::new(move |_env| {
        *recv_clone.lock().unwrap() += 1;
    }));

    let start = std::time::Instant::now();
    for i in 0..1000 {
        let payload = serde_json::json!({"index": i});
        let envelope = create_event("stress_publisher", &format!("stress.topic.{}", i % 10), payload);
        bus.publish(envelope).unwrap();
    }

    let duration = start.elapsed();
    let count = *received_count.lock().unwrap();

    assert_eq!(count, 1000, "Message loss: expected 1000, got {}", count);
    println!("    ✓ 1000 events processed in {:?}. No message loss.", duration);
}

// --- Test 5: Correlation ID Consistency ---
#[test]
fn test_correlation_id_consistency() {
    println!("\n>>> Running Test 5: Correlation ID Consistency");
    
    let bus = MemoryBus::new();
    let original_cid = CorrelationId::new();
    let received_cid = Arc::new(Mutex::new(None));
    let recv_clone = Arc::clone(&received_cid);

    let payload = serde_json::json!({"action": "check"});
    let mut envelope = create_event("client", "system.request", payload);
    envelope = envelope.with_correlation_id(original_cid.clone());

    bus.subscribe("system.request", Arc::new(move |env| {
        *recv_clone.lock().unwrap() = Some(env.correlation_id.clone());
    }));

    bus.publish(envelope).unwrap();

    let captured_cid = received_cid.lock().unwrap();
    
    assert!(captured_cid.is_some());
    assert_eq!(captured_cid.as_ref().unwrap().to_string(), original_cid.to_string());

    println!("    ✓ Correlation ID preserved end-to-end.");
}
