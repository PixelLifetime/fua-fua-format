# Include / exclude demo

Use config: `.fua/demo-config.json`

| File | Staged? | Expected |
|------|---------|----------|
| `src/checkout.component.html` | yes | **Formatted** (matches `**/*.html`) |
| `src/home.component.html` | yes | **Formatted** |
| `src/pop-up.component.html` | yes | **Skipped** (exclude: `pop-up.component.html`) |
| `legacy/old-page.html` | yes | **Skipped** (exclude: `legacy/`) |
| `src/notes.ts` | yes | **Skipped** (not `.html` — messy TS stays as-is) |

## Run (use demo config, not the default `.fua/config.json`)

```bash
cargo build -p cli
git add examples/demo-include-exclude/
target/debug/fua-fua --only-staged --config .fua/demo-config.json
```

Expected output:

```
formatted: .../src/checkout.component.html
formatted: .../src/home.component.html
done — 2 file(s) formatted.
```

**Not** formatted (exclude / outside include):

- `src/pop-up.component.html` — excluded by name
- `legacy/old-page.html` — excluded by `legacy/` path
- `src/notes.ts` — not HTML (look for `// ⚠️ DEMO` — file must be byte-identical after run)

## Pre-commit for this demo

Temporarily point the hook at the demo config, or run:

```bash
target/debug/fua-fua --only-staged --config .fua/demo-config.json
```
