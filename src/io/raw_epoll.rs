//! Unsafe low-level epoll APIs.
//!
//! # Safety
//!
//! These work with `RawFd` values. See the [`rsix::io::epoll`] module for
//! safe APIs.
//!
//! [`rsix::io::epoll`]: crate::io::epoll

#![allow(unsafe_code)]

use crate::{
    imp::{io::raw_epoll, syscalls},
    io::{self, RawFd},
};
use bitflags::bitflags;
use io_lifetimes::{AsFd, OwnedFd};
use std::os::raw::c_int;

/// `struct epoll_event`—Flags and data describing an event.
pub type EpollEvent = raw_epoll::EpollEvent;

/// The `EpollEvent` is a typedef for the platform `epoll_event` struct, and
/// there are a few different ways to name its `data` field. This trait
/// provides a portable way to get and set this field.
pub trait EpollEventExt {
    /// Constructs a new `EpollEvent`.
    fn new(events: EpollEventFlags, data: u64) -> Self;
    /// Sets the `data` field of an `EpollEvent`.
    fn set_data(&mut self, data: u64);
    /// Gets the `data` field of an `EpollEvent`.
    fn data(&self) -> u64;
}

bitflags! {
    /// `EPOLL_*` for use with [`epoll_create`].
    pub struct EpollCreateFlags: raw_epoll::EpollCreateFlagsBits {
        /// `EPOLL_CLOEXEC`
        const CLOEXEC = raw_epoll::EpollCreateFlags::CLOEXEC.bits();
    }
}

bitflags! {
    /// `EPOLL*` for use with [`epoll_add`] and [`epoll_mod`].
    #[derive(Default)]
    pub struct EpollEventFlags: raw_epoll::EpollEventFlagsBits {
        /// `EPOLLIN`
        const IN = raw_epoll::EpollEventFlags::IN.bits();

        /// `EPOLLOUT`
        const OUT = raw_epoll::EpollEventFlags::OUT.bits();

        /// `EPOLLPRI`
        const PRI = raw_epoll::EpollEventFlags::PRI.bits();

        /// `EPOLLERR`
        const ERR = raw_epoll::EpollEventFlags::ERR.bits();

        /// `EPOLLHUP`
        const HUP = raw_epoll::EpollEventFlags::HUP.bits();

        /// `EPOLLET`
        const ET = raw_epoll::EpollEventFlags::ET.bits();

        /// `EPOLLONESHOT`
        const ONESHOT = raw_epoll::EpollEventFlags::ONESHOT.bits();

        /// `EPOLLWAKEUP`
        const WAKEUP = raw_epoll::EpollEventFlags::WAKEUP.bits();

        /// `EPOLLEXCLUSIVE`
        const EXCLUSIVE = raw_epoll::EpollEventFlags::EXCLUSIVE.bits();
    }
}

/// `epoll_create(flags)`—Creates a new epoll file descriptor.
#[inline]
pub fn epoll_create(flags: EpollCreateFlags) -> io::Result<OwnedFd> {
    Ok(syscalls::epoll_create(flags)?.into())
}

/// `epoll_ctl(epfd, EPOLL_CTL_ADD, fd, event)`—Registers a file descriptor
/// with an epoll set.
///
/// # Safety
///
/// `fd` must contain a file descriptor which remains valid as long as `fd`
/// is registered with the epoll set.
///
/// See the [`rsix::io::epoll`] module for a safe epoll API.
///
/// [`rsix::io::epoll`]: crate::io::epoll
#[inline]
pub unsafe fn epoll_add<Fd: AsFd>(epfd: &Fd, fd: RawFd, event: &io::EpollEvent) -> io::Result<()> {
    let epfd = epfd.as_fd();
    syscalls::epoll_add(epfd, fd, event)
}

/// `epoll_ctl(epfd, EPOLL_CTL_MOD, fd, event)`—Changes the event flags
/// associated with a file descriptor in an epoll set.
///
/// # Safety
///
/// `fd` must contain a file descriptor in the epoll set.
///
/// See the [`rsix::io::epoll`] module for a safe epoll API.
///
/// [`rsix::io::epoll`]: crate::io::epoll
#[inline]
pub unsafe fn epoll_mod<Fd: AsFd>(epfd: &Fd, fd: RawFd, event: &io::EpollEvent) -> io::Result<()> {
    let epfd = epfd.as_fd();
    syscalls::epoll_mod(epfd, fd, event)
}

/// `epoll_ctl(epfd, EPOLL_CTL_DEL, fd, event)`—Remove a file descriptor
/// from an epoll set.
///
/// # Safety
///
/// `fd` must contain a file descriptor in the epoll set.
///
/// See the [`rsix::io::epoll`] module for a safe epoll API.
///
/// [`rsix::io::epoll`]: crate::io::epoll
#[inline]
pub unsafe fn epoll_del<Fd: AsFd>(epfd: &Fd, fd: RawFd) -> io::Result<()> {
    let epfd = epfd.as_fd();
    syscalls::epoll_del(epfd, fd)
}

/// `epoll_wait(epfd, events.as_mut_ptr(), events.len(), timeout)`—Waits for
/// events on an epoll file descriptor.
#[inline]
pub fn epoll_wait<Fd: AsFd>(
    epfd: &Fd,
    events: &mut [io::EpollEvent],
    timeout: c_int,
) -> io::Result<usize> {
    // Safety: We're passing a raw pointer and length to the epoll slice
    // which must be valid.
    unsafe { epoll_wait_raw(epfd, events.as_mut_ptr(), events.len(), timeout) }
}

/// `epoll_wait(epfd, events, num_events, timeout)`—Waits for events on an
/// epoll file descriptor.
///
/// This is similar to `epoll_wait`, but takes a raw pointer and length instead
/// of a slice.
///
/// # Safety
///
/// `events` must point to a memory buffer valid for storing `num_events`
/// `io::EpollEvent`s.
#[inline]
pub unsafe fn epoll_wait_raw<Fd: AsFd>(
    epfd: &Fd,
    events: *mut io::EpollEvent,
    num_events: usize,
    timeout: c_int,
) -> io::Result<usize> {
    let epfd = epfd.as_fd();
    syscalls::epoll_wait_raw(epfd, events, num_events, timeout)
}
