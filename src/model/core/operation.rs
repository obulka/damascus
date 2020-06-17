use std::hash::Hash;

pub trait Operation<Input, Output>: Hash {
    fn _knobs(&self);

    fn _validate(&self, for_real: bool);

    fn maximum_inputs(&self) -> usize;

    fn minimum_inputs(&self) -> usize;

    fn inputs(&self) -> Option<Vec<Input>>;

    fn outputs(&self) -> Option<Output>;
}

pub trait FloatOperation: Operation<f32, f32> {
    fn request(&self);

    fn engine(&mut self);
}
