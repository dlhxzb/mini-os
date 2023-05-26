# 感谢
https://os.phil-opp.com/zh-CN/

### 1. no_std
* 初识no_std，要自行处理panic（panic_handler），在Cargo.toml里配置`panic = "abort"`可禁用栈展开。  
* `#![no_main]`后`extern "C" fn _start()`作为程序入口，用`#[no_mangle]`防止函数重命名  
* x86_64-unknown-linux-gnu，它包含了 CPU 架构 x86_64 、供应商 unknown 、操作系统 linux 和二进制接口 gnu

### 2. 最小内核
引导启动过程:
* 启动时主板ROM固件将会：加电自检，可用内存，CPU及其它硬件预加载
* 寻找一个可引导的存储介质，启动存储在介质开头的第一阶段引导程序（512字节以内），然后加载更长的第二阶段引导程序
* 引导程序 1.决定内核的位置，并将内核加载到内存
* 引导程序 2.还需要将 CPU 从 16 位的实（real mode）模式，先切换到 32 位的保护模式（protected mode），最终切换到 64 位的长模式（long mode），这时所有的 64 位寄存器和整个主内存才能被访问
* 引导程序 3.将BIOS 查询特定的信息，并将其传递到内核
  
固件分两种： BIOS（Basic Input/Output System）和 UEFI（Unified Extensible Firmware Interface），BIOS比较落后，练习都用的BIOS

Multiboot 标准: 一个引导程序标准，在内核文件开头插入要求的数据片段即可，此次实验暂不支持  
  
通常状况下，core crate以预编译库（precompiled library）的形式与 Rust 编译器一同发布，但对于咱们自定义系统需要重新编译，`build-std = ["core", "compiler_builtins"]`  
  
内存相关的memset/memcopy一般需要操作系统相关的标准C库，咱们用`compiler_builtins`提供的即可`build-std-features = ["compiler-builtins-mem"]`
  
安装QEMU 虚拟机  
  
dependencies引入`bootloader`不需要自己编写引导程序，`cargo bootimage`先编译内核在编译引导程序`bootloader`，最终拼接成一个可引导的磁盘映像

`TBD`: 用Moba启动了远程图形界面，但是有一个关于虚表的警告不知道后续有没有影响，好像本地编译QEMU能解决
> GLib-WARNING **: gmem.c:483: custom memory allocation vtable not supported
  
![Alt text](media/image.png)
  
### 3.VGA字符模式
存储器映射输入输出（memory-mapped I/O）可以让我们像操作普通的内存区域一样操作VGA字符缓冲区（0xb8000）  
为避免编译器优化掉这个看似无用的写操作，引入了volatile crate，实际是封装了core::ptr::write(read)_volatile  
  
一个字符单元是一个u16，低位u8：ASCII，高位u8：颜色  

lazy_static可用在no_std，once_cell还不行  
no_std锁有lock_api和spin，这里选用简单易用的spin   

### 4. 测试内核
使用`custom_test_frameworks`自定义测试框架来应对`no_std`，它的工作原理是收集所有标注了 #[test_case]属性的函数，然后将这个测试函数的列表作为参数传递给用户指定的runner函数（test_runner）。它会生成一个main函数来调用test_runner，但由于`no_main`会忽略main函数，我们需要通过 reexport_test_harness_main属性来将生成的函数的名称更改为与main不同的名称（test_main）。然后自行在_start中调用test_main。  
关闭操作系统通常实现对电源管理标准的支持，这里取巧用QEMU支持的一种名为 isa-debug-exit 的特殊设备，可指定端口发退出码可退出QEMU。  
在x86平台上，CPU和外围硬件通信通常有两种方式，内存映射I/O和端口映射I/O。之前用内存映射方式访问VGA文本缓冲区  
端口映射I/O使用独立的I/O总线来进行通信。每个外围设备都有一个或数个端口号。CPU采用了特殊的in和out指令来和端口通信，这些指令要求一个端口号和一个字节的数据作为参数  
  
测试结果输出到串口，再通过bootimage设置串口输出到stdout，很巧妙！  
  
