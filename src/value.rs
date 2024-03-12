pub type Value = f64;

#[derive(Debug)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn init_value_array() -> Self {
        Self { values: Vec::new() }
    }
    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }
}

pub fn print_value(value: &Value) {
    print!("{}", value);
}
