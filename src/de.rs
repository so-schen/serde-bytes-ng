use core::fmt;
use core::marker::PhantomData;
use serde::{
    de::{Error, Visitor},
    Deserializer,
};

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

// Deserialize for [u8; N]
impl<'de, const N: usize> Deserialize<'de> for [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct _Vistor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for _Vistor<N> {
            type Value = [u8; N];

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(format_args!("byte array of length {}", N))
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                if v.len() < N {
                    return Err(E::invalid_length(v.len(), &self));
                }

                let mut arr = [0; N];
                arr.copy_from_slice(v);
                Ok(arr)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_bytes(v.as_bytes())
            }
        }

        let vistor = _Vistor;
        let arr: [u8; N] = deserializer.deserialize_bytes(vistor)?;
        Ok(arr)
    }
}

/// Deserialize for &'de [u8; N]
impl<'de, const N: usize> Deserialize<'de> for &'de [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct _Vistor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for _Vistor<N> {
            type Value = &'de [u8; N];

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(format_args!("byte array of length {}", N))
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v.len() < N {
                    return Err(E::invalid_length(v.len(), &self));
                }

                Ok(v.try_into().unwrap())
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_borrowed_bytes(v.as_bytes())
            }
        }

        let vistor = _Vistor;
        let arr: &'de [u8; N] = deserializer.deserialize_bytes(vistor)?;
        Ok(arr)
    }
}

impl<'de, T> Deserialize<'de> for Option<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct _Vistor<T> {
            _mark: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for _Vistor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("optional byte array")
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Some)
            }
        }

        let visitor = _Vistor { _mark: PhantomData };
        deserializer.deserialize_option(visitor)
    }
}
