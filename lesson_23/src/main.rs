use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

// #[derive(Debug, Default, PartialEq, Eq)]
// pub struct Identifier<T> {
//     inner: u64,
// }

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_should_not_be_the_same(){
        let user = User::default();
        let product = Product::default();

        assert_eq!(user.id.inner, product.id.inner);
    }
}

