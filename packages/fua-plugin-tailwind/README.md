# fua-plugin-tailwind

Tailwind CSS class sorting and wrapping plugin for [fua-fua](https://www.npmjs.com/package/fua-fua).

Handles:
- Sorts classes by Tailwind utility group order
- Wraps long class strings over multiple lines
- Inserts blank lines between utility groups (optional)

## Install

```bash
npm i -D fua-plugin-tailwind
```

## Config (`.fua/config.json`)

```json
{
  "plugins": [
    {
      "path": "./node_modules/fua-plugin-tailwind/dist/fua_plugin_tailwind.wasm",
      "options": {
        "class_wrap_tokens_per_line": 2,
        "group_blank_lines": true
      }
    }
  ]
}
```

## Options

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `class_wrap_tokens_per_line` | `int` | `null` | Max Tailwind classes per line (inherits core `class_wrap_tokens_per_line` if null) |
| `group_blank_lines` | `bool` | `false` | Insert a blank line between each Tailwind utility group |
