---
name: eu-law-compliance
description: Ensures compliance with EU AI Act requirements for large language models
license: MIT
compatibility: Rust 1.90+
user-invocable: true
allowed-tools:
  - ask_user_question
  - bash
  - grep
  - read_file
  - write_file
  - search_replace
---

# EU AI Act Compliance Skill Implementation

You are an expert in **EU AI Act compliance** for large language models and AI systems.

**User request:**
{{INPUT}}

**Guidelines:**

- Implement compliance checks based on EU AI Act requirements
- Follow the regulation requirements from [https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng](https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng)
- Ensure all implementations work with Mistral Vibe tools

**Key Compliance Requirements:**

1. **Transparency Obligations**:
  - Technical documentation requirements
  - Training data summaries
  - Copyright compliance information
2. **Risk Management**:
  - Systemic risk assessments for large models (>10²⁵ FLOPs)
  - Cybersecurity measures
  - Adversarial testing
3. **Reporting**:
  - Serious incident reporting to European AI Office
  - Compliance documentation

**Implementation Requirements:**

```rust
// Required implementation in src/modules/eu_law_compliance/mod.rs

pub fn check_compliance(model_info: &ModelInfo) -> Result<ComplianceResult, ComplianceError> {
    // Check system risk level
    let risk_level = determine_risk_level(model_info.training_scale)?;

    // Validate technical documentation
    let doc_check = validate_documentation(model_info)?;

    // Check copyright compliance
    let copyright_check = validate_copyright(model_info)?;

    // Check transparency requirements
    let transparency_check = validate_transparency(model_info)?;

    // Return comprehensive compliance status
    Ok(ComplianceResult {
        is_compliant: doc_check && copyright_check && transparency_check,
        risk_level,
        requirements: vec![
            doc_check,
            copyright_check,
            transparency_check
        ]
    })
}
```

**Key Compliance Checks:**

1. **Technical Documentation**:
  - Check for complete model documentation
  - Validate training data sources
  - Verify system capabilities documentation
2. **Copyright Compliance**:
  - Check data source licensing
  - Validate copyright notifications
  - Verify training data provenance
3. **Transparency**:
  - Validate public information requirements
  - Check content generation disclosures
  - Verify system capability documentation

**Example Compliance Checklist:**

```markdown
1. Technical Documentation:
   - Model architecture description
   - Training methodology
   - System capabilities and limitations

2. Copyright Compliance:
   - Data source licensing information
   - Copyright holder notifications
   - Training data provenance records

3. Transparency Requirements:
   - Public information about model capabilities
   - Content generation disclosures
   - System limitation documentation
```

**Implementation Notes:**

1. The skill should implement checks for all EU AI Act requirements
2. Use available tools for documentation processing and validation
3. Provide clear compliance reports for auditing
4. Implement risk assessment for large models

**Example Output:**

```json
{
  "is_compliant": true,
  "risk_level": "medium",
  "requirements": {
    "technical_documentation": {
      "status": "compliant",
      "details": "All required documentation provided"
    },
    "copyright_compliance": {
      "status": "compliant",
      "details": "All data sources properly licensed"
    },
    "transparency": {
      "status": "compliant",
      "details": "All public information requirements met"
    }
  },
  "recommendations": [
    "Maintain records of compliance checks",
    "Update documentation with model changes",
    "Monitor for regulatory updates"
  ]
}
```

**Testing Requirements:**

1. Unit tests for each compliance check
2. Integration tests for full compliance workflow
3. Validation against official EU guidelines
4. Edge case testing for different model types

**Best Practices:**

1. **Documentation**:
  - Store compliance requirements in files
  - Use `read_file` to load requirements at runtime
2. **Validation**:
  - Use `grep` for document content checks
  - Implement comprehensive validation rules
3. **Reporting**:
  - Use `write_file` to generate compliance reports
  - Provide clear compliance status information

**Research and Implementation:**
When implementing the compliance checks:

1. Study the official EU AI Act text
2. Follow the Code of Practice guidelines
3. Implement all required documentation
4. Document all compliance decisions
