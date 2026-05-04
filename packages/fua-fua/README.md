# fua-fua

Blazing-fast, highly-permissive HTML formatter written in Rust.

## Install

```bash
npm i -D fua-fua
```

Optional plugins:

```bash
npm i -D fua-plugin-angular fua-plugin-tailwind
```

## Usage

```bash
npx fua-fua --input src/app/app.component.html --output src/app/app.component.html --config .fua/config.json
```

## npm scripts (add to your project)

```json
{
  "scripts": {
    "format:html": "node scripts/format-html.js",
    "format:html:check": "node scripts/format-html.js --check",
    "format:html:changed": "node scripts/format-html.js --only-changed"
  }
}
```

## Config (`.fua/config.json`)

```json
{
  "indent_size": 4,
  "use_tabs": true,
  "print_width": 100,
  "wrap_attributes": true,
  "class_wrap_tokens_min": 6,
  "class_wrap_tokens_per_line": 2,
  "plugins": [
    {
      "path": "./node_modules/fua-plugin-tailwind/dist/fua_plugin_tailwind.wasm",
      "options": {
        "class_wrap_tokens_per_line": 2,
        "group_blank_lines": true
      }
    },
    {
      "path": "./node_modules/fua-plugin-angular/dist/fua_plugin_angular.wasm",
      "options": {
        "wrap_conditions_min": 2,
        "wrap_conditions_in_parens": true,
        "ngclass_wrap_entries_min": 2
      }
    }
  ]
}
```
