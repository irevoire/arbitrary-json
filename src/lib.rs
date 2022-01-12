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
            4 => Value::Array(
                u.arbitrary_iter()?
                    .map(|result| result.map(|json: ArbitraryValue| json.0))
                    .collect::<Result<Vec<Value>>>()?,
            ),
            5 => Value::Object(
                u.arbitrary_iter()?
                    .map(|result| {
                        result.map(|(key, value): (String, ArbitraryValue)| (key, value.0))
                    })
                    .collect::<Result<Map<String, Value>>>()?,
            ),
            _ => unreachable!(),
        };

        Ok(ArbitraryValue(variant))
    }
}

impl Deref for ArbitraryValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArbitraryValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Value> for ArbitraryValue {
    fn from(value: Value) -> Self {
        ArbitraryValue(value)
    }
}

impl From<ArbitraryValue> for Value {
    fn from(value: ArbitraryValue) -> Self {
        value.0
    }
}

impl Debug for ArbitraryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
