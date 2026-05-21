# fua-fua

Blazing-fast HTML formatter for Angular templates (and plain HTML), with optional WASM plugins.

The npm package ships a small Node launcher (`bin/run.js`) that runs the native `fua-fua` binary for your OS. **You do not need a custom `format-html.js` wrapper** — batch modes (`--only-staged`, `--all`, `--changed`, `--check`) and `include` / `exclude` are built into the CLI (requires a recent `fua-fua` release; see [Version](#version) below).

## What gets formatted

| Formatted | Not formatted |
|-----------|----------------|
| `*.html`, `*.htm` (templates, `index.html`) | `*.ts`, `*.scss`, `*.css`, `*.json` |

Angular TypeScript and styles are untouched. Only component templates and other HTML files are processed.

---

## Integrate into an Angular app

### 1. Install (project root)

```bash
npm i -D fua-fua @fua-fua/plugin-angular @fua-fua/plugin-tailwind husky
```

### 2. Create `.fua/config.json`

Adjust globs for your layout (classic `src/`, or Nx `apps/` / `libs/`).

```json
{
  "indent_size": 2,
  "use_tabs": false,
  "print_width": 100,
  "wrap_attributes": true,
  "wrap_content": true,
  "class_wrap_tokens_min": 6,
  "class_wrap_tokens_per_line": 2,
  "include": [
    "src/**/*.html",
    "projects/**/*.html"
  ],
  "exclude": [
    "node_modules/",
    "dist/",
    "legacy/",
    "third-party\\.component\\.html"
  ],
  "plugins": [
    {
      "path": "./node_modules/@fua-fua/plugin-tailwind/dist/fua_plugin_tailwind.wasm",
      "options": {
        "class_wrap_tokens_per_line": 2,
        "group_blank_lines": true
      }
    },
    {
      "path": "./node_modules/@fua-fua/plugin-angular/dist/fua_plugin_angular.wasm",
      "options": {
        "wrap_conditions_min": 2,
        "wrap_conditions_in_parens": true,
        "ngclass_wrap_entries_min": 2
      }
    }
  ]
}
```

**`include`** — glob patterns (repo-relative). Only paths that match and end in `.html` / `.htm` are considered.

**`exclude`** — regex if valid, otherwise substring match on the path (e.g. skip `legacy/` or one-off templates).

### 3. Add npm scripts (`package.json`)

```json
{
  "scripts": {
    "prepare": "husky",
    "format:html": "fua-fua --all --config .fua/config.json",
    "format:html:staged": "fua-fua --only-staged --config .fua/config.json",
    "format:html:check": "fua-fua --check --changed --config .fua/config.json"
  }
}
```

No `scripts/format-html.cjs` or extra Node wrapper is required.

### 4. Husky pre-commit

```bash
npx husky init
cp node_modules/fua-fua/husky/pre-commit .husky/pre-commit
chmod +x .husky/pre-commit
```

Hook content (already in the copied file):

```sh
#!/usr/bin/env sh
npx fua-fua --only-staged --config .fua/config.json
```

On commit, only **staged** HTML matching `include` / `exclude` is formatted and re-staged. `.ts` files are never modified.

### 5. GitHub Actions (PR check)

```yaml
name: HTML format

on:
  pull_request:

jobs:
  format-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - run: git fetch origin "${{ github.base_ref }}"

      - uses: actions/setup-node@v4
        with:
          node-version: "20"

      - run: npm ci

      - name: Verify HTML formatting
        env:
          GITHUB_BASE_REF: ${{ github.base_ref }}
        run: npx fua-fua --check --changed --config .fua/config.json
```

Fails the PR if changed HTML on the branch is not formatted.

### 6. Verify locally

```bash
# Format one template
npx fua-fua --input src/app/app.component.html \
  --output src/app/app.component.html \
  --config .fua/config.json

# Pre-commit dry run
git add src/app/app.component.html
npm run format:html:staged

# CI-style check before push
npm run format:html:check
```

---

## CLI reference

| Flag | Purpose |
|------|---------|
| `--config .fua/config.json` | Settings + plugins |
| `--input "src/**/*.html"` | Glob; multiple files = in-place |
| `--output path` | Single input file only |
| `--only-staged` | Git staged files (pre-commit) |
| `--all` | All files matching include/exclude |
| `--changed` | Branch diff vs base (`GITHUB_BASE_REF` in CI) |
| `--check` | Verify only; exit 1 if changes needed |

---

## Nx / monorepo

Point `include` at your apps and libs, for example:

```json
"include": [
  "apps/**/*.html",
  "libs/**/*.html"
]
```

Keep `exclude` covering `dist/`, `node_modules/`, and any vendor HTML you must not rewrite.

---

## Version

Published npm binaries must include the batch flags above. If `npx fua-fua --only-staged` prints `unexpected argument`, upgrade `fua-fua` (and platform packages) to the latest release from [fua-fua-format](https://github.com/PixelLifetime/fua-fua-format).

**Do not** add a Node `format-html.js` shim unless you are pinned to an old CLI — call `fua-fua` directly instead.

### Optional: binary permissions (Linux)

If you see `EACCES` when running `fua-fua`, ensure the platform binary is executable:

```bash
chmod +x node_modules/@fua-fua/linux-x64/bin/fua-fua
```

Or add a `postinstall` script in your app that chmods the platform package binary (not required for all environments).

---

## How the npm package works

```
npx fua-fua  →  bin/run.js  →  @fua-fua/<platform>/bin/fua-fua  (+ your CLI args)
```

`run.js` only resolves the OS binary and forwards arguments. All formatting logic runs in Rust.
