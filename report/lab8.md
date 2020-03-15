# Lab8 FS

代码在 New 分支上。

## 父进程还没写东西咋办？

和 Linux syscall 一致的话，不需要特殊处理，可读字节数为 0，所以直接返回 0。具体咋办 child process 自己决定 (sleep, etc...)

但是好像测试样例写的是错的啊...

因此使用另一种行为：不断读，直到读到需要的字节数为止。如果没读到就耗尽，那么 yield\_now。写也是一样。当然在这之前需要 Drop MutexGuard

这里使用了 Lab7 中的 Mutex，工作非常流畅。

## Sync?

如上节所述，`Mutex<Inner>`

## 死锁?

并没有发生过死锁
