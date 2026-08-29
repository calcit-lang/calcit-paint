use std::sync::Mutex;
use std::time::{Duration, Instant};

lazy_static! {
  static ref FRAME_SCHEDULER: Mutex<FrameSchedulerState> = Mutex::new(FrameSchedulerState::default());
}

#[derive(Default)]
struct FrameSchedulerState {
  active: bool,
  pending: bool,
}

#[derive(Debug)]
pub struct ActiveFrameLoop;

impl Drop for ActiveFrameLoop {
  fn drop(&mut self) {
    let mut state = FRAME_SCHEDULER.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    state.active = false;
    state.pending = false;
  }
}

pub fn activate() -> Result<ActiveFrameLoop, String> {
  let mut state = FRAME_SCHEDULER
    .lock()
    .map_err(|_| "frame scheduler lock is poisoned".to_owned())?;
  if state.active {
    return Err("a paint frame loop is already active".to_owned());
  }
  state.active = true;
  state.pending = false;
  Ok(ActiveFrameLoop)
}

/// Returns true only when this call created a new pending request.
pub fn request() -> Result<bool, String> {
  let mut state = FRAME_SCHEDULER
    .lock()
    .map_err(|_| "frame scheduler lock is poisoned".to_owned())?;
  if !state.active {
    return Err("request-frame requires an active paint window callback".to_owned());
  }
  let created = !state.pending;
  state.pending = true;
  Ok(created)
}

pub fn pending() -> Result<bool, String> {
  FRAME_SCHEDULER
    .lock()
    .map(|state| state.pending)
    .map_err(|_| "frame scheduler lock is poisoned".to_owned())
}

pub fn take_request() -> Result<bool, String> {
  let mut state = FRAME_SCHEDULER
    .lock()
    .map_err(|_| "frame scheduler lock is poisoned".to_owned())?;
  let pending = state.pending;
  state.pending = false;
  Ok(pending)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameTiming {
  pub number: u64,
  pub timestamp: Duration,
  pub delta: Duration,
}

pub struct FrameClock {
  started_at: Instant,
  last_frame_at: Option<Instant>,
  number: u64,
}

impl FrameClock {
  pub fn new(started_at: Instant) -> Self {
    Self {
      started_at,
      last_frame_at: None,
      number: 0,
    }
  }

  pub fn reset_delta(&mut self) {
    self.last_frame_at = None;
  }

  pub fn next_at(&mut self, now: Instant) -> FrameTiming {
    self.number = self.number.saturating_add(1);
    let timing = FrameTiming {
      number: self.number,
      timestamp: now.saturating_duration_since(self.started_at),
      delta: self
        .last_frame_at
        .map_or(Duration::ZERO, |last| now.saturating_duration_since(last)),
    };
    self.last_frame_at = Some(now);
    timing
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static TEST_LOCK: Mutex<()> = Mutex::new(());

  #[test]
  fn frame_requests_require_a_loop_coalesce_and_cancel_on_drop() {
    let _lock = TEST_LOCK.lock().unwrap();
    assert!(request().unwrap_err().contains("active paint window callback"));

    let active = activate().unwrap();
    assert!(request().unwrap());
    assert!(!request().unwrap());
    assert!(pending().unwrap());
    assert!(take_request().unwrap());
    assert!(!take_request().unwrap());
    assert!(activate().unwrap_err().contains("already active"));
    assert!(request().unwrap());
    drop(active);

    assert!(request().is_err());
    assert!(!pending().unwrap());
  }

  #[test]
  fn frame_clock_is_monotonic_and_resets_delta_after_a_pause() {
    let started_at = Instant::now();
    let mut clock = FrameClock::new(started_at);
    assert_eq!(
      clock.next_at(started_at + Duration::from_millis(10)),
      FrameTiming {
        number: 1,
        timestamp: Duration::from_millis(10),
        delta: Duration::ZERO,
      }
    );
    assert_eq!(
      clock.next_at(started_at + Duration::from_millis(26)),
      FrameTiming {
        number: 2,
        timestamp: Duration::from_millis(26),
        delta: Duration::from_millis(16),
      }
    );
    clock.reset_delta();
    assert_eq!(clock.next_at(started_at + Duration::from_secs(5)).delta, Duration::ZERO);
  }
}
