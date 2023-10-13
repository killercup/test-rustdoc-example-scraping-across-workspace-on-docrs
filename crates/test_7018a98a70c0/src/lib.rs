#[derive(Debug)]
pub struct Bar {
    /// pressure in Pascal
    pub amount: f32,
}

impl Bar {
    pub fn pa(&self) -> f32 {
        self.amount
    }
}
