#[derive(Clone, Copy, Debug)]
pub enum Value {
    None(usize),
    Int(i64),
    Float(f64),
}
