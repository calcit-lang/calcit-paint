#!/usr/bin/env python3
"""Reject mutable third-party GitHub Actions references."""

from pathlib import Path
import re
import sys


USES_LINE = re.compile(r"^\s*(?:-\s*)?uses:\s*")
USES = re.compile(r"^\s*(?:-\s*)?uses:\s+(?P<action>[^@\s]+)@(?P<revision>[^\s#]+)(?:\s+#\s+(?P<tag>\S+))?\s*$")
SHA = re.compile(r"[0-9a-f]{40}")
EXACT_TAG = re.compile(r"v?\d+\.\d+\.\d+")


def main() -> int:
    errors: list[str] = []
    checked = 0
    for workflow in sorted(Path(".github/workflows").glob("*.y*ml")):
        for line_number, line in enumerate(workflow.read_text().splitlines(), 1):
            if USES_LINE.match(line) is None:
                continue
            match = USES.match(line)
            if match is None:
                errors.append(f"{workflow}:{line_number}: use canonical unquoted 'uses: action@revision # tag' syntax")
                continue
            action = match.group("action")
            if action.startswith("./"):
                continue
            checked += 1
            if SHA.fullmatch(match.group("revision")) is None:
                errors.append(f"{workflow}:{line_number}: {action} is not pinned to a 40-character SHA")
            tag = match.group("tag")
            if tag is None or EXACT_TAG.fullmatch(tag) is None:
                errors.append(f"{workflow}:{line_number}: {action} is missing an exact release-tag comment")

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"Verified {checked} immutable third-party Actions references.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
