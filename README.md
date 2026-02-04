# safe-skill-search

Fast local search across agent skill registries with **quality filtering** - only returns skills with score >= 80 by default.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Based on analysis of **4,784 skills from 5 registries**: [Analyzing 4,784 AI Agent Skills →](https://skyfallsin.github.io/2026/02/03/ai-agent-skills-database.html)

## Features

- **Quality filtering**: Filters out low-quality skills (score < 80) by default
- **`--min-score` flag**: Override the minimum quality threshold (use 0 to show all)
- **Fast local search**: Tantivy (BM25) full-text search engine
- **Git-based sync**: Clones repos locally for instant access
- **Multiple registries**: Searches clawdhub, anthropic, openai, and jo skills
- **Trust indicators**: `[✓]` for trusted, `[⚠]` for untrusted
- **Quality scores**: Shows `[Q:score]` for each skill

## Requirements

- **git**: Required for cloning skill registries

## Installation

### Pre-built Binaries

Download from [releases](https://github.com/jo-inc/safe-skill-search/releases):

| Platform | Binary |
|----------|--------|
| macOS (Apple Silicon) | `safe-skill-search-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `safe-skill-search-x86_64-apple-darwin.tar.gz` |
| Linux (x86_64) | `safe-skill-search-x86_64-unknown-linux-gnu.tar.gz` |
| Linux (ARM64) | `safe-skill-search-aarch64-unknown-linux-gnu.tar.gz` |
| Windows (x86_64) | `safe-skill-search-x86_64-pc-windows-msvc.zip` |

### From Source

```bash
git clone https://github.com/jo-inc/safe-skill-search
cd safe-skill-search
cargo install --path .
```

## Usage

```bash
# Search for skills (only shows quality score >= 80)
safe-skill-search search "browser automation"

# Show all skills regardless of quality score
safe-skill-search search "browser automation" --min-score 0

# Show only premium quality skills (score >= 90)
safe-skill-search search "browser automation" --min-score 90

# Search with JSON output
safe-skill-search search "pdf" --json

# Filter by registry
safe-skill-search search "pdf" --registry anthropic

# Only trusted skills (anthropic + openai official)
safe-skill-search search "document" --trusted

# Show top skills by stars (quality filtered)
safe-skill-search top

# Show skill details including quality score
safe-skill-search show trello

# Get install URL for a skill
safe-skill-search url trello

# Force resync from GitHub
safe-skill-search sync --force
```

## Quality Scores

Quality scores are based on the [skills-db analysis](https://skyfallsin.github.io/2026/02/03/ai-agent-skills-database.html) which evaluated **4,784 skills from 5 registries**.

Scoring criteria:
- Documentation completeness
- Code examples
- Workflow structure
- Error handling
- Best practices

Scores range from 0-100, with 80+ being considered high-quality.

## Registries

| Registry | Source | Skills | Trust |
|----------|--------|--------|-------|
| clawdhub | github.com/openclaw/skills | ~3400 | ⚠ Community |
| anthropic | github.com/anthropics/skills | ~16 | ✓ Official |
| openai | github.com/openai/skills/.curated | ~31 | ✓ Official |
| openai-experimental | github.com/openai/skills/.experimental | varies | ⚠ Experimental |
| jo | github.com/jo-inc/skills | varies | ✓ Official |

## Data Storage

All data stored in `~/.local/share/skill-search/`:
- `skills.db` - SQLite database with skill metadata  
- `index/` - Tantivy full-text search index
- `repos/` - Cloned git repositories (~100MB total)

## Building

```bash
cargo build --release
# Binary at target/release/safe-skill-search
```

## License

MIT
