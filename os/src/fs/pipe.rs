use crate::consts::PAGE_SIZE;
use crate::sync::mutex::Mutex;
use crate::process::yield_now;
use crate::memory::*;
use riscv::addr::Frame;

pub struct Pipe {
    inner: Mutex<PipeInner>,
    frame: Frame,
}

struct PipeInner {
    head: usize,
    tail: usize,
    frame: &'static mut [u8],
}

impl Pipe {
    pub fn read(&self, base: *mut u8, len: usize) -> usize {
        let mut cnt = 0;
        let mut base = base;

        let mut inner = self.inner.lock();

        while inner.head != inner.tail {
            // TODO: use memcpy
            *unsafe { &mut *base } = inner.frame[inner.head];
            base = unsafe { base.offset(1) };
            inner.head += 1;
            cnt += 1;
            if inner.head == PAGE_SIZE { inner.head = 0; }
            if len == cnt { return cnt; }
        }

        cnt
    }

    pub fn write(&self, base: *const u8, len: usize) -> usize {
        let mut cnt = 0;
        let mut base = base;

        let mut inner = self.inner.lock();

        while (inner.tail + 1) % PAGE_SIZE != inner.head {
            let curtail = inner.tail;
            inner.frame[curtail] = unsafe { *base };
            base = unsafe { base.offset(1) };
            inner.tail = (inner.tail + 1) % PAGE_SIZE;
            cnt += 1;
            if len == cnt { return cnt; }
        }

        cnt
    }

    pub fn new() -> Self {
        let frame = alloc_frame().expect("Unable to alloc");
        let va = access_pa_via_va(frame.start_address().as_usize());

        let inner = PipeInner {
            head: 0,
            tail: 0,
            frame: unsafe { core::slice::from_raw_parts_mut(va as _, PAGE_SIZE) },
        };

        Pipe {
            inner: Mutex::new(inner),
            frame,
        }
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        dealloc_frame(self.frame);
    }
}
