# Slash Command Ideas

## Developer Workflow
- [ ] `/env` — list all env vars used in the project (grep for process.env, std::env, etc.) without exposing values
- [ ] `/dead-code` — find exported functions/types that aren't imported anywhere
- [ ] `/imports` — map the import graph for a file or module
- [ ] `/complexity` — analyze cyclomatic complexity of functions in current file
- [ ] `/duplicates` — find similar/duplicate code blocks

## Documentation
- [ ] `/readme` — generate README from project structure + package.json
- [ ] `/changelog` — generate changelog from git commits since last tag
- [ ] `/api-docs` — extract all exported functions/types with their signatures

## Project Context
- [ ] `/structure` — generate a tree view of the project with descriptions
- [ ] `/config` — summarize all config files (tsconfig, eslint, prettier, etc.)

## Utilities
- [ ] `/base64` — encode/decode base64 strings
- [ ] `/uuid` — generate UUIDs
- [ ] `/timestamp` — convert between timestamps and human dates
- [ ] `/json` — format/validate JSON
