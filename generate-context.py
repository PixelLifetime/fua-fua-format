import datetime
import fnmatch
import os
from collections import Counter
from pathlib import Path
from typing import Iterable

# --- CONFIGURATION ---
OUTPUT_FILE = "context_for_ai.md"

# Directories to ignore completely.
IGNORE_DIRS = {
    ".git",
    ".idea",
    ".vscode",
    ".vs",
    ".venv",
    "venv",
    "env",
    "__pycache__",
    "node_modules",
    "dist",
    "build",
    "target",
    "vendor",
    "bin",
    "obj",
    "out",
    "debug",
    "release",
    "coverage",
    ".nuxt",
    ".next",
    "cmake-build-debug",
    ".angular",
    ".husky",
    ".storybook",
    ".pytest_cache",
    ".trunk",
    "logs",
    ".cursor",
    "postgres-data",
    "pgdata",
}

# Files or path globs to exclude from the report entirely.
# These are usually lockfiles, generated artifacts, local crash dumps, or the
# report itself.
IGNORE_PATH_PATTERNS = {
    OUTPUT_FILE,
    "Cargo.lock",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "composer.lock",
    "go.sum",
    "Gemfile.lock",
    ".env",
    ".env.local",
    ".env.development",
    ".env.production",
    ".env.test",
    "secrets.yaml",
    "*.log",
    "*.exe",
    "*.dll",
    "*.so",
    "*.dylib",
    "*.pyc",
    "*.pyo",
    "*.pyd",
    "*.db",
    "*.csv",
    "*.pdf",
    "*.zip",
    "*.tar.gz",
    "*.swp",
    "*.swo",
    "*.bak",
    "*~",
    ".eslintcache",
    ".DS_Store",
    "Thumbs.db",
    "documentation.json",
    "audio-backend",
    "main",
    # Repo-specific scratch files that do not help model context.
    "panic*.txt",
    "test_output.html",
}

# Max file size to read (in bytes) to prevent bloating.
MAX_FILE_SIZE = 500 * 1024

# Map extensions to Markdown languages.
EXT_TO_LANG = {
    ".py": "python",
    ".js": "javascript",
    ".ts": "typescript",
    ".jsx": "jsx",
    ".tsx": "tsx",
    ".java": "java",
    ".c": "c",
    ".cpp": "cpp",
    ".cs": "csharp",
    ".go": "go",
    ".rs": "rust",
    ".php": "php",
    ".rb": "ruby",
    ".html": "html",
    ".css": "css",
    ".scss": "scss",
    ".json": "json",
    ".yaml": "yaml",
    ".yml": "yaml",
    ".xml": "xml",
    ".md": "markdown",
    ".sql": "sql",
    ".sh": "bash",
    ".bat": "batch",
    ".dockerfile": "dockerfile",
    "Dockerfile": "dockerfile",
    ".toml": "toml",
    ".ini": "ini",
    ".csproj": "xml",
}


def normalize_rel_path(path: Path) -> str:
    """Return relative paths in a stable, markdown-friendly format."""
    return path.as_posix()


def matches_patterns(rel_path: str, filename: str, patterns: Iterable[str]) -> bool:
    """Match both basename and relative path against glob patterns."""
    return any(
        fnmatch.fnmatch(filename, pattern) or fnmatch.fnmatch(rel_path, pattern)
        for pattern in patterns
    )


def should_skip_file(file_path: Path, root_dir: Path) -> bool:
    """Return True when a file should be excluded from the report."""
    rel_path = normalize_rel_path(file_path.relative_to(root_dir))
    return matches_patterns(rel_path, file_path.name, IGNORE_PATH_PATTERNS)


def iter_report_files(root_dir: Path):
    """Yield report-worthy files while applying one shared filter pipeline."""
    for current_root, dirs, files in os.walk(root_dir):
        dirs[:] = sorted(d for d in dirs if d not in IGNORE_DIRS)

        current_path = Path(current_root)
        for filename in sorted(files):
            file_path = current_path / filename
            if should_skip_file(file_path, root_dir):
                continue
            yield file_path


def build_tree(files, root_dir: Path) -> str:
    """Generate an ASCII tree from the already-filtered file list."""
    nested_tree = {}

    for file_path in files:
        rel_parts = file_path.relative_to(root_dir).parts
        cursor = nested_tree
        for directory in rel_parts[:-1]:
            cursor = cursor.setdefault(directory, {})
        cursor[rel_parts[-1]] = None

    lines = ["```text", f"{root_dir.name}/"]
    render_tree(nested_tree, lines)
    lines.append("```")
    return "\n".join(lines) + "\n"


def render_tree(tree, lines, prefix: str = "") -> None:
    """Render a nested dictionary produced by build_tree."""
    items = sorted(tree.items(), key=lambda item: (item[1] is None, item[0].lower()))

    for index, (name, child) in enumerate(items):
        is_last = index == len(items) - 1
        branch = "`-- " if is_last else "|-- "

        if child is None:
            lines.append(f"{prefix}{branch}{name}")
            continue

        lines.append(f"{prefix}{branch}{name}/")
        next_prefix = prefix + ("    " if is_last else "|   ")
        render_tree(child, lines, next_prefix)


def is_binary_file(filepath: Path) -> bool:
    """Check if a file is binary by reading a small chunk."""
    try:
        with filepath.open("rb") as handle:
            chunk = handle.read(1024)
            return b"\0" in chunk
    except OSError:
        return True


def get_language(filename: str) -> str:
    """Return the markdown language tag based on extension."""
    _, ext = os.path.splitext(filename)
    if filename in EXT_TO_LANG:
        return EXT_TO_LANG[filename]
    return EXT_TO_LANG.get(ext.lower(), "text")


def read_file_content(filepath: Path) -> str:
    """Read file content with size limit and binary checks."""
    if filepath.stat().st_size > MAX_FILE_SIZE:
        return f"[NOTE: File content skipped (Size > {MAX_FILE_SIZE / 1024:.1f} KB)]"

    if is_binary_file(filepath):
        return "[NOTE: Binary file detected and skipped]"

    for encoding in ("utf-8", "utf-16", "latin-1", "cp1252"):
        try:
            with filepath.open("r", encoding=encoding) as handle:
                return handle.read()
        except (OSError, UnicodeDecodeError):
            continue

    return "[ERROR: Could not decode file content]"


def get_project_stats(files) -> str:
    """Count filtered files by extension."""
    stats = Counter()

    for file_path in files:
        ext = file_path.suffix.lower() or "(no extension)"
        stats[ext] += 1

    report = f"**Total Files Scanned:** {len(files)}\n\n"
    report += "| Extension | Count |\n|---|---|\n"

    for ext, count in sorted(stats.items(), key=lambda item: (-item[1], item[0])):
        report += f"| {ext} | {count} |\n"

    return report


def collect_file_contents(files, root_dir: Path) -> str:
    """Collect content for each filtered file."""
    sections = []

    for file_path in files:
        rel_path = normalize_rel_path(file_path.relative_to(root_dir))
        lang = get_language(file_path.name)
        file_content = read_file_content(file_path)
        sections.append(f"\n# FILE: {rel_path}\n```{lang}\n{file_content}\n```\n")

    return "".join(sections)


def main() -> None:
    root_dir = Path.cwd()
    timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    print(f">>> Generating context file for project in: {root_dir}")
    print(">>> Please wait...")

    report_files = list(iter_report_files(root_dir))

    final_output = "# PROJECT CONTEXT REPORT\n"
    final_output += f"Generated: {timestamp}\n\n"

    final_output += "## 1. PROJECT STRUCTURE\n"
    final_output += build_tree(report_files, root_dir)
    final_output += "\n---\n"

    final_output += "## 2. PROJECT STATISTICS\n"
    final_output += get_project_stats(report_files)
    final_output += "\n---\n"

    final_output += "## 3. FILE CONTENTS\n"
    final_output += collect_file_contents(report_files, root_dir)

    output_path = root_dir / OUTPUT_FILE
    with output_path.open("w", encoding="utf-8") as handle:
        handle.write(final_output)

    size_mb = output_path.stat().st_size / (1024 * 1024)
    print(f"\n[SUCCESS] Context saved to: {OUTPUT_FILE}")
    print(f"   Size: {size_mb:.2f} MB")
    print("   You can now upload this file to ChatGPT/Claude/LLM.")


if __name__ == "__main__":
    main()
