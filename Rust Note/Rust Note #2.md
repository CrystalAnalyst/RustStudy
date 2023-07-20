### Rust Note2 特征与特征对象

1. Trait之于Rust, Interface之于Go.

2. 创建特征对象的两种方法

   1. 通过&引用创建,类型声明为: &dyn xxx.
   2. 通过Box<T>智能指针的方式创建: Box<dyn xxx>，智能指针所包裹的内容存放在堆上。
   3. dyn关键字用于特征对象的**类型声明**上,在创建时不需要使用dyn.（**特征对象在执行时的动态分配**）
   4. 可以用特征对象来代表泛型或者具体的类型，一旦代表，则该类型完全成为该特征的实例，只能调用实现于该特征的方法。

3.  特征对象的一些性质

   1. 特征对象大小不固定.
   2. 几乎总是使用**特征对象的引用方式.**
      1. 因为特征对象大小不固定,但它的**引用大小固定。由两个指针：ptr和vptr组成，因此占用两个指针大小。**
      2. ptr（数据指针）指向实现特征Trait的具体类型实例。
      3. vptr（行为指针）指向虚表vtable，vtable保存的是实现了特征Trait的各实例可以调用的实现于Trait的方法。当这些实例当作Trait来用的时候，有必要区分这些实例各自有哪些方法可以使用，再次强调这些方法**是只包含实现自特征Trait的方法，原本类型方法失效。**
      4. 当某个类型实例成为特征对象的实例后，**它不再是具体类型的实例**，而且只能调用基于该特征的方法。

4. Rust中的两个self

   1. Self：指代特征或方法类型的别名
   2. self：指代当前的实例对象

   ```rust
   trait Draw {
       fn draw(&self) -> Self; //Self指代的是实现Draw特征的类型
   }
   
   #[derive(Clone)] //附加在结构体上的宏,表明Button类型可以实现深复制Clone.
   struct Button;
   impl Draw for Button {
       fn draw(&self) -> Self { //Button实现了Draw特征，那么这里的Self就是Button
           return self.clone()  //self指代的是button实例,Rust的对象在访问自身成员/数据时自动解引
       }
   }
   
   fn main() {
       let button = Button;
       let newb = button.draw(); //这里的button就是函数中的self, &self表明传的是引用.
   }
   ```

5. 特征对象的限制（**特征对象的安全规则**）

   1. 只有对象安全的**特征**（强调一个**必须**）才能拥有特征对象。“对象安全”定义如下：

      1. 对象的方法的返回类型不能是Self
      2. 对象方法里没有任何泛型参数
      3. tips：特征对象是基于特征来说的，是特征的owning。

   2. 原因在于

      1. 一旦有了特征对象，就可以忘记实现该特征的具体类型，如果返回值是Self，那么我如何知道是哪个类型呢？所以对象方法的返回类型不能是Self。

      2. 对于泛型类型参数来说，使用特征时其会放入具体的类型参数（编译时静态分配）。

         此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时，其具体类型是会被抹去的（执行时动态分配），所以无从得知实现该特征的完整类型。（类型的完整性缺失）

6. 特征 与 泛型似乎是编码时的“二选一”抉择？

7. 特征的关联变量

   ```rust
   trait Add<RHS=Self> { //RHS叫做泛型参数,默认值为Self.
       type Output; //Output定义在trait里面,叫做关联变量.
   
       fn add(self, rhs: RHS) -> Self::Output; //可以在特征中使用关联变量作参数/返回值.
   }
   
   ```

8. 默认泛型类型参数

   ```rust
   use std::ops::Add;
   
   #[derive(Debug, PartialEq)]
   struct Point {
       x: i32,
       y: i32,
   }
   
   impl Add for Point {
       type Output = Point;
   
       fn add(self, other: Point) -> Point {
           Point {
               x: self.x + other.x,
               y: self.y + other.y,
           } //为Point结构体提供+的能力,这就是运算符重载 operator overload .
           //Rust不支持创建自定义运算符,也无法为所有的运算符进行重载,
           //只有定义在std::ops中的运算符才能进行重载.
       }
   }
   
   fn main() {
       assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
                  Point { x: 3, y: 3 });
   }
   ```

























