# Contributing Guide

Thank you for considering contributing to this project!

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch from `develop`

## Branch Strategy

- `main` - Production-ready code
- `develop` - Development branch (default)
- `feature/*` - New features
- `fix/*` - Bug fixes
- `hotfix/*` - Urgent production fixes

## Development Workflow

1. Create a branch: `git checkout -b feature/your-feature develop`
2. Make your changes
3. Write/update tests
4. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation
   - `refactor:` Code refactoring
   - `test:` Adding/updating tests
   - `chore:` Maintenance
5. Push and create a Pull Request to `develop`

## Code Style

- Follow the existing code style in the project
- Ensure all tests pass before submitting

## Pull Requests

- Fill out the PR template
- Link related issues
- Ensure CI checks pass
- Request review from code owners
