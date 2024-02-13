#![no_main]

use libfuzzer_sys::fuzz_target;
use stringzilla::StringZilla;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let split = std::cmp::max(data[0] as usize, 1) % data.len() as usize;
    let (needle, haystack) = (&data[..split], &data[split..]);
    // Locates first matching substring
    haystack.sz_find(needle);
});
