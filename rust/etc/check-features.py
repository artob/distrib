#!/usr/bin/env python3
# This is free and unencumbered software released into the public domain.

"""Check crates separately so Cargo cannot unify features across workspace roots.

Run from rust/: python3 etc/check-features.py
Set RUSTUP_TOOLCHAIN to select the compiler. --target checks allocation-only
configurations on a target without std; --package can be repeated to limit scope.
Native FFI backends are checked separately with their host prerequisites.
"""

import argparse
import json
import subprocess
import sys


def configurations(package, target):
    """Yield default, minimal, and supported feature-boundary configurations."""
    features = package["features"]
    if not target:
        yield None  # Default features.
    yield ()
    candidates = ("alloc", "serde", "tracing", "all") if target else (
        "alloc", "std", "serde", "parse", "tracing", "unstable", "all"
    )
    for feature in candidates:
        # Build scripts execute on the host; their language backends require std.
        if target and package["name"] == "distrib-build" and feature == "all":
            continue
        if feature in features:
            yield (feature,)
    if "serde" in features:
        yield ("alloc", "serde") if target else ("std", "serde")
    if not target and package["name"] == "distrib":
        yield ("cli",)
        for ecosystem in ("dart", "gleam", "js", "jsr", "npm", "python", "ruby", "rust"):
            yield (ecosystem,)
            yield ("std", ecosystem)
            yield ("cli", ecosystem)
        yield ("macros",)
        yield ("export",)  # Must not activate any language or native backend.


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package", action="append", default=[])
    parser.add_argument("--target")
    options = parser.parse_args()
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--no-deps", "--format-version", "1", "--locked"
    ], text=True))
    members = set(metadata["workspace_members"])
    packages = sorted(
        (p for p in metadata["packages"] if p["id"] in members),
        key=lambda p: p["name"],
    )
    unknown = set(options.package) - {p["name"] for p in packages}
    if unknown:
        parser.error("unknown packages: " + ", ".join(sorted(unknown)))
    failures = []
    count = 0
    for package in packages:
        if options.package and package["name"] not in options.package:
            continue
        for features in configurations(package, options.target):
            command = ["cargo", "check", "--locked", "-p", package["name"]]
            if features is not None:
                command.append("--no-default-features")
                if features:
                    command.extend(["--features", ",".join(features)])
            if options.target:
                command.extend(["--target", options.target])
            result = subprocess.run(command, text=True, capture_output=True)
            count += 1
            print(("PASS " if result.returncode == 0 else "FAIL ") + " ".join(command), flush=True)
            if result.returncode:
                failures.append(command)
                print(result.stdout + result.stderr, file=sys.stderr, flush=True)
    print(f"{count - len(failures)}/{count} feature configurations passed.")
    return bool(failures)


if __name__ == "__main__":
    sys.exit(main())
