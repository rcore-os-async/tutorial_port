## Lab3 Virtual memory

代码在 lab3 分支下

## 现有的算法有什么问题？

现有的换页算法是 FIFO，个人理解最大的问题是没有充分考虑时间局部性，访问频繁的页面和其他页面在算法实现中是等同考虑的。

其他的问题包括 Belady 现象。

## 代码实现的时候遇到了什么问题

引用一下文档的一句话

> 可以把 https://github.com/rcore-os/rCore\_tutorial 的 master 分支作为起点，逐步完成所有 8 个实验；**也可以按照 tutorial 一步一步完善内核的功能，每完成若干个章节，去做实验作为练习。**两种做实验的方式都是允许的，只要能够通过评测脚本的测试即可。

问题主要在这里（笑。之前一步一步建的 repo 到这里完全没办法把 patch 收进去，只能换 repo 了。

完成 Lab 本身没什么问题，但是我是在完成了 Tutorial 第五章的代码后才切换 Repo 的，在那之前主要的问题是: 写入口的时候加载符号地址的时候，没有用 lui + addi 而是用了 la，结果被编译成了 auipc，直接就变成物理地址了。

为了调试这个 Bug，搭了一下 GDB 相关的东西，还滚挂了一次系统（ArchLinux 太强了
