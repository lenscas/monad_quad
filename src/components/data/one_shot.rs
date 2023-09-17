pub struct OneShot<T> {
    data: Option<T>,
}

impl<T> Default for OneShot<T> {
    fn default() -> Self {
        Self { data: None }
    }
}

impl<T> OneShot<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_from(v: T) -> Self {
        Self { data: Some(v) }
    }
    pub fn merge_from(&mut self, v: Self) {
        self.data = v.data;
    }
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    pub fn get_data(&mut self) -> Option<T> {
        self.data.take()
    }
    pub fn peak(&self) -> Option<&T> {
        self.data.as_ref()
    }
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data)
    }
}
