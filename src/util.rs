//! Just some nifty little utilities, as all projects accumulate

// Just a little sugar around having to write 'num % 2 == 0'
pub trait IsEven {
    fn is_even(&self) -> bool;
}
impl IsEven for usize {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

pub trait IsOdd {
    fn is_odd(&self) -> bool;
}
impl IsOdd for usize {
    fn is_odd(&self) -> bool {
        *self % 2 == 1
    }
}
