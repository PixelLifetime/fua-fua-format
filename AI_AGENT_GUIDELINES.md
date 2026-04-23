# AI Agent Guidelines

This repository is being refactored for clarity, maintainability, and clean architecture.

The goal is not to add features first. The goal is to keep the codebase structured, readable, and easy to change without breaking unrelated parts.

## Core principle

Every file and module must have one clear responsibility.

Do not mix:
- CLI orchestration with formatting logic
- parsing with output formatting
- generic HTML logic with framework-specific logic
- helper utilities with business logic
- tests with production code

## Current architecture

The project is organized into these main layers:

- `crates/cli/src/main.rs`  
  Minimal entry point only.

- `crates/cli/src/app.rs`  
  CLI orchestration: input, config, plugin loading, engine execution, output.

- `crates/cli/src/args.rs`  
  Argument definitions only.

- `crates/core/src/engine.rs`  
  High-level pipeline: parse input, build syntax tree, format output.

- `crates/core/src/lexer.rs`  
  Tokenization only.

- `crates/core/src/parser.rs`  
  Lossless tree building only.

- `crates/core/src/syntax.rs`  
  Syntax kinds and tree type definitions only.

- `crates/core/src/formatter/`  
  Formatting logic split by responsibility:
  - `traversal.rs`
  - `tags.rs`
  - `content.rs`
  - `context.rs`
  - `hooks.rs`
  - `output.rs`

- `crates/core/src/plugins.rs`  
  Plugin host and dispatch logic only.

- `crates/core/src/architecture_tests.rs`  
  Structural guardrails only.

## Safe change rules

When making a change:

1. Keep the existing layer boundaries intact.
2. Make the smallest change that solves the task.
3. Prefer extracting a helper over expanding a large function.
4. Prefer renaming or moving code over adding more branching.
5. Keep the top-level flow simple and obvious:
   `read -> config -> parse -> format -> write`
6. Update tests when behavior changes.
7. Keep the project buildable after each meaningful step.
8. Remove dead code and leftovers after refactoring.

## File ownership rules

### CLI files
`crates/cli/*` may only handle:
- command-line args
- file input/output
- config loading
- plugin path resolution
- calling the engine

Do not put formatting rules or parsing logic in CLI files.

### Core engine
`crates/core/src/engine.rs` should only coordinate the formatting pipeline.

Do not put file I/O or CLI logic here.

### Lexer and parser
`lexer.rs` and `parser.rs` should only build the lossless syntax tree.

Do not add formatting behavior here.

### Formatter
`crates/core/src/formatter/*` should contain formatting rules only.

Keep each submodule focused:
- `traversal.rs`: tree walking
- `tags.rs`: tag formatting
- `content.rs`: text and whitespace formatting
- `context.rs`: helper logic for syntax context
- `hooks.rs`: hook dispatch and plugin interaction
- `output.rs`: indentation and output emission

Do not turn formatter modules into dumping grounds.

### Architecture tests
`architecture_tests.rs` should protect structure, not implement product behavior.

## Disallowed changes

Do not:
- collapse all logic into one file
- add unrelated helper functions to random modules
- move formatting concerns into CLI
- move parsing concerns into formatter
- add hidden side effects
- introduce architecture that only works for one example
- keep framework-specific branches inside generic code
- add complexity just because it seems clever

## Allowed refactors

Good changes are:
- splitting a large function into small focused helpers
- moving code into the correct module
- renaming confusing variables or functions
- removing duplication
- simplifying condition chains
- tightening module boundaries
- improving tests and guardrails

## Before editing code

State:
- which files will be changed
- why each file is the right place
- what behavior is expected to remain the same
- whether any tests need updating

## After editing code

Summarize:
- what changed
- which files were touched
- which tests were added or updated
- whether any follow-up cleanup is still needed

## Quality standard

Code should be:
- clear
- boring in the good sense
- easy to trace
- easy to extend
- easy to test
- hard to break accidentally

If a change makes the code harder to understand, the change is not finished yet.