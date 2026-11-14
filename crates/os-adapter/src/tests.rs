use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::{DragPoll, DragSessionEvent, DragStateMachine};

#[test]
fn requires_button_modifier_and_inclusive_euclidean_distance() {
    let mut state = DragStateMachine::default();
    let now = Instant::now();
    let mut poll = DragPoll::new(now, false, true, 0.0, 0.0);
    assert_eq!(state.update(poll, || panic!("idle")), None);
    poll.button_down = true;
    poll.shift_down = false;
    assert_eq!(state.update(poll, || panic!("unmodified press")), None);
    poll.cursor_x = -9.6;
    poll.cursor_y = 12.7;
    poll.shift_down = true;
    assert_eq!(state.update(poll, || panic!("below threshold")), None);
    poll.cursor_y = 12.8;
    let files = vec![PathBuf::from("sample file.txt")];
    assert_eq!(
        state.update(poll, || Some(files.clone())),
        Some(DragSessionEvent::DragStarted {
            files,
            cursor_x: -9.6,
            cursor_y: 12.8,
        })
    );
}

#[test]
fn hold_threshold_waits_for_a_drag_source_then_allows_unknown_files() {
    let mut state = DragStateMachine::default();
    let now = Instant::now();
    let mut poll = DragPoll::new(now, true, true, 10.0, 20.0);
    assert_eq!(state.update(poll, || panic!("first press")), None);
    poll.at = now + Duration::from_millis(199);
    assert_eq!(state.update(poll, || panic!("short press")), None);
    poll.at += Duration::from_millis(1);
    assert_eq!(state.update(poll, || None), None);
    assert_eq!(
        state.update(poll, || Some(vec![])),
        Some(DragSessionEvent::DragStarted {
            files: vec![],
            cursor_x: 10.0,
            cursor_y: 20.0,
        })
    );
    assert_eq!(state.update(poll, || panic!("already active")), None);
    poll.cursor_y = 22.0;
    assert_eq!(
        state.update(poll, || panic!("already active")),
        Some(DragSessionEvent::DragMoved {
            cursor_x: 10.0,
            cursor_y: 22.0,
        })
    );
    poll.shift_down = false;
    poll.button_down = false;
    assert_eq!(
        state.update(poll, || panic!("released")),
        Some(DragSessionEvent::DragEnded)
    );
    assert_eq!(state.update(poll, || panic!("idle")), None);
}

#[test]
fn modifier_release_cancels_once_and_requires_a_fresh_press() {
    let mut state = DragStateMachine::default();
    let now = Instant::now();
    let mut poll = DragPoll::new(now, true, true, 0.0, 0.0);
    state.update(poll, || None);
    poll.cursor_x = 16.0;
    assert!(matches!(
        state.update(poll, || Some(vec![])),
        Some(DragSessionEvent::DragStarted { .. })
    ));
    poll.shift_down = false;
    assert_eq!(
        state.update(poll, || panic!("cancelled")),
        Some(DragSessionEvent::DragModifierReleased)
    );
    poll.shift_down = true;
    poll.cursor_x = 100.0;
    assert_eq!(state.update(poll, || panic!("same press")), None);
    poll.button_down = false;
    assert_eq!(
        state.update(poll, || panic!("released after cancellation")),
        None
    );
    poll.button_down = true;
    poll.at += Duration::from_secs(1);
    assert_eq!(state.update(poll, || panic!("new press")), None);
    poll.cursor_x += 16.0;
    assert!(matches!(
        state.update(poll, || Some(vec![])),
        Some(DragSessionEvent::DragStarted { .. })
    ));
}

#[test]
fn unfinished_press_releases_silently_and_resets_distance() {
    let mut state = DragStateMachine::default();
    let mut poll = DragPoll::new(Instant::now(), true, true, 0.0, 0.0);
    state.update(poll, || None);
    poll.button_down = false;
    poll.cursor_x = 20.0;
    assert_eq!(
        state.update(poll, || panic!("released before activation")),
        None
    );
    poll.button_down = true;
    assert_eq!(state.update(poll, || panic!("new press")), None);
    poll.cursor_x = 31.0;
    assert_eq!(state.update(poll, || panic!("below threshold")), None);
}

#[test]
fn suspending_requires_a_fresh_mouse_press() {
    let mut state = DragStateMachine::default();
    let mut poll = DragPoll::new(Instant::now(), true, true, 0.0, 0.0);
    state.suspend(poll);
    poll.cursor_x = 20.0;
    assert_eq!(
        state.update(poll, || panic!("held during suspension")),
        None
    );
    poll.button_down = false;
    assert_eq!(state.update(poll, || panic!("idle")), None);
    poll.button_down = true;
    assert_eq!(state.update(poll, || panic!("first press")), None);
    poll.cursor_x += 20.0;
    assert!(matches!(
        state.update(poll, || Some(vec![])),
        Some(DragSessionEvent::DragStarted { .. })
    ));
}
