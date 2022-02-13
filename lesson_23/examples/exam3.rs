fn main() {}

pub fn consume_iterator<F, Iter, T>(mut f: F)
    where
        F: FnMut(i32) -> Iter,
        Iter: Iterator<Item=T>,
        T: std::fmt::Debug,
{
    for item in f(10) {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_iterator() {
        consume_iterator(|i| (0..i).into_iter())
    }
}