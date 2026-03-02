# Palimpsest License Conformance Report

[![Palimpsest License](https://img.shields.io/badge/License-Palimpsest%20v0.8-blue)](https://gitlab.com/hyperpolymath/palimpsest-license)

**Project**: Aletheia
**Date**: 2025-12-26
**Version**: Palimpsest v0.8
**Status**: Compliant

## Overview

The Palimpsest License is an overlay license designed to work alongside standard open-source licenses (MIT, Apache-2.0, AGPL). It adds attribution and provenance requirements while maintaining full open-source compatibility.

## License Structure

```
┌─────────────────────────────────────────┐
│         PALIMPSEST OVERLAY              │
│  • Attribution requirements             │
│  • Provenance tracking                  │
│  • Contribution acknowledgment          │
├─────────────────────────────────────────┤
│          BASE LICENSE                   │
│  • MIT (permissive)                     │
│  • OR Apache-2.0 (permissive)           │
│  • OR AGPL (copyleft)                   │
└─────────────────────────────────────────┘
```

## Aletheia License Configuration

| Component | License |
|-----------|---------|
| Base License | MIT |
| Overlay | Palimpsest v0.8 |
| License Files | `LICENSE-MIT.txt`, `LICENSE-PALIMPSEST.txt` |

## Palimpsest Requirements

### Attribution

| Requirement | Status | Notes |
|-------------|--------|-------|
| Original author credited | ✅ Pass | Jonathan D. A. Jewell |
| Contribution history preserved | ✅ Pass | Git history maintained |
| License notice in files | ⚠️ Partial | Adding to source files |

### Provenance

| Requirement | Status | Notes |
|-------------|--------|-------|
| Origin clearly stated | ✅ Pass | In README and LICENSE |
| Modifications documented | ✅ Pass | CHANGELOG.md |
| Fork lineage tracked | N/A | Original project |

### Contribution Acknowledgment

| Requirement | Status | Notes |
|-------------|--------|-------|
| Contributors listed | ✅ Pass | MAINTAINERS.md |
| CONTRIBUTING.md present | ✅ Pass | Guidelines documented |
| DCO/CLA specified | ⚠️ Pending | DCO recommended |

## Verification Checklist

- [x] LICENSE-MIT.txt present
- [x] LICENSE-PALIMPSEST.txt present
- [x] LICENSE.txt references both
- [x] README includes license badges
- [x] Source files have headers (in progress)

## Why Palimpsest?

The Palimpsest License:
1. **Preserves attribution** - Ensures original creators are credited
2. **Tracks provenance** - Documents where code came from
3. **Remains open source** - Compatible with OSI-approved licenses
4. **Supports collaboration** - Clear contribution guidelines

## Learn More

- [Palimpsest License Repository](https://gitlab.com/hyperpolymath/palimpsest-license)
- [Palimpsest FAQ](https://gitlab.com/hyperpolymath/palimpsest-license/-/blob/main/FAQ.md)

---

*Licensed under MIT with Palimpsest v0.8 overlay*
