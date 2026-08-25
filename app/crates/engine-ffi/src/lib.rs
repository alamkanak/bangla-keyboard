use engine_core::{CommitAction, Engine, LayoutMode};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;

static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

/// Initialize the engine with the path to the data directory.
/// Returns 0 on success, -1 on failure.
///
/// # Safety
/// `data_dir` must be a valid, null-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn bk_engine_init(data_dir: *const c_char) -> i32 {
    if data_dir.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(data_dir) };
    let path = match c_str.to_str() {
        Ok(s) => PathBuf::from(s),
        Err(_) => return -1,
    };

    match Engine::new(&path) {
        Ok(engine) => {
            let mut guard = ENGINE.lock().unwrap();
            *guard = Some(engine);
            0
        }
        Err(e) => {
            eprintln!("BanglaKeyboard engine init error: {e}");
            -1
        }
    }
}

/// Shut down the engine and free resources.
#[no_mangle]
pub extern "C" fn bk_engine_shutdown() {
    let mut guard = ENGINE.lock().unwrap();
    *guard = None;
}

/// Set the layout mode: 0 = Phonetic, 1 = UniBijoy.
#[no_mangle]
pub extern "C" fn bk_set_mode(mode: i32) {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        let layout_mode = match mode {
            0 => LayoutMode::Phonetic,
            1 => LayoutMode::UniBijoy,
            _ => return,
        };
        engine.set_mode(layout_mode);
    }
}

/// Get the current layout mode: 0 = Phonetic, 1 = UniBijoy.
#[no_mangle]
pub extern "C" fn bk_get_mode() -> i32 {
    let guard = ENGINE.lock().unwrap();
    match guard.as_ref() {
        Some(engine) => match engine.mode() {
            LayoutMode::Phonetic => 0,
            LayoutMode::UniBijoy => 1,
        },
        None => -1,
    }
}

/// Handle a key press. Returns: 0 = Commit, 1 = UpdatePreview, 2 = Nothing, 3 = CommitReplaceLast.
#[no_mangle]
pub extern "C" fn bk_handle_key(key: c_char, shift: bool) -> i32 {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        let ch = key as u8 as char;
        match engine.handle_key(ch, shift) {
            CommitAction::Commit => 0,
            CommitAction::UpdatePreview => 1,
            CommitAction::Nothing => 2,
            CommitAction::CommitReplaceLast => 3,
        }
    } else {
        2
    }
}

/// Handle backspace. Returns: 0 = Commit, 1 = UpdatePreview, 2 = Nothing.
#[no_mangle]
pub extern "C" fn bk_handle_backspace() -> i32 {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        match engine.handle_backspace() {
            CommitAction::Commit => 0,
            CommitAction::UpdatePreview => 1,
            CommitAction::Nothing => 2,
            CommitAction::CommitReplaceLast => 0,
        }
    } else {
        2
    }
}

/// Handle enter/return. Returns the committed text or NULL.
/// Caller must free the returned string with bk_free_string.
#[no_mangle]
pub extern "C" fn bk_handle_enter() -> *mut c_char {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        match engine.handle_enter() {
            Some(text) => CString::new(text)
                .map(|s| s.into_raw())
                .unwrap_or(ptr::null_mut()),
            None => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// Handle space. Returns the committed text or NULL.
/// Caller must free the returned string with bk_free_string.
#[no_mangle]
pub extern "C" fn bk_handle_space() -> *mut c_char {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        match engine.handle_space() {
            Some(text) => CString::new(text)
                .map(|s| s.into_raw())
                .unwrap_or(ptr::null_mut()),
            None => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// Get the current preview text. Caller must free with bk_free_string.
#[no_mangle]
pub extern "C" fn bk_get_preview() -> *mut c_char {
    let guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_ref() {
        CString::new(engine.preview())
            .map(|s| s.into_raw())
            .unwrap_or(ptr::null_mut())
    } else {
        ptr::null_mut()
    }
}

/// Check if the engine is currently composing text.
#[no_mangle]
pub extern "C" fn bk_is_composing() -> bool {
    let guard = ENGINE.lock().unwrap();
    guard.as_ref().map(|e| e.is_composing()).unwrap_or(false)
}

/// Reset the composing state.
#[no_mangle]
pub extern "C" fn bk_reset() {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        engine.reset();
    }
}

/// Get the number of candidates.
#[no_mangle]
pub extern "C" fn bk_candidate_count() -> i32 {
    let guard = ENGINE.lock().unwrap();
    guard
        .as_ref()
        .map(|e| e.candidates().len() as i32)
        .unwrap_or(0)
}

/// Get a candidate by index. Caller must free with bk_free_string.
#[no_mangle]
pub extern "C" fn bk_get_candidate(index: i32) -> *mut c_char {
    let guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_ref() {
        if let Some(candidate) = engine.candidates().get(index as usize) {
            return CString::new(candidate.text.as_str())
                .map(|s| s.into_raw())
                .unwrap_or(ptr::null_mut());
        }
    }
    ptr::null_mut()
}

/// Select a candidate by index. Returns the committed text or NULL.
/// Caller must free the returned string with bk_free_string.
#[no_mangle]
pub extern "C" fn bk_select_candidate(index: i32) -> *mut c_char {
    let mut guard = ENGINE.lock().unwrap();
    if let Some(engine) = guard.as_mut() {
        match engine.select_candidate(index as usize) {
            Some(text) => CString::new(text)
                .map(|s| s.into_raw())
                .unwrap_or(ptr::null_mut()),
            None => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

/// Free a string returned by any bk_* function.
///
/// # Safety
/// `s` must be a pointer previously returned by a `bk_*` function, or null.
#[no_mangle]
pub unsafe extern "C" fn bk_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}
