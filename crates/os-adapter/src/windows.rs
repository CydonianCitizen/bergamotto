use std::ffi::OsString;
use std::mem::ManuallyDrop;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use windows::core::Interface;
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IDispatch, IServiceProvider,
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};
use windows::Win32::System::Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_I4};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_LBUTTON, VK_RBUTTON, VK_SHIFT,
};
use windows::Win32::UI::Shell::{
    IFolderView2, IShellBrowser, IShellWindows, IWebBrowserApp, SID_STopLevelBrowser, ShellWindows,
    CSIDL_DESKTOP, SIGDN_FILESYSPATH, SWC_DESKTOP, SWFO_NEEDDISPATCH,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetAncestor, GetClassNameW, GetCursorPos, GetForegroundWindow,
    GetSystemMetrics, IsChild, PeekMessageW, WindowFromPoint, GA_ROOT, MSG, PM_REMOVE,
    SM_SWAPBUTTON,
};

use crate::{DragPoll, NativePoller, PollError};

struct ComApartment;

impl Drop for ComApartment {
    fn drop(&mut self) {
        // SAFETY: This guard is created only after successful initialization on the polling thread.
        unsafe { CoUninitialize() };
    }
}

enum ShellSource {
    Explorer(HWND),
    Desktop,
}

struct PressSource {
    shell: ShellSource,
    hit: HWND,
}

pub(crate) struct Poller {
    _apartment: ComApartment,
    button_down: bool,
    source: Option<PressSource>,
}

impl Poller {
    pub(crate) fn new() -> Result<Option<Self>, PollError> {
        // SAFETY: This worker initializes and services its own single-threaded COM apartment.
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok()?;
        }
        Ok(Some(Self {
            _apartment: ComApartment,
            button_down: false,
            source: None,
        }))
    }
}

impl NativePoller for Poller {
    fn poll(&mut self) -> Result<DragPoll, PollError> {
        // SAFETY: MSG and POINT have valid writable storage. This thread owns the COM message queue.
        let (button, cursor) = unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool() {
                DispatchMessageW(&message);
            }
            let button = if GetSystemMetrics(SM_SWAPBUTTON) == 0 {
                VK_LBUTTON
            } else {
                VK_RBUTTON
            };
            let mut cursor = POINT::default();
            (button, GetCursorPos(&mut cursor).map(|()| cursor))
        };

        let Ok(cursor) = cursor else {
            // The input desktop can be temporarily inaccessible, for example while locked.
            self.button_down = false;
            self.source = None;
            return Ok(DragPoll::new(Instant::now(), false, false, 0.0, 0.0));
        };

        let button_down = key_down(button);
        if button_down && !self.button_down {
            self.source = press_source(cursor);
        } else if !button_down {
            self.source = None;
        }
        self.button_down = button_down;

        Ok(DragPoll::new(
            Instant::now(),
            button_down,
            key_down(VK_SHIFT),
            f64::from(cursor.x),
            f64::from(cursor.y),
        ))
    }

    fn drag_files(&mut self) -> Option<Vec<PathBuf>> {
        let source = self.source.take()?;
        let folder = folder_view(source)?;
        // SAFETY: The view and all items belong to this thread's initialized COM apartment.
        let selection = unsafe { folder.GetSelection(false) }.ok()?;
        let count = unsafe { selection.GetCount() }.ok()?;
        let paths: Vec<_> = (0..count)
            .filter_map(|index| {
                let item = unsafe { selection.GetItemAt(index).ok()? };
                let name = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH).ok()? };
                if name.is_null() {
                    return None;
                }
                // SIGDN_FILESYSPATH returns an allocated, NUL-terminated UTF-16 path.
                let path = unsafe { OsString::from_wide(name.as_wide()) };
                unsafe { CoTaskMemFree(Some(name.0.cast())) };
                Some(PathBuf::from(path))
            })
            .collect();
        (!paths.is_empty()).then_some(paths)
    }

    fn interval(&self, _button_down: bool) -> Duration {
        Duration::from_millis(35)
    }
}

fn key_down(key: VIRTUAL_KEY) -> bool {
    // The high bit indicates current key state. Mouse virtual keys refer to physical buttons.
    unsafe { GetAsyncKeyState(i32::from(key.0)) < 0 }
}

fn window_class(window: HWND) -> String {
    let mut class = [0; 256];
    // SAFETY: The class buffer is writable. Failed or stale handles yield a length of zero.
    let length = unsafe { GetClassNameW(window, &mut class) } as usize;
    String::from_utf16_lossy(&class[..length])
}

fn press_source(point: POINT) -> Option<PressSource> {
    let (foreground, hit) = unsafe { (GetForegroundWindow(), WindowFromPoint(point)) };
    let root = unsafe { GetAncestor(hit, GA_ROOT) };
    let shell = match window_class(foreground).as_str() {
        "CabinetWClass" | "ExploreWClass" if root == foreground => {
            ShellSource::Explorer(foreground)
        }
        "Progman" | "WorkerW" if matches!(window_class(root).as_str(), "Progman" | "WorkerW") => {
            ShellSource::Desktop
        }
        _ => return None,
    };
    Some(PressSource { shell, hit })
}

fn integer_variant(value: i32) -> VARIANT {
    VARIANT {
        Anonymous: VARIANT_0 {
            Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                vt: VT_I4,
                Anonymous: VARIANT_0_0_0 { lVal: value },
                ..Default::default()
            }),
        },
    }
}

fn folder_view(source: PressSource) -> Option<IFolderView2> {
    // SAFETY: All calls use the polling thread's COM apartment. Interface lifetimes are owned.
    unsafe {
        let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL).ok()?;
        match source.shell {
            ShellSource::Explorer(hwnd) => (0..windows.Count().ok()?).find_map(|index| {
                let dispatch = windows.Item(&integer_variant(index)).ok()?;
                let explorer = dispatch.cast::<IWebBrowserApp>().ok()?;
                (explorer.HWND().ok()?.0 == hwnd.0 as isize)
                    .then(|| view_at(&dispatch, source.hit))
                    .flatten()
            }),
            ShellSource::Desktop => {
                let mut hwnd = 0;
                let dispatch = windows
                    .FindWindowSW(
                        &integer_variant(CSIDL_DESKTOP as i32),
                        &VARIANT::default(),
                        SWC_DESKTOP,
                        &mut hwnd,
                        SWFO_NEEDDISPATCH,
                    )
                    .ok()?;
                view_at(&dispatch, source.hit)
            }
        }
    }
}

fn view_at(dispatch: &IDispatch, hit: HWND) -> Option<IFolderView2> {
    // SAFETY: The shell window and returned view are used in the owning COM apartment.
    unsafe {
        let provider = dispatch.cast::<IServiceProvider>().ok()?;
        let browser: IShellBrowser = provider.QueryService(&SID_STopLevelBrowser).ok()?;
        let view = browser.QueryActiveShellView().ok()?;
        let hwnd = view.GetWindow().ok()?;
        // Explorer tabs can share a top-level HWND. Match the view where the press began.
        if hwnd != hit && !IsChild(hwnd, hit).as_bool() {
            return None;
        }
        view.cast().ok()
    }
}
