use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use lazy_static::lazy_static;

lazy_static! {
 pub(crate) static ref METERCS:Metrics = Metrics::new(
 &[
 "topics",
 "clients",
 "peers",
 "broadcasts",
 "servers",
 "states",
 "subscribes",
 ] );
}
fn main() {
    METERCS.inc("topics");
    METERCS.inc("subscribes");

    println!("{:?}", METERCS.snapshot());
}

pub struct Metrics(HashMap<&'static str, AtomicUsize>);

impl Metrics {
    pub fn new(names: &[&'static str]) -> Self {
        let mut metrics: HashMap<&'static str, AtomicUsize> = HashMap::new();
        for name in names.iter() {
            metrics.insert(name, AtomicUsize::new(0));
        }
        Self(metrics)
    }

    pub fn inc(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn dec(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn snapshot(&self) -> Vec<(&'static str, usize)> {
        self.0
            .iter()
            .map(|(k, v)| (*k, v.load(Ordering::Relaxed)))
            .collect()
    }
}

