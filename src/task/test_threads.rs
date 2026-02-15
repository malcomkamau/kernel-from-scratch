use crate::println;

/// Test kernel thread A - counts and prints periodically
pub extern "C" fn thread_a() {
    let mut counter = 0u64;
    loop {
        counter = counter.wrapping_add(1);
        if counter % 1_000_000 == 0 {
            println!("Thread A: {}", counter);
        }
    }
}

/// Test kernel thread B - counts and prints periodically
pub extern "C" fn thread_b() {
    let mut counter = 0u64;
    loop {
        counter = counter.wrapping_add(1);
        if counter % 1_500_000 == 0 {
            println!("Thread B: {}", counter);
        }
    }
}

/// Test kernel thread C - counts and prints periodically
pub extern "C" fn thread_c() {
    let mut counter = 0u64;
    loop {
        counter = counter.wrapping_add(1);
        if counter % 2_000_000 == 0 {
            println!("Thread C: {}", counter);
        }
    }
}
