use super::Tid;
use alloc::vec::Vec;

pub trait Scheduler {
    fn push(&mut self, tid: Tid);
    fn pop(&mut self) -> Option<Tid>;
    fn tick(&mut self) -> bool;
    fn exit(&mut self, tid: Tid);
    fn set_priority(&mut self, tid: Tid, priority: usize) {}
}

#[derive(Default)]
struct RRInfo {
    valid: bool,
    time: usize,
    prev: usize,
    next: usize,
}

pub struct RRScheduler {
    threads: Vec<RRInfo>,
    max_time: usize,
    current: usize,
}

impl RRScheduler {
    pub fn new(max_time_slice: usize) -> Self {
        let mut rr = RRScheduler {
            threads: Vec::default(),
            max_time: max_time_slice,
            current: 0,
        };
        rr.threads.push(RRInfo {
            valid: false,
            time: 0,
            prev: 0,
            next: 0,
        });
        rr
    }
}
impl Scheduler for RRScheduler {
    fn push(&mut self, tid: Tid) {
        let tid = tid + 1;
        if tid + 1 > self.threads.len() {
            self.threads.resize_with(tid + 1, Default::default);
        }

        if self.threads[tid].time == 0 {
            self.threads[tid].time = self.max_time;
        }

        let prev = self.threads[0].prev;
        self.threads[tid].valid = true;
        self.threads[prev].next = tid;
        self.threads[tid].prev = prev;
        self.threads[0].prev = tid;
        self.threads[tid].next = 0;
    }

    fn pop(&mut self) -> Option<Tid> {
        let ret = self.threads[0].next;
        if ret != 0 {
            let next = self.threads[ret].next;
            let prev = self.threads[ret].prev;
            self.threads[next].prev = prev;
            self.threads[prev].next = next;
            self.threads[ret].prev = 0;
            self.threads[ret].next = 0;
            self.threads[ret].valid = false;
            self.current = ret;
            Some(ret - 1)
        } else {
            None
        }
    }

    // 当前线程的可用时间片 -= 1
    fn tick(&mut self) -> bool {
        let tid = self.current;
        if tid != 0 {
            self.threads[tid].time -= 1;
            if self.threads[tid].time == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    fn exit(&mut self, tid: Tid) {
        let tid = tid + 1;
        if self.current == tid {
            self.current = 0;
        }
    }
}

struct StrideInfo {
    valid: bool,
    time: usize,
    stride: usize,
    pass: usize,
}

pub struct StrideScheduler {
    threads: Vec<StrideInfo>,
    max_time: usize,
    current: Option<usize>,
}

impl StrideScheduler {
    pub fn new(max_time_slice: usize) -> Self {
        let mut schd = StrideScheduler {
            threads: Vec::default(),
            max_time: max_time_slice,
            current: None,
        };
        schd
    }
}

impl Scheduler for StrideScheduler {
    fn push(&mut self, tid: Tid) {
        if tid >= self.threads.len() {
            self.threads.resize_with(tid + 1, || StrideInfo {
                valid: false,
                time: 0,
                stride: crate::consts::STRIDE_BIGSTRIDE,
                pass: crate::consts::STRIDE_BIGSTRIDE / 2,
            });
        }

        let entry: &mut StrideInfo = &mut self.threads[tid];
        if entry.time == 0 {
            entry.time = self.max_time;
        }
        entry.valid = true;
    }

    fn pop(&mut self) -> Option<Tid> {
        let mut min_stride = None;
        for (idx, thread) in self.threads.iter().enumerate() {
            if !thread.valid { continue; }
            match min_stride {
                None => min_stride = Some(idx),
                Some(prev) => {
                    if self.threads[prev].stride > thread.stride
                        || thread.stride - self.threads[prev].stride > crate::consts::STRIDE_BIGSTRIDE {
                        min_stride = Some(idx);
                    }
                }
            }
        }

        if let Some(next) = min_stride {
            self.current = Some(next);
            self.threads[next].stride = self.threads[next].stride.wrapping_add(self.threads[next].pass);
            self.threads[next].valid = false;
            if self.threads[next].time == 0 {
                self.threads[next].time = self.max_time;
            }
        }

        min_stride
    }

    // 当前线程的可用时间片 -= 1
    fn tick(&mut self) -> bool {
        if let Some(tid) = self.current {
            self.threads[tid].time -= 1;
            if self.threads[tid].time == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    fn exit(&mut self, tid: Tid) {
        if self.current == Some(tid) {
            self.current = None;
        }

        self.threads[tid].valid = false;
        self.threads[tid].stride = crate::consts::STRIDE_BIGSTRIDE;
        self.threads[tid].pass = crate::consts::STRIDE_BIGSTRIDE / 2;
    }

    fn set_priority(&mut self, id: Tid, priority: usize) {
        self.threads[id].pass = crate::consts::STRIDE_BIGSTRIDE / priority;
        // println!("PASS: {}", self.threads[id].pass);
    }
}
