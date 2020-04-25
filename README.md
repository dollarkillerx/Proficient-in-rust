# Ownership 所有权

### 内存管理
- 垃圾回收GC (java,go,php,py ...)
    - 在程序运行时不断寻找不再使用的内存
- 手动管理 (c,c++ ...)
    - 程序员必须亲自分配和释放内存
- rust
    - 通过所有权系统管理内存，编译器在(编译时)会更具一系列的规则进行检查
        - 在运行时  所有权系统任何功能都不会减慢程序



## 前置知识

### 堆(Heap)栈(Stack)
> RUST 中 在编译时 数据类型是固定的 大小是固定的 是分配在栈上的

栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。 在rust中非常重要
#### 栈
> 栈的操作是十分快速的，这主要是得益于它存取数据的方式：
>因为数据存取的位置总是在栈顶而不需要寻找一个位置存放或读取数据。
>另一个让操作栈快速的属性是，栈中的所有数据都必须占用已知且固定的大小。

#### 堆
编译时大小未知或大小可能变化的数据，要改为存储在堆上
> 堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），有时简称为 「分配」（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。
  
### 所有权系统
> 想象一下去餐馆就座吃饭。当进入时，你说明有几个人，餐馆员工会找到一个够大的空桌子并领你们过去。如果有人来迟了，他们也可以通过询问来找到你们坐在哪。
 
> 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。
 
> 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。
 
> 跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的存在就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。

### 所有权规则
- Rust中每一个值都有一个被称为 所有者(owner) 变量
- 值有且只有一个所有者
- 当有所者 (变量) 离开作用域，这个值将被丢弃

``` 
{                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的       「入栈」 硬编码的string是存储在内存的代码段当中

    // 使用 s
}                      // 此作用域已结束，s 不再有效  「出栈」
此处 数据存储在 栈上  
```
#### String 类型
需要寻找一个存储在堆上的数据来探索 Rust 是如何知道该在何时清理数据的。

这里使用 String 作为例子，并专注于 String 与所有权相关的部分。这些方面也同样适用于标准库提供的或你自己创建的其他复杂数据类型。在第八章会更深入地讲解 String。

String。这个类型被分配到堆上，所以能够存储在编译时未知大小的文本
``` 
let s = String::from("hello");

s.push_str(", world!"); // push_str() 在str后追加字面值
```
- String 内存分配 
    - 申请内存
    - 操作内存 
> 在有 垃圾回收（garbage collector，GC）的语言中， GC 记录并清除不再使用的内存，而我们并不需要关心它。没有 GC 的话，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。我们需要精确的为一个 allocate 配对一个 free。

Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
``` 
{
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
}                                  // 此作用域已结束，
                                   // s 不再有效
```
> 这是一个将 String 需要的内存返回给操作系统的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

## 核心
- 堆拷贝
    - `clone()` 深拷贝
    - 默认浅拷贝
- 栈拷贝
    - 均为深拷贝  (没有指针 不存在浅拷贝这一说法)
- 引用
- 借用
#### 栈拷贝 前拷贝  导致的所有权  交出 (s1)
 ``` 
fn heap1() {
    let name = String::from("name"); // String 为不固定大小 so:  存在内存的堆中
    print_name(name);
    println!("you name: {}",name);  // 此处name 失去来所有权  无法被调用

}

fn print_name(name:String) { //   这里name 做来浅拷贝  (rust 前拷贝 会导致所有权转移)
    println!("you name: {}",name)
} 
```
#### 栈拷贝 前拷贝  导致的所有权  交出 (s2)
``` 
fn heap2() {
    let name = String::from("name");
    let name2 = name;  //  此处 name 的所有权 交给 name2 了 name被移出了
    println!("name: {}",name);
    println!("name2: {}",name2);
}
```
#### 栈拷贝 前拷贝  导致的所有权  交出 (s3)
``` 
fn heap4() {
    let name = String::from("name");
    let name2= heap4_name(name); // 再次获得所有权
    println!("name: {}",name2);
}

fn heap4_name(name:String) -> String {
    name  // 所有权 再 交出
}
```
#### 栈拷贝 前拷贝  导致的所有权  交出 (s4)
``` 
fn heap4() {
    let mut name = String::from("name");
    name = heap4_name(name); // 再次获得所有权
    println!("name: {}",name);
}

fn heap4_name(name:String) -> String {
    name  // 所有权 再 交出
}
```
#### clone
``` 
fn heap3() {
    let name = String::from("name");
    let name2 = name.clone();
    println!("name: {}",name);
    println!("name2: {}",name2);
}
```
####  引用
``` 
fn heap5() {
    let name = String::from("dollarkiller");
    heap5_print(&name); //  引用
    println!("you name: {}",name);  
}

fn heap5_print(name:&String) {
    println!("you name: {}",name);
}
```

#### 借用
``` 
fn heap6() {
    let mut name = String::from("hello");
    heap6_modify_name(&mut name); // 借用
    heap6_print(&name);
    println!("name: {}",name);
}

fn heap6_print(name:&String) {
    println!("name: {}",name);
}

fn heap6_modify_name(name:&mut String) {
    name.push_str(", rust");
}
```
#### 借用 引用
``` 
fn heap7() {
    let mut name = String::from("dollarkiller");
    let a = &name;
    let b = &name;
    println!("a: {} b: {} ",a,b);
    let c = &mut name;
    println!("c: {}",c);
    // println!("a: {} b: {} c: {}",a,b,c);  当 存在借用的是哈     下面不能出现 引用
    // println!("a: {} b: {} ",a,b);
}
```

###  补充: rust默认  数据存储
- 堆
    - 类型不固定
    - 长度不固定
- 栈
    - cha
    - int double float bool ...
    - list[]  前提 list中的数据为上面的数据类型
    - tuple   前提 list中的数据为上面的数据类型