use std::fmt;

/// Constant value that appears as literal in program.
#[derive(Clone, Default, PartialEq)]
pub enum Value {
    /// Constant number for example `4.2`.
    Number(f64),
    #[default]
    /// Empty value literal value is `nil`.
    Nil,
}

/// Collection of constant values.
#[derive(Default)]
pub struct Values(Vec<Value>);

/// Position of value in [`Values`].
#[derive(PartialEq, Eq)]
pub struct ValuePosition(usize);

impl Values {
    /// Write value into values.
    /// Returns the position value can be found at.
    pub fn write(&mut self, value: Value) -> ValuePosition {
        self.0.push(value);

        ValuePosition(self.0.len() - 1)
    }

    /// Get value at given position.
    /// Returns [`Value::Nil`] if position is empty.
    pub fn get(&self, position: &ValuePosition) -> &Value {
        self.0.get(position.0).unwrap_or(&Value::Nil)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Debug for ValuePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let position = self.0;

        write!(f, "{position}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_write_adds_value_and_returns_position() {
        let mut values = Values::default();
        let number = 42.0;
        let value = Value::Number(number);

        let position = values.write(value);
        let output = values.get(&position);

        assert_eq!(output, &Value::Number(number));
    }

    #[test]
    fn values_get_empty_position_returns_null() {
        let values = Values::default();
        let position = ValuePosition(42);

        let output = values.get(&position);

        assert_eq!(output, &Value::Nil);
    }
}
