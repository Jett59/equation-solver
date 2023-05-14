use std::{cell::Cell, rc::Rc};

use operation::{Value, Variable};

mod operation;

fn main() {
    let variable_1 = Rc::new(Cell::new(Variable::new(1.0.into())));
    let value = Value::Scalar(2.0.into()) + Value::Variable(variable_1.clone());
    println!("{}", value.evaluate());
    variable_1.set(Variable::with_id(variable_1.get().id, 2.0.into()));
    println!("{}", value.evaluate());
}
