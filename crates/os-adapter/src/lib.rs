//! Native global input polling and platform-independent drag debouncing.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::is_wayland;
#[cfg(target_os = "linux")]
use linux::Poller;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::cursor_position;
#[cfg(target_os = "macos")]
use macos::Poller;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::Poller;

const MIN_DRAG_DISTANCE: f64 = 16.0;
const MIN_HOLD_TIME: Duration = Duration::from_millis(200);

#[derive(Clone, Debug, PartialEq)]
pub enum DragSessionEvent {
    DragStarted {
        files: Vec<PathBuf>,
        cursor_x: f64,
        cursor_y: f64,
    },
    DragMoved {
        cursor_x: f64,
        cursor_y: f64,
    },
    DragModifierReleased,
    DragEnded,
}

/// A pointer sample in native screen units (logical points on macOS, pixels elsewhere).
#[derive(Clone, Copy, Debug)]
pub struct DragPoll {
    pub at: Instant,
    pub button_down: bool,
    pub shift_down: bool,
    pub cursor_x: f64,
    pub cursor_y: f64,
}

impl DragPoll {
    pub fn new(
        at: Instant,
        button_down: bool,
        shift_down: bool,
        cursor_x: f64,
        cursor_y: f64,
    ) -> Self {
        Self {
            at,
            button_down,
            shift_down,
            cursor_x,
            cursor_y,
        }
    }

    fn position(self) -> (f64, f64) {
        (self.cursor_x, self.cursor_y)
    }
}

#[derive(Default)]
enum Phase {
    #[default]
    Idle,
    Pressed(DragPoll),
    Dragging((f64, f64)),
    Cancelled,
}

#[derive(Default)]
pub struct DragStateMachine {
    phase: Phase,
}

impl DragStateMachine {
    /// `drag_files` is queried after debouncing. `None` means no eligible drag yet.
    /// An empty vector means a drag is eligible and paths will arrive through the drop target.
    pub fn update(
        &mut self,
        poll: DragPoll,
        drag_files: impl FnOnce() -> Option<Vec<PathBuf>>,
    ) -> Option<DragSessionEvent> {
        if !poll.button_down {
            return matches!(std::mem::take(&mut self.phase), Phase::Dragging(_))
                .then_some(DragSessionEvent::DragEnded);
        }

        let DragPoll {
            cursor_x, cursor_y, ..
        } = poll;
        match self.phase {
            Phase::Idle => self.phase = Phase::Pressed(poll),
            Phase::Pressed(press) if poll.shift_down => {
                let distance = (cursor_x - press.cursor_x).hypot(cursor_y - press.cursor_y);
                if distance >= MIN_DRAG_DISTANCE
                    || poll.at.duration_since(press.at) >= MIN_HOLD_TIME
                {
                    let files = drag_files()?;
                    self.phase = Phase::Dragging(poll.position());
                    return Some(DragSessionEvent::DragStarted {
                        files,
                        cursor_x,
                        cursor_y,
                    });
                }
            }
            Phase::Dragging(_) if !poll.shift_down => {
                self.phase = Phase::Cancelled;
                return Some(DragSessionEvent::DragModifierReleased);
            }
            Phase::Dragging(previous) if previous != poll.position() => {
                self.phase = Phase::Dragging(poll.position());
                return Some(DragSessionEvent::DragMoved { cursor_x, cursor_y });
            }
            _ => {}
        }
        None
    }

    fn suspend(&mut self, poll: DragPoll) {
        self.phase = if poll.button_down {
            Phase::Cancelled
        } else {
            Phase::Idle
        };
    }
}

/// Shared toggle. A generation change invalidates events queued before a toggle.
#[derive(Clone)]
pub struct MonitorControl(Arc<AtomicU64>);

impl Default for MonitorControl {
    fn default() -> Self {
        Self(Arc::new(AtomicU64::new(1)))
    }
}

impl MonitorControl {
    pub fn enabled(&self) -> bool {
        self.0.load(Ordering::Relaxed) % 2 == 1
    }

    pub fn toggle(&self) -> bool {
        self.0.fetch_add(1, Ordering::Relaxed).is_multiple_of(2)
    }

    pub fn accepts(&self, generation: u64) -> bool {
        self.0.load(Ordering::Relaxed) == generation
    }

    pub fn start(
        &self,
        send: impl Fn(DragSessionEvent, u64) + Send + 'static,
    ) -> std::io::Result<()> {
        let control = Arc::downgrade(&self.0);
        std::thread::Builder::new()
            .name("global-shift-drag".into())
            .spawn(move || {
                if let Err(error) = monitor(control, send) {
                    eprintln!("Global Shift+Drag monitor stopped: {error}");
                }
            })?;
        Ok(())
    }
}

type PollError = Box<dyn std::error::Error>;

trait NativePoller {
    fn poll(&mut self) -> Result<DragPoll, PollError>;
    fn drag_files(&mut self) -> Option<Vec<PathBuf>>;

    fn interval(&self, button_down: bool) -> Duration {
        Duration::from_secs_f64(1.0 / if button_down { 60.0 } else { 30.0 })
    }
}

fn monitor(
    control: Weak<AtomicU64>,
    send: impl Fn(DragSessionEvent, u64),
) -> Result<(), PollError> {
    let Some(mut poller) = Poller::new()? else {
        return Ok(());
    };
    let mut state = DragStateMachine::default();
    let mut previous = None;
    while let Some(control) = control.upgrade() {
        let generation = control.load(Ordering::Relaxed);
        drop(control);
        if generation.is_multiple_of(2) {
            std::thread::sleep(poller.interval(false));
            continue;
        }
        let poll = poller.poll()?;
        if previous != Some(generation) {
            state.suspend(poll);
        } else if let Some(event) = state.update(poll, || poller.drag_files()) {
            send(event, generation);
        }
        previous = Some(generation);
        std::thread::sleep(
            poller
                .interval(poll.button_down)
                .saturating_sub(poll.at.elapsed()),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests;
