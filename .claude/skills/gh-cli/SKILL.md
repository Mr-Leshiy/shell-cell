---
name: GitHub CLI
description: Expert help with GitHub CLI (gh) for managing pull requests, issues, repositories, workflows, and releases. Use this when working with GitHub operations from the command line.
---
# GitHub CLI (gh)
Expert guidance for GitHub CLI operations and workflows.

## Pull Requests

### Creating PRs
While creating PRs use Pull Request template from `../../../.github/.github/pull_request_template.md`
```shell
# Create PR interactively
gh pr create

# Create PR with title and body
gh pr create --title "Add feature" --body "Description"

# Create PR to specific branch
gh pr create --base main --head feature-branch

# Create draft PR
gh pr create --draft

# Create PR from current branch
gh pr create --fill  # Uses commit messages
```

### Viewing PRs
```shell
# List PRs
gh pr list

# List my PRs
gh pr list --author @me

# View PR details
gh pr view 123

# View PR in browser
gh pr view 123 --web

# View PR diff
gh pr diff 123

# Check PR status
gh pr status
```

## Issues
When creating an issue always use templates from `../../../.github/.github/ISSUE_TEMPLATE`, if dont know which to use, ask.

### Creating Issues
```shell
# Create issue interactively
gh issue create

# Create issue with title and body
gh issue create --title "Bug report" --body "Description"

# Create issue with labels
gh issue create --title "Bug" --label bug,critical

# Assign issue
gh issue create --title "Task" --assignee @me
```

### Viewing Issues
```shell
# List issues
gh issue list

# List my issues
gh issue list --assignee @me

# List by label
gh issue list --label bug

# View issue details
gh issue view 456

# View in browser
gh issue view 456 --web
```

### Managing Issues
```
# Close issue
gh issue close 456

# Reopen issue
gh issue reopen 456

# Edit issue
gh issue edit 456 --title "New title"
gh issue edit 456 --add-label bug
gh issue edit 456 --add-assignee @user

# Comment on issue
gh issue comment 456 --body "Update"
```

## Tips
Use --web flag: Open items in browser for detailed view
Interactive prompts: Most commands work interactively if you omit parameters
Filters: Use --author, --label, --state to filter lists
JSON output: Add --json flag for scriptable output
Template repos: Use gh repo create --template for templates
Auto-merge: Enable with gh pr merge --auto