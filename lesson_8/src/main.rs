fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let data = vec![1, 2, 3, 4];
        let data1 = &data;
        println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
                 &data, data1, &&data, &data1
        );
        println!("sum of data1: {}", sum(data1));
        println!("addr of items: [{:p}, {:p}, {:p}, {:p}]",
                 &data[0], &data[1], &data[2], &data[3]
        )
    }

    fn sum(data: &Vec<u32>) -> u32 {
        // 值的地址会改变么？引用的地址会改变么？
        println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
        data.iter().fold(0, |acc, x| acc + x)
    }

    // 取消运行报错
    // #[test]
    // fn example_2() {
    //     let r = local_ref();
    //     println!("r: {:p}", r);
    // }

    // fn local_ref<'a>() -> &'a i32 {
    //     let a = 42;
    //     &a
    // }

    #[test]
    fn example_3() {
        let mut data = Vec::new();
        let v = 42;
        data.push(&v);
        println!("data: {:?}", data);
    }

    // 取消运行报错
    // #[test]
    // fn example_4() {
    //     let mut data = Vec::new();
    //     push_local_ref(&mut data);
    //     println!("data: {:?}", data);
    // }
    //
    // fn push_local_ref(data: &mut Vec<&u32>) {
    //     let v = 42;
    //     data.push(&v);
    // }

    // #[test]
    // fn example_5() {
    //     let mut data = vec![1, 2, 3];
    //     for item in data.iter_mut() {
    //         data.push(*item + 1);
    //     }
    // }

    #[test]
    fn example_6() {
        let mut data = vec![1, 2, 3];
        // let data1 = vec![&data[0]];
        println!("data[0]: {:p}", &data[0]);

        for i in 0..100 {
            data.push(i);
        }
        // 堆上的数据预留的空间不够了，就会重新分配一片足够大的内存，把之前的值拷过来，然后释放旧的内存。
        // 这样就会让 data1 中保存的 &data[0] 引用失效，导致内存安全问题。
        println!("data[0]: {:p}", &data[0]);
        // println!("boxed: {:p}", &data1);
    }

    // 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
    // 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。类似：数据在并发下的读写访问（比如 RwLock）。
}