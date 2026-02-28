use std::collections::HashSet;

use super::dtos::{BiasScanRequest, BiasScanResult};
use super::model::{BiasCategory, BiasLevel};

#[derive(Clone, Debug)]
pub struct BiasDetectionService {
    default_threshold: f32,
}

#[derive(Clone, Debug)]
struct BiasRule {
    category: BiasCategory,
    terms: &'static [&'static str],
    weight: f32,
    hint: &'static str,
}

const RULES: &[BiasRule] = &[
    BiasRule {
        category: BiasCategory::Gender,
        terms: &[
            "women are bad at",
            "men are naturally better",
            "female drivers",
        ],
        weight: 0.35,
        hint: "Avoid gender generalizations and attribute behavior to individuals.",
    },
    BiasRule {
        category: BiasCategory::RaceEthnicity,
        terms: &["those people are", "all immigrants", "racially inferior"],
        weight: 0.45,
        hint: "Avoid race/ethnicity stereotypes and use evidence-based wording.",
    },
    BiasRule {
        category: BiasCategory::Age,
        terms: &["too old to", "young people are lazy", "elderly cannot"],
        weight: 0.30,
        hint: "Reframe age assumptions as role-specific skill criteria.",
    },
    BiasRule {
        category: BiasCategory::Religion,
        terms: &["all muslims", "all christians", "religion makes people"],
        weight: 0.40,
        hint: "Use respectful, non-generalizing language about faith groups.",
    },
    BiasRule {
        category: BiasCategory::Disability,
        terms: &["disabled people cannot", "wheelchair bound people are"],
        weight: 0.40,
        hint: "Use person-first wording and avoid assumptions about capability.",
    },
    BiasRule {
        category: BiasCategory::SocioEconomic,
        terms: &["poor people are lazy", "low income people are dishonest"],
        weight: 0.35,
        hint: "Avoid socioeconomic stereotyping and reference context factors.",
    },
];

impl BiasDetectionService {
    pub fn new(default_threshold: f32) -> Self {
        Self { default_threshold }
    }

    pub fn scan(&self, request: BiasScanRequest) -> BiasScanResult {
        let threshold = request.threshold.unwrap_or(self.default_threshold);
        let normalized = request.text.to_ascii_lowercase();

        let mut score = 0.0f32;
        let mut categories = HashSet::new();
        let mut matched_terms = Vec::new();
        let mut mitigation_hints = HashSet::new();

        for rule in RULES {
            for term in rule.terms {
                if normalized.contains(term) {
                    score += rule.weight;
                    categories.insert(rule.category.clone());
                    matched_terms.push((*term).to_owned());
                    mitigation_hints.insert(rule.hint.to_owned());
                }
            }
        }

        score = score.min(1.0);
        let level = if score >= (threshold + 0.30).min(0.85) {
            BiasLevel::High
        } else if score >= threshold {
            BiasLevel::Medium
        } else {
            BiasLevel::Low
        };

        let mut categories = categories.into_iter().collect::<Vec<_>>();
        categories.sort_by_key(|category| format!("{category:?}"));

        let mut mitigation_hints = mitigation_hints.into_iter().collect::<Vec<_>>();
        mitigation_hints.sort();

        BiasScanResult {
            score,
            level,
            categories,
            matched_terms,
            mitigation_hints,
        }
    }
}

impl Default for BiasDetectionService {
    fn default() -> Self {
        Self {
            default_threshold: 0.35,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_low_for_neutral_text() {
        let service = BiasDetectionService::default();
        let result = service.scan(BiasScanRequest {
            text: "Summarize the quarterly financial report".to_owned(),
            threshold: None,
        });
        assert_eq!(result.level, BiasLevel::Low);
    }

    #[test]
    fn returns_high_for_multiple_categories() {
        let service = BiasDetectionService::default();
        let result = service.scan(BiasScanRequest {
            text: "Women are bad at math and poor people are lazy".to_owned(),
            threshold: None,
        });
        assert_eq!(result.level, BiasLevel::High);
        assert!(result.score > 0.5);
    }
}
