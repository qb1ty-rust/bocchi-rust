pub struct RandomValue {
    pub value: u8
}

impl RandomValue {
    pub fn new(value: u8) -> Self {
        Self {
            value
        }
    }
}