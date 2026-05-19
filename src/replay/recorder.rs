use crate::kernel::node::NodeState;
use crate::observer::ObservationReport;

pub struct StateSnapshot {
    pub tick: u64,
    pub states: Vec<NodeState>,
    pub report: Option<ObservationReport>,
}

pub struct Recorder {
    pub snapshots: Vec<StateSnapshot>,
    max_snapshots: usize,
}

impl Recorder {
    pub fn new(max_snapshots: usize) -> Self {
        Self { snapshots: Vec::new(), max_snapshots }
    }

    pub fn record(&mut self, tick: u64, states: &[NodeState], report: Option<ObservationReport>) {
        if self.snapshots.len() >= self.max_snapshots {
            // Rolling window: remove oldest
            self.snapshots.remove(0);
        }
        self.snapshots.push(StateSnapshot {
            tick,
            states: states.to_vec(),
            report,
        });
    }

    pub fn get(&self, tick: u64) -> Option<&StateSnapshot> {
        self.snapshots.iter().find(|s| s.tick == tick)
    }

    pub fn latest(&self) -> Option<&StateSnapshot> {
        self.snapshots.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::node::NodeState;

    #[test]
    fn test_recorder_basic() {
        let mut rec = Recorder::new(3);
        let states = vec![NodeState::default(); 4];
        rec.record(1, &states, None);
        rec.record(2, &states, None);
        assert_eq!(rec.snapshots.len(), 2);
        assert!(rec.get(1).is_some());
        assert!(rec.get(2).is_some());
        assert!(rec.get(3).is_none());
    }

    #[test]
    fn test_recorder_rolling_window() {
        let mut rec = Recorder::new(3);
        let states = vec![NodeState::default(); 4];
        rec.record(1, &states, None);
        rec.record(2, &states, None);
        rec.record(3, &states, None);
        rec.record(4, &states, None); // should evict tick 1
        assert_eq!(rec.snapshots.len(), 3);
        assert!(rec.get(1).is_none(), "oldest should be evicted");
        assert!(rec.get(4).is_some());
    }

    #[test]
    fn test_recorder_latest() {
        let mut rec = Recorder::new(5);
        let states = vec![NodeState::default(); 4];
        assert!(rec.latest().is_none());
        rec.record(10, &states, None);
        rec.record(20, &states, None);
        assert_eq!(rec.latest().expect("should have latest").tick, 20);
    }
}
