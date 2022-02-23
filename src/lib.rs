//! Arbitrary JSON
//! ==============
//!
//! This crate provide a compatibility layer between
//! [serde_json](https://github.com/serde-rs/json) and
//! [arbitrary](https://github.com/rust-fuzz/arbitrary).
//! This allow you to generate random valid json when fuzzing your rust code. See
//! the following example:
//!
//! ```ignore
//! #![no_main]
//! use arbitrary_json::ArbitraryValue;
//! use libfuzzer_sys::fuzz_target;
//!
//! fuzz_target!(|data: ArbitraryValue| {
//!     // call your very complex code here
//!     if data["truc"] == serde_json::json!(42) {
//!         panic!("Found the magic value");
//!     }
//! });
//! ```

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use arbitrary::{Arbitrary, Error, Result, Unstructured};
use serde_json::{Map, Number, Value};

#[derive(Clone)]
pub struct ArbitraryValue(Value);

impl<'a> Arbitrary<'a> for ArbitraryValue {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let variant: u8 = u.arbitrary()?;

        let variant = match variant % 6 {
            0 => Value::Null,
            1 => Value::Bool(u.arbitrary()?),
            2 => {
                let variant: u8 = u.arbitrary()?;
                let number = match variant % 3 {
                    0 => Number::from_f64(u.arbitrary()?).ok_or(Error::IncorrectFormat)?,
                    1 => u.arbitrary::<u64>()?.into(),
                    2 => u.arbitrary::<i64>()?.into(),
                    _ => unreachable!(),
                };
                Value::Number(number)
            }
            3 => Value::String(u.arbitrary()?),
            4 => Value::Array(u.arbitrary::<ArbitraryArray>()?.into()),
            5 => Value::Object(u.arbitrary::<ArbitraryObject>()?.into()),
            _ => unreachable!(),
        };

        Ok(ArbitraryValue(variant))
    }
}

#[derive(Clone)]
pub struct ArbitraryObject(Map<String, Value>);

impl<'a> Arbitrary<'a> for ArbitraryObject {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let map = u
            .arbitrary_iter()?
            .map(|result| result.map(|(key, value): (String, ArbitraryValue)| (key, value.0)))
            .collect::<Result<Map<String, Value>>>()?;

        Ok(ArbitraryObject(map))
    }
}

#[derive(Clone)]
pub struct ArbitraryArray(Vec<Value>);

impl<'a> Arbitrary<'a> for ArbitraryArray {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let array = u
            .arbitrary_iter()?
            .map(|result| result.map(|json: ArbitraryValue| json.0))
            .collect::<Result<Vec<Value>>>()?;

        Ok(ArbitraryArray(array))
    }
}

macro_rules! impl_derefrom {
    ($arbitrary:ty, $serde:ty) => {
        impl Deref for $arbitrary {
            type Target = $serde;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $arbitrary {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$serde> for $arbitrary {
            fn from(value: $serde) -> Self {
                Self(value)
            }
        }

        impl From<$arbitrary> for $serde {
            fn from(value: $arbitrary) -> Self {
                value.0
            }
        }

        impl Debug for $arbitrary {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

impl_derefrom!(ArbitraryValue, Value);
impl_derefrom!(ArbitraryObject, Map<String, Value>);
impl_derefrom!(ArbitraryArray, Vec<Value>);
