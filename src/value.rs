use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Number(OrderedFloat<f64>),
    Int(i64),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(x) => write!(f, r#""{x}""#),
            Value::Number(x) => write!(f, "{x}"),
            Value::Int(x) => write!(f, "{x}"),
            Value::Nil => f.write_str("null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    String,
    Number,
    Int,
    Nil,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => f.write_str("string"),
            ValueType::Number => f.write_str("number"),
            ValueType::Int => f.write_str("integer"),
            ValueType::Nil => f.write_str("null"),
        }
    }
}

impl Value {
    pub fn as_type(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String,
            Value::Number(_) => ValueType::Number,
            Value::Int(_) => ValueType::Int,
            Value::Nil => ValueType::Nil,
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Int(value as i64)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::String(s) => s == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        match self {
            Value::String(s) => s == other,
            _ => false,
        }
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Value::Number(s) => s == &OrderedFloat(*other),
            _ => false,
        }
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Value::Int(s) => s == other,
            _ => false,
        }
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Value::Int(s) => *s == (*other as i64),
            _ => false,
        }
    }
}

impl PartialEq<u32> for Value {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Value::Int(s) => *s == (*other as i64),
            _ => false,
        }
    }
}

impl PartialOrd<i64> for Value {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&Value::from(*other))
    }
}

impl TryFrom<Value> for String {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(x) => Ok(x),
            _ => Err(format!(
                "Invalid type expected string found {}",
                value.as_type()
            )),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(x) => Ok(*x),
            _ => Err(format!(
                "Invalid type expected number found {}",
                value.as_type()
            )),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(x) => Ok(x),
            _ => Err(format!(
                "Invalid type expected integer found {}",
                value.as_type()
            )),
        }
    }
}

impl TryFrom<Value> for () {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Nil => Ok(()),
            _ => Err(format!(
                "Invalid type expected nil found {}",
                value.as_type()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_display_test() {
        assert_eq!(Value::Nil.to_string(), "null");
        assert_eq!(Value::Int(123).to_string(), "123");
        assert_eq!(Value::String("oke".to_string()).to_string(), "\"oke\"");
    }

    #[test]
    fn value_type_display_test() {
        assert_eq!(ValueType::Nil.to_string(), "null");
        assert_eq!(ValueType::Int.to_string(), "integer");
        assert_eq!(ValueType::String.to_string(), "string");
    }

    #[test]
    fn ordering_with_integer() {
        assert!(Value::Int(65) > 34);
    }
}
