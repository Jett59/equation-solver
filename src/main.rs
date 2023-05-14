use operation::{Value, Variable};
use solver::solve;

mod operation;
mod solver;

fn main() {
    let variable1 = Variable::new_contained(3.0.into());
    let variable2 = Variable::new_contained(4.0.into());
    let input = Variable::new_contained(6.0.into());
    let left = Value::E
        .pow(Value::from(&variable1) * Value::E.pow(Value::from(&variable2) * Value::from(&input)));
    let expected = Variable::new_contained(20.0.into());
    let right = Value::from(&expected);
    let solution = solve(left, right, variable1.get().id);
}
