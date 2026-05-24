# Migration notes

## From `httpgenerator`

- Use `httpfiletools generate` instead of a standalone generator binary.
- Layout, header, auth, and IntelliJ test flags map directly onto `httpgenerator-core` settings.
- `--dry-run` previews the generated file plan without writing files.

## From `httprunner`

- Use `httpfiletools run` instead of a standalone runner binary.
- Recursive suite discovery is available through `--discover`.
- Markdown and HTML reports remain available through `--report`.
- Exported request and response artifacts remain available through `--export`.
