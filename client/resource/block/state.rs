use std::fmt;

use anyhow::Result;
use arrayvec::ArrayVec;
use serde::de::{
    self, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Unexpected,
    Visitor,
};
use serde_state::DeserializeState;
use world::block::{state, Block};

use super::model::Bakery;

const MAX_PROPERTIES: usize = 6;
const MAX_PROPERTY_VALUES: usize = 16;

/// Multi-variant block state predicates only need to compare against one block state.
#[derive(Debug, Default)]
pub(super) struct VariantPredicate {
    /// Mask of block state properties to compare.
    block_state_mask: u32,
    /// The reference block state to compare against.
    block_state: u32,
}

impl VariantPredicate {
    pub fn call(&self, block: Block) -> bool {
        block.as_u32() & self.block_state_mask == self.block_state
    }
}

pub(super) struct VariantPredicateSeed<'seed, 'arena> {
    pub state: &'seed mut Bakery<'arena>,
}

impl<'de: 'arena, 'arena> DeserializeSeed<'de> for VariantPredicateSeed<'_, 'arena> {
    type Value = VariantPredicate;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de: 'arena, 'arena> Visitor<'de> for VariantPredicateSeed<'_, 'arena> {
    type Value = VariantPredicate;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string")
    }

    fn visit_str<E>(self, block_states: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if block_states.is_empty() {
            return Ok(VariantPredicate::default());
        }

        let block_id = self.state.block_id;
        let state_definition = block_id.state_definition();
        let property_name_seed = PropertyNameSeed { state_definition };

        let mut block_state_mask = 0;
        let mut block_state = 0;

        for name_value in block_states.split(',') {
            let Some((name, value)) = name_value.split_once('=') else {
                return Err(de::Error::custom(format_args!(
                    "invalid key-value pair `{name_value}`, expected to find an `=`",
                )));
            };
            let property_index = property_name_seed.visit_str(name)?;
            let state::PropertyDefinition { id, offset, .. } =
                state_definition.properties[property_index];
            let property = id.deserialize(value.into_deserializer())?;

            block_state_mask |= ((1 << id.bits()) - 1) << offset;
            block_state |= property.to_bits() << offset;
        }

        Ok(VariantPredicate {
            block_state_mask,
            block_state,
        })
    }
}

/// Multipart block state predicates can have OR-conditions, so there are potentially countless
/// comparisons necessary. We instead use a bit-field indexed by the block state.
#[derive(Debug, Default)]
pub(super) struct PartPredicate {
    /// Each bit in the field determines whether the block state used as index matches.
    bitfield: Option<Box<[u64]>>,
}

impl PartPredicate {
    pub fn call(&self, block: Block) -> bool {
        if let Some(bitfield) = &self.bitfield {
            let state = block.as_u32() >> u16::BITS;
            let word_index = state / u64::BITS;
            let bit_index = state % u64::BITS;

            bitfield[word_index as usize] & (1 << bit_index) != 0
        } else {
            true
        }
    }
}

impl<'de: 'arena, 'arena> DeserializeState<'de, Bakery<'arena>> for PartPredicate {
    fn deserialize_state<D>(state: &mut Bakery<'arena>, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PartPredicateVisitor { state })
    }
}

struct PartPredicateVisitor<'seed, 'arena> {
    state: &'seed mut Bakery<'arena>,
}

impl<'de: 'arena, 'arena> Visitor<'de> for PartPredicateVisitor<'_, 'arena> {
    type Value = PartPredicate;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let block_id = self.state.block_id;
        let state_definition = block_id.state_definition();
        let property_name_seed = PropertyNameSeed { state_definition };
        let last_property_definition = state_definition.properties.last().unwrap();
        let bits = last_property_definition.offset + last_property_definition.id.bits() - u16::BITS;
        let words = ((1 << bits) - 1) / u64::BITS + 1;

        let mut bitfield = (0..words).map(|_| 0).collect::<Box<[u64]>>();
        let mut scratch = [const { ArrayVec::new_const() }; MAX_PROPERTIES];
        let mut first = true;

        loop {
            let property_index = if first {
                match map.next_key_seed(FieldSeed(property_name_seed))? {
                    Some(Field::OR) => {
                        return map.next_value_seed(OrSeed {
                            property_name_seed,
                            bitfield,
                            scratch: &mut scratch,
                        });
                    }
                    Some(Field::Property(index)) => index,
                    None => return Ok(PartPredicate::default()),
                }
            } else {
                match map.next_key_seed(property_name_seed)? {
                    Some(index) => index,
                    None => break,
                }
            };
            first = false;

            map.next_value_seed(PropertyValueSeed {
                property_definition: &state_definition.properties[property_index],
                scratch: &mut scratch[property_index],
            })?;
        }

        generate_combinations(&mut bitfield, &scratch);

        Ok(PartPredicate {
            bitfield: Some(bitfield),
        })
    }
}

fn generate_combinations(
    bitfield: &mut [u64],
    scratch: &[ArrayVec<u32, MAX_PROPERTY_VALUES>; MAX_PROPERTIES],
) {
    let mut current_indices = [0; MAX_PROPERTIES];

    loop {
        let mut bits = 0;

        for (properties, &current_index) in scratch.iter().zip(&current_indices) {
            bits |= properties.get(current_index).unwrap_or(&0);
        }

        let word_index = bits / u64::BITS;
        let bit_index = bits % u64::BITS;

        bitfield[word_index as usize] |= 1 << bit_index;

        for (i, (properties, current_index)) in scratch.iter().zip(&mut current_indices).enumerate()
        {
            *current_index += 1;

            if *current_index < properties.len() {
                break;
            }

            *current_index = 0;

            if i == MAX_PROPERTIES - 1 {
                return;
            }
        }
    }
}

enum Field {
    OR,
    Property(usize),
}

#[derive(Clone, Copy)]
struct FieldSeed(PropertyNameSeed);

impl<'de> DeserializeSeed<'de> for FieldSeed {
    type Value = Field;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for FieldSeed {
    type Value = Field;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let field = match s {
            "OR" => Field::OR,
            _ => Field::Property(self.0.visit_str(s)?),
        };

        Ok(field)
    }
}

#[derive(Clone, Copy)]
struct PropertyNameSeed {
    state_definition: &'static state::Definition,
}

impl<'de> DeserializeSeed<'de> for PropertyNameSeed {
    type Value = usize;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for PropertyNameSeed {
    type Value = usize;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        for (index, &name) in self.state_definition.property_names.iter().enumerate() {
            if s == name {
                return Ok(index);
            }
        }

        Err(de::Error::unknown_field(
            s,
            self.state_definition.property_names,
        ))
    }
}

struct OrSeed<'seed> {
    property_name_seed: PropertyNameSeed,
    bitfield: Box<[u64]>,
    scratch: &'seed mut [ArrayVec<u32, MAX_PROPERTY_VALUES>; MAX_PROPERTIES],
}

impl<'de> DeserializeSeed<'de> for OrSeed<'_> {
    type Value = PartPredicate;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> Visitor<'de> for OrSeed<'_> {
    type Value = PartPredicate;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a sequence")
    }

    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(()) = seq.next_element_seed(PropertyNameValueSeed {
            property_name_seed: self.property_name_seed,
            scratch: self.scratch,
        })? {
            generate_combinations(&mut self.bitfield, self.scratch);
        }

        Ok(PartPredicate {
            bitfield: Some(self.bitfield),
        })
    }
}

struct PropertyNameValueSeed<'seed> {
    property_name_seed: PropertyNameSeed,
    scratch: &'seed mut [ArrayVec<u32, MAX_PROPERTY_VALUES>; MAX_PROPERTIES],
}

impl<'de> DeserializeSeed<'de> for PropertyNameValueSeed<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for PropertyNameValueSeed<'_> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let state_definition = self.property_name_seed.state_definition;

        while let Some(property_index) = map.next_key_seed(self.property_name_seed)? {
            map.next_value_seed(PropertyValueSeed {
                property_definition: &state_definition.properties[property_index],
                scratch: &mut self.scratch[property_index],
            })?;
        }

        Ok(())
    }
}

struct PropertyValueSeed<'seed> {
    property_definition: &'static state::PropertyDefinition,
    scratch: &'seed mut ArrayVec<u32, MAX_PROPERTY_VALUES>,
}

impl<'de> DeserializeSeed<'de> for PropertyValueSeed<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for PropertyValueSeed<'_> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string")?;

        // WHY MOJANG?? WHYYYY????
        match self.property_definition.id.ty() {
            state::PropertyType::Boolean => f.write_str(" or a boolean")?,
            state::PropertyType::Integer => f.write_str(" or an integer")?,
            state::PropertyType::Enum => {}
        }

        Ok(())
    }

    fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.property_definition.id.ty() == state::PropertyType::Boolean {
            let value = if b { "true" } else { "false" };

            add_property(self.property_definition, value, self.scratch)
        } else {
            Err(de::Error::invalid_type(Unexpected::Bool(b), &self))
        }
    }

    fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.property_definition.id.ty() == state::PropertyType::Integer {
            let value = n.to_string();

            add_property(self.property_definition, &value, self.scratch)
        } else {
            Err(de::Error::invalid_type(Unexpected::Unsigned(n), &self))
        }
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        for value in s.split('|') {
            add_property(self.property_definition, value, self.scratch)?;
        }

        Ok(())
    }
}

fn add_property<E>(
    property_definition: &'static state::PropertyDefinition,
    value: &str,
    scratch: &mut ArrayVec<u32, MAX_PROPERTY_VALUES>,
) -> Result<(), E>
where
    E: de::Error,
{
    let deserializer = value.into_deserializer();
    let property = property_definition.id.deserialize(deserializer)?;
    let bits = property.to_bits() << (property_definition.offset - u16::BITS);

    if !scratch.contains(&bits) {
        scratch.push(bits);
    }

    Ok(())
}
