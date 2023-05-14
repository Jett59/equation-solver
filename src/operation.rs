use std::{
    cell::Cell,
    ops::Deref,
    rc::Rc,
    sync::atomic::{self, AtomicUsize},
};

/// The main reason for this needing to exist is NaN.
///
/// NaN is not equal to itself, so we need to implement PartialEq manually to make it behave in the expected way.
#[derive(Clone, Copy)]
pub struct Scalar(pub f64);

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_nan() && other.0.is_nan() || self.0 == other.0
    }
}
impl Eq for Scalar {}

impl Deref for Scalar {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Variable {
    pub id: usize,
    pub value: Scalar,
}

impl Variable {
    pub fn unique_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new(id: usize, value: Scalar) -> Self {
        Self { id, value }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Value {
    Sum(Vec<Value>),
    Multiplication(Vec<Value>),
    Power(Box<Value>, Box<Value>),
    Log(Box<Value>, Box<Value>),
    Scalar(Scalar),
    Variable(Rc<Cell<Variable>>),
    E,
    Pi,
}

impl Value {
    pub fn evaluate(&self) -> f64 {
        match self {
            Value::Sum(values) => values.iter().map(|value| value.evaluate()).sum(),
            Value::Multiplication(values) => values.iter().map(|value| value.evaluate()).product(),
            Value::Power(base, exponent) => base.evaluate().powf(exponent.evaluate()),
            Value::Log(base, argument) => base.evaluate().log(argument.evaluate()),
            Value::Scalar(scalar) => scalar.0,
            Value::Variable(cell) => cell.get().value.0,
            Value::E => std::f64::consts::E,
            Value::Pi => std::f64::consts::PI,
        }
    }
}
