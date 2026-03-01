use std::collections::HashSet;
use std::sync::Arc;

use super::dtos::{BiasScanRequest, BiasScanResult};
use super::model::{BiasCategory, BiasLevel};

#[derive(Clone)]
pub struct BiasDetectionService {
    default_threshold: f32,
    mistral_service: Option<Arc<dyn crate::modules::mistral_ai::client::MistralClient>>,
}

#[derive(Clone, Debug)]
struct BiasRule {
    category: BiasCategory,
    terms: &'static [&'static str],
    weight: f32,
    hint: &'static str,
}

const RULES: &[BiasRule] = &[
    // Gender bias
    BiasRule {
        category: BiasCategory::Gender,
        terms: &[
            "women are bad at",
            "women are generally bad at",
            "men are naturally better",
            "female drivers",
            "women belong in",
            "men are superior",
            "man is superior",
            "male superiority",
            "men are better",
            "man is better",
            "girls can't",
            "boys are better at",
            "women can't",
            "men are smarter",
            "man is smarter",
            "explain why men make better",
            "why men are better",
            "men make better engineers",
        ],
        weight: 0.35,
        hint: "Avoid gender generalizations and attribute behavior to individuals.",
    },
    // Race/Ethnicity bias
    BiasRule {
        category: BiasCategory::RaceEthnicity,
        terms: &[
            "those people are", 
            "all immigrants", 
            "racially inferior",
            "all [race] are",
            "[race] people are",
            "illegal alien",
            "ghetto",
            "thug",
        ],
        weight: 0.45,
        hint: "Avoid race/ethnicity stereotypes and use evidence-based wording.",
    },
    // Age bias
    BiasRule {
        category: BiasCategory::Age,
        terms: &[
            "too old to", 
            "young people are lazy", 
            "elderly cannot",
            "old people are",
            "millennials are",
            "boomers are",
            "senile",
        ],
        weight: 0.30,
        hint: "Reframe age assumptions as role-specific skill criteria.",
    },
    // Religion bias
    BiasRule {
        category: BiasCategory::Religion,
        terms: &[
            "all muslims", 
            "all christians", 
            "religion makes people",
            "all [religion] are",
            "[religion] people are",
            "infidel",
            "heathen",
        ],
        weight: 0.40,
        hint: "Use respectful, non-generalizing language about faith groups.",
    },
    // Disability bias
    BiasRule {
        category: BiasCategory::Disability,
        terms: &[
            "disabled people cannot", 
            "wheelchair bound people are",
            "retarded",
            "crippled",
            "mentally ill people are",
            "autistic people are",
        ],
        weight: 0.40,
        hint: "Use person-first wording and avoid assumptions about capability.",
    },
    // Socioeconomic bias
    BiasRule {
        category: BiasCategory::SocioEconomic,
        terms: &[
            "poor people are lazy", 
            "low income people are dishonest",
            "rich people are greedy",
            "welfare queens",
            "trailer trash",
            "white trash",
        ],
        weight: 0.35,
        hint: "Avoid socioeconomic stereotyping and reference context factors.",
    },
    // Harmful language
    BiasRule {
        category: BiasCategory::HarmfulLanguage,
        terms: &[
            "credit card",
            "ash hole",
            "asshole",
            "fuck",
            "shit",
            "bitch",
            "whore",
            "cunt",
            "dick",
            "pussy",
            "nigger",
            "faggot",
            "retard",
            "kill yourself",
            "die",
            "suicide",
            "rape",
            "pedo",
            "child porn",
        ],
        weight: 0.50,
        hint: "Avoid offensive, harmful, or dangerous language.",
    },
];

impl BiasDetectionService {
    pub fn new(default_threshold: f32) -> Self {
        Self {
            default_threshold,
            mistral_service: None,
        }
    }

    pub fn new_with_mistral(
        default_threshold: f32,
        mistral_service: Arc<dyn crate::modules::mistral_ai::client::MistralClient>,
    ) -> Self {
        Self {
            default_threshold,
            mistral_service: Some(mistral_service),
        }
    }

    async fn translate_if_needed(&self, text: &str) -> String {
        let Some(mistral_service) = &self.mistral_service else {
            return text.to_owned();
        };

        // Always translate to English for consistent analysis when Mistral service is available
        let Ok(translation) = mistral_service
            .translate_text(crate::modules::mistral_ai::dtos::TranslationRequest {
                text: text.to_owned(),
                target_language: "English".to_owned(),
            })
            .await
        else {
            return text.to_owned();
        };
        
        translation.translated_text
    }

    pub async fn scan(&self, request: BiasScanRequest) -> BiasScanResult {
        let text_to_analyze = self.translate_if_needed(&request.text).await;
        let threshold = normalize_threshold(request.threshold, self.default_threshold);
        let normalized = text_to_analyze.to_ascii_lowercase();

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
        let high_cutoff = high_risk_cutoff(threshold);
        let level = if score >= high_cutoff {
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

/// Applies an optional caller override and clamps the effective threshold
/// into a safe range so scoring stays predictable across inputs.
fn normalize_threshold(override_threshold: Option<f32>, default_threshold: f32) -> f32 {
    let threshold = override_threshold
        .filter(|value| value.is_finite())
        .unwrap_or(default_threshold);
    threshold.clamp(0.0, 1.0)
}

/// Derives a stricter "high risk" cutoff from the base threshold while
/// preserving monotonic ordering (`high >= threshold`).
fn high_risk_cutoff(threshold: f32) -> f32 {
    let mut cutoff = (threshold + 0.30).clamp(0.60, 0.95);
    if cutoff < threshold {
        cutoff = threshold;
    }
    cutoff
}

impl Default for BiasDetectionService {
    fn default() -> Self {
        Self {
            default_threshold: 0.35,
            mistral_service: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_low_for_neutral_text() {
        let service = BiasDetectionService::default();
        let result = service.scan(BiasScanRequest {
            text: "Summarize the quarterly financial report".to_owned(),
            threshold: None,
        }).await;
        assert_eq!(result.level, BiasLevel::Low);
    }

    #[tokio::test]
    async fn returns_high_for_multiple_categories() {
        let service = BiasDetectionService::default();
        let result = service.scan(BiasScanRequest {
            text: "Women are bad at math and poor people are lazy".to_owned(),
            threshold: None,
        }).await;
        assert_eq!(result.level, BiasLevel::High);
        assert!(result.score > 0.5);
    }

    #[tokio::test]
    async fn nan_threshold_falls_back_to_default_threshold() {
        let service = BiasDetectionService::default();
        let default_result = service.scan(BiasScanRequest {
            text: "Women are bad at math".to_owned(),
            threshold: None,
        }).await;
        let nan_result = service.scan(BiasScanRequest {
            text: "Women are bad at math".to_owned(),
            threshold: Some(f32::NAN),
        }).await;
        assert_eq!(default_result.level, nan_result.level);
    }
}
