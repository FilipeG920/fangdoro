use std::time::{Duration, Instant};

use fangdoro::domain::{PomodoroPlan, TaskId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerKind {
    Focus,
    Break,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerStatus {
    Idle,
    Running { ends_at: Instant },
    Paused { remaining: Duration },
    Finished,
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub kind: TimerKind,
    pub planned: Duration,
    pub status: TimerStatus,
}

impl Timer {
    pub fn start(&mut self, now: Instant) {
        todo!()
    }

    pub fn pause(&mut self, now: Instant) {
        todo!()
    }

    pub fn resume(&mut self, now: Instant) {
        todo!()
    }

    pub fn remaining(&self, now: Instant) -> Duration {
        todo!()
    }

    pub fn add_time(&mut self, extra: Duration) {
        todo!()
    }

    pub fn tick(&mut self, now: Instant) -> TimerEvent {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct PomodoroSession {
    pub task_id: TaskId,
    pub plan: PomodoroPlan,
    pub completed_slices: u8,
    pub timer: Timer,
}

impl PomodoroSession {
    pub fn next_kind(session: &PomodoroSession) -> Option<TimerKind> {
        todo!()
    }
    pub fn finish_current_timer(session: &PomodoroSession) -> SessionEvent {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerEvent {
    NoChange,
    Finished(TimerKind),
}
