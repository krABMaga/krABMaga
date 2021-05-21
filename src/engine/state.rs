pub trait State {
    fn update(&mut self, step: usize) {}
}
