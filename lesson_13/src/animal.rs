struct Cat;

struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

fn name2(animal: &dyn Animal) -> &'static str {
    animal.name()
}

pub fn print_name(animals: Vec<&dyn Animal>) {
    for animal in animals {
        println!("This is : {}", animal.name());
    }
}

fn main() {
    let cat = Cat;
    println!("cat: {}", name2(&cat));
    println!("cat: {}", name(cat));

    let cat: &dyn Animal = &Cat;
    let dog: &dyn Animal = &Dog;
    let animals = vec![cat, dog];
    print_name(animals);
}

