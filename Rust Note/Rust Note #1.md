### Rust Note1 引用与借用

1. 前情提要：由于所有权以及所有权的转让问题，同一个值会被传来传去多次比较麻烦。Rust也可以通过指针或者引用来获取变量。

2. 这里对Rust中变量获取的几种方式进行小结。

   1. let绑定变量。
   2. 所有权转移(不采用引用的情形，单纯的值类型，都要考虑所有权转移)。
   3. **引用：有了引用，就不考虑担心所有权转移，消失或者不消失的问题了。**

3. Borrowing：获取变量的引用。通过Borrowing来获取某个变量并对其进行想要进行的一系列操作。

   ```rust
   fn main() {
       let x = 5;
       let y = &x; //y是x的引用,其实这里是取了变量x的地址赋予y.
       assert_eq!(5,x);  //可以断言x等于5.
       assert_eq!(5,*y); //如前所述,需要用*y来解引(类似于指针的用法).
   } //Rust中，除了引用访问自身的成员函数/数据成员的时候不需要解引，其它时候都需要解引。
   ```

4. 关于Rust与C++中对引用的处理：先看C++, 指针类型没有任何自动功能,而引用“完全自动”。

   ```c++
   auto a = some_val; 	//copy
   auto& b = a;		//ref
   auto* c = &some_val;//pointer (another sort of ref)
   
   b += a; //auto-dereference.
   *c += a;
   
   b.some_method(); //auto-dereference.
   *c.some_method();
   
   c->a, c->some_method; //C++提供的方便指针类型访问自身的成员/方法的"->"运算符.
   ```

   再看Rust, Rust中只有访问自身的成员函数/数据成员的时候才自动解引用(不加*),否则都需要加.

   ```rust
   let val = some_val;
   let my_ref = &val; //ref
   
   my_ref.some_method(); //引用访问自身成员,有自动解引功能(类似于C++中的b,auto-dereference).
   
   let mut a = 1;  //变量a是可变变量,绑定的值为1.
   let mut b = &a; //变量b也是可变变量,是a的一个引用.
   
   *b += 1; //需要解引.
   
   fn my_func(arg: &i32) {...}
   my_func(&a); //根据传入参数类型,需要写上&.
   ```

5. Google的C++编程规范,其中要求C++函数的参数不能是可变引用,只能是不可变(const)引用.

   ```cpp
   void my_func(
   	myType copied,	//copy 
       myType const& immutably_referenced,	//const&,为了避免混淆。
       myType *pointer_referenced	//pointer
   );
   ```

6. 关于Rust的**不可变引用** (Rust的变量默认都是不可变的,只有加了mut才是可变).

   ```rust
   fn main() {
       let s1 = String::from("hello");
       let len = calculate_length(&s1); //&符号即是引用，允许你使用值，但不获取所有权。
       
       println!("The length of '{}' is {}.",s1,len);
   }
   fn calculate_length(s: &String) -> usize {
       s.len()
   } //由于s是对String的引用，所以当s离开作用域时，由于它并不拥有所有权，所以什么都不会发生。
   ```

7. 如果想通过引用来改变原来变量的值，则原变量要是可变的mut，引用即为**可变引用(&mut)**。

   ```rust
   fn main() {
       let mut s = String::from("hello"); //声明变量s是可变变量.
       change(&mut s); //fn change的参数是&mut s. (&mut 可变引用).
   }
   fn change(some_string: &mut String) {
       some_string.push_str(", world!");
   } 
   //tips: 可变引用同时只能存在一个,即同一作用域,同一个变量只能有一个可变引用。
   let mut s = String::from("hello, world!");
   let r1 = &mut s; //r1 is mutable reference to s.
   let r2 = &mut s; //Went wrong, why?
   println!("{}, {}",r1,r2); //borrow checker: r2 is illegal.
   
   //using {} to solve the problem.
   let mut s = String::from("hello");
   {
       let r1 = &mut s;
   } //r1在这里离开了作用域，所以下面可以创建一个新的引用.
   let r2 = &mut s; //Okay.
   ```

8. 在某一作用域内，可变引用与不可变引用不可以同时存在.

   ```rust
   let mut s = String::from("hello");
   
   let r1 = &s; //没问题
   let r2 = &s; //没问题，侧面说明了可以有多个不可变借用，因为没人能改动原数据。
   let r3 = &mut s; //大问题：无法借用可变s，因为它早已被借用了不可变。
   
   //同一时刻，可以拥有任意多个不可变引用&, 或者唯一一个可变引用&mut, 二选一.
   
   //tips:
   fn main() {
       let mut s = String::from("hello");
       let r1 = &s; //俩个不可变引用.
       let r2 = &s; 
       println!("{} and {}",r1,r2); //r1,r2引用的作用域结束->NLL专门找到这种位置.
       //哪种位置？就是这种某个引用在作用域'}'结束前就不能再被使用的代码的位置.
       
       let r3 = &mut s; //Okay的，因为前俩个不可变引用的作用域已结束, &mut可变引用.
       println!("{}",r3);
   }
   ```

9. 变量的作用域 vs 引用的作用域

   1. 变量的作用域：从创建持续到某一个花括号结束 '}'.
   2. 引用的作用域：从创建一直到它最后一次使用的地方？

10. **关于悬挂指针(Dangling References)**

    1. 现有一个指针/引用指向/引用了某个值，但这个值被释放掉了(离开作用域而失效了)，而指向它指针/引用仍然存在，就叫悬挂指针(也就是悬挂引用)。

    2. 在Rust中，**编译器可以确保引用永远也不会变成悬挂指针，即引用总是有效的**。方法如下：

    3. 当你获取数据引用后，rustc可以确保**数据不会在引用结束之前释放。**

       要想释放数据，必须先停止引用的使用。即最先回收的肯定是引用，然后再是值本身。

11. 比对代码，从中理解悬挂指针是怎么被编译器rustc干掉的.

    ```rust
    fn dangle() -> &Stirng {
        let s = String::from("hello");
        & s
    } //返回的是引用，但这个引用的本体已经离开作用域而失效，&s成了悬挂指针，所以会报错.
    
    /*----------------正确的写法----------------*/
    fn no_dangle() -> String {
        let s = String::from("hello");
        s 
    } //没有返回引用，最终String的所有权被转移给外面的caller.
    ```