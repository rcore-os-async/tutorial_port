# rCore Lab 2 - report

> 2017011426 刘晓义

这是在旧 Repo 上完成的 Lab。参考 Commit 在 legacy branch 上

## 如何让 OS 知道物理内存范围

Bootloader 一般会提供，在 RV 上的 S 态操作系统应该由 M 态的软件提供，例如 OpenSBI 提供的 Device Tree 中会包含。

如果 Bootloader 没有提供，可能需要手动探测。按照 RV 的说明，如果一个读操作访问了非法地址，应该会出现 Illegal address exception，通过这个可以探测到内存的大小和可用区域。当然这个没有办法判断外设和内存的区别，只能判断出可用的地址，同时读操作本身可能会对外设带来副作用，破坏系统的状态。所以可以给出一个最小需要的内存大小，然后从一个固定地址开始探测。

## FirstFitAlloc
在 Commit 15a9d93 中包含了这部分代码。因为目录结构稍有不同，因此修改了测试脚本和 testcase，相关的代码修改也在同一个 commit 内。

实现角度，直接开了个大数组，存每个页是否占用，然后 O(n^2) 搜索。
