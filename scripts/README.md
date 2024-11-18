# GitHub Issue Generation Scripts

This directory contains scripts for generating and managing GitHub issues from markdown templates. The primary tool is `generate_issues.sh`, which creates a set of linked issues from markdown templates.

## Directory Structure

```
scripts/
├── generate_issues.sh         # Main issue generation script
├── cleanup.sh                # Script to clean up generated issues
├── templates/                # Template root directory
│   └── z80_implementation/   # Example template set for Z80 CPU
│       ├── 00_*.md          # Tracking issue template
│       ├── [01-99]_*.md     # Implementation issue templates
│       └── dependencies.txt  # Issue dependency definitions
└── .last_generated_issues   # Tracks last generated issue numbers
```

## Usage

```bash
# Generate issues
./generate_issues.sh <template_directory>

# Clean up the last generated set of issues
./cleanup.sh
```

Example:
```bash
# Generate Z80 implementation issues
./generate_issues.sh templates/z80_implementation

# Remove the generated issues
./cleanup.sh
```

## Template Format

### Template Directory Structure

Each template set should be in its own directory under `templates/` and contain:
1. One tracking issue template (prefixed with `00_`)
2. Multiple implementation issue templates (prefixed with `[01-99]_`)
3. Optional `dependencies.txt` file

### Issue Template Format

Each template must be a markdown file with YAML frontmatter:

```markdown
---
name: Issue Name
about: Brief description
title: "[TYPE] Issue Title"
labels: label1, label2
assignees: ''
---

# Title

## Overview
Brief description of the task.

[... rest of template ...]
```

Required sections:
- YAML frontmatter with `name`, `about`, `title`, and `labels`
- Markdown content with at least an `Overview` section

### Placeholder System

The script supports dynamic issue linking using placeholders in the format `{{IMPL_X}}` where X is the implementation number (1-based index). These placeholders are automatically replaced with actual issue numbers during generation.

Example usage in tracking issue:
```markdown
## Dependencies
\`\`\`mermaid
graph TD
    A[{{IMPL_1}} 8-bit Loads] --> B[{{IMPL_3}} 8-bit Arithmetic]
    A --> C[{{IMPL_2}} 16-bit Loads]
\`\`\`

### Implementation Tasks
- [ ] {{IMPL_1}}
- [ ] {{IMPL_2}}
- [ ] {{IMPL_3}}
```

### Dependencies File Format

The optional `dependencies.txt` defines dependencies between issues. Format:

```
template_number:dependency1,dependency2,dependency3
```

Example:
```
01:#1,#3,#4
02:#1,#3,#4,#{{IMPL_1}}
03:#1,#3,#4,#{{IMPL_1}}
```

Where:
- `template_number` matches the prefix of the template file (e.g., "01" for "01_something.md")
- Dependencies can be:
  - Fixed issue numbers (e.g., `#1`)
  - Implementation placeholders (e.g., `#{{IMPL_1}}`)

## Creating New Template Sets

1. Create a new directory under `templates/`
2. Create a tracking issue template:
   - Name it `00_something.md`
   - Include required frontmatter
   - Use `{{IMPL_X}}` placeholders for implementation issues
   - Add overview and implementation strategy

3. Create implementation templates:
   - Name them `XX_something.md` where XX is 01-99
   - Include clear requirements and acceptance criteria
   - Add specific implementation details
   - List edge cases and testing requirements

4. Optional: Create `dependencies.txt`:
   - List dependencies for each template
   - Use actual issue numbers for foundation tasks
   - Use `{{IMPL_X}}` placeholders for implementation tasks

### Template Guidelines

1. Tracking Issue Template:
   - Use `[TRACKING]` prefix in title
   - Use placeholders for implementation references
   - Let GitHub expand issue titles where possible
   - Include descriptive text only in mermaid graphs

2. Implementation Templates:
   - Use `[IMPL]` prefix in title
   - Include clear overview
   - List specific requirements
   - Add acceptance criteria
   - Document edge cases
   - Include testing requirements

3. Common Sections:
   ```markdown
   # Title
   
   ## Overview
   Brief description.
   
   ## Implementation Requirements
   - Requirement 1
   - Requirement 2
   
   ## Testing Requirements
   - Test case 1
   - Test case 2
   
   ## Edge Cases
   - Edge case 1
   - Edge case 2
   
   ## Acceptance Criteria
   - [ ] Criterion 1
   - [ ] Criterion 2
   
   ## Notes
   Additional information.
   ```

## Example Template Set

See `templates/z80_implementation/` for a complete example of:
- Tracking issue template with placeholders
- Implementation templates
- Dependencies file with placeholders
- Section organization
- Task breakdown
- Dependency management

## Script Features

The `generate_issues.sh` script:
1. Creates issues from all templates
2. Replaces `{{IMPL_X}}` placeholders with actual issue numbers
3. Links implementation issues to tracking issue
4. Adds dependencies between issues
5. Supports custom template directories
6. Validates input and templates
7. Uses GitHub CLI for operations
8. Records generated issue numbers for cleanup

## Requirements

- GitHub CLI (`gh`) installed and authenticated
- Write access to the target repository
- Bash shell environment

## Best Practices

1. Keep templates focused and specific
2. Use placeholders for issue references
3. Let GitHub expand issue titles
4. Include descriptive text only where needed
5. Document dependencies explicitly
6. Add comprehensive testing requirements
7. Consider edge cases
8. Include relevant documentation links

## Troubleshooting

1. Script fails to create issues:
   - Check GitHub CLI authentication
   - Verify repository permissions
   - Ensure template directory exists

2. Dependencies not showing:
   - Check `dependencies.txt` format
   - Verify placeholder format
   - Ensure template numbers match

3. Issue content formatting issues:
   - Verify markdown syntax
   - Check YAML frontmatter
   - Ensure required sections exist

4. Cleanup fails:
   - Check .last_generated_issues exists
   - Verify issue numbers are valid
   - Ensure GitHub access is working
