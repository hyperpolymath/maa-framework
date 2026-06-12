<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please report it responsibly:

1. **Do NOT** create a public GitHub issue for security vulnerabilities
2. Send a private report to the repository maintainers via GitHub's private vulnerability reporting feature
3. Include as much detail as possible:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Security Measures

This repository implements the following security practices:

### GitHub Actions Security

- **SHA-pinned actions**: All GitHub Actions are pinned to specific commit SHAs to prevent supply chain attacks
- **Minimal permissions**: Workflows use the least privilege principle with `contents: read` permission
- **SSH host key verification**: Known hosts are verified to prevent MITM attacks during mirroring
- **Concurrency controls**: Prevents race conditions during parallel workflow runs
- **Timeout limits**: All jobs have timeout limits to prevent resource exhaustion

### Secret Management

- SSH keys for mirror targets are stored as GitHub encrypted secrets
- Mirror operations are conditionally enabled via repository variables
- No secrets are logged or exposed in workflow outputs

### Force Push Warning

This repository uses `--force` push for mirroring operations. This is intentional for maintaining exact mirrors but means:
- History on mirror targets will be overwritten
- Tags on mirror targets will be force-updated
- Only trusted maintainers should have push access to the main branch

## Dependency Updates

This project monitors for security updates in:
- GitHub Actions (checkout, ssh-agent)
- SSH host keys for mirror targets

## Contact

For security concerns, use GitHub's private security advisory feature or contact the repository maintainers directly.
