fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //生命周期标注本身不具有意义,其作用是告诉编译器多个引用之间的关系.
    // 'a的含义是: 生命周期>='a, 所以x,y和返回值至少活得和'a一样久. 
    //  因为有>号的存在，函数参数x和y的生命周期可能不一样.
    //  返回值的生命周期 与 参数中生命周期较小者 保持一致.
    //  当把具体的引用传给longest函数时, 'a的大小等于x和y中生命周期较小者.
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

