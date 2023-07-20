### Rust Note #3 Rust的字符串

1. Rust字符串String不允许下标索引字符。

2. 字符串的操作

   ```rust
   //1.尾部追加push
   fn main() {
       let mut s = String::from("Hello ");
       //由于字符串追加要修改原来的字符串，所以该字符串必须可变mut.
       s.push_str("rust");
       println!("追加字符串 push_str() -> {}",s);
       s.push('!');
       println!("追加字符 push() -> {}",s);
   }
   
   //2.任意位置插入insert.
   fn main() {
       let mut s = String::from("Hello rust!");
       s.insert(5,','); //第一个参数是插入位置的索引. 
       println!("插入字符 insert() -> {}",s);
       s.insert_str(6," I like");
       println!("插入字符串 insert_str() -> {}",s);
   }
   
   //3.替换replace.
   fn main() {
       let string_replace = String::from("Learning rust, rusty");
       let new_string_replace1 = string_replace.replace("rust","Rust");
       let new_string_replace2 = string_replace.replacen("rust","Rust",1);
       dbg!(new_string_replace1);
       dbg!(new_string_replace2);
   } //replace适用于String与&str类型，replacen()方法接收三个参数.
     //三个参数分别为：想替换掉的串，新串，替换的个数.
   
   fn main(){
       let mut string_replace_range = String::from("I like rust!");
       string_replace_range.replace_range(7..8,"R");
       dbg!(string_replace_range);
   } //replace_range:只用于String类型,俩个参数: Range(都是左闭右开)和新串. 
   
   //4.删除: pop(), remove(), truncate(), clear(), 仅适用于String类型. 四种删除方法都是"直接操作原来的字符串".
   fn main() {
       //pop():删除并返回字符串最后一个字符,返回值类型是Option类型,若为空返回None.
       let mut string_pop = String::from("rust pop 中文!");
       let p1 = string_pop.pop();
       let p2 = string_pop.pop();
       dbg!(p1); //dbg!(something), dbg Macro -> display something.
       dbg!(p2);
       dbg!(string_pop);
   }
   
   fn main() {
       //remove():删除指定位置idx的字符,返回值是删除后的字符串.
       //remove()方法按照字节来处理字符串,要确保idx落在合法字符边界.
       let mut string_remove = String::from("测试remove方法");
       println!(
           "string_remove 占 {} 个字节",
           std::mem::size_of_val(string_remove.as_str())
       );
       string_remove.remove(0);
       dbg!(string_remove);
   }
   
   //String.truncate() 与 String.clear()
   //指定一个开始位置,删除从此位置到结尾的所有字符,无返回值.
   //clear()就是idx为0的情况,从开始删到末尾.
   ```
   
   