fn main() {}

// example_1
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}

// example_2
fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

// example_3
fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();
    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();
    // function (actually a pointer) is Copy
    is_copy::<fn()>();
    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();
    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();
    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // unsized or dynamic sized type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();
    // mutable reference is not Copy
    is_copy::<&mut String>();
    // array / tuple with values that not Copy is not Copy
    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let data = vec![10, 42, 9, 8];
        let v = 42;
        if let Some(pos) = find_pos(data, v) {
            println!("Found {} at {}", v, pos);
        }
    }

    // 报错
    #[test]
    fn example_2() {
        let data = vec![1, 2, 3, 4];
        let data1 = data;
        println!("sum of data1: {}", sum(data1));
        // println!("data1: {:?}", data1);
        // println!("sum of data2: {}", sum(data));
    }

    // 报错
    #[test]
    fn example_3() {
        types_impl_copy_trait();
        types_not_impl_copy_trait();
    }
}