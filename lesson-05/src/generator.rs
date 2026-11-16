use rand::Rng;

use crate::prelude::{LOW, HIGH, LOW_NEG, HIGH_NEG};
use crate::generator::random_value::RandomValue;
use crate::generator::random_value_negative::RandomValueNegative;

mod random_value;
mod random_value_negative;

pub fn generate() -> RandomValue {
    let random_num = rand::thread_rng().gen_range(LOW..=HIGH);
    RandomValue::new(random_num)
}

pub fn generate_negative() -> RandomValueNegative {
    let random_negative_num = rand::thread_rng().gen_range(LOW_NEG..=HIGH_NEG);
    RandomValueNegative::new(random_negative_num)
}