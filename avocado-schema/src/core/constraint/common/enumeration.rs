use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::TypeId;
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Enumeration<T> {
    pub values: Vec<T>,
}

impl<T> Serialize for Enumeration<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.values.len()))?;
        for element in &self.values {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for Enumeration<T>
where
    T: Deserialize<'de> + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(EnumerationVisitor {
            phantom: Default::default(),
        })
    }
}

struct EnumerationVisitor<T> {
    phantom: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for EnumerationVisitor<T>
where
    T: Deserialize<'de> + 'static,
{
    type Value = Enumeration<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let message = if TypeId::of::<T>() == TypeId::of::<String>() {
            "string field [enum] needs to be an array of strings"
        } else if TypeId::of::<T>() == TypeId::of::<i64>() {
            "integer [enum] needs to be an array of integers"
        } else {
            "[enum] needs to be a array of corresponding type"
        };
        write!(formatter, "{}", message)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<T> = vec![];
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Enumeration { values })
    }
}
