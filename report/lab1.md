# rCore Lab 1 - report

> 2017011426 刘晓义

> 注意！在完成 Lab1 和 Lab2 的时候，我还在从零开始搭建 rCore 的目录结构，在 Lab3 的时候切换到了 rCore\_tutorial/master 上。旧的代码保留在 legacy 分支上。

根据第一次实验的说明，跟随 rCore tutorial 搭建了基本的项目结构，没有使用 `rCore\_tutorial` 中的代码，因此会和 `rCore_tutorial` 的项目在细节上稍有不同，包括：

- 项目结构稍有不同，所有汇编代码包含在 `src/asm` 目录下，linker script 直接包含在 `src` 目录下
- 没有单独拆分一个 Rust lib 出来
- Linker 的选项在 `src/main.rs` 内通过 attribute 提供
- 命名区别

## rCore 中断异常处理流程

首先，异常可能来自 M-mode 的软件 delegate，来自硬件 delegate，或者来自 U-mode 的调用，等等。 

RV 核在 S 态发生异常的时候，检查 sstatus 的条件（对于中断，是否开启了 sie），如果符合，跳转到 stvec 指定的地址，在 rCore 中我们将 SIE 置 1，stvec 设置到汇编编写的 trap handler，因此 RV 核开始执行这一 trap handler 的代码

在 trap handler 一开始保存现场，保存部分和异常有关的 CSR，然后调用 Rust 代码。Rust 代码中针对不同异常进行不同处理（目前实现了 Breakpoint，Timer），返回时回到 trap handler 中，之后 trap handler 恢复现场，通过 SRET 跳转回原先的执行流。

## 是否需要保存所有寄存器？

感觉不一定？

需要的例子包括外部中断打断了 U-mode 的执行，这个时候必须保存，因为随后要回到原先的执行流中

不需要的例子可能包括：某个进程在执行时发生了 illegal instruction address 异常，来自 U 态，我们可以选择直接将这个进程杀死，这个时候不需要保存寄存器。

## Illegal instruction in action

额外加一个 match arm 即可，同时根据 ISA，RV 核可选实现将出现异常的指令存入 xtval 中：

在 `src/main.rs` 中第 26 行的注释，执行得到一下输出

```
> Setting up interrupt
> Setting up timer
BREAKPOINT at 0x0x80200dee
panicked at 'ILLEGAL INSTRUCTION at 0x0x80200df0: 0x30200073', src/interrupt.rs:51:5
```

其中 `0x30200073 = 0011000 00010 00000 000 00000 1110011 = MRET`
