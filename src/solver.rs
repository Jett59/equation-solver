use crate::operation::Value;

/// Solve the equation, returning the right-hand side (which can be used to find the value of the variable).
pub fn solve(mut left: Value, mut right: Value, variable_id: usize) -> Value {
    let variable_on_left = left.depends_on(variable_id);
    let variable_on_right = right.depends_on(variable_id);
    assert!(
        !variable_on_right,
        "Variable on right-hand side not implemented"
    );
    assert!(variable_on_left, "Variable not part of equation");
    while !left.is_variable() {
        left = match left {
            Value::Sum(values) => {
                todo!();
            }
            Value::Multiplication(values) => {
                let mut unrelated_values: Vec<Value> = values
                    .iter()
                    .filter(|value| !value.depends_on(variable_id))
                    .cloned()
                    .collect();
                assert!(unrelated_values.len() > 0, "Expansion not implemented");
                assert!(
                    unrelated_values.len() < values.len(),
                    "Variable not part of equation"
                );
                let mut related_values: Vec<Value> = values
                    .into_iter()
                    .filter(|value| value.depends_on(variable_id))
                    .collect();
                if unrelated_values.len() == 1 {
                    right = right / unrelated_values.pop().unwrap();
                } else {
                    right = right / Value::Multiplication(unrelated_values);
                }
                if related_values.len() == 1 {
                    related_values.pop().unwrap()
                } else {
                    Value::Multiplication(related_values)
                }
            }
            Value::Power(ref base, ref exponent) => {
                let variable_in_base = base.depends_on(variable_id);
                let variable_in_exponent = exponent.depends_on(variable_id);
                assert!(
                    variable_in_base != variable_in_exponent,
                    "Variable in both base and exponent not implemented"
                );
                assert!(!variable_in_base, "Variable in base not implemented");
                // The variable therefore must be in the exponent.
                right = right.log((**base).clone());
                (**exponent).clone()
            }
            Value::Log(base, argument) => {
                todo!();
            }
            Value::Scalar(_) => {
                panic!("Reduced left-hand to scalar");
            }
            Value::Variable(variable) => {
                panic!("Loop should have terminated");
            }
            Value::E => {
                panic!("Reduced left-hand to e");
            }
            Value::Pi => {
                panic!("Reduced left-hand to pi");
            }
        };
    }
    right
}
