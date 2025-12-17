# MAA Framework - Roadmap

## Current Status

The MAA Framework repository is configured as a **hub-and-spoke mirror system** that automatically synchronizes code across multiple Git hosting platforms:

- **GitHub** (hub/primary)
- **GitLab** (mirror)
- **Codeberg** (mirror)
- **Bitbucket** (mirror)

---

## Phase 1: Infrastructure & Security (Completed)

- [x] Hub-and-spoke mirror workflow
- [x] SHA-pinned GitHub Actions for supply chain security
- [x] SSH host key verification (MITM protection)
- [x] Concurrency controls for race condition prevention
- [x] Job timeouts to prevent resource exhaustion
- [x] SECURITY.md policy document
- [x] CODEOWNERS for review requirements
- [x] Dynamic repository owner (not hardcoded)

---

## Phase 2: Enhanced CI/CD Pipeline

### Workflow Improvements
- [ ] Add status badges to README
- [ ] Implement failure notifications (Slack/Discord/Email)
- [ ] Add workflow status checks before mirroring
- [ ] Create PR preview environments (if applicable)

### Testing & Quality
- [ ] Add workflow linting (actionlint)
- [ ] Implement YAML schema validation
- [ ] Add Dependabot for action version updates
- [ ] Set up CodeQL for security scanning

### Documentation
- [ ] Add README.md with setup instructions
- [ ] Document mirror configuration process
- [ ] Create CONTRIBUTING.md guidelines
- [ ] Add architecture diagrams

---

## Phase 3: Advanced Mirroring Features

### Selective Mirroring
- [ ] Branch filtering (exclude feature branches from mirrors)
- [ ] Tag filtering (only release tags)
- [ ] Path-based exclusions for platform-specific files

### Mirror Health
- [ ] Add mirror verification step (compare HEADs)
- [ ] Implement automatic retry with exponential backoff
- [ ] Create mirror drift detection alerts
- [ ] Add periodic full-sync workflow (scheduled)

### Platform-Specific Features
- [ ] GitLab CI/CD configuration mirroring
- [ ] Codeberg-specific integrations
- [ ] Bitbucket Pipelines compatibility

---

## Phase 4: Governance & Compliance

### Access Control
- [ ] Branch protection rules
- [ ] Required reviews for workflow changes
- [ ] Signed commits enforcement
- [ ] Deploy key rotation schedule

### Audit & Compliance
- [ ] Mirror operation audit logs
- [ ] Secret rotation reminders
- [ ] Compliance documentation (GDPR, etc.)
- [ ] License compliance checking

### Disaster Recovery
- [ ] Backup verification procedures
- [ ] Mirror promotion playbook (if GitHub unavailable)
- [ ] Recovery time objectives (RTO) documentation

---

## Phase 5: Framework Development

*Pending project direction - the following are suggested based on "MAA Framework" naming:*

### Core Framework
- [ ] Define framework purpose and scope
- [ ] Establish coding standards
- [ ] Create project structure
- [ ] Set up dependency management

### Developer Experience
- [ ] Local development setup
- [ ] Testing framework
- [ ] Code formatting/linting
- [ ] Pre-commit hooks

---

## Security Checklist

| Item | Status | Notes |
|------|--------|-------|
| SHA-pinned actions | Done | v4.2.2 checkout, v0.9.1 ssh-agent |
| SSH host verification | Done | ed25519 keys for all platforms |
| Minimal permissions | Done | contents: read only |
| Secret management | Done | GitHub encrypted secrets |
| Concurrency control | Done | Prevents parallel runs |
| Job timeouts | Done | 10 minutes per job |
| CODEOWNERS | Done | Maintainer review required |
| Security policy | Done | SECURITY.md |
| Dependabot | Pending | Phase 2 |
| CodeQL scanning | Pending | Phase 2 |
| Signed commits | Pending | Phase 4 |

---

## Version History

| Date | Version | Changes |
|------|---------|---------|
| 2025-12-17 | 0.1.0 | Initial mirror workflow |
| 2025-12-17 | 0.2.0 | Security hardening, SECURITY.md, CODEOWNERS |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) (to be created) for guidelines on contributing to this project.

## License

AGPL-3.0-or-later
