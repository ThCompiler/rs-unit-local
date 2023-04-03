
pub trait Suite {
    fn before_test(&mut self);
    fn after_test(&mut self);
}

pub struct SuiteSingleton<T>
    where T: Default + Suite {
    value: Option<T>,
}

impl<T> SuiteSingleton<T>
    where T: Default + Suite {
    pub const fn new() -> Self {
        Self {
            value: None,
        }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            self.value = Some(T::default())
        }

        return self.value.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        if self.value.is_none() {
            self.value = Some(T::default())
        }

        return self.value.as_mut().unwrap()
    }
}