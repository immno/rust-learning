use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for customer {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for customer {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("Dear {}(as our valueble customer {}), enjoy this advanced feature!",
                 self.name, self.id
        )
    }
}

pub struct FreePlan;

pub struct PersonalPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);

    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        let customer = Customer::<FreePlan>::new("Mno".into());

        customer.feature1();
        customer.feature2();

        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature2();

        customer.advance_feature();
    }
}