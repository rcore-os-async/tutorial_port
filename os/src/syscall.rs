use crate::context::TrapFrame;
use crate::process;
use crate::fs::file::FileDescriptorType;
use alloc::sync::Arc;

pub const SYS_OPEN: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_READ: usize = 63;
pub const SYS_FORK: usize = 220;
pub const SYS_EXEC: usize = 221;
pub const SYS_SETPRIORITY: usize = 140;
pub const SYS_TIMES: usize = 153;
pub const SYS_PIPE: usize = 59;

pub fn syscall(id: usize, args: [usize; 3], tf: &mut TrapFrame) -> isize {
    match id {
        SYS_OPEN => sys_open(args[0] as *const u8, args[1] as i32),
        SYS_CLOSE => sys_close(args[0] as i32),
        SYS_READ => unsafe { sys_read(args[0], args[1] as *mut u8, args[2]) },
        SYS_WRITE => unsafe { sys_write(args[0], args[1] as *const u8, args[2]) },
        SYS_EXIT => {
            sys_exit(args[0]);
            0
        }
        SYS_EXEC => sys_exec(args[0] as *const u8),
        SYS_FORK => sys_fork(tf),
        SYS_SETPRIORITY => sys_setpriority(args[0]),
        SYS_TIMES => crate::timer::now() as isize,
        SYS_PIPE => unsafe { sys_pipe(args[0] as *mut _) },
        _ => {
            panic!("unknown syscall id {}", id);
        }
    }
}

fn sys_open(path: *const u8, flags: i32) -> isize {
    let thread = process::current_thread_mut();
    let fd = thread.alloc_fd() as isize;
    thread.ofile[fd as usize]
        .as_ref()
        .unwrap()
        .lock()
        .open_file(unsafe { from_cstr(path) }, flags);
    fd
}

unsafe fn sys_pipe(pipefd: *mut [i32; 2]) -> isize {
    let pipe = Arc::new(crate::fs::pipe::Pipe::new());

    let thread = process::current_thread_mut();
    let rfd = thread.alloc_fd() as isize;
    let wfd = thread.alloc_fd() as isize;
    thread.ofile[rfd as usize].as_ref().unwrap().lock().with_pipe(pipe.clone());
    thread.ofile[wfd as usize].as_ref().unwrap().lock().with_pipe(pipe);

    let pipefd = unsafe { &mut *pipefd };
    pipefd[0] = rfd as i32;
    pipefd[1] = wfd as i32;

    0
}

fn sys_close(fd: i32) -> isize {
    let thread = process::current_thread_mut();
    assert!(thread.ofile[fd as usize].is_some());
    thread.dealloc_fd(fd);
    0
}

fn sys_exit(code: usize) {
    process::exit(code);
}

unsafe fn sys_read(fd: usize, base: *mut u8, len: usize) -> isize {
    if fd == 0 {
        // 如果是标准输入
        unsafe {
            *base = crate::fs::stdio::STDIN.pop() as u8;
        }
        return 1;
    } else {
        let mut thread = process::current_thread_mut();
        assert!(thread.ofile[fd].is_some());
        let mut file = thread.ofile[fd as usize].as_ref().unwrap().lock();
        assert!(file.get_readable());
        match file.get_fdtype() {
            FileDescriptorType::FD_INODE => {
                let mut offset = file.get_offset();
                let s = file
                    .inode
                    .clone()
                    .unwrap()
                    .read_at(offset, core::slice::from_raw_parts_mut(base, len))
                    .unwrap();
                offset += s;
                file.set_offset(offset);
                return s as isize;
            },
            FileDescriptorType::FD_PIPE => {
                let pipe = file.pipe.as_mut().unwrap();
                return pipe.read(base, len) as isize;
            },
            _ => {
                panic!("fdtype not handled!");
            },
        }
    }
}

unsafe fn sys_write(fd: usize, base: *const u8, len: usize) -> isize {
    if fd == 1 {
        assert!(len == 1);
        unsafe { crate::io::putchar(*base as char); }
        return 1;
    } else {
        let thread = process::current_thread_mut();
        assert!(thread.ofile[fd].is_some());
        let mut file = thread.ofile[fd as usize].as_ref().unwrap().lock();
        assert!(file.get_writable());
        match file.get_fdtype() {
            FileDescriptorType::FD_INODE => {
                let mut offset = file.get_offset();
                let s = file
                    .inode
                    .clone()
                    .unwrap()
                    .write_at(offset, core::slice::from_raw_parts(base, len))
                    .unwrap();
                offset += s;
                file.set_offset(offset);
                return s as isize;
            },
            FileDescriptorType::FD_PIPE => {
                let pipe = file.pipe.as_mut().unwrap();
                return pipe.write(base, len) as isize;
            },
            _ => {
                panic!("fdtype not handled!");
            }
        }
        0
    }
}

pub unsafe fn from_cstr(s: *const u8) -> &'static str {
    use core::{slice, str};
    let len = (0usize..).find(|&i| *s.add(i) == 0).unwrap();
    str::from_utf8(slice::from_raw_parts(s, len)).unwrap()
}

fn sys_exec(path: *const u8) -> isize {
    let valid = process::execute(unsafe { from_cstr(path) }, Some(process::current_tid()));
    if valid {
        process::park();
    }
    return 0;
}

fn sys_fork(tf: &mut TrapFrame) -> isize {
    let forked = process::current_thread_mut().fork(tf.clone());
    let tid = process::add_thread(forked);
    // println!("FORK: Return to parent at {:#x}, tid = {}", tf.sepc, tid);
    tid as isize
}

fn sys_setpriority(priority: usize) -> isize {
    process::set_current_priority(priority);
    0
}
