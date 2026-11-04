# GitHub Actions PR Validation

This document describes the GitHub Actions CI pipeline for validating pull requests.

## Overview

The `Validate Pull Request` workflow (`.github/workflows/validate-pr.yml`) automatically validates pull requests before they can be merged. It runs multiple independent validation jobs in parallel.

## Validation Jobs

The following jobs run independently and in parallel:

| Job ID | Display Name | Description |
|--------|-------------|-------------|
| `pre-commit` | Pre-commit Hooks | Runs all configured pre-commit hooks with automatic fix and retry |
| `vitest` | Vitest Tests | Runs all Vitest tests in CI mode |
| `rust-tests` | Rust Unit Tests | Executes Rust unit tests using Cargo |
| `clippy` | Rust Clippy | Runs Clippy linter with warnings as errors |
| `typescript-types` | TypeScript Type Checking | Runs `vue-tsc --noEmit` for type checking |
| `frontend-build` | Frontend Build | Builds the frontend to verify it compiles |

### Pre-commit Hooks

The following checks run as part of the pre-commit job:

| Hook | Description |
|------|-------------|
| `end-of-file-fixer` | Ensures files end with a newline |
| `trailing-whitespace` | Removes trailing whitespace |
| `check-yaml` | Validates YAML syntax |
| `check-added-large-files` | Prevents committing large files |
| `check-merge-conflict` | Detects merge conflict markers |
| `check-case-conflict` | Detects filename case conflicts |
| `cargo-fmt` | Formats Rust code using `cargo fmt` |

**Note:** Rust formatting (`cargo fmt`) is handled by pre-commit with automatic fixes, so there is no separate `Rust Formatting` CI job.

## Required GitHub Secrets

### `GH_PR_VALIDATION_TOKEN` (Optional but Recommended)

A GitHub App installation token or Personal Access Token with `contents: write` permission on the repository.

**Why it's needed:** The default `GITHUB_TOKEN` does not trigger new workflow runs when pushing commits. When pre-commit fixes are pushed, a custom token ensures the workflow re-runs to validate the new commit.

**How to create a GitHub App token:**

1. Go to your repository settings → Settings → Developer settings → GitHub Apps
2. Create a new GitHub App with:
   - Repository permissions: Contents (Read & Write), Pull requests (Read & Write)
   - Subscribe to repository events: No
3. Generate a token and add it as `GH_PR_VALIDATION_TOKEN` in your repository secrets

**How to create a Personal Access Token:**

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens
2. Create a token with:
   - Repository access: This repository only
   - Permissions: Contents (Read & Write), Pull requests (Read & Write)
3. Add the token as `GH_PR_VALIDATION_TOKEN` in your repository secrets

**Without this secret:** The workflow will still work, but when pre-commit pushes automatic fixes, the validation workflow won't re-run on the new commit. The old successful run will still appear as passing for the PR.

## Pre-commit Behavior

### First Execution
1. All pre-commit hooks run on all files
2. If hooks pass without modifications → Validation succeeds
3. If hooks fail without modifications → Validation fails
4. If hooks modify files → Retry process begins

### Retry Process
1. Modified files are staged
2. All pre-commit hooks run again
3. Result of second execution determines final status

### Commit and Push
When pre-commit modifies files:
1. All changes are collected, including any additional changes from the retry
2. Only modified files are staged
3. A commit is created with message `ci: apply pre-commit fixes`
4. The commit is pushed to the PR's source branch
5. The job reports success or failure based on the second execution's exit status

**Important:** The commit and push step runs regardless of whether the second pre-commit execution succeeds or fails. This ensures that automatic fixes are always applied to the branch.

**Safety measures:**
- Never creates empty commits
- Never force-pushes
- Never pushes to the target branch
- Only pushes to PRs from the same repository (not forks)
- Handles concurrent commits safely via concurrency group

## Workflow Retriggering

When pre-commit pushes fixes, the workflow automatically re-triggers via the `push` event. The `detect-pr` job finds the associated pull request and ensures all validation jobs run against the new commit.

The `push` trigger is limited to paths that are relevant to validation:
- `.pre-commit-config.yaml`
- `src/**`
- `src-tauri/**`
- `package.json`
- `pnpm-lock.yaml`
- `tsconfig.json`

This prevents the workflow from running on unrelated pushes (e.g., documentation-only changes).

## Branch Protection Rules

To enforce PR validation, configure the following required status checks in your branch protection rules or rulesets:

1. `Pre-commit Hooks`
2. `Vitest Tests`
3. `Rust Unit Tests`
4. `Rust Clippy`
5. `TypeScript Type Checking`
6. `Frontend Build`

### Configuring via GitHub UI

1. Go to Repository → Settings → Branches → Branch protection rules
2. Edit or create a rule for your default branch
3. Under "Require status checks to pass before merging", add the job names above
4. Enable "Require branches to be up to date before merging"

### Configuring via GitHub CLI

```bash
gh api repos/{owner}/{repo}/branches/{branch}/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["Pre-commit Hooks","Vitest Tests","Rust Unit Tests","Rust Clippy","TypeScript Type Checking","Frontend Build"]}' \
  --field enforce_admins=true \
  --field restrictions=null
```

## Manual Reruns

You can manually re-run the workflow:

1. Go to Repository → Actions → Validate Pull Request
2. Select the workflow run
3. Click "Re-run jobs"

Or use the `workflow_dispatch` trigger from the Actions tab.

## Caching

The workflow uses the following caches to speed up execution:

- **Pre-commit:** `~/.cache/pre-commit` keyed by `.pre-commit-config.yaml` hash
- **Rust:** `Swatinem/rust-cache@v2` for Cargo dependencies
- **Node.js:** `actions/setup-node@v4` with pnpm cache

## Fork Pull Request Security

For PRs from forks:
- Pre-commit hooks still run and report results
- Automatic fixes are **not** pushed to fork branches
- No write credentials are exposed to untrusted code
- The `GH_PR_VALIDATION_TOKEN` is only used for same-repository PRs

## Concurrency

Outdated workflow runs are automatically cancelled via the concurrency group:

```yaml
concurrency:
  group: pr-validation-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: true
```

This ensures only the latest commit is validated.

## Limitations

1. **Fork PRs:** Automatic pre-commit fixes cannot be pushed to fork branches. Contributors must apply fixes locally.
2. **Workflow retriggering:** Without `GH_PR_VALIDATION_TOKEN`, the workflow won't re-run after pre-commit pushes fixes. The previous successful run may still appear as passing.
3. **Rust build:** The workflow runs tests and linting but does not perform a full Tauri build, as that requires platform-specific dependencies and is covered by the release workflow.
4. **ESLint/Prettier:** No ESLint or Prettier configuration exists in this project. If added in the future, they can be integrated into pre-commit hooks.
