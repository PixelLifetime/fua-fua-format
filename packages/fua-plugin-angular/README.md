# fua-plugin-angular

Angular-specific formatting plugin for [fua-fua](https://www.npmjs.com/package/fua-fua).

Handles:
- `@if` / `@for` / `@switch` condition wrapping
- `[ngClass]` object multi-line expansion

## Install

```bash
npm i -D @fua-fua/plugin-angular
```

## Config (`.fua/config.json`)

```json
{
  "plugins": [
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

## Options

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `wrap_conditions_min` | `int` | `2` | Min logical operators before wrapping a condition expression |
| `wrap_conditions_in_parens` | `bool` | `true` | Indent wrapped condition operators with aligned parentheses |
| `ngclass_wrap_entries_min` | `int` | `2` | Min `[ngClass]` entries before expanding to multi-line |
