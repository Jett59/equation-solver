use std::{cell::Cell, rc::Rc};

use crate::{
    operation::{Value, Variable},
    solver,
};

pub fn solve(mut equations: Vec<(Value, Value)>, unknowns: Vec<Rc<Cell<Variable>>>) {
    assert!(equations.len() == unknowns.len());
    let mut solution_equations = Vec::with_capacity(unknowns.len());
    for i in 0..equations.len() {
        let unknown = unknowns[i].get().id;
        let equation = &equations[i];
        let solution = solver::solve(equation.0.clone(), equation.1.clone(), unknown);
        for equation in equations.iter_mut().skip(i) {
            equation.0.substitute(unknown, solution.clone());
            equation.1.substitute(unknown, solution.clone());
        }
        solution_equations.push(solution);
    }
    for (solution_equation, unknown) in solution_equations
        .into_iter()
        .zip(unknowns.into_iter())
        .rev()
    {
        Variable::modify(solution_equation.evaluate().into(), unknown);
    }
}
