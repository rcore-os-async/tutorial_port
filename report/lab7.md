# Lab7 Sync

代码实现在 lab7 分支底下

## 怎么写的？
Atomic xchg + Memory ordering

根据访存语义，拿锁的时候使用 cmpxchg + Acq，放锁的时候用 store + Rel。

在 Rust 中，可以使用 intrinsics：

- `atomic_cxchg_acq`
- `atomic_store_rel`

当然直接使用 atomic xchg 然后看换出来是不是 0 应该效率更高 (cache coherence protocol 之类的？)，但是当时写代码的时候脑抽了没想到这个...

由于这两个 intrinsic 只能吃进去 simple primitive，因此需要一个 *mut bool -> *mut u8

另外一些细节是，如果一次尝试拿锁失败 ，在我的实现中调用了 yield\_now。比较科学的 Mutex 实现应该是告诉 scheduler 自己等在这个 Mutex 上，但是现在没有类似的基础设施。所以现在其实就是一个 spin lock。但是不知道为什么在 crate::sync 里面导出的时候管这个叫 SleepLock? 是应该用 Condvar 实现嘛

## 为什么要有 `MutexGuard`?

用 Rust RAII (Drop) + lifetime 保证我在拿着锁的时候才能访问里面的数据。更进一步的，这保证了 Rust 里面 Send / Sync 的语义正确。

## 都用 `yield_now` 会怎样？

sleep 和等在 condvar 上的线程会占用 CPU，不断的换入然后又被换出。

## `idle_main` 打开中断？

为了让 sleep 的线程醒过来变得可以被 schedule。这个时候 M 态在收到 timer interrupt 之后应该已经通过软件或者硬件转发给 S 态了，打开一瞬间就可以跳转到 trap handler 里面。

## `spie`?

修改 SPIE 使得除了 idle 的线程执行过程中中断不会打开。在这个测试中，也就意味着线程在执行过程中不会被换出。

本质上而言，就算被换出了也不会影响正确性...因为用了 Atomic operations，所以就算在 Mutex 里面也没有 Critical section。（用户态 Spinlock 的自我修养？

（事实上我我把 spie 打开跑了几次也没有问题，甚至在写 lab8 的时候忘了打开了也没有什么问题（（（（
