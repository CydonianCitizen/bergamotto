use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::time::Instant;

use core_graphics::display::CGDisplay;
use objc2::rc::{autoreleasepool, Retained};
use objc2_app_kit::{
    NSEvent, NSEventModifierFlags, NSPasteboard, NSPasteboardNameDrag, NSPasteboardTypeFileURL,
};
use objc2_foundation::{NSString, NSURL};

use crate::{DragPoll, ModifierKey, NativePoller, PollError};

const LEFT_MOUSE_BUTTON: usize = 1 << 0;

/// Returns the cursor in logical screen points from the top left of the primary display.
pub fn cursor_position() -> (f64, f64) {
    let position = NSEvent::mouseLocation();
    let main_height = CGDisplay::main().bounds().size.height;
    (position.x, main_height - position.y)
}

pub(crate) struct Poller {
    pasteboard: Retained<NSPasteboard>,
    idle_change_count: isize,
}

impl Poller {
    pub(crate) fn new() -> Result<Option<Self>, PollError> {
        Ok(Some(autoreleasepool(|_| {
            // SAFETY: AppKit exports the immutable drag pasteboard name.
            let pasteboard = NSPasteboard::pasteboardWithName(unsafe { NSPasteboardNameDrag });
            let idle_change_count = pasteboard.changeCount();
            Self {
                pasteboard,
                idle_change_count,
            }
        })))
    }
}

impl NativePoller for Poller {
    fn poll(&mut self, modifier: ModifierKey) -> Result<DragPoll, PollError> {
        Ok(autoreleasepool(|_| {
            let at = Instant::now();
            let button_down = NSEvent::pressedMouseButtons() & LEFT_MOUSE_BUTTON != 0;
            let flags = NSEvent::modifierFlags_class();
            let modifier_down = match modifier {
                ModifierKey::Shift => flags.contains(NSEventModifierFlags::Shift),
                ModifierKey::Ctrl => flags.contains(NSEventModifierFlags::Control),
                ModifierKey::Alt => flags.contains(NSEventModifierFlags::Option),
                ModifierKey::AltShift => {
                    flags.contains(NSEventModifierFlags::Option | NSEventModifierFlags::Shift)
                }
                ModifierKey::CtrlShift => {
                    flags.contains(NSEventModifierFlags::Control | NSEventModifierFlags::Shift)
                }
                ModifierKey::CtrlAlt => {
                    flags.contains(NSEventModifierFlags::Control | NSEventModifierFlags::Option)
                }
            };
            let (x, y) = cursor_position();
            if !button_down {
                // The shared pasteboard can retain the previous drag's contents after mouse-up.
                self.idle_change_count = self.pasteboard.changeCount();
            }
            DragPoll::new(at, button_down, modifier_down, x, y)
        }))
    }

    fn drag_files(&mut self) -> Option<Vec<PathBuf>> {
        autoreleasepool(|_| {
            let change_count = self.pasteboard.changeCount();
            if change_count == self.idle_change_count {
                return None;
            }

            // SAFETY: AppKit exports the immutable type for public.file-url.
            let file_type = unsafe { NSPasteboardTypeFileURL };
            if !self.pasteboard.types()?.containsObject(file_type) {
                return None;
            }

            let files = self.pasteboard.pasteboardItems().and_then(|items| {
                items
                    .iter()
                    .filter(|item| item.types().containsObject(file_type))
                    .map(|item| item.stringForType(file_type).as_deref().and_then(file_path))
                    .collect::<Option<Vec<_>>>()
            });

            // Use paths from the drop target if AppKit advertised files but withheld any URL.
            (self.pasteboard.changeCount() == change_count).then(|| files.unwrap_or_default())
        })
    }
}

fn file_path(value: &NSString) -> Option<PathBuf> {
    let url = NSURL::URLWithString(value)?;
    if !url.isFileURL() {
        return None;
    }
    let url = url.filePathURL()?;
    // SAFETY: NSURL returns a NUL-terminated path owned by the current autorelease pool.
    let bytes = unsafe { CStr::from_ptr(url.fileSystemRepresentation().as_ptr()) }.to_bytes();
    let path = PathBuf::from(OsStr::from_bytes(bytes));
    path.is_absolute().then_some(path)
}
