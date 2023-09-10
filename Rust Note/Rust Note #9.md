### Rust Note # 9：迭代器

1. 三段论阐述迭代器

   1. Rust可以直接对实现了IntoIterator特征的类型进行迭代。
   2. 数组实现了IntoIterator特征。
   3. Rust可以自动实现对数组的迭代。（具体方法是：把数组转换为迭代器）

2. 一般化的三段论

   1. Rust可以直接对实现了IntoIterator特征的类型进行迭代。
   2. 你为某一种类型实现了上述特征。
   3. Rust可以自动实现对你这种类型的迭代。具体办法是把你这种类型利用特征转化为迭代器。

3. 迭代器的惰性初始化

   ```rust
   let v1 = vec![1, 2, 3];
   let v1_iter = v1.iter();
   // 在for循环之前,只是简单地创建了v1_iter迭代器,此时不会发生任何迭代行为。
   for val in v1_iter {
       println!("{}"，val);
   }
   //只有在for循环开始后,迭代器才会迭代其中的元素,然后针对每一个元素进行{}中的处理。
   //这种惰性初始化确保了创建迭代器不需要任何其他额外的性能损耗，其中的元素也不会被消耗。
   ```

4. 问：在for循环遍历迭代器的过程中，具体是取出迭代器的内容的？

   预备知识：某种类型之所以成为迭代器，是因为实现了Iterator特征。

   ```rust
   pub trait Iterator {
       type Item;
       
       fn next(&mut self) -> Option<Self::Item>;
       //要实现Iterator特征,关键是要实现next方法。
       //next方法控制如何从集合（迭代器）中取值,并返回关联类型Item。
       //skip--
   }
   ```

   答：for循环通过不断调用迭代器上的next方法，来获取迭代器中的元素。

   试一试：既然for循环可以调用next方法，我们自己也可以调用！

   ```rust
   fn main() {
       let arr = [1,2,3];
       let mut arr_iter = arr.into_iter(); //.into_iter()方法可以将数组转换为迭代器.
   
   /*----这是一段迭代器内部实现into_iter的源码----*/
   impl<I: Iterator> IntoIterator for I {
       type Item = I::Item;
       type IntoIter = I;
   
       #[inline]
       fn into_iter(self) -> I {
           self
       }//这个into_iter很好地解决了如果本身是迭代器,又转换为迭代器的问题,解决的方法就是返回自身。
   }
       
       //手动遍历必须将迭代器声明为mut可变,因为调用next会改变迭代器其中的状态数据。
       //而for循环去迭代器时无需标注mut，因为它会帮我们自动完成。
       assert_eq!(arr_iter.next(),Some(1));
       assert_eq!(arr_iter.next(),Some(2));
       assert_eq!(arr_iter.next(),Some(3));
       assert_eq!(arr_iter.next(),None);
       //从None中可以看出来,next()方法对迭代器的遍历是消耗性的,每遍历一个元素就消耗一个元素,最终返回None.
   }
   ```

5. 三种转化为迭代器的方式

   1. into_iter 会夺走ownership (into_之类的,都是拿走所有权)
   2. iter 借用 (默认就是不可变借用)
   3. iter_mut 可变借用 (_mut之类的,都是可变借用,可变借用只能有一个而且不能同时有不可变借用)
   4. 注意: .iter()和.iter_mut()方法实现的迭代器的返回类型都是Some(&mut T).

6. Iterator和IntoIterator的区别

   1. IntoIterator强调的是某个类型如果实现了该特征，则可以通过into_iter等方法变成一个迭代器。
   2. Iterator就是迭代器特征，只有实现了Iterator特征才能成为迭代器，才能调用next。

7. 消费器与适配器

   1. 消费器是迭代器上的方法，它会consume掉迭代器中的元素，然后返回该元素类型的值。

      所有消费器有一个共同的特点：在它们的定义中，都依赖next方法来消费元素。

   2. 现有迭代器上的某个方法A，在A的内部调用了next方法，则称A为**消费性适配器**。

      因为next方法会消费掉迭代器上的元素，而next方法在A的内部（在适配器的内部），

      所以方法A会消费掉迭代器上的元素。故称A为消费性适配器。

   3. 消费性适配器的一个例子

      ```rust
      fn main() {
          let v1 = vec![1, 2, 3];
          let v1_iter = v1.iter(); //调用iter()函数把它变为迭代器.
          let total: i32 = v1_iter.sum(); //调用迭代器的sum()适配器,生成total的值.
          
          assert_eq!(total,6);
          println!("{:?}",v1); //因为iter()是借用,所以v1可以照常使用。
      }
      ```

   4. 迭代器适配器：会返回一个新的迭代器，是实现链式方法调用的关键。v.iter().map().filter()...

      迭代器适配器是惰性的、意味着你需要用一个消费者适配器来收尾，最终将迭代器转换为一个具体的值。

      “惰性”意味着不产生任何的行为，没有返回值。

      ```rust
      let v1: Vec<i32> = vec![1, 2, 3];
      v1.iter().map(|x| x+1); //map是一个迭代器适配器,它是惰性的,不产生任何行为。
      //还需要一个消费者适配器收尾，像下面这样
      let v2: Vec<_> = v1.iter().map(|x| x+1).collect(); //collect()方法是一个消费者适配器.
      //使用collect()可以将一个迭代器中的元素收集到指定的类型中。
      assert_eq!(v2,vec![2,3,4]);
      ```

      































