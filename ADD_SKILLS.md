---
# SKILLs to add

name: bias-detection
description: Detects and analyzes biased content in LLM interactions using Mistral Vibe tools  
license: MIT  
compatibility: Rust 1.90+  
user-invocable: true  
allowed-tools:  

- ask_user_question  
- bash  
- grep  
- read_file  
- search_replace  
- write_file  
- analyze_text  
- classify_content

# Bias Detection Skill Implementation

You are an expert in **bias detection and mitigation** for LLM interactions using the Mistral Vibe ecosystem.

**User request:**  
{{INPUT}}

**Guidelines:**

- Use the Mistral Vibe tools for bias detection and mitigation
- Follow Rust best practices for implementation
- Ensure all implementations work with the Mistral Vibe toolset
- Avoid creating dummy data or workarounds - if implementation fails, research and retry

**Key Concepts:**

1. **Text Analysis**:
  - Use `read_file` to load bias patterns and configuration
  - Use `grep` for pattern matching in text
  - Use `search_replace` for text modifications
2. **Bias Detection**:
  - Use the available tools to implement detection functionality
  - Calculate bias scores based on pattern matches
3. **Classification**:
  - Use pattern matching to categorize bias types
  - Supported categories: racial, gender, religious, political, socioeconomic
4. **Reporting**:
  - Use `write_file` to save detection results
  - Use `ask_user_question` for clarification when needed

**Implementation Requirements:**

```rust
// Required implementation in src/modules/bias_detection/mod.rs

pub fn detect_bias(text: &str) -> Result<BiasDetectionResult, DetectionError> {
    // Load bias patterns from file
    let patterns = vibe_tools::read_file("bias_patterns.txt")?;

    // Search for bias patterns using grep
    let matches = vibe_tools::grep(text, &patterns)?;

    // Classify matches by category
    let categories = classify_matches(&matches);

    // Calculate bias score
    let score = calculate_bias_score(&matches);

    // Return results
    Ok(BiasDetectionResult {
        score,
        categories,
        matches
    })
}
```

**Example: Using Built-in Tools**

```rust
// Load patterns
let patterns = vibe_tools::read_file("bias_patterns.txt")?;

// Detect bias using grep
let matches = vibe_tools::grep(input_text, &patterns)?;

// Calculate score based on match count and severity
let score = matches.len() as f32 / 100.0; // Example calculation

// Save results
vibe_tools::write_file("detection_results.json", &results)?;
```

**Configuration:**

Create `.vibe/config.toml`:

```toml
[skills.bias_detection]
path = "./skills/bias_detection"
enabled = true

[tools]
ask_user_question = { permission = "ALWAYS" }
bash = { permission = "ASK" }
grep = { permission = "ALWAYS" }
read_file = { permission = "ALWAYS" }
write_file = { permission = "ALWAYS" }
search_replace = { permission = "ALWAYS" }
```

**Best Practices:**

1. **Pattern Management**:
  - Store bias patterns in text files
  - Use `read_file` and `write_file` for pattern management
2. **Text Processing**:
  - Use `grep` for efficient pattern matching
  - Use `search_replace` for text sanitization
3. **User Interaction**:
  - Use `ask_user_question` for ambiguous cases
  - Use `bash` for external processing when needed
4. **Configuration**:
  - Store patterns in text files
  - Make thresholds configurable

**File Structure:**

```
.vibe/
├── config.toml         # Main configuration
├── prompts/
│   └── bias_detection.md  # Custom prompt
└── skills/
    └── bias_detection/  # Skill directory
        ├── SKILL.md     # Skill configuration
        └── patterns.txt # Bias patterns

src/
└── modules/
    └── bias_detection/  # Implementation
        ├── mod.rs       # Main implementation
        ├── dtos.rs      # Data structures
        └── service.rs   # Detection logic
```

**Testing Requirements:**

1. Unit tests for pattern matching
2. Integration tests for full workflow
3. Performance tests with sample texts
4. Edge case testing for different text types

**Example Output:**

```json
{
  "bias_score": 0.72,
  "categories": ["gender", "socioeconomic"],
  "matches": [
    {"pattern": "gender_bias", "text": "chairman", "line": 3},
    {"pattern": "socioeconomic", "text": "poor people", "line": 7}
  ],
  "suggested_fixes": [
    "Replace 'chairman' with 'chairperson'",
    "Consider more inclusive language"
  ]
}
```

**Implementation Notes:**

1. All tools must be properly configured in Vibe
2. Leverage built-in tool capabilities
3. Pattern files should be in project root
4. Error handling should be comprehensive

**Research and Implementation:**  
When implementing tools, always:

1. Test with real-world examples
2. Validate pattern files
3. Iterate until all functionality works
4. Document configuration options

---

## name: bias-detection

description: Detects and analyzes biased content in LLM interactions using Mistral Vibe tools  
license: MIT  
compatibility: Rust 1.90+  
user-invocable: true  
allowed-tools:  

- ask_user_question  
- bash  
- grep  
- read_file  
- search_replace  
- write_file  
- analyze_text  
- classify_content

# Bias Detection Skill Implementation

You are an expert in **bias detection and mitigation** for LLM interactions using the Mistral Vibe ecosystem.

**User request:**  
{{INPUT}}

**Guidelines:**

- Use the Mistral Vibe tools for bias detection and mitigation
- Follow Rust best practices for implementation
- Ensure all implementations work with the Mistral Vibe toolset
- Avoid creating dummy data or workarounds - if implementation fails, research and retry

**Key Concepts:**

1. **Text Analysis**:
  - Use `read_file` to load bias patterns and configuration
  - Use `grep` for pattern matching in text
  - Use `search_replace` for text modifications
2. **Bias Detection**:
  - Use the available tools to implement detection functionality
  - Calculate bias scores based on pattern matches
3. **Classification**:
  - Use pattern matching to categorize bias types
  - Supported categories: racial, gender, religious, political, socioeconomic
4. **Reporting**:
  - Use `write_file` to save detection results
  - Use `ask_user_question` for clarification when needed

**Implementation Requirements:**

```rust
// Required implementation in src/modules/bias_detection/mod.rs

pub fn detect_bias(text: &str) -> Result<BiasDetectionResult, DetectionError> {
    // Load bias patterns from file
    let patterns = vibe_tools::read_file("bias_patterns.txt")?;

    // Search for bias patterns using grep
    let matches = vibe_tools::grep(text, &patterns)?;

    // Classify matches by category
    let categories = classify_matches(&matches);

    // Calculate bias score
    let score = calculate_bias_score(&matches);

    // Return results
    Ok(BiasDetectionResult {
        score,
        categories,
        matches
    })
}
```

**Example: Using Built-in Tools**

```rust
// Load patterns
let patterns = vibe_tools::read_file("bias_patterns.txt")?;

// Detect bias using grep
let matches = vibe_tools::grep(input_text, &patterns)?;

// Calculate score based on match count and severity
let score = matches.len() as f32 / 100.0; // Example calculation

// Save results
vibe_tools::write_file("detection_results.json", &results)?;
```

**Configuration:**

Create `.vibe/config.toml`:

```toml
[skills.bias_detection]
path = "./skills/bias_detection"
enabled = true

[tools]
ask_user_question = { permission = "ALWAYS" }
bash = { permission = "ASK" }
grep = { permission = "ALWAYS" }
read_file = { permission = "ALWAYS" }
write_file = { permission = "ALWAYS" }
search_replace = { permission = "ALWAYS" }
```

**Best Practices:**

1. **Pattern Management**:
  - Store bias patterns in text files
  - Use `read_file` and `write_file` for pattern management
2. **Text Processing**:
  - Use `grep` for efficient pattern matching
  - Use `search_replace` for text sanitization
3. **User Interaction**:
  - Use `ask_user_question` for ambiguous cases
  - Use `bash` for external processing when needed
4. **Configuration**:
  - Store patterns in text files
  - Make thresholds configurable

**File Structure:**

```
.vibe/
├── config.toml         # Main configuration
├── prompts/
│   └── bias_detection.md  # Custom prompt
└── skills/
    └── bias_detection/  # Skill directory
        ├── SKILL.md     # Skill configuration
        └── patterns.txt # Bias patterns

src/
└── modules/
    └── bias_detection/  # Implementation
        ├── mod.rs       # Main implementation
        ├── dtos.rs      # Data structures
        └── service.rs   # Detection logic
```

**Testing Requirements:**

1. Unit tests for pattern matching
2. Integration tests for full workflow
3. Performance tests with sample texts
4. Edge case testing for different text types

**Example Output:**

```json
{
  "bias_score": 0.72,
  "categories": ["gender", "socioeconomic"],
  "matches": [
    {"pattern": "gender_bias", "text": "chairman", "line": 3},
    {"pattern": "socioeconomic", "text": "poor people", "line": 7}
  ],
  "suggested_fixes": [
    "Replace 'chairman' with 'chairperson'",
    "Consider more inclusive language"
  ]
}
```

**Implementation Notes:**

1. All tools must be properly configured in Vibe
2. Leverage built-in tool capabilities
3. Pattern files should be in project root
4. Error handling should be comprehensive

**Research and Implementation:**  
When implementing tools, always:

1. Test with real-world examples
2. Validate pattern files
3. Iterate until all functionality works
4. Document configuration options

## name: prompt-firewall

description: Detects and blocks malicious prompt injection attempts using Mistral Vibe tools  
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

# Prompt Firewall Skill Implementation

You are an expert in **prompt injection detection and mitigation** for LLM interactions using the Mistral Vibe ecosystem.

**User request:**  
{{INPUT}}

**Guidelines:**

- Use the Mistral Vibe tools for prompt injection detection and mitigation
- Follow Rust best practices for implementation
- Ensure all implementations work with the Mistral Vibe toolset
- Avoid creating dummy data or workarounds - if implementation fails, research and retry

**Key Concepts:**

1. **Injection Detection**:
  - Use `read_file` to load injection patterns and rules
  - Use `grep` for pattern matching in prompts
  - Use `search_replace` for sanitization
2. **Validation Rules**:
  - Implement common injection patterns using regular expressions
  - Use pattern files for maintainable rule management
3. **Sanitization**:
  - Use `search_replace` to clean malicious inputs
  - Implement safe replacements for common patterns
4. **Reporting**:
  - Use `write_file` to log injection attempts
  - Use `ask_user_question` for user confirmation when needed

**Implementation Requirements:**

```rust
// Required implementation in src/modules/prompt_firewall/mod.rs

pub fn validate_prompt(prompt: &str) -> Result<ValidationResult, FirewallError> {
    // Load injection patterns from file
    let patterns = vibe_tools::read_file("injection_patterns.txt")?;

    // Search for malicious patterns using grep
    let matches = vibe_tools::grep(prompt, &patterns)?;

    if matches.is_empty() {
        // No malicious patterns found
        Ok(ValidationResult {
            is_valid: true,
            sanitized_prompt: prompt.to_string(),
            detected_patterns: vec![],
        })
    } else {
        // Malicious patterns detected
        // Option 1: Reject the prompt
        // Option 2: Sanitize the prompt
        let sanitized = sanitize_prompt(prompt, &matches)?;

        Ok(ValidationResult {
            is_valid: false,
            sanitized_prompt: sanitized,
            detected_patterns: matches,
        })
    }
}

pub fn sanitize_prompt(prompt: &str, matches: &[PatternMatch]) -> Result<String, FirewallError> {
    // Load replacement rules
    let rules = load_replacement_rules()?;

    // Apply sanitization using search_replace
    let mut sanitized = prompt.to_string();

    for match in matches {
        if let Some(rule) = rules.get(match.pattern.as_str()) {
            sanitized = vibe_tools::search_replace(&sanitized, &match.text, &rule.replacement)?;
        }
    }

    Ok(sanitized)
}
```

**Configuration:**

Create `.vibe/config.toml`:

```toml
[skills.prompt_firewall]
path = "./skills/prompt_firewall"
enabled = true

[tools]
ask_user_question = { permission = "ALWAYS" }
bash = { permission = "ASK" }
grep = { permission = "ALWAYS" }
read_file = { permission = "ALWAYS" }
write_file = { permission = "ALWAYS" }
search_replace = { permission = "ALWAYS" }
```

**File Structure:**

```
.vibe/
├── config.toml             # Main configuration
├── prompts/
│   └── prompt_firewall.md  # Custom prompt
└── skills/
    └── prompt_firewall/    # Skill directory
        ├── SKILL.md        # Skill configuration
        ├── patterns.txt    # Injection patterns
        └── rules.txt       # Sanitization rules

src/
└── modules/
    └── prompt_firewall/    # Implementation
        ├── mod.rs          # Main implementation
        ├── dtos.rs         # Data structures
        └── service.rs      # Firewall logic
```

**Pattern Management:**

1. Create `patterns.txt` with common injection patterns:

```
--ignore-previous-instruction
--forget-earlier-instructions
--role-play-as
--simulate-scenario
--override-defaults
```

2. Create `rules.txt` with replacement rules:

```
pattern=--ignore-previous-instruction,replacement=--continue-conversation
pattern=--forget-earlier-instructions,replacement=--maintain-context
```

**Best Practices:**

1. **Pattern Management**:
  - Store injection patterns in text files
  - Use `read_file` to load patterns at runtime
2. **Validation**:
  - Use `grep` for efficient pattern matching
  - Implement both detection and sanitization modes
3. **User Interaction**:
  - Use `ask_user_question` for confirmation
  - Provide clear explanations of detected patterns
4. **Configuration**:
  - Make rules configurable
  - Allow easy updates to patterns

**Example Usage:**

```rust
// Validate a prompt
let result = validate_prompt(user_input)?;

match result.is_valid {
    true => {
        // Safe to use the prompt
        let response = call_llm(&result.sanitized_prompt).await?;
    }
    false => {
        // Handle malicious input
        if result.detected_patterns.is_empty() {
            // User notification
        } else {
            // Log the attempt
            vibe_tools::write_file("injection_log.txt", &result)?;
        }
    }
}
```

**Testing Requirements:**

1. Unit tests for pattern matching
2. Integration tests for full validation workflow
3. Performance tests with various prompt lengths
4. Edge case testing for different injection patterns

**Example Output:**

```json
{
  "is_valid": false,
  "sanitized_prompt": "Continue our conversation about the weather...",
  "detected_patterns": [
    {
      "pattern": "--ignore-previous-instruction",
      "text": "--ignore all previous instructions",
      "line": 1,
      "severity": "high"
    }
  ],
  "suggested_action": "Use sanitized prompt or reject request"
}
```

**Implementation Notes:**

1. The prompt firewall should be the first step in the processing pipeline
2. Combine detection and sanitization for best results
3. Keep pattern files up-to-date with new injection techniques
4. Provide clear feedback to users about detected patterns

**Research and Implementation:**  
When implementing the firewall:

1. Study common prompt injection techniques
2. Test with real-world malicious prompts
3. Update patterns regularly
4. Document all detection rules

---

## name: eu-law-compliance  
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

**Configuration:**

Create `.vibe/config.toml`:

```toml
[skills.eu_law_compliance]
path = "./skills/eu_law_compliance"
enabled = true

[tools]
ask_user_question = { permission = "ALWAYS" }
bash = { permission = "ASK" }
grep = { permission = "ALWAYS" }
read_file = { permission = "ALWAYS" }
write_file = { permission = "ALWAYS" }
search_replace = { permission = "ALWAYS" }
```

**File Structure:**

```
.vibe/
├── config.toml             # Main configuration
├── prompts/
│   └── eu_compliance.md    # Custom prompt
└── skills/
    └── eu_law_compliance/  # Skill directory
        ├── SKILL.md        # Skill configuration
        └── requirements.txt # Compliance requirements

src/
└── modules/
    └── eu_law_compliance/  # Implementation
        ├── mod.rs          # Main implementation
        ├── dtos.rs         # Data structures
        └── service.rs      # Compliance logic
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
