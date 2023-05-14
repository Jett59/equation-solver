use crate::operation::Value;

/// Solve the equation, returning the right-hand side (which can be used to find the value of the variable).
pub fn solve(left: Value, right: Value, variable_id: usize) -> Value {
    let variable_on_left = left.depends_on(variable_id);
    let variable_on_right = right.depends_on(variable_id);
    assert!(
        !variable_on_right,
        "Variable on right-hand side not implemented"
    );
    assert!(variable_on_left, "Variable not part of equation");
    todo!();
}
