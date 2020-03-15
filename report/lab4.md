# Lab4 Threads!


## `process::init` 都干了什么

```rust
// src/process/mod.rs

pub fn init() {
    // 获得内核初始化的线程，也就是在这里的当前线程
    let mut boot_thread = Thread::get_boot_thread();

    // 创建了另一个线程
    let mut temp_thread = Thread::new_kernel(temp_thread as usize);

    unsafe {
        // 把 Boot thread 和 Temp thread 的地址作为 temp thread 入口接受到的参数传入
        temp_thread.append_initial_arguments([&*boot_thread as *const Thread as usize, &*temp_thread as *const Thread as usize, 0]);
    }

    // 从 Boot thread 切换到 Temp thread
    boot_thread.switch_to(&mut temp_thread);

    // (Temp thread 已经交回执行流)

    println!("switched back from temp_thread!");

    // 死循环，相当于 halt
    loop {}
}
```

## switch 的时候发生了什么

调用 switch 时的执行流在调用者角度来看，相当于调用了一个普通函数，因此 switch 需要做的事情就是把 **Callee-saved** 寄存器保存在这个线程对应的栈顶，同时保存 ra、页表，然后从被切换的线程的栈顶恢复了这些寄存器（包括 ra 和页表 + sfence.vma），返回时就会返回到切换到的线程上次调用 switch 的位置（或者对于新建的线程而言，入口）

重要的寄存器使用包括：
- a0: 参数，在这里是 &mut self: &mut Context，也就是被换出的线程的 Context，这个指针指向的位置保存了栈顶位置
- a1: 参数，在这里是 target: &mut Context，是被换入线程的 Context
- ra: 返回值地址，切换前是被换出线程调用 switch 的位置，切换后是被换入线程 调用 switch 的地址
- sp: 栈指针，被存进、恢复于 Context，被修改时分配、回收了保存寄存器和其他参数的空间

被暂停线程的栈顶：

- ...
- s11   + 13*8
- s10   + 12*8
- s9    + 11*8
- s8    + 10*8
- s7    + 9*8
- s6    + 8*8
- s5    + 7*8
- s4    + 6*8
- s3    + 5*8
- s2    + 4*8
- s1    + 3*8
- s0    + 2*8
- satp  + 1*8
- ra    + 0*8
