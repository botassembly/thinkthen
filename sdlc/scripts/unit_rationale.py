"""Require an owning rationale on actual Rust unit-test modules."""
import re


def missing_rationales(text, tokenize):
    offsets = []
    tokens = tokenize(text, literals=True, offsets=offsets)
    modules = [("file", 0, len(text), [])]
    stack = [0]
    braces = []
    for index, token in enumerate(tokens):
        if token == "{":
            parent = stack[-1]
            if index >= 2 and tokens[index - 2] == "mod":
                modules.append((tokens[index - 1], offsets[index] + 1, len(text), []))
                parent = len(modules) - 1
            stack.append(parent)
            braces.append(index)
        elif token == "}" and braces:
            opened = braces.pop()
            owner = stack.pop()
            if modules[owner][1] == offsets[opened] + 1:
                name, start, _, tests = modules[owner]
                modules[owner] = (name, start, offsets[index], tests)
        elif tokens[index:index + 4] == ["#", "[", "test", "]"]:
            # A cfg(test) import, empty module or literal example is no test.
            modules[stack[-1]][3].append(index)
    missing = []
    justified = []
    for name, start, end, tests in modules:
        first = next((offset for offset in offsets if start <= offset < end), end)
        header = text[start:first]
        if re.search(r"(?m)^\s*//!\s*Stays:[ \t]*\S[^\n]*", header):
            justified.append((start, end))
        elif tests and not any(outer <= start and end <= close for outer, close in justified):
            missing.append((name, start))
    return missing


def check_unit_rationale(repo, tokenize, fail):
    fixtures = (
        ("#[cfg(test)] mod tests { #[test] fn a() {} }", True),
        ("#[cfg(test)] mod tests {\n//! Stays:   \n#[test] fn a() {} }", True),
        ("#[cfg(test)] mod tests {\n//! Stays: private cancellation races.\n#[test] fn a() {} }", False),
        ("//! Stays: parser boundaries.\n#[test] fn a() {}", False),
        ('// #[test] fn a() {}\nconst S: &str = "#[test] fn a() {}";', False),
        ("#[cfg(test)] use super::*; #[cfg(test)] mod empty {}", False),
        ("mod outer { mod tests { #[test] fn a() {} } }", True),
        ("//! Stays: private owner state.\nmod tests { #[test] fn a() {} }", False),
    )
    for text, refused in fixtures:
        if bool(missing_rationales(text, tokenize)) != refused:
            fail("unit-rationale", "the planted module rationale has the wrong result")
    paths = (path for folder in ("crates", "libraries", "databases")
             for path in (repo / folder).glob("*/src/**/*.rs"))
    for path in sorted(paths):
        relative = path.relative_to(repo).as_posix()
        for name, start in missing_rationales(path.read_text(), tokenize):
            line = path.read_text()[:start].count("\n") + 1
            fail("unit-rationale", f"{relative}:{line} module {name} needs //! Stays: and its reason")
