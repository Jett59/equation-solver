use operation::{Value, Variable};

mod operation;
mod simultaneous_solver;
mod solver;

fn main() {
    const POINTS: usize = 2;

    let variable1 = Variable::new_contained(3.0.into());
    let variable2 = Variable::new_contained(4.0.into());
    let inputs = (0..POINTS)
        .map(|_| Variable::new_contained(1.0.into()))
        .collect::<Vec<_>>();
    let outputs = (0..POINTS)
        .map(|_| Variable::new_contained(1.0.into()))
        .collect::<Vec<_>>();
    let equations = inputs
        .iter()
        .zip(outputs.iter())
        .map(|(input, output)| {
            let left = Value::E.pow(
                Value::from(&variable1)
                    * Value::E.pow(Value::from(&variable2) * Value::from(input)),
            );
            let right = Value::from(output);
            (left, right)
        })
        .collect::<Vec<_>>();
    simultaneous_solver::solve(equations, vec![variable1.clone(), variable2.clone()]);
    println!("Variable1: {}", variable1.get().value.0);
    println!("Variable2: {}", variable2.get().value.0);
}
