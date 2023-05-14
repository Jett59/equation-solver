use std::{
    cell::Cell,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
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

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self(value)
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

    pub fn with_id(id: usize, value: Scalar) -> Self {
        Self { id, value }
    }

    pub fn new(value: Scalar) -> Self {
        Self {
            id: Self::unique_id(),
            value,
        }
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

impl From<Scalar> for Value {
    fn from(scalar: Scalar) -> Self {
        Self::Scalar(scalar)
    }
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

    pub fn depends_on(&self, variable_id: usize) -> bool {
        match self {
            Value::Sum(values) => values.iter().any(|value| value.depends_on(variable_id)),
            Value::Multiplication(values) => {
                values.iter().any(|value| value.depends_on(variable_id))
            }
            Value::Power(base, exponent) => {
                base.depends_on(variable_id) || exponent.depends_on(variable_id)
            }
            Value::Log(base, argument) => {
                base.depends_on(variable_id) || argument.depends_on(variable_id)
            }
            Value::Scalar(_) => false,
            Value::Variable(cell) => cell.get().id == variable_id,
            Value::E => false,
            Value::Pi => false,
        }
    }

    pub fn pow(self, exponent: Value) -> Self {
        Self::Power(Box::new(self), Box::new(exponent))
    }
}

impl Add<Value> for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Sum(mut values) => {
                if let Self::Sum(rhs_values) = rhs {
                    values.extend(rhs_values);
                    Self::Sum(values)
                } else {
                    values.push(rhs);
                    Self::Sum(values)
                }
            }
            _ => Self::Sum(vec![self, rhs]),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Multiplication(mut values) => {
                values.push(Self::Scalar(Scalar(-1.0)));
                Self::Multiplication(values)
            }
            _ => Self::Multiplication(vec![Self::Scalar(Scalar(-1.0)), self]),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<Value> for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Multiplication(mut values) => {
                if let Self::Multiplication(rhs_values) = rhs {
                    values.extend(rhs_values);
                    Self::Multiplication(values)
                } else {
                    values.push(rhs);
                    Self::Multiplication(values)
                }
            }
            _ => Self::Multiplication(vec![self, rhs]),
        }
    }
}

impl Div<Value> for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.pow(Scalar(-1.0).into())
    }
}
