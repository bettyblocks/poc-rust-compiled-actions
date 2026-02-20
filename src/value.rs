mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f64 {}
    impl Sealed for () {}
    // Add whatever subtypes you need
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "javascript", derive(rquickjs::class::Trace))]
pub enum Value {
    String(String),
    Number(f64),
    Int(i64),
    Vec(Vec<Value>),
    #[default]
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(x) => write!(f, r#""{x}""#),
            Value::Number(x) => write!(f, "{x}"),
            Value::Int(x) => write!(f, "{x}"),
            Value::Vec(x) => write!(f, "{x:?}"),
            Value::Nil => f.write_str("null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    String,
    Number,
    Int,
    Vec,
    Nil,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => f.write_str("string"),
            ValueType::Number => f.write_str("number"),
            ValueType::Int => f.write_str("integer"),
            ValueType::Vec => f.write_str("vector"),
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
            Value::Vec(_) => ValueType::Vec,
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
        Value::Number(value)
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

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Vec(value)
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

impl<T> PartialEq<T> for Value
where
    T: Copy + sealed::Sealed,
    Value: From<T>,
{
    fn eq(&self, other: &T) -> bool {
        self == &Value::from(*other)
    }
}

impl PartialOrd<String> for Value {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        match self {
            Value::String(s) => s.partial_cmp(other),
            // figure out what to do
            _ => None,
        }
    }
}

impl PartialOrd<&str> for Value {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        match self {
            Value::String(s) => s.as_str().partial_cmp(other),
            // figure out what to do
            _ => None,
        }
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
            Value::Number(x) => Ok(x),
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

impl TryFrom<Value> for Vec<Value> {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Vec(x) => Ok(x),
            _ => Err(format!(
                "Invalid type expected vec found {}",
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

#[cfg(feature = "javascript")]
impl<'js> rquickjs::IntoJs<'js> for Value {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        match self {
            Value::String(x) => x.into_js(ctx),
            Value::Number(x) => x.into_js(ctx),
            Value::Int(x) => x.into_js(ctx),
            Value::Vec(x) => x.into_js(ctx),
            Value::Nil => ().into_js(ctx),
        }
    }
}

#[cfg(feature = "javascript")]
impl<'js> rquickjs::FromJs<'js> for Value {
    fn from_js(ctx: &rquickjs::Ctx<'js>, value: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
        match value.type_of() {
            rquickjs::Type::Array => Ok(Value::Vec(<Vec<Value>>::from_js(ctx, value)?)),
            rquickjs::Type::BigInt => Ok(Value::Int(i64::from_js(ctx, value)?)),
            rquickjs::Type::Float => Ok(Value::Number(f64::from_js(ctx, value)?)),
            rquickjs::Type::Null => Ok(Value::Nil),
            rquickjs::Type::Undefined => Ok(Value::Nil),
            rquickjs::Type::String => Ok(Value::String(String::from_js(ctx, value)?)),
            rquickjs::Type::Int => Ok(Value::Int(i64::from_js(ctx, value)?)),
            unsupported => panic!("tried to convert {unsupported} to a Value")
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
}
