use std::path::PathBuf;
use std::time::Instant;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, ConnectionExt, KeyButMask, Window};
use x11rb::rust_connection::RustConnection;

use crate::{DragPoll, ModifierKey, NativePoller, PollError};

fn session_type(session: Option<&str>, wayland: bool, x11: bool) -> Session {
    match session {
        Some("wayland") => Session::Wayland,
        Some("x11") => Session::X11,
        _ if wayland => Session::Wayland,
        _ if x11 => Session::X11,
        _ => Session::Headless,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Session {
    X11,
    Wayland,
    Headless,
}

fn session() -> Session {
    let has_display = |key| std::env::var_os(key).is_some_and(|value| !value.is_empty());
    session_type(
        std::env::var("XDG_SESSION_TYPE").ok().as_deref(),
        has_display("WAYLAND_DISPLAY"),
        has_display("DISPLAY"),
    )
}

pub fn is_wayland() -> bool {
    session() == Session::Wayland
}

pub(crate) struct Poller {
    connection: RustConnection,
    root: Window,
    drag_selection: Atom,
}

impl Poller {
    pub(crate) fn new() -> Result<Option<Self>, PollError> {
        match session() {
            Session::Wayland => {
                eprintln!("Wayland restricts global input polling. Use bergamot --radial-here <files...> to open the wheel.");
                return Ok(None);
            }
            Session::Headless => return Ok(None),
            Session::X11 => {}
        }
        let (connection, screen) = x11rb::connect(None)?;
        let root = connection.setup().roots[screen].root;
        let drag_selection = connection
            .intern_atom(false, b"XdndSelection")?
            .reply()?
            .atom;
        Ok(Some(Self {
            connection,
            root,
            drag_selection,
        }))
    }
}

impl NativePoller for Poller {
    fn poll(&mut self, modifier: ModifierKey) -> Result<DragPoll, PollError> {
        let at = Instant::now();
        let pointer = self.connection.query_pointer(self.root)?.reply()?;
        let mask = pointer.mask;
        let modifier_down = match modifier {
            ModifierKey::Shift => mask.contains(KeyButMask::SHIFT),
            ModifierKey::Ctrl => mask.contains(KeyButMask::CONTROL),
            ModifierKey::Alt => mask.contains(KeyButMask::MOD1),
            ModifierKey::AltShift => mask.contains(KeyButMask::MOD1 | KeyButMask::SHIFT),
            ModifierKey::CtrlShift => mask.contains(KeyButMask::CONTROL | KeyButMask::SHIFT),
            ModifierKey::CtrlAlt => mask.contains(KeyButMask::CONTROL | KeyButMask::MOD1),
        };
        Ok(DragPoll::new(
            at,
            mask.contains(KeyButMask::BUTTON1),
            modifier_down,
            f64::from(pointer.root_x),
            f64::from(pointer.root_y),
        ))
    }

    fn drag_files(&mut self) -> Option<Vec<PathBuf>> {
        // A source owns XdndSelection during an Xdnd drag. Let the drop target
        // negotiate and read the offered paths; polling never requests their data.
        let selection = self
            .connection
            .get_selection_owner(self.drag_selection)
            .ok()?
            .reply()
            .ok()?;
        (selection.owner != x11rb::NONE).then(Vec::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wayland_takes_precedence_over_xwayland_display() {
        assert_eq!(session_type(Some("wayland"), false, true), Session::Wayland);
        assert_eq!(session_type(None, true, true), Session::Wayland);
        assert_eq!(session_type(Some("x11"), true, true), Session::X11);
        assert_eq!(session_type(None, false, true), Session::X11);
        assert_eq!(session_type(None, false, false), Session::Headless);
    }
}
