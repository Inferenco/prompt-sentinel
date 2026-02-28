# Prompt Sentinel Configuration Guide

## Table of Contents

1. [Configuration Overview](#configuration-overview)
2. [Firewall Rules Configuration](#firewall-rules-configuration)
3. [EU Risk Keywords Configuration](#eu-risk-keywords-configuration)
4. [Framework Configuration](#framework-configuration)
5. [Environment Variables](#environment-variables)
6. [Advanced Configuration](#advanced-configuration)

## Configuration Overview

Prompt Sentinel uses a multi-layered configuration approach:

1. **JSON Configuration Files**: For rule-based configurations
2. **FrameworkConfig**: For runtime configuration
3. **Environment Variables**: For deployment-specific settings

## Firewall Rules Configuration

The firewall rules are defined in `config/firewall_rules.json`.

### Structure

```json
{
  "block_rules": [
    {
      "id": "string",
      "pattern": "string"
    }
  ],
  "sanitize_patterns": [
    {
      "id": "string",
      "pattern": "string"
    }
  ],
  "fuzzy_matching": {
    "enabled": boolean,
    "max_distance": integer
  }
}
```

### Block Rules

Block rules define patterns that should completely block a prompt from processing.

**Fields:**
- `id`: Unique identifier for the rule (e.g., "PFW-001")
- `pattern`: Regular expression or string pattern to match

**Examples:**
```json
{
  "id": "PFW-001",
  "pattern": "ignore previous instructions"
},
{
  "id": "PFW-002",
  "pattern": "reveal system prompt"
}
```

### Sanitize Patterns

Sanitize patterns define content that should be removed from prompts.

**Fields:**
- `id`: Unique identifier for the pattern (e.g., "PFW-SAN-001")
- `pattern`: Regular expression or string pattern to remove

**Examples:**
```json
{
  "id": "PFW-SAN-001",
  "pattern": "```"
},
{
  "id": "PFW-SAN-002",
  "pattern": "<script"
}
```

### Fuzzy Matching

Fuzzy matching allows for detection of similar but not identical patterns.

**Fields:**
- `enabled`: Boolean to enable/disable fuzzy matching
- `max_distance`: Maximum Levenshtein distance for fuzzy matches (0-5 recommended)

**Example:**
```json
"fuzzy_matching": {
  "enabled": true,
  "max_distance": 2
}
```

### Best Practices

1. **Start with strict rules**: Begin with conservative patterns
2. **Test thoroughly**: Validate rules don't block legitimate prompts
3. **Monitor false positives**: Adjust patterns based on real-world usage
4. **Update regularly**: Keep rules current with emerging threats

## EU Risk Keywords Configuration

EU AI Act compliance keywords are defined in `config/eu_risk_keywords.json`.

### Structure

```json
{
  "unacceptable": [
    "string"
  ],
  "high": [
    "string"
  ],
  "limited": [
    "string"
  ]
}
```

### Risk Categories

#### Unacceptable Risk

Patterns that indicate prohibited AI use cases under EU AI Act Article 5.

**Examples:**
```json
"unacceptable": [
  "social scoring",
  "biometric surveillance",
  "biometric categorization",
  "emotion recognition in workplace",
  "emotion recognition in school",
  "manipulative subliminal"
]
```

#### High Risk

Patterns that indicate high-risk AI systems requiring special compliance.

**Examples:**
```json
"high": [
  "employment",
  "hiring",
  "education",
  "credit",
  "insurance",
  "critical infrastructure",
  "law enforcement",
  "migration",
  "asylum",
  "border control",
  "justice",
  "judicial",
  "essential public service",
  "medical triage"
]
```

#### Limited Risk

Patterns that indicate limited-risk AI systems with transparency requirements.

**Examples:**
```json
"limited": [
  "chatbot",
  "recommendation",
  "generative assistant",
  "customer support bot",
  "deepfake"
]
```

### Best Practices

1. **Legal review**: Consult with legal experts for EU AI Act compliance
2. **Context matters**: Consider the context in which keywords appear
3. **Update regularly**: Keep keywords aligned with regulatory changes
4. **Document decisions**: Maintain records of compliance decisions

## Framework Configuration

The `FrameworkConfig` struct provides runtime configuration options.

### Structure

```rust
pub struct FrameworkConfig {
    pub server_port: u16,
    pub sled_db_path: String,
    pub mistral_api_key: Option<String>,
}
```

### Fields

#### server_port

- **Type**: `u16`
- **Default**: `3000`
- **Description**: TCP port for the HTTP server
- **Example**: `8080`

#### sled_db_path

- **Type**: `String`
- **Default**: `"prompt_sentinel_data"`
- **Description**: Filesystem path for Sled database storage
- **Example**: `"/var/lib/prompt_sentinel/data"`

#### mistral_api_key

- **Type**: `Option<String>`
- **Default**: `None` (reads from `MISTRAL_API_KEY` env var)
- **Description**: API key for Mistral AI services
- **Example**: `Some("sk-1234567890".to_string())`

### Usage Examples

```rust
// Default configuration
let config = FrameworkConfig::default();

// Custom configuration
let config = FrameworkConfig {
    server_port: 8080,
    sled_db_path: "/custom/path/to/db".to_string(),
    mistral_api_key: Some("your-api-key".to_string()),
};
```

## Environment Variables

### Supported Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MISTRAL_API_KEY` | Mistral AI API key | None |
| `RUST_LOG` | Logging level | `info` |
| `SERVER_PORT` | Server port override | `3000` |
| `SLED_DB_PATH` | Database path override | `prompt_sentinel_data` |

### Usage

```bash
# Set environment variables
export MISTRAL_API_KEY="your-api-key"
export RUST_LOG="debug"
export SERVER_PORT="8080"

# Run the server
cargo run --release
```

## Advanced Configuration

### Custom AppSettings

For advanced use cases, you can create custom `AppSettings`:

```rust
use prompt_sentinel::config::settings::AppSettings;

let settings = AppSettings {
    server_port: 8080,
    mistral_api_key: Some("your-api-key".to_string()),
    mistral_base_url: "https://custom.mistral.endpoint".to_string(),
    generation_model: "custom-model".to_string(),
    moderation_model: Some("custom-moderation-model".to_string()),
    embedding_model: "custom-embedding-model".to_string(),
    bias_threshold: 0.40,  // Higher threshold for stricter bias detection
    max_input_length: 8192, // Increased input length limit
};
```

### Configuration Precedence

1. **Explicit FrameworkConfig values** (highest priority)
2. **Environment variables**
3. **Default values** (lowest priority)

### Production Recommendations

1. **Database**: Use a dedicated volume for Sled database
2. **Logging**: Set `RUST_LOG=info` for production
3. **Security**: Store API keys in secret management systems
4. **Monitoring**: Monitor health endpoints regularly
5. **Backups**: Regularly backup the Sled database

## Troubleshooting Configuration

### Common Issues

**Issue**: Configuration file not found
- Ensure files are in the `config/` directory
- Check file permissions

**Issue**: Invalid JSON syntax
- Validate JSON using `jq` or online validators
- Check for trailing commas and proper quoting

**Issue**: Environment variables not loaded
- Verify variable names are correct
- Check for typos in variable names
- Restart the application after setting variables

**Issue**: Database path not writable
- Check directory permissions
- Ensure parent directories exist
- Use absolute paths for production deployments

## Configuration Validation

The framework validates configuration at startup:

1. **JSON syntax validation** for configuration files
2. **Pattern compilation** for firewall rules
3. **Model validation** for Mistral AI integration
4. **Database connectivity** for audit storage

Validation errors will prevent the server from starting and will be logged with detailed error messages.

## Updating Configuration

### Hot Reloading

Currently, configuration changes require a server restart. For production deployments:

1. Update configuration files
2. Gracefully shut down the server
3. Restart the server process
4. Verify new configuration is loaded

### Version Control

Store configuration files in version control with appropriate access controls. Consider:

- Using `.gitignore` for sensitive configuration
- Implementing configuration management tools
- Documenting configuration changes in commit messages

## Configuration Examples

### Development Configuration

```json
// firewall_rules.json - more permissive for development
{
  "block_rules": [
    {"id": "PFW-001", "pattern": "ignore previous instructions"}
  ],
  "sanitize_patterns": [],
  "fuzzy_matching": {
    "enabled": false,
    "max_distance": 1
  }
}
```

### Production Configuration

```json
// firewall_rules.json - strict for production
{
  "block_rules": [
    {"id": "PFW-001", "pattern": "ignore previous instructions"},
    {"id": "PFW-001B", "pattern": "ignore all previous instructions"},
    {"id": "PFW-002", "pattern": "reveal system prompt"},
    {"id": "PFW-003", "pattern": "developer instructions"},
    {"id": "PFW-004", "pattern": "bypass policy"},
    {"id": "PFW-005", "pattern": "jailbreak"}
  ],
  "sanitize_patterns": [
    {"id": "PFW-SAN-001", "pattern": "```"},
    {"id": "PFW-SAN-002", "pattern": "<script"},
    {"id": "PFW-SAN-003", "pattern": "</script>"}
  ],
  "fuzzy_matching": {
    "enabled": true,
    "max_distance": 2
  }
}
```

## Configuration Reference

### Firewall Rules Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `block_rules` | Array | Yes | Rules that block prompts |
| `block_rules[].id` | String | Yes | Unique rule identifier |
| `block_rules[].pattern` | String | Yes | Pattern to match |
| `sanitize_patterns` | Array | Yes | Patterns to remove |
| `sanitize_patterns[].id` | String | Yes | Unique pattern identifier |
| `sanitize_patterns[].pattern` | String | Yes | Pattern to sanitize |
| `fuzzy_matching` | Object | Yes | Fuzzy matching settings |
| `fuzzy_matching.enabled` | Boolean | Yes | Enable fuzzy matching |
| `fuzzy_matching.max_distance` | Integer | Yes | Max Levenshtein distance |

### EU Risk Keywords Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unacceptable` | Array | Yes | Unacceptable risk keywords |
| `high` | Array | Yes | High risk keywords |
| `limited` | Array | Yes | Limited risk keywords |

## Configuration Tools

### JSON Validation

```bash
# Validate JSON syntax
jq . config/firewall_rules.json
jq . config/eu_risk_keywords.json
```

### Configuration Testing

```bash
# Test configuration with dry run
cargo run -- --dry-run
```

### Configuration Backup

```bash
# Backup configuration
cp config/firewall_rules.json config/firewall_rules.json.backup
cp config/eu_risk_keywords.json config/eu_risk_keywords.json.backup
```

## Configuration Migration

When upgrading between versions, follow the migration guide in the release notes. Typically:

1. Backup existing configuration
2. Review new default configuration
3. Merge changes manually
4. Test thoroughly before production deployment

## Support

For configuration issues:

1. Check the troubleshooting section
2. Review error messages and logs
3. Consult the documentation
4. Open an issue with detailed reproduction steps