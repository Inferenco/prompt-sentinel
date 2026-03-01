use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum BiasLevel {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum BiasCategory {
    Gender,
    RaceEthnicity,
    Age,
    Religion,
    Disability,
    SocioEconomic,
    HarmfulLanguage,
}
