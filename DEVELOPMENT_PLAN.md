# Portable AI Compliance Layer Documentation

## Project Overview

This project implements a Rust-based compliance layer that intercepts LLM calls, screens inputs/outputs for security and ethical issues, and generates verifiable evidence of all interactions. The system integrates Mistral Vibe skills and Mistral API for comprehensive compliance with EU AI Act requirements.

## Project Structure

```
project_root/
├── .vibe/                  # Mistral Vibe configuration
│   ├── config.toml         # Main Vibe configuration
│   ├── prompts/            # Custom prompts
│   │   ├── bias_detection.md
│   │   ├── eu_compliance.md
│   │   └── mistral_expert.md
│   └── skills/             # Vibe skills configurations
│       ├── bias_detection/
│       │   └── SKILL.md
│       ├── eu_law_compliance/
│       │   └── SKILL.md
│       ├── mistral_expert/
│       │   └── SKILL.md
│       └── prompt_firewall/
│           └── SKILL.md
├── src/
│   ├── config/             # Configuration management
│   │   ├── mod.rs
│   │   ├── settings.rs     # App settings
│   │   └── vibe_config.rs  # Vibe configuration
│   ├── modules/            # Business logic modules
│   │   ├── bias_detection/ # Bias detection module
│   │   │   ├── handler.rs  # HTTP endpoints
│   │   │   ├── dtos.rs     # Data transfer objects
│   │   │   ├── service.rs  # Core logic
│   │   │   └── model.rs    # Domain models
│   │   ├── eu_law_compliance/ # EU compliance module
│   │   │   ├── handler.rs
│   │   │   ├── dtos.rs
│   │   │   ├── service.rs
│   │   │   └── model.rs
│   │   ├── mistral_ai/ # Mistral integration
│   │   │   ├── handler.rs
│   │   │   ├── dtos.rs
│   │   │   ├── service.rs
│   │   │   └── client.rs   # API client
│   │   ├── prompt_firewall/ # Prompt injection firewall
│   │   │   ├── handler.rs
│   │   │   ├── dtos.rs
│   │   │   ├── service.rs
│   │   │   └── rules.rs
│   │   └── audit/          # Audit and compliance
│   │       ├── logger.rs
│   │       ├── proof.rs    # Cryptographic proofs
│   │       └── storage.rs  # Audit storage
│   ├── lib.rs              # Main library exports
│   └── main.rs             # Application entry point
├── Cargo.toml              # Rust project configuration
└── README.md               # Project documentation
```

## Key Components

### 1. Mistral Vibe Configuration

The `.vibe/` directory contains all Mistral Vibe configurations:

- `config.toml`: Main configuration file
- `prompts/`: Directory for custom prompts
- `skills/`: Directory for all Vibe skills

### 2. Rust Modules

#### Bias Detection Module

**Files:**

- `handler.rs`: HTTP endpoint handlers
- `dtos.rs`: Request/response structures
- `service.rs`: Core bias detection logic
- `model.rs`: Domain models

**Functionality:**

- Uses Mistral Vibe tools for text analysis
- Implements pattern matching for bias detection
- Generates comprehensive detection reports

#### EU Law Compliance Module

**Files:**

- `handler.rs`: Endpoint handlers
- `dtos.rs`: Data transfer objects
- `service.rs`: Core compliance logic
- `model.rs`: Domain models

**Functionality:**

- Implements EU AI Act compliance checks
- Validates technical documentation
- Ensures copyright compliance
- Provides transparency information

#### Mistral AI Module

**Files:**

- `handler.rs`: Endpoint handlers
- `dtos.rs`: Data transfer objects
- `service.rs`: Core API integration
- `client.rs`: Mistral API client

**Functionality:**

- Integrates with Mistral API
- Implements chat completions
- Provides embeddings functionality
- Handles content moderation

#### Prompt Firewall Module

**Files:**

- `handler.rs`: Endpoint handlers
- `dtos.rs`: Data transfer objects
- `service.rs`: Core firewall logic
- `rules.rs`: Injection rule definitions

**Functionality:**

- Detects prompt injection attempts
- Sanitizes malicious inputs
- Validates prompt structure
- Maintains allow/deny lists

#### Audit Module

**Files:**

- `logger.rs`: Audit logging
- `proof.rs`: Cryptographic proofs
- `storage.rs`: Audit storage

**Functionality:**

- Generates tamper-proof logs
- Creates cryptographic proofs
- Manages audit trail storage
- Provides compliance reporting

## Required Crates

```toml
[dependencies]
actix-web = "4.0"          # Web framework
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"         # JSON handling
reqwest = { version = "0.11", features = ["json"] } # HTTP client
tokio = { version = "1.0", features = ["full"] }   # Async runtime
tracing = "0.1"            # Logging
thiserror = "1.0"          # Error handling
```

## Implementation Details

### 1. Vibe Configuration

Configure Mistral Vibe in `.vibe/config.toml`:

```toml
[skills]
bias_detection = { path = "./skills/bias_detection" }
eu_law_compliance = { path = "./skills/eu_law_compliance" }
mistral_expert = { path = "./skills/mistral_expert" }
prompt_firewall = { path = "./skills/prompt_firewall" }

[agent]
name = "compliance_layer"
description = "AI Compliance Layer"

[mistral]
api_key = "your_api_key_here"
default_model = "mistral-large-latest"
```

### 2. Skill Configurations

#### Bias Detection Skill

```markdown
---
name: bias-detection
description: Detects biased content in LLM interactions
allowed-tools:
  - ask_user_question
  - grep
  - read_file
  - write_file
  - search_replace
```

#### EU Law Compliance Skill

```markdown
---
name: eu-law-compliance
description: Ensures compliance with EU AI Act requirements
allowed-tools:
  - ask_user_question
  - grep
  - read_file
  - write_file
```

#### Mistral Expert Skill

```markdown
---
name: mistral-expert
description: Expert in Mistral AI models and API integration
allowed-tools:
  - ask_user_question
  - grep
  - read_file
  - write_file
```

#### Prompt Firewall Skill

```markdown
---
name: prompt-firewall
description: Detects and blocks malicious prompt injection attempts
allowed-tools:
  - ask_user_question
  - grep
  - read_file
  - write_file
  - search_replace
```

### 3. Module Implementation

#### Bias Detection Module

```rust
// src/modules/bias_detection/service.rs

pub async fn detect_bias(text: &str) -> Result<BiasDetectionResult, DetectionError> {
    // Load bias patterns
    let patterns = vibe_tools::read_file("bias_patterns.txt")?;

    // Search for bias patterns
    let matches = vibe_tools::grep(text, &patterns)?;

    // Classify and score matches
    let result = classify_and_score(&matches);

    Ok(result)
}
```

#### EU Law Compliance Module

```rust
// src/modules/eu_law_compliance/service.rs

pub async fn check_compliance(model_info: &ModelInfo) -> Result<ComplianceResult, ComplianceError> {
    // Validate technical documentation
    let doc_check = validate_documentation(model_info)?;

    // Check copyright compliance
    let copyright_check = validate_copyright(model_info)?;

    // Verify transparency requirements
    let transparency_check = validate_transparency(model_info)?;

    Ok(ComplianceResult {
        is_compliant: doc_check && copyright_check && transparency_check,
        // Additional details
    })
}
```

#### Mistral Expert Module

```rust
// src/modules/mistral_ai/client.rs

pub async fn generate_completion(prompt: &str) -> Result<CompletionResult, MistralError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .bearer_auth(&config.api_key)
        .json(&json!({
            "model": "mistral-large-latest",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()
        .await?;

    response.json().await
}
```

## Development Process

### Day 1 Tasks

**Developer 1:**

1. Set up project structure
2. Implement prompt firewall module
3. Create basic Vibe configurations
4. Set up Mistral API integration

**Developer 2:**

1. Implement bias detection module
2. Set up EU compliance checks
3. Create initial DTOs
4. Begin service integration

### Day 2 Tasks

**Developer 1:**

1. Complete prompt firewall implementation
2. Implement audit module
3. Set up logging infrastructure

**Developer 2:**

1. Complete bias detection implementation
2. Integrate with Vibe skills
3. Implement endpoint handlers

## Testing Strategy

1. Unit tests for each module
2. Integration tests for module interactions
3. End-to-end tests for full request flow
4. Performance testing for latency
5. Security testing for injection attempts

## Deployment

1. Build release version:

```bash
cargo build --release
```

2. Configure environment variables:

```bash
export MISTRAL_API_KEY=your_api_key
export REDIS_URL=redis://localhost:6379
```

3. Run the application:

```bash
./target/release/ai-compliance
```

## Configuration

Example `settings.rs`:

```rust
pub struct AppSettings {
    pub mistral_api_key: String,
    pub default_model: String,
    pub bias_threshold: f32,
    pub max_input_length: usize,
}
```

## Error Handling

Implement comprehensive error handling:

```rust
#[derive(Error, Debug)]
pub enum ComplianceError {
    #[error("Bias detection failed")]
    BiasDetectionError(#[from] BiasError),
    #[error("Prompt validation failed")]
    PromptValidationError(#[from] FirewallError),
    #[error("API request failed")]
    ApiError(#[from] reqwest::Error),
    #[error("Compliance check failed")]
    ComplianceCheckError(String),
}
```

## Future Enhancements

1. Add more Vibe skills for additional compliance checks
2. Implement rate limiting
3. Add support for multiple LLM providers
4. Enhance audit reporting capabilities
5. Implement user management and authentication

This documentation provides a complete guide to implementing the Portable AI Compliance Layer with all required components and detailed implementation instructions for each module.
