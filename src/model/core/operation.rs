use std::{mem, hash::Hash};
use rustc_hash::FxHasher as Hasher;

/// Returns the mantissa, exponent and sign as integers.
fn integer_decode(val: f32) -> (u64, i16, i8) {
    let bits: u32 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0x7fffff) << 1
    } else {
        (bits & 0x7fffff) | 0x800000
    };
    // Exponent bias + mantissa shift
    exponent -= 127 + 23;
    (mantissa as u64, exponent, sign)
}

pub trait Operation<Input, Output> {
    fn _knobs(&self);

    fn _validate(&self, for_real: bool);

    fn maximum_inputs(&self) -> Option<usize>;

    fn minimum_inputs(&self) -> usize;

    // fn inputs(&self) -> Option<Vec<Input>>;

    // fn outputs(&self) -> Option<Output>;

    fn label(&self) -> String;
    
    fn hash(&self, state: &mut Hasher);
}

pub trait FloatOperation: Operation<f32, f32> {
    fn request(&self);

    fn engine(&mut self);
}

pub struct ReadFloatOperation {
    label: String,
    value: f32,
    children: Vec<Box<dyn FloatOperation>>,
}

impl Operation<f32, f32> for ReadFloatOperation {
    fn _knobs(&self) {}

    fn _validate(&self, _for_real: bool) {}

    fn maximum_inputs(&self) -> Option<usize> {
        Some(0)
    }

    fn minimum_inputs(&self) -> usize {
        0
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn hash(&self, state: &mut Hasher) {
        integer_decode(self.value).hash(state);
    }
}

impl FloatOperation for ReadFloatOperation {
    fn request(&self) { todo!() }
    fn engine(&mut self) { todo!() }
}

pub struct ViewFloatOperation {
    label: String,
    value: f32,
    parents: Vec<Box<dyn FloatOperation>>,
}

impl Operation<f32, f32> for ViewFloatOperation {
    fn _knobs(&self) {}

    fn _validate(&self, _for_real: bool) {}

    fn maximum_inputs(&self) -> Option<usize> {
        None
    }

    fn minimum_inputs(&self) -> usize {
        1
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn hash(&self, state: &mut Hasher) {
        for parent in &self.parents {
            parent.hash(state);
        }
        // self.knobs.hash(state);
        integer_decode(self.value).hash(state);
    }
}

impl FloatOperation for ViewFloatOperation {
    fn request(&self) { todo!() }
    fn engine(&mut self) { todo!() }
}
