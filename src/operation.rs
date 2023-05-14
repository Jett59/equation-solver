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

    pub fn new_contained(value: Scalar) -> Rc<Cell<Self>> {
        Rc::new(Cell::new(Self::new(value)))
    }

    pub fn modify(value: Scalar, variable: Rc<Cell<Self>>) {
        variable.set(Self::with_id(variable.get().id, value))
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

impl From<f64> for Value {
    fn from(scalar: f64) -> Self {
        Self::Scalar(scalar.into())
    }
}

impl From<&Rc<Cell<Variable>>> for Value {
    fn from(variable: &Rc<Cell<Variable>>) -> Self {
        Self::Variable(variable.clone())
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

    pub fn is_sum(&self) -> bool {
        matches!(self, Self::Sum(_))
    }
    pub fn is_multiplication(&self) -> bool {
        matches!(self, Self::Multiplication(_))
    }
    pub fn is_power(&self) -> bool {
        matches!(self, Self::Power(_, _))
    }
    pub fn is_log(&self) -> bool {
        matches!(self, Self::Log(_, _))
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    pub fn is_variable(&self) -> bool {
        matches!(self, Self::Variable(_))
    }
    pub fn is_e(&self) -> bool {
        matches!(self, Self::E)
    }
    pub fn is_pi(&self) -> bool {
        matches!(self, Self::Pi)
    }

    pub fn pow(self, exponent: Value) -> Self {
        Self::Power(Box::new(self), Box::new(exponent))
    }
    pub fn log(self, base: Value) -> Self {
        Self::Log(Box::new(base), Box::new(self))
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
