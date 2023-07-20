### Rust Note #6 Rust的项目、包和模块

1. 包Crate

   1. 包是一个**独立的可编译单元**，包经过编译后生成一个**二进制可执行文件**或者一个**第三方库**以**分发**。

   2. 一个包将多个相关联的功能打包在一起，可以**用于在多个其他项目中分享**。

   3. ```rust
      use rand;
      rand::XXX //通过use导入Crate包，再通过::符进行功能调用。
      ```

   4. 同一个包中**不能有同名的类型**，在不同包中就可以。

2. 项目：Rust中Package叫做一个项目

   1. 包被Crate占用，库被library占用，斟酌后把Package作“项目“，一个Package就是一个工程。

   2. ```bash
      $ cargo new my-project
           Created binary (application) `my-project` package
      $ ls my-project
      Cargo.toml //一个Package包含独立的Cargo.toml文件,以及因为功能性被组织在一起的以一个或多个包。一个Package只能包含一个库类型的包，但是可以包含多个二进制可执行类型的包。
      src
      $ ls my-project/src
      main.rs
      ```

   3. 库类型的Package(Crate)只能被其他项目当做第三方库来引用,而不能自己独立运行。

   4. 而只有之前的二进制Package才可以运行  √

   5. 一个典型的Rust项目目录结构

      ```
      . # 当前目录
      ├── Cargo.toml # 当前package下的配置文件 
      ├── Cargo.lock # 自动修改的Package版本追踪文件
      ├── src # Rust源代码目录
      │   ├── main.rs # 默认二进制包,是所有二进制包的根文件,该二进制包名与所属Package相同。
      │   ├── lib.rs  # 唯一库包，每一个Package只能包含一个库包。该包的根文件是src/lib.rs。
      │   └── bin		# 存放其余二进制包的目录 
      │       └── main1.rs # 二进制包，编译后会生成一个“文件同名”的二进制可执行文件
      │       └── main2.rs # 同上
      ├── tests # 测试目录
      │   └── some_integration_tests.rs # 集成测试的代码文件
      ├── benches # 基准性能测试目录
      │   └── simple_bench.rs   # 基准性能测试文件
      └── examples # 工程案例目录
          └── simple_example.rs # 案例文件
      ```

3. 模块：

   1. 模块树：用于描述与展示模块之间的嵌套关系。

      ```
      crate # crate的root是src/lib.rs文件。
       └── front_of_house # 餐厅前厅模块（父模块）
           ├── hosting # 前厅的接待模块
           │   ├── add_to_waitlist
           │   └── seat_at_table
           └── serving # 前厅的服务用餐模块
               ├── take_order
               ├── serve_order
               └── take_payment
      ```

   2. Rust的模块树与文件系统目录树很相似。

      1. 结构上的相似，都可以画树形图，一目了然。

      2. 用法上的相似，文件系统目录通过文件**路径**来检索，而模块也一样。

         1. 在Rust中，可以用路径调用模块，进而调用特定的函数。

            1. 绝对路径：从crate root开始，路径名以包名或者crate作为开头。
            2. 相对路径：从当前模块开始，以**self，super或者当前模块的标识符**作为开头。

         2. 路径使用原则：

            1. 当代码被挪动位置时，尽量减少引用路径的修改。
            2. 一种最差的情况是：修改了某处的代码，导致所有路径都要挨个替换。尽量避免这种情况。
            3. 优先考虑绝对路径。

               1. 因为调用的地方和定义的地方往往是分离的。
               2. 调用的地方可能很多，情况复杂。而定义的地方一般不会变动。
         
         3. **Rust的“默认私有原则”**
         
            1. Rust出于安全的考虑，**默认情况下，所有的类型都是私有化的（private）。**
         
               包括但不限于函数、对象/方法、结构体/枚举、常量、模块等等
         
            2. 在Rust中
         
               1. **父模块完全无法访问子模块的私有项。**
               2. **但是子模块却可以访问父模块、父模块的父模块的私有项。**

            3. pub关键字

               ```rust
               mod front_of_house {
                   pub mod hosting {   //通过pub指定hosting模块可见性。
                       pub fn add_to_waitlist() {} //通过pub指定模块内部项的可见性。    
                   }
               }
               ```
         
            4. 关于可见性
         
               1. 模块可见性：
         
                  1. 仅仅允许其他模块去引用它
                  2. 不代表模块内部项的可见性

               2. 内部可见性

                  1. 想要引用模块内部的项（函数），
         
                     要继续将对应的项标为pub
         
            5. 使用super引用模块
         
               1. super代表以**父模块为开始**的引用方式。
         
                  类似于文件系统中的..语法
         
               2. ```rust
                  fn serve_order() {}
                  // 厨房模块
                  mod back_of_house {
                      fn fix_incorrect_order() {
                          cook_order();
                          super::serve_order(); //调用父模块中的serve_order函数.
                      }
                  
                      fn cook_order() {}
                  }
                  ```
         
            6. 使用self引用模块
         
               1. self就是引用自身模块的项。
         
               2. ```rust
                  fn serve_order() {
                      self::back_of_house::cook_order() //用self调用自身模块中的项
                  }
                  
                  mod back_of_house {
                      fn fix_incorrect_order() {
                          cook_order();
                          crate::serve_order();
                      }
                  
                      pub fn cook_order() {} //后厨模块中有一个cook_order项.
                  }
                  ```
         
            7. 结构体/枚举的可见性
         
               1. **将结构体设置为pub，但它的所有字段依然是私有的。（结构体成员的可见性独立）**
               2. **将枚举设置为pub，它的所有字段也跟着变为pub，对外界可见。（枚举成员可见性一致）**

            

         4. 模块与文件分离
         
            1. 之前我们的模块都定义在src/lib.rs中，但当模块很多的时候，
         
               可以**把这些模块放入一个单独的文件中**，增加代码的可维护性。
         
            2. 关键字use：概念介绍
         
               1. use关键字用于**把外部模块的项引入到当前作用域来。**这样就无需冗长的父模块前缀即可调用某个函数/某个模块内的项。
         
               2. 当一个模块有许多个子模块时，也**可以通过文件夹的方式来组织这些子模块**。
         
                  1. 如果你想要将文件夹作为一个模块，你需要**显式指定暴露哪些子模块**。
         
                  2. 方法是：在该文件夹同级目录里建一个与模块（模块文件夹/模块目录）同名的rs文件。
         
                  3. 模块的声明与实现大多数时候是分离的。
         
                     声明（定义）的话放在src/lib.rs中,也就是库的crate root中。
         
                     具体实现的话放在单独的文件中，然后通过mod XXX,从该文件中把模块内容加载进来
         
            3. **使用use引入模块或者函数。**
         
               ```rust
               mod front_of_house {
                   pub mod hosting {
                       pub fn add_to_waitlist() {}
                   }
               }
               
                //用use通过绝对路径引入模块。
               use crate::front_of_house::hosting;
               
               pub fn eat_at_restaurant() {
                   hosting::add_to_waitlist(); //使用的引入模块，当然能不能用里面的函数另说。
                   hosting::add_to_waitlist();
                   hosting::add_to_waitlist();
               }
               
               //用use通过相对路径引入模块内的项（函数）。
               use front_of_house::hosting::add_to_waitlist; 
               
               pub fn eat_at_restaurant() {
                   add_to_waitlist(); //直接使用函数
                   add_to_waitlist();
                   add_to_waitlist();
               }
               ```
         
               **到底引用模块还是函数？**
         
               1. 以下情况引用模块会更好	
         
                  1. 需要引用同一个模块的多个函数。
                  2. （当前）作用域中存在同名函数。
         
               2. 其他情况，处于简洁性，引用函数即可。
         
               3. 如何判断它引用的函数还是模块？
         
                  1. 名词的话，一般是模块。
                  2. 动宾结构，一般是函数。
                  3. !:如果是名词的话，还有可能是结构体，**所以还得根据调用的方式来看。**
         
               4. 优先使用最细粒度（引入函数、结构体等的引入方式。例如引入collections模块中的HashMap）。
         
                  ```rust
                  use std::collections::HashMap;
                  /*
                  	在此，只用到了collections集合模块中的HashMap结构。
                  	所以直接引用结构体是最好的。（函数、结构体都是模块内的项）
                  */
                  fn main() {
                      let mut map = HashMap::new(); //new()函数是结构的关联函数。
                      map.insert(1, 2);
                  }
                  ```
         
               **避免同名引用的两种方式**
         
               Solution1:通过使用父模块来调用
         
               ```rust
               use std::fmt
               use std::io
               
               fn function1() -> fmt::Result {
                   // --snip--
               }
               
               fn function2() -> io::Result<()> {
                   // --snip--
               }
               ```
         
               Solution2:给予所引入的项一个别名：**通过as关键字解决同名问题。**
         
               ```rust
               use std::fmt::Result;
               use std::io::Result as IoResult;
               
               fn function1() -> Result {
               	// --snip--
               }
               fn function2() -> IoResult<()> {
                   // --snip--
               }
               ```
         
               
         
               **把引入项再导出**
         
               ```rust
               mod front_of_house {
                   pub mod hosting {
                       pub fn add_to_waitlist() {}
                   }
               }
               
               pub use crate::front_of_house::hosting;
               /*
               	当外部项被引入到当前模块中时，自动被设置为私有的（private）。
               	如果你希望允许其它外部代码引用我们的模块项A，可以对它进行再导出。
               	实现的方式是：pub use
               	use代表引入hosting模块到当前作用域。
               	pub表示将该引用的内容再度设置为可见。
               */
               pub fn eat_at_restaurant() {
                   hosting::add_to_waitlist();
                   hosting::add_to_waitlist();
                   hosting::add_to_waitlist();
               }
               ```
         
               pub use（引入再导出）常用于你希望**将内部的实现细节隐藏起来**或者按照某个目的组织代码时。
         
               现在我用**一个统一的模块来提供对外的API**，那么该模块通过pub use可以引入其他模块中的API，进行再导出。
         
               最终对于用户来说，所有的API都是由这个模块统一来提供的。
         
               
         
               **使用第三方包的步骤**
         
               1. 修改.toml文件的[dependecies]，加一行：包名 = 版本号
         
               2. 在src/下的.rs文件中,use引用包之后就可以使用啦！
         
                  ```rust
                  use rand::Rng;
                  fn main() {
                      let secret_number = rand::thread_rng().gen_range(1..101);
                  } //使用use引入第三方包rand中的Rng特征(Trait),因为我们需要调用的.gen_range方法定义在该特征中。
                  ```
         
               
         
               **哪里去找第三方包？**
         
               1. 在crates.io下载依赖包。
         
               2. 在lib.rs检索查找使用包（用法、解释等）。
         
               3. 使用{}**简化**引入方式
         
                  ```rust
                  //未简化之前
                  use std::collections::HashMap;
                  use std::collections::BTreeMap;
                  use std::collections::HashSet;
                  
                  use std::cmp::Ordering;
                  use std::io;
                  
                  //简化之后
                  use std::collections::{HashMap,BTreeMap,HashSet};
                  use std::{cmp::Ordering, io};
                  
                  //对于下面的同时引用模块和模块中的项
                  use std::io;
                  use std::io::Write;
                  //可以简化为
                  use std::io::{self, Write};
                  ```
         
               1. 总结：关于self的两个用途（self可以用来替代模块自身）
         
                  1. use self::xxx, 表示加载当前模块中的xxx, 此时self可以省略。
                  2. use xxx::{self, yyy}, 表示加载路径下模块xxx本身,以及模块xxx下的yyy。如上面的use std::io::{self, Write}
         
               2. 使用*来引入模块下的所有项，多用于写test测试代码。
         
                  ```rust
                  use std::collections::*;
                  //tips：当心“名称冲突”问题，本地同名类型的优先级更高。
                  //一个例子如下：
                  
                  struct HashMap; //本地类型HashMap. 
                  fn main() {
                     let mut v =  HashMap::new(); //编译错误,HashMap没有new()函数.
                     v.insert("a", 1);
                  }
                  ```
         
               11. 可见性pub：控制哪些内容能被外部看见（即能被外部模块所导入）。
         
                   **受限可见性：控制哪些人能看 ** 类似于权限控制(Authorization).
         
                   ```rust
                   //目标：由模块'a'导出'I'，'bar','foo', 小秘密函数不想对外可见（对外可见了还叫小秘密吗？）
                   //一种妥协的方案。
                   pub mod a { // a is a module, and can be seen by other modules(pub).
                       pub const I: i32 = 3;
                       use self::b::semisecret; //a用了它的子模块b的semisecret项,
                       
                       pub fn bar(z: i32) -> i32 {
                           semisecret(I) * z
                       }
                       pub fn foo(y: i32) -> i32 {
                           semisecret(I) + y
                       }
                       
                       mod b { //注意看,b包含在a的大括号内，b模块是a模块的子模块.
                           pub use self::c::semisecret; //而b模块是用了c的semisecret项,通过pub导出.
                           mod c {
                               const J: i32 = 4;
                               pub fn semisecret(x: i32) -> i32 {
                                   x + J  //由于使用了pub设置为可见,所以现在这样a可以调用到c的项
                               }
                           }
                       }
                   }
                   ```
         
                   ```rust
                   //目标：由模块'a'导出'I'，'bar','foo', 小秘密函数不想对外可见（对外可见了还叫小秘密吗？）
                   //使用pub(in crate::)完美实现目的.
                   pub mod a {
                       pub const I: i32 = 3;
                   
                       fn semisecret(x: i32) -> i32 {
                           use self::b::c::J; //父模块通过链式调用子模块的项(当然要有前提)
                           x + J
                       } //在此,小秘密函数是放在a模块里的,而且这个函数对外不可见（没有pub）.
                   
                       pub fn bar(z: i32) -> i32 {
                           semisecret(I) * z
                       }
                       pub fn foo(y: i32) -> i32 {
                           semisecret(I) + y
                       }
                   
                       mod b {
                           pub(in crate::a) mod c {
                           /*  通过 pub(in crate::a) 的方式，我们指定了模块 c 和它的内部项常量 J 的可见范围都只是 a 模块中，
                               a 之外的模块是完全访问不到它们的。
                           */   
                               pub(in crate::a) const J: i32 = 4;
                           }
                       }
                   }
                   ```
         
               12. **一个综合的例子：关于包，模块和可见性。**
         
                   ```rust
                   // 一个名为 `my_mod` 的模块
                   mod my_mod {
                       // 模块中的项默认具有私有的可见性
                       fn private_function() {
                           println!("called `my_mod::private_function()`");
                       }
                   
                       // 使用 `pub` 修饰语来改变默认可见性。
                       pub fn function() {
                           println!("called `my_mod::function()`");
                       }
                   
                       // 在同一模块中，项可以访问其它项，即使它是私有的。
                       pub fn indirect_access() {
                           print!("called `my_mod::indirect_access()`, that\n> ");
                           private_function();
                       }
                   
                       // 模块也可以嵌套
                       pub mod nested {
                           pub fn function() {
                               println!("called `my_mod::nested::function()`");
                           }
                   
                           #[allow(dead_code)]
                           fn private_function() {
                               println!("called `my_mod::nested::private_function()`");
                           }
                   
                           // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
                           // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
                           pub(in crate::my_mod) fn public_function_in_my_mod() {
                               print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
                               public_function_in_nested()
                           }
                   
                           // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
                           pub(self) fn public_function_in_nested() {
                               println!("called `my_mod::nested::public_function_in_nested");
                           }
                   
                           // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
                           pub(super) fn public_function_in_super_mod() {
                               println!("called my_mod::nested::public_function_in_super_mod");
                           }
                       }
                   
                       pub fn call_public_function_in_my_mod() {
                           print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
                           nested::public_function_in_my_mod();
                           print!("> ");
                           nested::public_function_in_super_mod();
                       }
                   
                       // `pub(crate)` 使得函数只在当前包中可见
                       pub(crate) fn public_function_in_crate() {
                           println!("called `my_mod::public_function_in_crate()");
                       }
                   
                       // 嵌套模块的可见性遵循相同的规则
                       mod private_nested {
                           #[allow(dead_code)]
                           pub fn function() {
                               println!("called `my_mod::private_nested::function()`");
                           }
                       }
                   }
                   
                   fn function() {
                       println!("called `function()`");
                   }
                   
                   fn main() {
                       // 模块机制消除了相同名字的项之间的歧义。
                       function();
                       my_mod::function();
                   
                       // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
                       my_mod::indirect_access();
                       my_mod::nested::function();
                       my_mod::call_public_function_in_my_mod();
                   
                       // pub(crate) 项可以在同一个 crate 中的任何地方访问
                       my_mod::public_function_in_crate();
                   
                       // pub(in path) 项只能在指定的模块中访问
                       // 报错！函数 `public_function_in_my_mod` 是私有的
                       //my_mod::nested::public_function_in_my_mod();
                       // 试一试 ^ 取消该行的注释
                   
                       // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的
                   
                       // 报错！`private_function` 是私有的
                       //my_mod::private_function();
                       // 试一试 ^ 取消此行注释
                   
                       // 报错！`private_function` 是私有的
                       //my_mod::nested::private_function();
                       // 试一试 ^ 取消此行的注释
                   
                       // 报错！ `private_nested` 是私有的
                       //my_mod::private_nested::function();
                       // 试一试 ^ 取消此行的注释
                   }
                   
                   ```











