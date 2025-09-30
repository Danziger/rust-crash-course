// To use a module at the same level than this one:
use super::log;

pub fn first(v: &[u32]) -> Option<u32> {
    log::debug("FIRST");

    let n = v.len();
    if n > 0 { Some(v[0]) } else { None }
}

pub fn last(v: &[u32]) -> Option<u32> {
    log::debug("LAST");

    let n = v.len();
    if n > 0 { Some(v[n - 1]) } else { None }
}