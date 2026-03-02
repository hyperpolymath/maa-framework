# RSR Compliance Badges

Display your RSR compliance status with badges in your README.

## Available Badges

### Bronze Level

**Achieved**:
```markdown
[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze%20Compliant-green?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze%20Compliant-green?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)

**In Progress**:
```markdown
[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze%20In%20Progress-yellow?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze%20In%20Progress-yellow?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)

**Not Compliant**:
```markdown
[![RSR Bronze](https://img.shields.io/badge/RSR-Not%20Compliant-red?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

[![RSR Bronze](https://img.shields.io/badge/RSR-Not%20Compliant-red?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)

### Silver Level (Future)

```markdown
[![RSR Silver](https://img.shields.io/badge/RSR-Silver%20Compliant-blue?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

### Gold Level (Future)

```markdown
[![RSR Gold](https://img.shields.io/badge/RSR-Gold%20Compliant-gold?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

### Platinum Level (Future)

```markdown
[![RSR Platinum](https://img.shields.io/badge/RSR-Platinum%20Compliant-platinum?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
```

## Dynamic Badges (Future Feature)

In the future, Aletheia may support dynamic badges that automatically update:

```markdown
[![RSR Compliance](https://rsr-badges.example.com/compliance/owner/repo)](https://gitlab.com/owner/repo)
```

This would:
1. Run Aletheia verification
2. Generate badge with current status
3. Update automatically on push

## Badge Styles

### Flat Square (Recommended)
```markdown
![RSR](https://img.shields.io/badge/RSR-Bronze-green?style=flat-square)
```

### Flat
```markdown
![RSR](https://img.shields.io/badge/RSR-Bronze-green?style=flat)
```

### Plastic
```markdown
![RSR](https://img.shields.io/badge/RSR-Bronze-green?style=plastic)
```

### For the Badge
```markdown
![RSR](https://img.shields.io/badge/RSR-Bronze-green?style=for-the-badge)
```

### Social
```markdown
![RSR](https://img.shields.io/badge/RSR-Bronze-green?style=social)
```

## Badge Verification

Users can verify your badge claim by:

1. **Clone repository**:
   ```bash
   git clone https://your-repo-url.git
   cd your-repo
   ```

2. **Run Aletheia**:
   ```bash
   aletheia .
   ```

3. **Check output**:
   ```
   🏆 Bronze-level RSR compliance: ACHIEVED
   ```

## README Example

```markdown
# My Amazing Project

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze%20Compliant-green?style=flat-square&logo=rust)](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT.txt)
[![Build Status](https://gitlab.com/your/repo/badges/main/pipeline.svg)](https://gitlab.com/your/repo/-/commits/main)

> RSR Bronze-level compliant project demonstrating best practices

## Features

- Feature 1
- Feature 2
- Feature 3

## RSR Compliance

This project maintains RSR Bronze-level compliance. Verify with:

\`\`\`bash
aletheia .
\`\`\`

## Installation

\`\`\`bash
cargo install my-project
\`\`\`

## License

Dual-licensed under MIT OR Apache-2.0
```

## Custom Badges

You can create custom badges with specific scores:

```markdown
<!-- 16/16 checks -->
![RSR](https://img.shields.io/badge/RSR-16%2F16%20Checks-green)

<!-- 87.5% compliance -->
![RSR](https://img.shields.io/badge/RSR-87.5%25%20Compliant-yellow)
```

## Badge in CI/CD

Generate badge URL based on CI results:

### GitLab CI

```yaml
# .gitlab-ci.yml
rsr-verify:
  script:
    - |
      if aletheia .; then
        echo "BADGE_STATUS=compliant" >> build.env
        echo "BADGE_COLOR=green" >> build.env
      else
        echo "BADGE_STATUS=not_compliant" >> build.env
        echo "BADGE_COLOR=red" >> build.env
      fi
  artifacts:
    reports:
      dotenv: build.env
```

## Badge Guidelines

### Do's ✅

- ✅ Update badge when compliance changes
- ✅ Link badge to Aletheia repo or RSR docs
- ✅ Keep badge current with latest verification
- ✅ Use appropriate color (green/yellow/red)

### Don'ts ❌

- ❌ Don't claim compliance without verification
- ❌ Don't use outdated badge status
- ❌ Don't modify badge colors misleadingly
- ❌ Don't use higher-level badges prematurely

## Verification Required

To use a compliance badge:

1. **Run Aletheia**: `aletheia .`
2. **Verify output**: Ensure 16/16 checks pass
3. **Add badge**: Use appropriate badge code
4. **Keep updated**: Re-verify after changes

## Badge Hosting

Badges use shields.io service:
- **Service**: https://shields.io
- **Format**: `https://img.shields.io/badge/{subject}-{status}-{color}`
- **Customization**: See https://shields.io for options

## Future: Automated Badge Service

Planned features:
- Real-time verification API
- Automatic badge generation
- Webhook integration
- Historical compliance tracking
- Multi-repo dashboards

## Contributing

Help improve RSR badges:
- Suggest new badge designs
- Propose dynamic badge service
- Contribute to Aletheia
- Share badge usage examples

---

**Displaying your RSR compliance shows commitment to quality!**
