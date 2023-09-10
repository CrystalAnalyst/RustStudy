# Rust Note #8：Rust编译器介绍

## 前端（Rust语言是基于LLVM后端实现的语言）

1. 词法分析：rust 文本代码 -> tokenStream          (token:对rust有意义的词条, 也有人把它叫做word, 和token同义)

   Lexical analysis / Scanning  < = > Lexical analyzer / Scanner

2. 语法分析：token -> AST 抽象语法树 Abstract Syntax Tree. 

   Syntax analysis / Parsing    < = > Syntax analyzer  / Parser: 可以不断地调用Scanner,得到后续的token.

   Parser还可以调用elaborator做一些其他的计算，比如创建IR，type-checking等等。

   **词法分析的Scanner和语法分析的Parser应用非常广泛：在HTML、SQL的解析中也常常会看见**

   Parsing分为两种：top-down 和 bottom-up, equally Important.

   形式化表述：如果有一个正式语法G, 一串词语s符合语法G, 则称 G derives s.

   现有一串词语s', Parser尝试去证明s'符合语法G, 这个过程叫做parsing.

3. 语义分析：AST -> LLVM IR (AST -> HIR -> MIR -> LLVM IR)    (IR: Intermediate Represetation)

   Semantic analysis 对 声明式语句 和 过程式语句 进行不同的分析

   1. **HIR阶段**：是AST对编译器更友好的表示形式。主要包含type check, type reference.
   
   2. THIR阶段：是从HIR到MIR的一个过渡中间代码表示。在源码层级属MIR的一部分。
   
   3. **MIR阶段**：是Rust代码的中级中间代表，基于HIR进一步简化构建。主要包含borrow checker.
   
      1. MIR还包含：代码优化、增量编译、Unsafe代码中的UB check、生成LLVM IR等等。
   
      2. MIR是基于控制流图（Control-Flow Graph）的。
      3. 没有嵌套表达式。
      4. MIR的所有类型都是完全明确的，不存在隐形表达。人类也可以读，可通过MIR了解Rust代码的一些行为。
   
   4. **广义的Compiler的分类**
   
      1. 按照组织结构分：
         1. Graphical IR：把中间代码夹带的信息转换为图结构
         2. Linear IR：重组伪指令, 用一串有序的(ordered)指令表示程序, Assembly就是一个典例.
            1. 单地址 / 双地址码
            2. 三地址码：在AST的基础上可以做三地址码的生成。
         3. Hybrid IR：杂交图结构和线性重组的方法
      2. 按照抽象等级分：
         1. near-source Level：更靠近源码
         2. near-machine Level：更靠近机器
      3. 使用模式分：
         1. Definite IR：确定性强的IR, IR的主要形态, 一般传到下一个pass时有确定的IR。 
         2. Derivative IR：用来完成一些特殊/临时的任务,可作为确定IR的补充,一般是在一个pass内部生成。
   
      
   
      
   
      



## 后端（Rust编译器仅仅是一个前端，生成LLVM IR后，交由LLVM来编译生成最终机器码）

1. 编译器后端：LLVM IR -> Machine Code

2. LLVM后端的优点 & 缺点
   1. 支持的平台多，不需要担心CPU Arch、操作系统的问题。（runtime除外）
   2. 优化水平较高，前端只需要生成LLVM IR，交由后端作相应优化即可。
   3. LLVM IR本身比较贴近Assembly，同时也提供了许多ABI层面的定制化功能。
   4. 缺点：LLVM的编译比较慢。（注意分清：**编译过程**和**执行过程**）

3. 编译器后端大致做的事情：扫描IR, 生成目标机器的代码(Target Machine code)

   1. 把 IR -> ISA (Instruction Set Architecture)指令集架构

   2. 指令调度：Instruction Scheduling 确定操作的顺序(在多核并发的今天尤其重要) (NP-complete)

   3. 寄存器的安排：Register Allocation (NP-complete)

      以上三者统称为Code Generation



## Rust的宏展开

1. 声明宏(Declearative Macros)

   1. 在Scanning的时候生成TokenStream遇到Macros时，会使用专门的Macro Parser来解析宏代码，将宏代码展开为TokenSteram，再合并到普通代码生成的TokenStream中。

   2. 相比之下，其他语言都是在语法分析（Parsing）后操作AST来进行宏解析。为什么Rust的宏在Token层面（词法分析 Scanning ）就来解析了？

   3. 因为Rust高速迭代时AST变动频繁，而Scanning相对稳定。

      所以目前Rust的Macro机制都是基于TokenStrema来完成的。

   4. 声明宏完全基于TokenStrema,根据指定的匹配规则将匹配的Token替换为指定的Token从而达到代码生成的目的，仅仅是Token的替换。（类似于正则表达式）

2. 过程宏(Procedural Macros)

   1. 过程宏允许在宏展开的过程中进行任意的计算。
   2. 过程宏还基于一套语言外的AST，经过这一个AST的操作即可实现目的。
   3. 过程宏三件套：
      1. proc_macro2：对proc_macro的封装。
      2. syn：基于proc_macro中暴露的TokenStream API来生成AST。
      3. quote：配合syn，将AST转回TokenStream，回归到普通文本代码生成的TokenStream中。













































