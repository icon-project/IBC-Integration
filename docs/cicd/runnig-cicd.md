# CICD on IBC-Integration Repo

<style>
ul {
    list-style-type: square; /* or disc, circle, none, etc. */
}
</style>

## Introduction
CICD build pipeline has been setup using Github Workflow Actions.

## Workflow trigger events
We've established workflow triggers for push events, pull request requests, and modifications to specific paths. The release build will exclusively occur upon tagging.

```
on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main
    paths:
      - 'contracts/javascore/**'
      - '.github/workflows/codecov-javascore.yml'
```  
Runs post-PR merge to main.
```
on:
  push:
    branches:
      - "main"
```  

Trigger for the release build workflow.
```
on:
  push:
    tags:
      - 'v*.*.*-alpha.*'
```

## Github Badges
- Project Status
- License
- Repo Size
- Version
- Codecov

```
[![Project Status: Initial Release](https://img.shields.io/badge/repo%20status-active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration/blob/main/LICENSE)
[![Lines Of Code](https://img.shields.io/tokei/lines/github/icon-project/IBC-Integration?style=flat-square)](https://github.com/icon-project/IBC-Integration)
[![Version](https://img.shields.io/github/tag/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration)
```
## CICD Workflows
1. Build and Deploy Workflows:

	- [basic-rust](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/basic-rust.yml)
	- [Cosmwasm Contracts Test Deployment](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/deploy-cosmwasm-contracts.yml)
	- [Build Proto Generated Go Library](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/go.yaml)
	- [Deploy Contracts After PR Merge to main branch](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/uat-deploy-java-contracts.yml)

2. Test Workflows:
    - [Test Java contracts](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/java-contracts-test.yml)

3. Linting Workflows:
    - [Lint PR](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/lint-pr.yaml)
4. Codecov Workflow:
    - [Codecov javascore](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/codecov-javascore.yml)
    - [CosmWasm contracts Codecov](https://github.com/icon-project/IBC-Integration/blob/main/.github/workflows/cosmwasm-contracts-code-coverage.yml)
4. Release Workflows:
    - [Pre-release](https://github.com/icon-project/IBC-Integration/blob/459-create-release-workflow-for-contracts_2/.github/workflows/release.yaml)

## Pull Request Template
Project contributors will automatically see the template's contents in the pull request body.
[PULL_REQUEST_TEMPLATE](https://github.com/icon-project/IBC-Integration/blob/main/.github/PULL_REQUEST_TEMPLATE.md)