#[must_use]
pub enum MustUseOption<T> {
    Some(T),
    None
}

impl<T> MustUseOption<T> {
    #[must_use]
    pub fn unwrap(self) -> T {
        match self {
            MustUseOption::Some(t) => t,
            MustUseOption::None => panic!("Called unwrap on None")
        }
    }
}