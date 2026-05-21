# Integrating fua-fua into an Angular application

This guide is for **consumer Angular repos**. The formatter itself lives in this repository; your app only installs the npm packages.

## Overview

```text
Angular app
  .fua/config.json          ← settings, include/exclude, WASM plugins
  package.json              ← npm scripts + husky
  .husky/pre-commit         ← calls fua-fua --only-staged
  src/**/*.component.html   ← formatted
  src/**/*.component.ts     ← never touched
```

## Step-by-step

### 1. Install dependencies

From the Angular project root:

```bash
npm i -D fua-fua @fua-fua/plugin-angular @fua-fua/plugin-tailwind husky
```

### 2. Add `.fua/config.json`

See [packages/fua-fua/README.md](../packages/fua-fua/README.md) for a full example. Minimum for Angular + Tailwind:

- `include`: globs to your templates (`src/**/*.html`, etc.)
- `exclude`: paths or regex for files to skip
- `plugins`: paths to WASM files under `node_modules/@fua-fua/...`

### 3. Wire npm scripts

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

### 4. Enable Husky

```bash
npx husky init
cp node_modules/fua-fua/husky/pre-commit .husky/pre-commit
chmod +x .husky/pre-commit
```

### 5. Add CI (recommended)

Copy the `format-check` job from [packages/fua-fua/README.md](../packages/fua-fua/README.md) into `.github/workflows/format-html.yml`.

### 6. First run

Format the codebase once before enforcing in CI:

```bash
npm run format:html
git add -A
git commit -m "chore: format HTML templates with fua-fua"
```

## What not to do

- **No** custom `scripts/format-html.cjs` unless you are stuck on an old `fua-fua` npm release without `--only-staged` / `--check`.
- **Do not** expect `.ts` or `.scss` to be formatted.
- **Do not** point `include` at `node_modules/` or `dist/`.

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `unexpected argument '--only-staged'` | Upgrade `fua-fua` to a release that includes the Rust batch CLI |
| `EACCES` on Linux | `chmod +x node_modules/@fua-fua/linux-x64/bin/fua-fua` |
| Pre-commit formats nothing | Check `include` globs; staged paths must be repo-relative HTML |
| Plugin load error | Confirm WASM paths in config match `node_modules/@fua-fua/plugin-*/dist/` |

## Related docs

- [Main README](../README.md) — full config options and architecture
- [packages/fua-fua/README.md](../packages/fua-fua/README.md) — npm consumer reference (published to npm)
