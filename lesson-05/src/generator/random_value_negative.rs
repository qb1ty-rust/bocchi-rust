pub struct RandomValueNegative {
    pub value_negative: i8
}

impl RandomValueNegative {
    pub fn new(value: i8) -> Self {
        Self {
            value_negative: value
        }
    }
}