---
name: skill-search
description: Search and discover skills across clawdhub, anthropic, and openai registries. Find skills by keyword, view details, and get install URLs.
version: 0.1.0
---

# skill-search

Search and discover agent skills across multiple registries with full-text search.

## Supported Registries

| Registry | Source | Trust Level |
|----------|--------|-------------|
| clawdhub | github.com/openclaw/skills | ⚠ Community (verify before use) |
| anthropic | github.com/anthropics/skills | ✓ Trusted (official) |
| openai | github.com/openai/skills/.curated | ✓ Trusted (official) |
| openai-experimental | github.com/openai/skills/.experimental | ⚠ Experimental |

## Requirements

- **git**: Required for cloning skill registries

## Installation

### Pre-built Binary

Download from [releases](https://github.com/jo-inc/skill-search/releases).

### From Source (Rust)

```bash
git clone https://github.com/jo-inc/skill-search
cd skill-search
cargo install --path .
```

## Usage

### Search for skills

```bash
# Basic search
skill-search search "calendar integration"

# Search with JSON output (for programmatic use)
skill-search search "browser automation" --json

# Filter by registry
skill-search search "pdf" --registry anthropic

# Only trusted skills (anthropic + openai)
skill-search search "document" --trusted
```

### View skill details

```bash
# Show full SKILL.md content
skill-search show trello
```

### Get install URL

```bash
# Get GitHub URL for installation
skill-search url trello
# Output: https://github.com/openclaw/skills/tree/main/skills/steipete/trello
```

### Top skills by stars

```bash
# Show top 20 skills by star count
skill-search top

# Only trusted skills
skill-search top --trusted
```

### Sync registries

```bash
# Update skill index (runs automatically on first use)
skill-search sync

# Force full resync
skill-search sync --force
```

## Output Format

### Human-readable (default)

```
1. calendar ★15 (clawdhub) - Calendar management and scheduling...
   https://github.com/openclaw/skills/tree/main/skills/0xterrybit/calendar

2. apple-calendar (clawdhub) - Apple Calendar.app integration for macOS...
   https://github.com/openclaw/skills/tree/main/skills/tyler6204/apple-calendar
```

### JSON (--json flag)

```json
[
  {
    "slug": "calendar",
    "name": "calendar",
    "registry": "clawdhub",
    "description": "Calendar management and scheduling...",
    "github_url": "https://github.com/openclaw/skills/tree/main/...",
    "stars": 15,
    "trusted": false,
    "score": 20.5
  }
]
```

## Integration Examples

### Install skill after search

```bash
# Find a skill
skill-search search "trello" --limit 1 --json

# Get the URL and install (using your agent's install command)
URL=$(skill-search url trello)
# Then use $URL with your agent's skill installer
```

### Claude Code

```bash
/skill-installer $URL
```

### Codex

```bash
$skill-installer $URL
```

## Data Storage

All data stored in `~/.local/share/skill-search/`:
- `skills.db` - SQLite database with skill metadata
- `index/` - Tantivy full-text search index
- `repos/` - Cloned git repositories (~100MB total)

## Performance

- **First sync**: ~10 seconds (clones 3 repos, indexes 3400+ skills)
- **Search**: <10ms (local Tantivy index)
- **Update sync**: ~5 seconds (git pull + reindex)
