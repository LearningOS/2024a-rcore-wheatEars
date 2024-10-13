# 实验报告
1. 修改了Task Control Block 的 inner 部分，增加了记录syscall_times、start_time的域，分别在创建TCB和第一次调入PCB时更新。
2. 为TCB结构体实现了设置start_time的函数，第一次被调度时将start_time设置为Some(_)。
3. 为TASK_MANAGER实现了获取当前信息的四个方法和记录一次syscall的方法，并在task模块封装为接口。
4. 在trap分发部分新增记录syscall的逻辑。
5. 实现sys_task_info的逻辑。
   
# 简答题
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
```bash
git checkout ch2
cd os
make BASE=1 run
```
运行问题测例，在U态使用S指令出现以下输出：
```bash
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
[kernel] num_app = 7
[kernel] app_0 [0x8020a048, 0x8020e0f0)
[kernel] app_1 [0x8020e0f0, 0x80212198)
[kernel] app_2 [0x80212198, 0x80216240)
[kernel] app_3 [0x80216240, 0x8021a2e8)
[kernel] app_4 [0x8021a2e8, 0x8021e390)
[kernel] app_5 [0x8021e390, 0x80222438)
[kernel] app_6 [0x80222438, 0x802264e0)
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
```
报段错、非法指令错。

2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
   1. L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。  
    a0是要恢复的trap上下文地址。
   __restore分别在trap控制流返回和第一次加载task时调用。

    2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
    分别处理了sp、sstatus、spec和sscratch。
    sp指向了Trap上下文的位置，以此读取原先的寄存器。
    sstatus保存了Trap前的特权级,将其设置为U以进入用户态。
    spec记录了Trap处理完后要执行的下一条指令地址。
    sscratch指向了用户栈的栈顶，用于从内核栈切换到用户栈。

    3. L50-L56：为何跳过了 x2 和 x4？
    x2是sp寄存器，从sscratch获取用户栈地址;x4寄存器tp应用程序不使用。

    4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    sp指向内核栈顶， sscratch指向用户栈顶。

    5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
    sret。sret从S态转回U态。

    6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
    sp指向内核栈顶，sscratch指向用户栈顶。
    
    7. 从 U 态进入 S 态是哪一条指令发生的？
    ecall。


1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无      

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。