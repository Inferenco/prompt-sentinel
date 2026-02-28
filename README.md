# Prompt Sentinel Framework

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue)](https://www.rust-lang.org/)

A comprehensive framework for safe, compliant, and ethical AI interactions with Mistral AI models.

## Features

- **Prompt Firewall**: Protects against prompt injection attacks
- **Bias Detection**: Analyzes prompts for potential biases
- **EU AI Act Compliance**: Ensures compliance with EU regulations
- **Audit Logging**: Comprehensive audit trail for all operations
- **Mistral Integration**: Seamless integration with Mistral AI services

## Quick Start

```bash
# Clone the repository
git clone https://github.com/Inferenco/prompt_sentinel.git
cd prompt_sentinel

# Build the project
cargo build --release

# Set environment variables
export MISTRAL_API_KEY="your-api-key"
export RUST_LOG="info"

# Run the server
cargo run --release
```

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Mistral API key (for full functionality)

### Build from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Inferenco/prompt_sentinel.git
cd prompt_sentinel
cargo build --release
```

### Docker Installation

```bash
# Build Docker image
docker build -t prompt-sentinel .

# Run container
docker run -d \
  -p 3000:3000 \
  -e MISTRAL_API_KEY="your-api-key" \
  -e RUST_LOG="info" \
  --name prompt-sentinel \
  prompt-sentinel
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MISTRAL_API_KEY` | Mistral AI API key | None (required) |
| `RUST_LOG` | Logging level | `info` |
| `SERVER_PORT` | Server port | `3000` |
| `SLED_DB_PATH` | Database path | `prompt_sentinel_data` |

### Configuration Files

Edit configuration files in the `config/` directory:

- `firewall_rules.json`: Prompt firewall rules
- `eu_risk_keywords.json`: EU AI Act compliance keywords

See [CONFIGURATION_GUIDE.md](CONFIGURATION_GUIDE.md) for detailed configuration options.

## Usage

### Basic Usage

```rust
use prompt_sentinel::FrameworkConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Use default configuration
    let config = FrameworkConfig::default();

    // Initialize the framework
    let server = config.initialize().await?;

    // Start the server
    server.start().await?;

    Ok(())
}
```

### Custom Configuration

```rust
use prompt_sentinel::FrameworkConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = FrameworkConfig {
        server_port: 8080,
        sled_db_path: "/custom/path/to/db".to_string(),
        mistral_api_key: Some("your-api-key".to_string()),
    };

    let server = config.initialize().await?;
    server.start().await?;

    Ok(())
}
```

## API Endpoints

### POST /api/compliance/check

Check a prompt for compliance with all framework rules.

**Request:**
```json
{
  "correlation_id": "optional-uuid",
  "prompt": "Your prompt text here"
}
```

**Response:**
```json
{
  "correlation_id": "generated-or-provided-uuid",
  "status": "Completed|BlockedByFirewall|BlockedByInputModeration|BlockedByOutputModeration",
  "firewall": {
    "action": "Allow|Block",
    "reasons": ["reason1", "reason2"],
    "sanitized_prompt": "cleaned prompt text"
  },
  "bias": {
    "score": 0.25,
    "level": "Low|Medium|High",
    "categories": ["gender", "race"]
  },
  "input_moderation": {
    "flagged": false,
    "categories": []
  },
  "output_moderation": {
    "flagged": false,
    "categories": []
  },
  "generated_text": "AI response text",
  "audit_proof": {
    "audit_id": "uuid",
    "timestamp": "iso8601",
    "signature": "base64"
  }
}
```

### GET /health

Health check endpoint.

**Response:** `OK`

### GET /api/mistral/health

Check Mistral API integration health.

**Response:**
```json
{
  "status": "healthy|unhealthy",
  "message": "status message",
  "models": ["model1", "model2", "model3"]
}
```

## API Client Examples

### Python Example

```python
import requests
import json

# Check compliance endpoint
url = "http://localhost:3000/api/compliance/check"

payload = {
    "prompt": "Tell me about the best programming language"
}

headers = {
    "Content-Type": "application/json"
}

response = requests.post(url, data=json.dumps(payload), headers=headers)

if response.status_code == 200:
    result = response.json()
    print(f"Status: {result['status']}")
    print(f"Bias Score: {result['bias']['score']}")
    if result['status'] == 'Completed':
        print(f"Generated Text: {result['generated_text']}")
else:
    print(f"Error: {response.text}")
```

### JavaScript Example

```javascript
const axios = require('axios');

async function checkCompliance() {
    try {
        const response = await axios.post('http://localhost:3000/api/compliance/check', {
            prompt: "What are the benefits of Rust programming?"
        });
        
        console.log('Status:', response.data.status);
        console.log('Bias Score:', response.data.bias.score);
        if (response.data.status === 'Completed') {
            console.log('Generated Text:', response.data.generated_text);
        }
    } catch (error) {
        console.error('Error:', error.response?.data || error.message);
    }
}

checkCompliance();
```

### cURL Example

```bash
curl -X POST http://localhost:3000/api/compliance/check \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Explain quantum computing"}' \
  | jq .
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Tests

```bash
# Compliance flow tests
cargo test --test compliance_flow

# Security regression tests
cargo test --test security_regressions

# EU compliance tests
cargo test --test eu_compliance_rules

# Firewall benchmark
cargo bench
```

## Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                     Prompt Sentinel Framework                  │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐    │
│  │ Prompt      │    │ Bias        │    │ EU Law          │    │
│  │ Firewall    │    │ Detection   │    │ Compliance      │    │
│  └─────────────┘    └─────────────┘    └─────────────────┘    │
│        │               │                     │                 │
│        ▼               ▼                     ▼                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 Compliance Engine                   │    │
│  └─────────────────────────────────────────────────────┘    │
│                        │                                  │
│                        ▼                                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 Mistral Service                    │    │
│  └─────────────────────────────────────────────────────┘    │
│                        │                                  │
│                        ▼                                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 Audit Logger                       │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

## Modules

### Prompt Firewall

- Detects and blocks prompt injection attempts
- Sanitizes potentially harmful content
- Configurable rules with fuzzy matching

### Bias Detection

- Analyzes prompts for potential biases
- Scoring system with configurable thresholds
- Categorization of bias types

### EU Law Compliance

- Ensures compliance with EU AI Act
- Risk classification system
- Audit trail for compliance decisions

### Mistral Service

- Integration with Mistral AI APIs
- Text generation and moderation
- Model validation and health checks

### Audit Logger

- Immutable audit trail
- Cryptographic proof generation
- Sled database storage

## Deployment

### Production Deployment

```bash
# Build for production
cargo build --release

# Create systemd service
sudo nano /etc/systemd/system/prompt-sentinel.service
```

```ini
[Unit]
Description=Prompt Sentinel Framework
After=network.target

[Service]
User=prompt-sentinel
WorkingDirectory=/opt/prompt-sentinel
Environment="MISTRAL_API_KEY=your-api-key"
Environment="RUST_LOG=info"
ExecStart=/opt/prompt-sentinel/target/release/prompt_sentinel
Restart=always

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable prompt-sentinel
sudo systemctl start prompt-sentinel
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: prompt-sentinel
spec:
  replicas: 3
  selector:
    matchLabels:
      app: prompt-sentinel
  template:
    metadata:
      labels:
        app: prompt-sentinel
    spec:
      containers:
      - name: prompt-sentinel
        image: prompt-sentinel:latest
        ports:
        - containerPort: 3000
        env:
        - name: MISTRAL_API_KEY
          valueFrom:
            secretKeyRef:
              name: mistral-api-key
              key: api-key
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            cpu: "500m"
            memory: "512Mi"
          limits:
            cpu: "1"
            memory: "1Gi"
        volumeMounts:
        - name: sled-data
          mountPath: /data
      volumes:
      - name: sled-data
        persistentVolumeClaim:
          claimName: sled-pvc
```

## Monitoring

### Health Endpoints

- `GET /health`: Basic health check
- `GET /api/mistral/health`: Mistral API health check

### Logging

Configure logging level with `RUST_LOG` environment variable:

```bash
# Debug logging
export RUST_LOG="debug"

# Info logging (default)
export RUST_LOG="info"

# Trace logging (verbose)
export RUST_LOG="trace"
```

### Metrics

Integrate with Prometheus for monitoring:

```rust
// Add to your main.rs
use prometheus::Encoder;

async fn metrics() -> String {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

## Security

### Best Practices

1. **API Keys**: Store Mistral API keys securely
2. **Network**: Use HTTPS in production
3. **Updates**: Keep dependencies updated
4. **Firewall**: Regularly update firewall rules
5. **Backups**: Backup the Sled database regularly

### Security Features

- Input validation and sanitization
- Rate limiting (recommended to add)
- Audit logging for all operations
- Secure configuration management

## Performance

### Optimization Tips

1. **Caching**: Implement response caching
2. **Batching**: Batch requests where possible
3. **Connection Pooling**: Use connection pooling for Mistral API
4. **Async**: Leverage async I/O throughout

### Benchmarking

```bash
# Run firewall benchmark
cargo bench

# Profile with flamegraph
cargo flamegraph --bench firewall_benchmark
```

## Troubleshooting

### Common Issues

**Server fails to start:**
- Check Mistral API key is valid
- Verify database path is writable
- Review error logs for details

**High latency:**
- Check network connectivity to Mistral API
- Review bias detection threshold
- Monitor system resources

**False positives:**
- Adjust firewall rules
- Review fuzzy matching settings
- Update configuration files

### Debugging

```bash
# Enable debug logging
export RUST_LOG="debug"

# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Check logs
journalctl -u prompt-sentinel -f
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Submit a pull request
5. Follow the existing code style

### Development Setup

```bash
# Clone repository
git clone https://github.com/Inferenco/prompt_sentinel.git

# Install pre-commit hooks
cargo install cargo-husky
cargo husky install

# Run tests
cargo test
```

## Roadmap

- Enhanced bias detection algorithms
- Additional compliance frameworks
- Performance optimizations
- Extended API capabilities
- Improved documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For issues, questions, or feature requests:

1. Check the [documentation](DOCUMENTATION.md)
2. Review the [configuration guide](CONFIGURATION_GUIDE.md)
3. Open an issue on GitHub
4. Join our community discussions

## Acknowledgements

- Mistral AI for their powerful language models
- The Rust community for excellent tools and libraries
- All contributors who help improve this framework

## Contact

For more information, visit our [website](https://inferenco.com) or contact us at [info@inferenco.com](mailto:info@inferenco.com).