use crate::io;
use bitflags::bitflags;
use std::os::raw::c_int;

pub(crate) type EpollEvent = libc::epoll_event;

impl io::EpollEventExt for EpollEvent {
    #[inline]
    fn new(events: io::EpollEventFlags, data: u64) -> Self {
        EpollEvent {
            events: events.bits(),
            r#u64: data,
        }
    }

    #[inline]
    fn set_data(&mut self, data: u64) {
        self.r#u64 = data;
    }

    #[inline]
    fn data(&self) -> u64 {
        self.r#u64
    }
}

pub(crate) type EpollCreateFlagsBits = c_int;

bitflags! {
    pub(crate) struct EpollCreateFlags: EpollCreateFlagsBits {
        const CLOEXEC = libc::EPOLL_CLOEXEC;
    }
}

pub(crate) type EpollEventFlagsBits = u32;

bitflags! {
    #[derive(Default)]
    pub(crate) struct EpollEventFlags: EpollEventFlagsBits {
        const IN = libc::EPOLLIN as _;
        const OUT = libc::EPOLLOUT as _;
        const PRI = libc::EPOLLPRI as _;
        const ERR = libc::EPOLLERR as _;
        const HUP = libc::EPOLLHUP as _;
        const ET = libc::EPOLLET as _;
        const ONESHOT = libc::EPOLLONESHOT as _;
        const WAKEUP = libc::EPOLLWAKEUP as _;
        #[cfg(not(target_os = "android"))]
        const EXCLUSIVE = libc::EPOLLEXCLUSIVE as _;
    }
}
