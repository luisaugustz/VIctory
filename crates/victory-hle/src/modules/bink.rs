//! The title's movie opens (`Bink2Scarlett.dll!BinkOpenWithOptions`), seen on
//! the way: each name is logged once, and movies whose name contains one of
//! `VICTORY_SKIP_MOVIES=<part>[,<part>...]` (case ignored) are opened and
//! told they have one frame, so the title sees them over at once.
//! GTA VI opens its movies from memory: the "name" is then
//! `kb2 size=<bytes> frames=<n> <w>x<h>`, and a part such as `frames=150`
//! matches it.
//! The title then goes on as it does when a movie is missing: what the PS5
//! patch "Skip intro logos" does by marking the logo movie finished.
//!
//! Off, nothing is wrapped: the slot keeps the library's function.

use std::ffi::{CStr, c_char, c_void};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const DLL: &str = "Bink2Scarlett.dll";
const OPEN: &str = "BinkOpenWithOptions";

/// The library's `BinkOpenWithOptions`.
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
type Open = unsafe extern "system" fn(*const c_char, *const c_void, u32) -> *mut c_void;

/// The parts of names refused (`VICTORY_SKIP_MOVIES`), lower case.
fn skipped() -> &'static [String] {
    static SET: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    SET.get_or_init(|| {
        std::env::var("VICTORY_SKIP_MOVIES")
            .unwrap_or_default()
            .split(',')
            .map(|p| p.trim().to_ascii_lowercase())
            .filter(|p| !p.is_empty())
            .collect()
    })
}

/// Whether movie opens are wrapped: names logged (`VICTORY_LOG_MOVIES=1`) or
/// some refused (`VICTORY_SKIP_MOVIES`).
fn wanted() -> bool {
    !skipped().is_empty() || std::env::var_os("VICTORY_LOG_MOVIES").is_some_and(|v| v == "1")
}

/// What the title opens: a name, or (a movie read into memory by the
/// title, which starts with the Bink 2 header `KB2`) its size, frames and
/// picture size, as `kb2 size=<bytes> frames=<n> <w>x<h>`.
fn name_of(name: *const c_char) -> String {
    if name.is_null() {
        return String::from("(none)");
    }
    // SAFETY: the title passes a name or a movie in its memory; both are
    // readable for at least the header's 32 bytes when they start "KB2".
    let head = unsafe { std::slice::from_raw_parts(name.cast::<u8>(), 3) };
    if head == b"KB2" {
        let word = |at: usize| unsafe { name.cast::<u8>().add(at).cast::<u32>().read_unaligned() };
        return format!("kb2 size={} frames={} {}x{}", word(4).wrapping_add(8), word(8), word(20), word(24));
    }
    // SAFETY: the title passes a NUL-terminated name.
    unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned()
}

unsafe extern "system" fn open(name: *const c_char, options: *const c_void, flags: u32) -> *mut c_void {
    let shown = name_of(name);
    let lower = shown.to_ascii_lowercase();
    let skip = skipped().iter().any(|p| lower.contains(p.as_str()));
    static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());
    {
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if !seen.contains(&shown) {
            seen.push(shown.clone());
            tracing::info!(name = %shown, flags = format_args!("{flags:#x}"), skipped = skip, "movie opened (BinkOpenWithOptions)");
        }
    }
    let was = ORIGINAL.load(Ordering::Acquire);
    // SAFETY: the library's function, kept when the slot was wrapped.
    let bink = unsafe { std::mem::transmute::<usize, Open>(was)(name, options, flags) };
    // A movie skipped is opened and told it has one frame, so the title,
    // which plays a movie until its last frame, is done after the first (a
    // title waits for a movie it asked for: refusing the open left the
    // screen black; `BinkGoto` right after an asynchronous open faulted in
    // the library). The frame count is the handle's third word
    // (`HBINK.Frames`, after width and height).
    if skip && !bink.is_null() {
        // SAFETY: a handle the library just gave; its first words are
        // width, height and frame count.
        let words = bink.cast::<u32>();
        let (width, height, frames) = unsafe { (words.read(), words.add(1).read(), words.add(2).read()) };
        tracing::info!(width, height, frames, "movie skipped: its handle now says one frame");
        if frames > 1 {
            // SAFETY: as above.
            unsafe { words.add(2).write(1) };
        }
    }
    bink
}

/// Points the bound import slots of the image at `base` (described by
/// `info`) that name `BinkOpenWithOptions` here. How many. Call after the
/// imports are bound and before the title runs.
pub fn wrap_imports(base: usize, info: &victory_pe::ImageInfo) -> usize {
    use windows_sys::Win32::System::Memory::{PAGE_READWRITE, VirtualProtect};
    if !wanted() {
        return 0;
    }
    let thunk = open as *const () as usize;
    let mut wrapped = 0;
    for module in info.imports.iter().chain(&info.delay_imports) {
        if !module.dll.eq_ignore_ascii_case(DLL) {
            continue;
        }
        for entry in &module.entries {
            let victory_pe::ImportSymbol::Name { name, .. } = &entry.symbol else { continue };
            if name != OPEN {
                continue;
            }
            let slot = (base + entry.iat_slot_rva as usize) as *mut usize;
            // SAFETY: an import slot of the mapped image, bound by now.
            let was = unsafe { slot.read_volatile() };
            if was == 0 || was == thunk {
                continue;
            }
            ORIGINAL.store(was, Ordering::Release);
            let mut old = 0;
            // SAFETY: the slot's page, made writable for the write and put
            // back.
            unsafe {
                if VirtualProtect(slot.cast(), 8, PAGE_READWRITE, &mut old) == 0 {
                    continue;
                }
                slot.write_volatile(thunk);
                VirtualProtect(slot.cast(), 8, old, &mut old);
            }
            wrapped += 1;
        }
    }
    tracing::info!(wrapped, skip = ?skipped(), "movie opens are seen on the way (VICTORY_LOG_MOVIES / VICTORY_SKIP_MOVIES)");
    wrapped
}
