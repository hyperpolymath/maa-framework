# Deployment Guide

This document describes various ways to deploy and use Aletheia in different environments.

## Table of Contents

1. [Local Installation](#local-installation)
2. [Docker Deployment](#docker-deployment)
3. [CI/CD Integration](#cicd-integration)
4. [System-Wide Installation](#system-wide-installation)
5. [Cloud Deployment](#cloud-deployment)
6. [Air-Gapped Environments](#air-gapped-environments)

## Local Installation

### Quick Install (Recommended)

```bash
# Using the install script
curl -sSf https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/raw/main/scripts/install.sh | bash

# Or download and inspect first
curl -O https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/raw/main/scripts/install.sh
chmod +x install.sh
./install.sh
```

### Manual Build

```bash
# Clone repository
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia

# Build release binary
cargo build --release

# Binary is at: target/release/aletheia
# Copy to PATH location:
cp target/release/aletheia ~/.local/bin/
```

### Using Cargo Install

```bash
# Install directly from Git
cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git

# Or from crates.io (when published)
cargo install aletheia
```

## Docker Deployment

### Build Docker Image

```bash
# Build the image
docker build -t aletheia:0.1.0 .

# Or build with specific target
docker build -t aletheia:latest --target=runtime .
```

### Run with Docker

```bash
# Verify current directory
docker run -v $(pwd):/repo aletheia:0.1.0

# Verify specific repository
docker run -v /path/to/repo:/repo aletheia:0.1.0

# Save results to file
docker run -v $(pwd):/repo aletheia:0.1.0 > report.txt
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  aletheia:
    image: aletheia:0.1.0
    volumes:
      - ./:/repo:ro
    command: ["/repo"]
```

```bash
# Run with docker-compose
docker-compose run aletheia
```

### Multi-Repository Verification

```bash
# Script to verify multiple repositories
#!/bin/bash
for repo in /path/to/repos/*; do
  echo "Verifying: $repo"
  docker run -v "$repo:/repo" aletheia:0.1.0
  echo "---"
done
```

## CI/CD Integration

### GitLab CI

#### Simple Integration

```yaml
# .gitlab-ci.yml
rsr-compliance:
  stage: test
  image: rust:1.75
  before_script:
    - cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
  script:
    - aletheia
  allow_failure: false
```

#### With Caching

```yaml
# .gitlab-ci.yml
variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

rsr-compliance:
  stage: test
  image: rust:1.75
  cache:
    key: aletheia
    paths:
      - .cargo/
  before_script:
    - |
      if [ ! -f .cargo/bin/aletheia ]; then
        cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
      fi
  script:
    - .cargo/bin/aletheia
```

#### Using Docker

```yaml
# .gitlab-ci.yml
rsr-compliance:
  stage: test
  image: aletheia:0.1.0
  script:
    - /aletheia .
```

### GitHub Actions

```yaml
# .github/workflows/rsr-compliance.yml
name: RSR Compliance

on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Cache Aletheia
        uses: actions/cache@v3
        with:
          path: ~/.cargo/bin/aletheia
          key: aletheia-${{ hashFiles('**/Cargo.lock') }}

      - name: Install Aletheia
        run: cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git

      - name: Verify RSR Compliance
        run: aletheia
```

### Jenkins

```groovy
// Jenkinsfile
pipeline {
    agent {
        docker {
            image 'rust:1.75'
        }
    }

    stages {
        stage('Install Aletheia') {
            steps {
                sh 'cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git'
            }
        }

        stage('Verify RSR Compliance') {
            steps {
                sh 'aletheia'
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'rsr-report.txt', allowEmptyArchive: true
        }
    }
}
```

### CircleCI

```yaml
# .circleci/config.yml
version: 2.1

jobs:
  rsr-compliance:
    docker:
      - image: rust:1.75
    steps:
      - checkout
      - restore_cache:
          keys:
            - aletheia-{{ .Branch }}
      - run:
          name: Install Aletheia
          command: cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
      - save_cache:
          key: aletheia-{{ .Branch }}
          paths:
            - ~/.cargo/bin/aletheia
      - run:
          name: Verify RSR Compliance
          command: aletheia

workflows:
  version: 2
  build:
    jobs:
      - rsr-compliance
```

## System-Wide Installation

### Linux (systemd)

```bash
# Install binary
sudo cp target/release/aletheia /usr/local/bin/

# Create systemd service for scheduled checks
cat <<EOF | sudo tee /etc/systemd/system/aletheia-check.service
[Unit]
Description=Aletheia RSR Compliance Check
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/aletheia /path/to/repositories
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Create timer
cat <<EOF | sudo tee /etc/systemd/system/aletheia-check.timer
[Unit]
Description=Run Aletheia RSR compliance check daily

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
EOF

# Enable and start timer
sudo systemctl enable aletheia-check.timer
sudo systemctl start aletheia-check.timer
```

### macOS

```bash
# Install binary
cp target/release/aletheia /usr/local/bin/

# Create LaunchAgent for scheduled checks
cat <<EOF > ~/Library/LaunchAgents/org.maa-framework.aletheia.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.maa-framework.aletheia</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/aletheia</string>
        <string>/path/to/repositories</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Hour</key>
        <integer>9</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
</dict>
</plist>
EOF

# Load LaunchAgent
launchctl load ~/Library/LaunchAgents/org.maa-framework.aletheia.plist
```

## Cloud Deployment

### AWS Lambda

```python
# lambda_function.py
import subprocess
import json

def lambda_handler(event, context):
    """
    AWS Lambda function to run Aletheia verification
    """
    repo_path = event.get('repo_path', '/tmp/repo')

    # Run aletheia
    result = subprocess.run(
        ['/opt/aletheia', repo_path],
        capture_output=True,
        text=True
    )

    return {
        'statusCode': 200 if result.returncode == 0 else 400,
        'body': json.dumps({
            'compliant': result.returncode == 0,
            'output': result.stdout,
            'error': result.stderr if result.returncode != 0 else None
        })
    }
```

### Google Cloud Functions

```python
# main.py
import subprocess
from flask import jsonify

def verify_rsr(request):
    """
    Google Cloud Function to run Aletheia verification
    """
    request_json = request.get_json()
    repo_path = request_json.get('repo_path', '/tmp/repo')

    result = subprocess.run(
        ['/usr/local/bin/aletheia', repo_path],
        capture_output=True,
        text=True
    )

    return jsonify({
        'compliant': result.returncode == 0,
        'output': result.stdout,
        'error': result.stderr if result.returncode != 0 else None
    })
```

### Kubernetes

```yaml
# kubernetes/deployment.yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: aletheia-rsr-check
spec:
  schedule: "0 9 * * *"  # Daily at 9 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: aletheia
            image: aletheia:0.1.0
            volumeMounts:
            - name: repo
              mountPath: /repo
              readOnly: true
          volumes:
          - name: repo
            hostPath:
              path: /path/to/repositories
          restartPolicy: OnFailure
```

## Air-Gapped Environments

### Offline Installation

```bash
# On a machine with internet access:

# 1. Clone repository
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia

# 2. Download Rust toolchain
rustup toolchain install 1.75 --offline

# 3. Build binary
cargo build --release --target x86_64-unknown-linux-musl

# 4. Create tarball
tar -czf aletheia-offline.tar.gz \
    target/x86_64-unknown-linux-musl/release/aletheia \
    README.md \
    LICENSE.txt \
    docs/

# Transfer aletheia-offline.tar.gz to air-gapped environment

# On air-gapped machine:
tar -xzf aletheia-offline.tar.gz
sudo cp aletheia /usr/local/bin/
```

### Vendored Dependencies (Zero deps = already done!)

```bash
# Aletheia has ZERO dependencies, so it works perfectly in air-gapped environments
# No cargo vendor needed!

# Just copy the binary:
scp target/release/aletheia airgapped-machine:/usr/local/bin/
```

## Best Practices

### Security

1. **Verify checksums**:
   ```bash
   sha256sum aletheia > aletheia.sha256
   sha256sum -c aletheia.sha256
   ```

2. **Use specific versions**:
   ```bash
   # Don't use :latest in production
   docker pull aletheia:0.1.0
   ```

3. **Run as non-root**:
   ```dockerfile
   USER nobody:nobody
   ```

### Performance

1. **Cache installations**:
   - Use Docker layer caching
   - Cache cargo installations in CI/CD

2. **Parallel verification**:
   ```bash
   find /repos -type d -maxdepth 1 | \
     parallel -j4 'aletheia {}'
   ```

### Monitoring

1. **Log results**:
   ```bash
   aletheia | tee -a /var/log/aletheia.log
   ```

2. **Alert on failures**:
   ```bash
   aletheia || notify-send "RSR compliance failed"
   ```

## Troubleshooting

### Issue: Binary not found

```bash
# Check installation
which aletheia

# Add to PATH
export PATH="$PATH:$HOME/.cargo/bin"
```

### Issue: Permission denied

```bash
# Make binary executable
chmod +x /path/to/aletheia
```

### Issue: Old version

```bash
# Force reinstall
cargo install --force --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
```

## Support

- **Issues**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues
- **Documentation**: https://gitlab.com/maa-framework/6-the-foundation/aletheia
- **Email**: maintainers@maa-framework.org

---

*Deploy Aletheia everywhere - verify truth everywhere!*
