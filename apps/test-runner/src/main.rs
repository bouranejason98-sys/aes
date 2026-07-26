// Sprint 2 Final Acceptance Tests
// Runs as a binary to prove Event Bus functionality

use aes_protocol::{create_event, CorrelationId};
use aes_event_bus::{MemoryBus, Publisher, Subscriber};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("========================================");
    println!("  Sprint 2 Final Acceptance Tests     ");
    println!("========================================\n");

    let mut passed = 0;
    let mut failed = 0;

    // Test 1
    if run_test("Test 1: One Publisher, Three Subscribers", || {
        let bus = MemoryBus::new();
        let a = Arc::new(Mutex::new(0));
        let b = Arc::new(Mutex::new(0));
        let c = Arc::new(Mutex::new(0));
        let aa = Arc::clone(&a);
        let bb = Arc::clone(&b);
        let cc = Arc::clone(&c);

        bus.subscribe("sys.t", Arc::new(move |_| *aa.lock().unwrap() += 1));
        bus.subscribe("sys.t", Arc::new(move |_| *bb.lock().unwrap() += 1));
        bus.subscribe("sys.t", Arc::new(move |_| *cc.lock().unwrap() += 1));

        bus.publish(create_event("p", "sys.t", serde_json::json!({}))).unwrap();

        assert_eq!(*a.lock().unwrap(), 1);
        assert_eq!(*b.lock().unwrap(), 1);
        assert_eq!(*c.lock().unwrap(), 1);
        Ok(())
    }) { passed += 1; } else { failed += 1; }

    // Test 2: Crash Isolation (Fixed)
    if run_test("Test 2: Subscriber Crash Isolation", || {
        let bus = MemoryBus::new();
        // Use a separate atomic counter to avoid mutex poisoning issues
        let good_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let gg = Arc::clone(&good_count);

        // Subscriber A: Panics
        bus.subscribe("crash", Arc::new(|_| {
            panic!("Subscriber A crashed intentionally");
        }));

        // Subscriber B: Healthy
        bus.subscribe("crash", Arc::new(move |_| {
            gg.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }));

        // Run publish in a separate thread to catch the panic from Subscriber A
        let handle = thread::spawn(move || {
            let _ = bus.publish(create_event("p", "crash", serde_json::json!({})));
        });

        // Wait for the thread to finish (it will panic due to Subscriber A)
        // We ignore the panic result because we expect it
        let _ = handle.join();

        // Verify Subscriber B still ran
        let count = good_count.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(count, 1, "Subscriber B did not receive the event after A crashed");
        
        Ok(())
    }) { passed += 1; } else { failed += 1; }

    // Test 3
    if run_test("Test 3: Unknown Topic Graceful", || {
        let bus = MemoryBus::new();
        bus.subscribe("known", Arc::new(|_| {}));
        let res = bus.publish(create_event("p", "unknown", serde_json::json!({})));
        assert!(res.is_ok());
        Ok(())
    }) { passed += 1; } else { failed += 1; }

    // Test 4
    if run_test("Test 4: 1000 Events Stress", || {
        let bus = MemoryBus::new();
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let cc = Arc::clone(&count);
        bus.subscribe("stress.*", Arc::new(move |_| {
            cc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }));

        for i in 0..1000 {
            bus.publish(create_event("p", &format!("stress.{}", i % 10), serde_json::json!({}))).unwrap();
        }
        let final_count = count.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(final_count, 1000);
        Ok(())
    }) { passed += 1; } else { failed += 1; }

    // Test 5
    if run_test("Test 5: Correlation ID Consistency", || {
        let bus = MemoryBus::new();
        let cid = CorrelationId::new();
        let rc = Arc::new(Mutex::new(None));
        let rr = Arc::clone(&rc);

        let mut env = create_event("c", "req", serde_json::json!({}));
        env = env.with_correlation_id(cid.clone());

        bus.subscribe("req", Arc::new(move |e| {
            *rr.lock().unwrap() = Some(e.correlation_id.clone());
        }));
        bus.publish(env).unwrap();

        let res = rc.lock().unwrap();
        assert!(res.is_some());
        assert_eq!(res.as_ref().unwrap().to_string(), cid.to_string());
        Ok(())
    }) { passed += 1; } else { failed += 1; }

    println!("\n========================================");
    println!("  Results: {} Passed, {} Failed", passed, failed);
    println!("========================================");

    if failed > 0 {
        std::process::exit(1);
    }
}

fn run_test(name: &str, test_fn: fn() -> Result<(), Box<dyn std::error::Error>>) -> bool {
    print!("Running {}... ", name);
    match test_fn() {
        Ok(()) => {
            println!("✓ PASS");
            true
        }
        Err(e) => {
            println!("✗ FAIL: {}", e);
            false
        }
    }
}
