use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Not;

use crate::cc_parser::ast::Type;

use super::object::Object;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Value(u64);

impl Default for Value {
    fn default() -> Self {
        Self::NIL
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_nil() {
            Self::TRUE
        } else if self.is_bool() {
            Self::from(!self.as_bool())
        } else {
            Self::FALSE
        }
    }
}

impl Value {
    const SIGN_BIT: u64 = 0x8000000000000000;
    const QUIET_NAN: u64 = 0x7ffc000000000000;

    pub const NIL: Self = Self(Self::QUIET_NAN | 0b01);
    pub const FALSE: Self = Self(Self::QUIET_NAN | 0b10);
    pub const TRUE: Self = Self(Self::QUIET_NAN | 0b11);

    pub fn type_(self) -> Type {
        if self.is_nil() {
            Type::Nil
        } else if self.is_bool() {
            Type::Bool
        } else if self.is_number() {
            Type::Int
        } else if self.is_object() {
            Type::Object(Box::new(self.as_object().type_()))
        } else {
            unreachable!()
        }
    }

    pub fn is_nil(self) -> bool {
        self == Self::NIL
    }

    pub fn is_bool(self) -> bool {
        Self(self.0 | 0b01) == Self::TRUE
    }

    pub const fn is_number(self) -> bool {
        (self.0 & Self::QUIET_NAN) != Self::QUIET_NAN
    }

    pub const fn is_object(self) -> bool {
        self.0 & (Self::QUIET_NAN | Self::SIGN_BIT) == (Self::QUIET_NAN | Self::SIGN_BIT)
    }

    pub fn _is_iterable(self) -> bool {
        self.is_object() && self.as_object()._is_iterable()
    }

    pub fn is_false(self) -> bool {
        Self(self.0) == Self::FALSE
    }

    pub fn is_true(self) -> bool {
        Self(self.0) == Self::TRUE
    }

    /// # Safety
    /// This is undefined behavior if the [`Value`] is not of type [`ValueType::Bool`].
    pub fn as_bool(self) -> bool {
        self == Self::TRUE
    }

    /// # Safety
    /// This is undefined behavior if the [`Value`] is not of type [`ValueType::Number`].
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    /// # Safety
    /// This is undefined behavior if the [`Value`] is not of type [`ValueType::Object`].
    pub const fn as_object(self) -> Object {
        Object {
            main: (self.0 & !(Self::SIGN_BIT | Self::QUIET_NAN)) as _,
        }
    }

    pub const fn to_bool(self) -> bool {
        !matches!(self, Self::FALSE | Self::NIL)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{self}")
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_nil() {
            write!(f, "nil")
        } else if self.is_true() {
            write!(f, "true")
        } else if self.is_false() {
            write!(f, "false")
        } else if self.is_number() {
            write!(f, "{}", self.as_number())
        } else if self.is_object() {
            write!(f, "{}", self.as_object())
        } else {
            unreachable!()
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        if value {
            Self::TRUE
        } else {
            Self::FALSE
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(value.to_bits())
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl<O: Into<Object>> From<O> for Value {
    fn from(object: O) -> Self {
        Self((unsafe { object.into().main } as u64) | Self::SIGN_BIT | Self::QUIET_NAN)
    }
}

// #[derive(Debug, PartialEq, Clone)]
// #[repr(u8)]
// pub enum ValueType {
//     Nil,
//     Bool,
//     Number,
//     Object(ObjectType),
// }
//
// impl Display for ValueType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Nil => write!(f, "nil"),
//             Self::Bool => write!(f, "bool"),
//             Self::Number => write!(f, "number"),
//             Self::Object(type_) => write!(f, "{}", type_),
//         }
//     }
// }
//
//
// impl From<ObjectType> for ValueType {
//     fn from(type_: ObjectType) -> Self {
//         Self::Object(type_)
//     }
// }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn convert_to_and_from_values() {
        let value = false;
        assert_eq!(Value::from(value).as_bool(), value);

        let value = true;
        assert_eq!(Value::from(value).as_bool(), value);

        let value = 0.0;
        assert_eq!(Value::from(value).as_number(), value);

        let value = f64::NAN;
        assert!(Value::from(value).as_number().is_nan());
    }

    #[test]
    fn test_value_type() {
        assert_eq!(Value::NIL.type_(), Type::Nil);
        assert_eq!(Value::FALSE.type_(), Type::Bool);
        assert_eq!(Value::TRUE.type_(), Type::Bool);
        assert_eq!(Value(123).type_(), Type::Int);

        assert_eq!(Value::from(0.0).type_(), Type::Int);
        assert_eq!(Value::from(f64::NAN).type_(), Type::Int);
    }

    #[test]
    fn test_value_is_nil() {
        use super::Value;

        assert!(Value::NIL.is_nil());
        assert!(!Value::FALSE.is_nil());
        assert!(!Value::TRUE.is_nil());
        assert!(!Value(123).is_nil());
    }

    #[test]
    fn test_value_is_bool() {
        // Falsey
        assert!(!Value::NIL.to_bool());
        assert!(!Value::FALSE.to_bool());

        // Truthy
        assert!(Value::TRUE.to_bool());
        assert!(Value(123).to_bool());
    }

    #[test]
    fn test_value_is_number() {
        assert!(!Value::NIL.is_number());
        assert!(!Value::FALSE.is_number());
        assert!(!Value::TRUE.is_number());

        assert!(Value(123).is_number());
    }

    #[test]
    fn test_qnan() {
        let value = Value(0x3093 & Value::QUIET_NAN);
        println!("{:?}", u64::MAX);
        assert!(value.is_number());
    }
}
