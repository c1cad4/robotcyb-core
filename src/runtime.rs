use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct WorkerContract {
    pub cell: &'static str,
    pub started: Instant,
    pub deadline: Instant,
    status: std::sync::Arc<std::sync::Mutex<&'static str>>,
    heartbeat: std::sync::Arc<std::sync::Mutex<Instant>>,
    runs: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl WorkerContract {
    pub fn new(cell: &'static str, budget: Duration) -> Self {
        let now = Instant::now();
        Self {
            cell,
            started: now,
            deadline: now + budget,
            status: std::sync::Arc::new(std::sync::Mutex::new("RUNNING")),
            heartbeat: std::sync::Arc::new(std::sync::Mutex::new(now)),
            runs: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }
    }

    pub fn heartbeat(&self) {
        if let Ok(mut value) = self.heartbeat.lock() {
            *value = Instant::now();
        }
        self.runs.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn expired(&self) -> bool {
        Instant::now() >= self.deadline
    }

    pub fn remaining(&self) -> Duration {
        self.deadline.saturating_duration_since(Instant::now())
    }

    pub fn finish(&self, status: &'static str) {
        if let Ok(mut value) = self.status.lock() {
            *value = status;
        }
        self.heartbeat();
    }

    pub fn status(&self) -> &'static str {
        self.status.lock().map(|value| *value).unwrap_or("ERROR")
    }

    pub fn heartbeat_age_ms(&self) -> u128 {
        self.heartbeat
            .lock()
            .map(|value| value.elapsed().as_millis())
            .unwrap_or(u128::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_transitions_and_deadline() {
        let worker = WorkerContract::new("TEST", Duration::from_secs(1));
        assert_eq!(worker.status(), "RUNNING");
        worker.finish("READY");
        assert_eq!(worker.status(), "READY");
        assert!(!worker.expired());
        assert!(WorkerContract::new("TEST", Duration::ZERO).expired());
    }
}
