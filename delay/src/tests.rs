#![cfg(test)]
use crate::{Delay, Waiter};
use std::time::{Duration, Instant};

#[test]
fn throttle_works() {
    let start = Instant::now();

    let mut waiter = Delay::throttle(Duration::from_millis(50));
    waiter.start();
    waiter.wait().unwrap();

    assert!(Instant::now().duration_since(start).as_millis() >= 50);
}

#[test]
fn timeout_works() {
    let mut waiter = Delay::timeout(Duration::from_millis(50));
    waiter.start();

    assert!(waiter.wait().is_ok());
    assert!(waiter.wait().is_ok());
    std::thread::sleep(Duration::from_millis(10));
    assert!(waiter.wait().is_ok());
    std::thread::sleep(Duration::from_millis(50));
    assert!(waiter.wait().is_err());
}

#[test]
fn counter_works() {
    let mut waiter = Delay::count_timeout(3);
    waiter.start();

    assert!(waiter.wait().is_ok());
    assert!(waiter.wait().is_ok());
    assert!(waiter.wait().is_err());
    assert!(waiter.wait().is_err());
}

#[test]
fn clone_works() {
    let mut waiter1 = Delay::count_timeout(3);
    eprintln!("1");
    let mut waiter2 = waiter1.clone();
    eprintln!("2");

    waiter1.start();
    assert!(waiter1.wait().is_ok());
    assert!(waiter1.wait().is_ok());
    assert!(waiter1.wait().is_err());

    waiter2.start();
    assert!(waiter2.wait().is_ok());
    assert!(waiter2.wait().is_ok());
    assert!(waiter2.wait().is_err());
}
