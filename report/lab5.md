# Lab5 Process w/ friends

代码位于分支 Lab5

真难啊

遇到最大的问题是，rCore\_tutorial 本身其他部分对 fork 没有描述，然后我也没看 rCore，因此首先要猜测 fork 到底怎么实现:

- 和 Linux 类似，fork 可以完成进程级别隔离。这个时候需要 CoW，要不然真的太慢
- fork 不进行进程级别隔离，但是完成线程级别隔离。这个时候需要复制栈，因为栈上可能有被引用数据，不能直接改个 sp 然后 call that finished
- fork 不进行进程级别隔离，并且栈也不复制，而是直接新分配一个栈空间，然后改 sp。感觉能过测试样例？

最后是第二种。需要在 Thread 内保存 MemorySet，并且对 MemorySet 添加一个方法：过滤掉栈，并且原样复制其他的映射。

之后只需要在栈虚拟地址的地方映射新的物理页就可以了。

中间遇到的一些坑：
- 尝试不保存 MemorySet，而是通过直接复制页表。因为用户栈的虚拟地址是固定的，这样就可以把 MemorySet 用完了之后扔掉。结果发现 Rv39PageTable 并没有提供递归克隆，所以没办法做到这件事情。
- 新线程 tf.spec += 4，然后各种跑飞。结果发现抓到的 tf 在进行 syscall 流程前就已经加过了。
