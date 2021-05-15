//! Exposes a tiny subset of MPI.
//!
//! This module is not trying to be a general-purpose MPI wrapper crate. If
//! that's what you're looking for, see [Rust MPI][1]. Task parallelism in
//! `gridiron` only requires point-to-point messaging with encoded byte
//! streams, we don't need the vast majority of what MPI is able to do. We're
//! really just exploiting MPI for
//! 
//! - fast, site-specific interconnect (Infiniband, etc.)
//! - blocking send and receive operations (immediate sends can be emulated in
//!   threads)
//! - interaction with PBS or other job scheduler at HPC sites (discovering
//!   the process group)
//! 
//! [1]: http://rsmpi.github.io/rsmpi/mpi/index.html

#![cfg(feature = "mpi")]

#[repr(C)]
pub struct Status {
    pub count: i32,
    pub source: i32,
    pub tag: i32,
}

extern {
    pub fn init() -> i32;
    pub fn finalize();
    pub fn barrier();
    pub fn comm_rank() -> i32;
    pub fn comm_size() -> i32;
    pub fn send(buf: *const u8, count: i32, dest: i32, tag: i32);
    pub fn recv(buf: *mut u8, count: i32, source: i32, tag: i32);
    pub fn probe_tag(tag: i32) -> Status;
}
