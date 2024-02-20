use super::PersonaId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Recipe {
    Normal(PersonaId, PersonaId),
    Special(Vec<PersonaId>),
}
