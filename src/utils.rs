use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GlobalFree, HANDLE, HGLOBAL},
        System::{
            Memory::{GlobalAlloc, GLOBAL_ALLOC_FLAGS},
            Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        },
    },
};

use crate::{Error, Result};

pub fn bufcopy<T: Copy>(s: &mut [T], v: &[T]) {
    let l = s.len().min(v.len());
    s[..l].copy_from_slice(&v[..l])
}

pub fn get_mutex_name(s: &str) -> String {
    let pid = std::process::id();
    // F5 uses 'Monster Hunter Frontier Online', but it's probably fine
    format!("Monster Hunter Frontier Z {s} {pid}")
}

pub fn create_mutex(name: impl Into<HSTRING>) -> Result<HANDLE> {
    unsafe { CreateMutexW(None, false, &name.into()) }.or(Err(Error::Mutex))
}

pub fn get_or_create_mutex(name: impl Into<HSTRING> + Copy) -> Result<HANDLE> {
    unsafe { OpenMutexW(SYNCHRONIZATION_ACCESS_RIGHTS(0x1F0001), false, &name.into()) }
    .or_else(|_| create_mutex(name))
    .or(Err(Error::Mutex))
}

// Alias per compatibilità con mhf.rs
pub fn alloc_global(size: usize) -> Result<HGLOBAL> {
    unsafe { GlobalAlloc(GLOBAL_ALLOC_FLAGS(0x42), size) }.or(Err(Error::GlobalAlloc))
}

// Funzione originale (manteniamo per compatibilità)
pub fn create_global_alloc() -> Result<HGLOBAL> {
    alloc_global(0x8ae0)
}

pub fn release_global_alloc(handle: HGLOBAL) -> Result<()> {
    unsafe { GlobalFree(handle) }
    .map(|_| ())  // Converti HGLOBAL → ()
    .or(Err(Error::GlobalAlloc))
}

// Nuova funzione per creare entrambi i mutex
pub fn build_mutexes(
    master_name: &[u8],
    ready_name: &[u8],
) -> Result<(HANDLE, HANDLE)> {
    // Converti da byte slice a String (rimuovi null terminator)
    let master_str = std::str::from_utf8(master_name)
    .map_err(|_| Error::Mutex)?
    .trim_end_matches('\0');

    let ready_str = std::str::from_utf8(ready_name)
    .map_err(|_| Error::Mutex)?
    .trim_end_matches('\0');

    // Crea mutex con PID
    let master_full = get_mutex_name(master_str);
    let ready_full = get_mutex_name(ready_str);

    let mutex_master = get_or_create_mutex(&master_full)?;
    let mutex_master_ready = get_or_create_mutex(&ready_full)?;

    Ok((mutex_master, mutex_master_ready))
}
