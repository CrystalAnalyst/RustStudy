fn main() {
    let s = String::from("hello");
    takes_ownership(s); //将值传递给函数，会发生移动或者复制，同let语句。
    //s的值被移到了函数中，所有权转移给了some_string,所以到这里s已经失效。
    //println!("在move进函数后继续使用s:{}",s);  报错
    let x  = 5;
    makes_copy(x); //将x传递给函数，x是i32类型的，发生复制(copy),所以不会失效。
} //这里，x先移出作用域(栈：后进先出)，然后是s，因为s的值早转移了，所以不会有特殊操作。

fn takes_ownership(some_string: String) { //some_string进入作用域。
    println!("{}", some_string);
} //这里, some_string移出作用域并调用"drop"方法,占用的内存被释放。

fn makes_copy(some_integer: i32) {
    println!("{}",some_integer);
}