---
name: prompt-firewall
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
---

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
