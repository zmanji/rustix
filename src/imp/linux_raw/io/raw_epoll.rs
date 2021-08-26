use crate::io;
use bitflags::bitflags;
use std::os::raw::c_uint;

pub(crate) type EpollEvent = linux_raw_sys::general::epoll_event;

impl io::EpollEventExt for EpollEvent {
    #[inline]
    fn new(events: io::EpollEventFlags, data: u64) -> Self {
        EpollEvent {
            events: events.bits(),
            data,
        }
    }

    #[inline]
    fn set_data(&mut self, data: u64) {
        self.data = data;
    }

    #[inline]
    fn data(&self) -> u64 {
        self.data
    }
}

pub(crate) type EpollCreateFlagsBits = c_uint;

bitflags! {
    pub(crate) struct EpollCreateFlags: EpollCreateFlagsBits {
        const CLOEXEC = linux_raw_sys::general::EPOLL_CLOEXEC;
    }
}

pub(crate) type EpollEventFlagsBits = u32;

bitflags! {
    #[derive(Default)]
    pub(crate) struct EpollEventFlags: EpollEventFlagsBits {
        const IN = linux_raw_sys::general::EPOLLIN as _;
        const OUT = linux_raw_sys::general::EPOLLOUT as _;
        const PRI = linux_raw_sys::general::EPOLLPRI as _;
        const ERR = linux_raw_sys::general::EPOLLERR as _;
        const HUP = linux_raw_sys::general::EPOLLHUP as _;
        const ET = linux_raw_sys::general::EPOLLET as _;
        const ONESHOT = linux_raw_sys::general::EPOLLONESHOT as _;
        const WAKEUP = linux_raw_sys::general::EPOLLWAKEUP as _;
        const EXCLUSIVE = linux_raw_sys::general::EPOLLEXCLUSIVE as _;
    }
}
