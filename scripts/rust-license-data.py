#!/usr/bin/env python3
"""Collect complete, source-backed Rust license texts for Bergamotto.

This deliberately uses cargo-about for dependency reachability and selected
SPDX branches. Its stock SPDX template text is never used as a substitute for
package copyright/license files: every emitted notice is read from an actual
package file or a checksum-verified recovered upstream file.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import tomllib
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
ABOUT_CONFIG = ROOT / "about.toml"
ABOUT_TEMPLATE = ROOT / "about.hbs"
TARGETS = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]
EXPECTED_ABOUT_VERSION = "0.9.3"
EXPECTED_WORKSPACE_IGNORED = {"bergamot-core", "bergamot-tauri", "os-adapter"}
LEDGER_PATH = ROOT / "third-party/rust-license-fallbacks.json"
ARTIFACTS_DIR = ROOT / "third-party"


class CollectionError(RuntimeError):
    pass


def sha256(data: bytes | str) -> str:
    if isinstance(data, str):
        data = data.encode("utf-8")
    return hashlib.sha256(data).hexdigest()


def run(args: list[str], *, cwd: Path = ROOT, timeout: int = 900) -> str:
    try:
        result = subprocess.run(
            args,
            cwd=cwd,
            check=True,
            text=True,
            encoding="utf-8",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
        )
    except subprocess.CalledProcessError as error:
        details = error.stderr.strip() or error.stdout.strip() or str(error)
        raise CollectionError(f"command failed ({' '.join(args)}): {details}") from error
    except subprocess.TimeoutExpired as error:
        raise CollectionError(f"command timed out ({' '.join(args)})") from error
    return result.stdout


def run_cargo(args: list[str], *, cwd: Path = ROOT, timeout: int = 900) -> str:
    if "--offline" in args:
        try:
            return run(args, cwd=cwd, timeout=timeout)
        except CollectionError:
            online_args = [arg for arg in args if arg != "--offline"]
            return run(online_args, cwd=cwd, timeout=timeout)
    return run(args, cwd=cwd, timeout=timeout)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CollectionError(f"cannot read {path}: {error}") from error


def cargo_metadata() -> dict[str, Any]:
    return json.loads(
        run_cargo(["cargo", "metadata", "--locked", "--offline", "--all-features", "--format-version", "1"])
    )


def package_key(package: dict[str, Any]) -> tuple[str, str]:
    return (str(package.get("name", "")), str(package.get("version", "")))


def metadata_maps(metadata: dict[str, Any]) -> tuple[dict[tuple[str, str], dict[str, Any]], set[str]]:
    by_key: dict[tuple[str, str], dict[str, Any]] = {}
    for package in metadata.get("packages", []):
        by_key[package_key(package)] = package
    workspace_ids = set(metadata.get("workspace_members", []))
    return by_key, workspace_ids


def temporary_about_config(metadata_packages: dict[tuple[str, str], dict[str, Any]]) -> Path:
    """Make clarify paths absolute; cargo-about otherwise resolves them from the workspace."""
    ring = next(
        (p for (name, _version), p in metadata_packages.items() if name == "ring"),
        None,
    )
    if ring is None:
        raise CollectionError("locked Cargo metadata has no ring package for about.toml clarification")
    ring_root = Path(ring["manifest_path"]).resolve().parent
    if not ring_root.is_dir():
        raise CollectionError(f"ring source directory is missing: {ring_root}")

    source = ABOUT_CONFIG.read_text(encoding="utf-8")
    clarify_package: str | None = None
    output: list[str] = []
    for line in source.splitlines():
        stripped = line.strip()
        table = re.fullmatch(r"\[\[([A-Za-z0-9_-]+)\.clarify\.files\]\]", stripped)
        if table:
            clarify_package = table.group(1)
        elif stripped.startswith("["):
            clarify_package = None
        if clarify_package and stripped.startswith("path = "):
            match = re.fullmatch(r'\s*path\s*=\s*"([^"]+)"\s*', line)
            if not match:
                raise CollectionError(f"unsupported clarification path syntax: {line}")
            path_text = match.group(1)
            if clarify_package == "ring":
                absolute = (ring_root / path_text).resolve()
            else:
                absolute = (ROOT / path_text).resolve()
            if not absolute.is_relative_to(ROOT) and clarify_package != "ring":
                raise CollectionError(f"clarification path escapes the workspace: {path_text}")
            if not absolute.is_file():
                raise CollectionError(f"{clarify_package} clarification file is missing: {absolute}")
            line = f'path = "{absolute.as_posix()}"'
        output.append(line)

    temp = tempfile.NamedTemporaryFile(
        mode="w", encoding="utf-8", prefix="bergamotto-about-", suffix=".toml", delete=False
    )
    try:
        temp.write("\n".join(output) + "\n")
        return Path(temp.name)
    finally:
        temp.close()


def cargo_about_version() -> str:
    version_line = run(["cargo", "about", "--version"]).strip()
    match = re.search(r"(?:cargo-about|about)\s+(\d+\.\d+\.\d+)", version_line, re.IGNORECASE)
    if not match:
        match = re.search(r"(\d+\.\d+\.\d+)", version_line)
    version = match.group(1) if match else ""
    if version != EXPECTED_ABOUT_VERSION:
        raise CollectionError(
            f"expected cargo-about {EXPECTED_ABOUT_VERSION}, found {version_line or 'unknown version'}"
        )
    return version


def cargo_about_data(config: Path, target: str) -> dict[str, Any]:
    print(f"cargo-about JSON: {target}", file=sys.stderr, flush=True)
    output = run_cargo(
        [
            "cargo", "about", "generate", "--config", str(config), "--manifest-path",
            str(ROOT / "Cargo.toml"), "--locked", "--offline", "--workspace", "--all-features", "--fail",
            "--format", "json", "--target", target,
        ]
    )
    try:
        data = json.loads(output)
    except json.JSONDecodeError as error:
        raise CollectionError(f"cargo-about emitted invalid JSON for {target}: {error}") from error
    if not isinstance(data.get("crates"), list) or not isinstance(data.get("licenses"), list):
        raise CollectionError(f"cargo-about JSON for {target} has an unexpected schema")
    return data


def about_template_license_texts(data: dict[str, Any]) -> list[dict[str, str]]:
    entries: dict[tuple[str, str], dict[str, str]] = {}
    for license_info in data.get("licenses", []):
        spdx_id = license_info.get("id")
        text = license_info.get("text")
        if not isinstance(spdx_id, str) or not spdx_id or not isinstance(text, str) or not text.strip():
            raise CollectionError("cargo-about union JSON contains a license without SPDX ID or text")
        digest = sha256(text)
        entries[(spdx_id, digest)] = {"spdxId": spdx_id, "text": text, "sha256": digest}
    if not entries:
        raise CollectionError("cargo-about union JSON contains no license texts")
    return [entries[key] for key in sorted(entries)]


def cargo_about_union_license_texts(config: Path) -> list[dict[str, str]]:
    print("cargo-about JSON: configured target union", file=sys.stderr, flush=True)
    output = run_cargo(
        [
            "cargo", "about", "generate", "--config", str(config), "--manifest-path",
            str(ROOT / "Cargo.toml"), "--locked", "--offline", "--workspace", "--all-features", "--fail",
            "--format", "json",
        ]
    )
    try:
        data = json.loads(output)
    except json.JSONDecodeError as error:
        raise CollectionError(f"cargo-about emitted invalid configured-union JSON: {error}") from error
    return about_template_license_texts(data)


def render_about_template(config: Path) -> str:
    print("cargo-about template: about.hbs for configured target union", file=sys.stderr, flush=True)
    return run_cargo(
        [
            "cargo", "about", "generate", str(ABOUT_TEMPLATE), "--config", str(config),
            "--manifest-path", str(ROOT / "Cargo.toml"), "--locked", "--offline", "--workspace",
            "--all-features", "--fail", "--format", "handlebars",
        ]
    )


def is_workspace(crate: dict[str, Any], workspace_ids: set[str]) -> bool:
    package = crate.get("package", crate)
    package_id = package.get("id")
    if package_id and package_id in workspace_ids:
        return True
    manifest = package.get("manifest_path")
    if manifest:
        try:
            return Path(manifest).resolve().is_relative_to(ROOT)
        except OSError:
            return False
    return False


def license_file_candidates(package: dict[str, Any], fallback: dict[str, Any] | None) -> list[dict[str, Any]]:
    candidates: list[dict[str, Any]] = []
    if fallback:
        artifact_dir = ROOT / "third-party/license-texts" / f"{package['name']}-{package['version']}"
        for item in fallback.get("files", []):
            path = (artifact_dir / item["path"]).resolve()
            candidates.append({
                "path": path,
                "spdxId": item.get("spdxId", ""),
                "sourceUrl": item.get("sourceUrl", ""),
                "displayPath": (Path("third-party/license-texts") / f"{package['name']}-{package['version']}" / item["path"]).as_posix(),
                "expectedSha256": item.get("sha256", ""),
                "method": fallback.get("method"),
                "fallback": True,
            })

    manifest = package.get("manifest_path")
    package_root = Path(manifest).resolve().parent if manifest else None
    if package_root and package_root.is_dir():
        license_file = package.get("license_file")
        if license_file:
            path = (package_root / license_file).resolve()
            candidates.append({
                "path": path,
                "spdxId": "",
                "sourceUrl": "",
                "displayPath": f"{package['name']}@{package['version']}/{path.relative_to(package_root).as_posix()}",
                "expectedSha256": "",
                "fallback": False,
            })
        files: list[Path] = []
        for base, dirs, names in os.walk(package_root):
            depth = len(Path(base).relative_to(package_root).parts)
            dirs[:] = sorted(d for d in dirs if d not in {".git", "target", "vendor"}) if depth < 6 else []
            for name in names:
                upper = name.upper()
                if upper.startswith(("LICENSE", "LICENCE", "COPYING")) or upper in {"COPYRIGHT", "NOTICE"}:
                    files.append(Path(base) / name)
        for path in sorted(files):
            candidates.append({
                "path": path,
                "spdxId": "",
                "sourceUrl": "",
                "displayPath": f"{package['name']}@{package['version']}/{path.relative_to(package_root).as_posix()}",
                "expectedSha256": "",
                "fallback": False,
            })

    # Cargo-about may identify a real selected license file. Accept it only if it
    # belongs to the current package, avoiding the globally selected first source.
    return candidates


LICENSE_FILENAMES: dict[str, tuple[str, ...]] = {
    "MIT": ("LICENSE-MIT", "LICENSE_MIT", "LICENSE-MIT.TXT", "LICENSE_MIT.TXT", "MIT-LICENSE", "COPYING.MIT"),
    "Apache-2.0": ("LICENSE-APACHE", "LICENSE_APACHE", "LICENSE-APACHE-2.0", "LICENSE_APACHE-2.0", "APACHE-LICENSE"),
    "Apache-2.0 WITH LLVM-exception": ("LICENSE-APACHE", "LICENSE_APACHE", "LICENSE-APACHE-2.0"),
    "ISC": ("LICENSE-ISC", "LICENSE_ISC", "ISC-LICENSE"),
    "BSD-2-Clause": ("LICENSE-BSD", "LICENSE-BSD-2-CLAUSE", "LICENSE_BSD"),
    "BSD-3-Clause": ("LICENSE-BSD", "LICENSE-BSD-3-CLAUSE", "LICENSE_BSD"),
    "Zlib": ("LICENSE-ZLIB", "LICENSE_ZLIB"),
    "CC0-1.0": ("LICENSE-CC0", "LICENSE_CC0", "CC0-1.0.TXT"),
    "MIT-0": ("LICENSE-MIT-0", "LICENSE_MIT_0", "MIT-0.TXT"),
    "Unicode-3.0": ("LICENSE-UNICODE", "UNICODE-LICENSE.TXT", "UNICODE-LICENSE"),
    "Unicode-DFS-2016": ("LICENSE-UNICODE-DFS-2016", "UNICODE-DFS-2016.TXT"),
    "MPL-2.0": ("LICENSE-MPL-2.0", "LICENSE-MPL", "MPL-2.0.TXT"),
    "CDLA-Permissive-2.0": ("LICENSE-CDLA-PERMISSIVE-2.0", "CDLA-PERMISSIVE-2.0.TXT"),
    "BSL-1.0": ("LICENSE-BOOST", "LICENSE-BSL", "BOOST-LICENSE"),
    "0BSD": ("LICENSE-0BSD", "0BSD-LICENSE"),
}


def body_matches_license(spdx_id: str, body: str) -> bool:
    lower = body.lower()
    if len(body.strip()) < 100:
        return False
    if spdx_id == "MIT":
        return "permission is hereby granted" in lower and "software is provided" in lower
    if spdx_id == "Apache-2.0" or spdx_id == "Apache-2.0 WITH LLVM-exception":
        return "apache license" in lower and "terms and conditions" in lower and "version 2.0" in lower
    if spdx_id == "ISC":
        return "permission to use, copy, modify, and/or distribute" in lower and "as is" in lower
    if spdx_id in {"BSD-2-Clause", "BSD-3-Clause"}:
        return "redistribution and use" in lower and "neither the name" in lower
    if spdx_id == "Zlib":
        return "this software is provided 'as-is'" in lower or "this software is provided \"as-is\"" in lower
    if spdx_id == "CC0-1.0":
        return "cc0 1.0 universal" in lower and "creative commons" in lower and "affirmer" in lower
    if spdx_id == "MIT-0":
        return "permission is hereby granted" in lower and "without restriction" in lower and "notice" not in lower[-700:]
    if spdx_id == "MPL-2.0":
        return "mozilla public license" in lower and "version 2.0" in lower
    if spdx_id == "CDLA-Permissive-2.0":
        return "community data license agreement" in lower and "permissive" in lower and "version 2.0" in lower
    if spdx_id == "Unicode-3.0":
        return "unicode license v3" in lower and "unicode, inc." in lower
    if spdx_id == "Unicode-DFS-2016":
        return "unicode data files and software license" in lower or "unicode license" in lower and "dfs" in lower
    if spdx_id == "BSL-1.0":
        return "boost software license" in lower and "version 1.0" in lower
    if spdx_id == "0BSD":
        return "permission to use, copy, modify, and/or distribute" in lower and "for any purpose" in lower
    return False


def candidate_score(path: Path, spdx_id: str, hinted_id: str, body: str) -> tuple[int, int, str]:
    if hinted_id and hinted_id != spdx_id:
        return (-1, 0, str(path))
    if not body_matches_license(spdx_id, body):
        return (-1, 0, str(path))
    base = path.name.upper()
    preferred = [name.upper() for name in LICENSE_FILENAMES.get(spdx_id, ())]
    name_score = 100 - preferred.index(base) if base in preferred else (20 if base in {"LICENSE", "LICENSE.TXT", "COPYING"} else 5)
    # Keep notices that include the package's original copyright statement.
    copyright_score = 10 if re.search(r"copyright|©", body, re.IGNORECASE) else 0
    return (name_score + copyright_score, len(body), str(path))


def resolve_license_text(crate: dict[str, Any], spdx_id: str, cargo_about_license: dict[str, Any] | None, fallback: dict[str, Any] | None) -> list[tuple[str, str, str]] | None:
    package = crate.get("package", crate)
    matches: dict[tuple[str, str], tuple[str, str, str]] = {}
    candidates = license_file_candidates(package, fallback)

    # Resolve cargo-about's source path only when it points inside this package.
    if cargo_about_license:
        source = cargo_about_license.get("source_path")
        if source:
            source_path = Path(source)
            if not source_path.is_absolute() and package.get("manifest_path"):
                source_path = Path(package["manifest_path"]).resolve().parent / source_path
            try:
                source_path = source_path.resolve()
                package_root = Path(package.get("manifest_path", ".")).resolve().parent
                if source_path.is_relative_to(package_root):
                    candidates.append({
                        "path": source_path,
                        "spdxId": spdx_id,
                        "sourceUrl": "",
                        "displayPath": f"{package['name']}@{package['version']}/{source_path.relative_to(package_root).as_posix()}",
                        "expectedSha256": "",
                        "fallback": False,
                    })
            except OSError:
                pass

    seen: set[Path] = set()
    for candidate in candidates:
        path = candidate["path"]
        try:
            resolved = path.resolve()
            if resolved in seen or not resolved.is_file():
                continue
            seen.add(resolved)
            raw = resolved.read_bytes()
            body = raw.decode("utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        actual_hash = sha256(raw)
        expected_hash = candidate.get("expectedSha256")
        if expected_hash and actual_hash != expected_hash:
            raise CollectionError(
                f"verified upstream license bytes changed for {package['name']}@{package['version']} "
                f"at {candidate.get('displayPath')}: expected {expected_hash}, found {actual_hash}"
            )
        score = candidate_score(resolved, spdx_id, candidate.get("spdxId", ""), body)
        if score[0] >= 0:
            display_path = candidate.get("displayPath") or resolved.name
            identity = (sha256(raw), display_path)
            matches[identity] = (body, display_path, candidate.get("sourceUrl", ""))

    return list(matches.values()) if matches else None


def load_fallback_ledger() -> dict[tuple[str, str], dict[str, Any]]:
    if not LEDGER_PATH.exists():
        return {}
    ledger = load_json(LEDGER_PATH)
    if ledger.get("schemaVersion") != 1 or not isinstance(ledger.get("packages"), list):
        raise CollectionError(f"fallback ledger has an unsupported schema: {LEDGER_PATH}")
    result: dict[tuple[str, str], dict[str, Any]] = {}
    for item in ledger["packages"]:
        if not item.get("verified"):
            continue
        name, version = item.get("name", ""), item.get("version", "")
        if item.get("method") not in {2, 3} or not item.get("repository") or not item.get("commit"):
            raise CollectionError(f"verified fallback {name}@{version} lacks method, repository, or source revision provenance")
        provenance_path = ROOT / "third-party/license-texts" / f"{name}-{version}" / "PROVENANCE.md"
        try:
            provenance = provenance_path.read_text(encoding="utf-8")
        except OSError as error:
            raise CollectionError(f"verified fallback {name}@{version} has no readable PROVENANCE.md: {error}") from error
        if not provenance.strip():
            raise CollectionError(f"verified fallback {name}@{version} PROVENANCE.md is empty")
        if item["commit"] not in provenance or item["repository"] not in provenance:
            raise CollectionError(f"verified fallback {name}@{version} PROVENANCE.md omits its package repository or source revision")
        if item["method"] == 3:
            evidence = item.get("repositoryEvidence", {})
            package_archive = evidence.get("crateArchive", {}).get("sha256", "")
            canonical = evidence.get("canonicalSpdxSource", {})
            canonical_url = canonical.get("sourceUrl", "")
            canonical_commit = canonical.get("commit", "")
            tree_sha = evidence.get("completeUpstreamTree", {}).get("treeSha", "")
            if (
                not package_archive or package_archive not in provenance
                or not canonical_url or canonical_url not in provenance
                or not canonical_commit or canonical_commit not in provenance
                or not tree_sha or tree_sha not in provenance
                or not item.get("annotation") or item["annotation"] not in provenance
            ):
                raise CollectionError(f"verified method-3 fallback {name}@{version} PROVENANCE.md omits canonical-source, package-archive, tree, or reconstruction evidence")
        if not item.get("files"):
            raise CollectionError(f"verified fallback {name}@{version} has no source-backed license files")
        for source_file in item["files"]:
            relative = Path(source_file.get("path", ""))
            if relative.is_absolute() or ".." in relative.parts:
                raise CollectionError(f"verified fallback {name}@{version} has an unsafe license path {relative}")
            expected_hash = source_file.get("sha256", "")
            source_url = source_file.get("sourceUrl", "")
            spdx_id = source_file.get("spdxId", "")
            if not re.fullmatch(r"[0-9a-f]{64}", expected_hash) or not source_url.startswith("https://"):
                raise CollectionError(f"verified fallback {name}@{version}/{relative} lacks a SHA-256 or upstream source URL")
            if not spdx_id or spdx_id not in item.get("selectedLicenseIds", []):
                raise CollectionError(f"verified fallback {name}@{version}/{relative} does not identify a selected SPDX branch")
            artifact_path = provenance_path.parent / relative
            try:
                actual_hash = sha256(artifact_path.read_bytes())
            except OSError as error:
                raise CollectionError(f"verified fallback {name}@{version}/{relative} is missing: {error}") from error
            if actual_hash != expected_hash:
                raise CollectionError(
                    f"verified fallback {name}@{version}/{relative} SHA-256 mismatch: expected {expected_hash}, found {actual_hash}"
                )
            if expected_hash not in provenance or source_url not in provenance:
                raise CollectionError(f"verified fallback {name}@{version}/{relative} is absent from PROVENANCE.md")
        policy_notice = item.get("repositoryEvidence", {}).get("upstreamPolicyNotice")
        if policy_notice:
            notice_path = Path(policy_notice.get("path", ""))
            if notice_path.is_absolute() or ".." in notice_path.parts:
                raise CollectionError(f"verified fallback {name}@{version} has an unsafe policy-notice path")
            notice_file = provenance_path.parent / notice_path
            expected_hash = policy_notice.get("sha256", "")
            source_url = policy_notice.get("sourceUrl", "")
            try:
                actual_hash = sha256(notice_file.read_bytes())
            except OSError as error:
                raise CollectionError(f"verified fallback {name}@{version} policy notice is missing: {error}") from error
            if actual_hash != expected_hash or not source_url.startswith("https://"):
                raise CollectionError(f"verified fallback {name}@{version} policy notice has an invalid hash or URL")
            if expected_hash not in provenance or source_url not in provenance:
                raise CollectionError(f"verified fallback {name}@{version} policy notice is absent from PROVENANCE.md")
        result[(item.get("name", ""), item.get("version", ""))] = item
    return result


def raw_crate_map(data: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    return {package_key(crate.get("package", {})): crate for crate in data.get("crates", [])}


def used_license_map(data: dict[str, Any]) -> dict[tuple[str, str], dict[str, list[dict[str, Any]]]]:
    result: dict[tuple[str, str], dict[str, list[dict[str, Any]]]] = defaultdict(lambda: defaultdict(list))
    for license_info in data.get("licenses", []):
        for use in license_info.get("used_by", []):
            crate = use.get("crate", {})
            result[package_key(crate)][license_info["id"]].append(license_info)
    return result


def resolve_sources(
    crate: dict[str, Any],
    spdx_id: str,
    license_infos: list[dict[str, Any]],
    fallback: dict[str, Any] | None,
) -> dict[tuple[str, str], tuple[str, str, str]]:
    result: dict[tuple[str, str], tuple[str, str, str]] = {}
    for license_info in license_infos or [None]:
        for item in resolve_license_text(crate, spdx_id, license_info, fallback) or []:
            body, source_path, source_url = item
            result[(sha256(body), source_path)] = item
    return result


def license_ids_in_expression(expression: str, accepted: list[str]) -> list[str]:
    found: list[str] = []
    for spdx_id in accepted:
        pattern = r"(?<![A-Za-z0-9])" + re.escape(spdx_id) + r"(?![A-Za-z0-9.-])"
        if not re.search(pattern, expression):
            continue
        if any(
            other.startswith(spdx_id + " WITH ")
            and re.search(r"(?<![A-Za-z0-9])" + re.escape(other) + r"(?![A-Za-z0-9.-])", expression)
            for other in accepted
        ):
            continue
        found.append(spdx_id)
    return found


def normalize_target_data(
    raw: dict[str, Any],
    target: str,
    metadata_packages: dict[tuple[str, str], dict[str, Any]],
    workspace_ids: set[str],
    fallbacks: dict[tuple[str, str], dict[str, Any]],
) -> dict[str, Any]:
    crates: list[dict[str, Any]] = []
    licenses: dict[tuple[str, str], dict[str, Any]] = {}
    unresolved: list[dict[str, str]] = []
    raw_crates = raw_crate_map(raw)
    used = used_license_map(raw)

    # Keep each selected branch tied to cargo-about's package reachability.
    for key, raw_crate in raw_crates.items():
        cargo_package = raw_crate.get("package", {})
        if is_workspace(raw_crate, workspace_ids):
            continue
        metadata = metadata_packages.get(key)
        if not metadata:
            unresolved.append({"crate": f"{key[0]}@{key[1]}", "reason": "target package absent from locked cargo metadata"})
            continue
        selected = used.get(key, {})
        if not selected:
            unresolved.append({"crate": f"{key[0]}@{key[1]}", "reason": "cargo-about did not resolve a selected SPDX license"})
            continue
        normalized_package = dict(metadata)
        declared_license = metadata.get("license") or raw_crate.get("license") or cargo_package.get("license")
        normalized_package.update({
            "name": key[0],
            "version": key[1],
            "license": declared_license,
        })
        crates.append({"package": normalized_package, "license": normalized_package.get("license")})

        fallback = fallbacks.get(key)
        resolved_by_id: dict[str, dict[tuple[str, str], tuple[str, str, str]]] = {}
        for spdx_id, license_infos in sorted(selected.items()):
            resolved_by_id[spdx_id] = resolve_sources(raw_crate, spdx_id, license_infos, fallback)

        # cargo-about can resolve to a stock SPDX template even when the package
        # actually ships a different complete branch. For a simple OR expression,
        # prefer a permitted branch whose full bytes exist in that package or in
        # the verified ledger; do not invent a body for the synthetic choice.
        has_or = bool(re.search(r"\bOR\b|/", declared_license or "", re.IGNORECASE))
        has_and = bool(re.search(r"\bAND\b", declared_license or "", re.IGNORECASE))
        if has_or and not has_and:
            usable = {spdx_id: sources for spdx_id, sources in resolved_by_id.items() if sources}
            if usable:
                resolved_by_id = usable
            else:
                allowed_ids = tomllib.loads(ABOUT_CONFIG.read_text(encoding="utf-8")).get("accepted", [])
                alternatives = license_ids_in_expression(declared_license or "", allowed_ids)
                for spdx_id in alternatives:
                    source_infos = selected.get(spdx_id, [])
                    sources = resolve_sources(raw_crate, spdx_id, source_infos, fallback)
                    if sources:
                        resolved_by_id = {spdx_id: sources}
                        break

        if has_or and not has_and and resolved_by_id:
            cargo_about_ids = sorted(selected)
            actual_ids = sorted(resolved_by_id)
            if cargo_about_ids != actual_ids:
                normalized_package["licenseSelection"] = (
                    f"cargo-about selected {', '.join(cargo_about_ids)} but those branches had no complete package text; "
                    f"selected {', '.join(actual_ids)} from the declared OR expression because full source-backed text was available"
                )
            else:
                normalized_package["licenseSelection"] = "cargo-about branch matched a complete package or checksum-verified license file"

        for spdx_id, resolved_bodies in sorted(resolved_by_id.items()):
            if not resolved_bodies:
                unresolved.append({"crate": f"{key[0]}@{key[1]}", "spdxId": spdx_id, "reason": "no complete, matching license file in the package or verified fallback ledger"})
                continue
            for (sha, source_path), (body, source_path, source_url) in resolved_bodies.items():
                identity = (spdx_id, sha)
                license_info = selected.get(spdx_id, [{"name": spdx_id}])[0]
                license_entry = licenses.setdefault(identity, {
                    "id": spdx_id,
                    "name": license_info.get("name") or spdx_id,
                    "text": body,
                    "source_path": source_path,
                    "used_by": [],
                })
                if license_entry["text"] != body:
                    unresolved.append({"crate": f"{key[0]}@{key[1]}", "spdxId": spdx_id, "reason": "same license identity resolved to inconsistent text bytes"})
                    continue
                repo = metadata.get("repository") or cargo_package.get("repository")
                crate_ref = {
                    "name": key[0],
                    "version": key[1],
                    "license": normalized_package["license"],
                    "repository": repo,
                    "source": metadata.get("source") or cargo_package.get("source"),
                    "id": cargo_package.get("id"),
                    "authors": metadata.get("authors", []),
                }
                if fallback and source_url:
                    crate_ref["licenseSourceUrl"] = source_url
                    crate_ref["licenseSourceCommit"] = fallback.get("commit")
                    crate_ref["licenseRecoveryMethod"] = fallback.get("method")
                    crate_ref["licenseProvenancePath"] = f"third-party/license-texts/{key[0]}-{key[1]}/PROVENANCE.md"
                if not any(
                    use["crate"].get("name") == key[0]
                    and use["crate"].get("version") == key[1]
                    and use.get("path") == source_path
                    for use in license_entry["used_by"]
                ):
                    license_entry["used_by"].append({
                        "crate": crate_ref,
                        "path": source_path,
                        "license_file_sha256": sha,
                        "source_url": source_url or None,
                    })

    return {"crates": crates, "licenses": list(licenses.values()), "unresolved": unresolved}


def merge_target_data(target_data: dict[str, dict[str, Any]]) -> dict[str, Any]:
    packages: dict[tuple[str, str], dict[str, Any]] = {}
    notices: dict[tuple[str, str], dict[str, Any]] = {}
    unresolved: list[dict[str, str]] = []
    for target, data in target_data.items():
        unresolved.extend(data.get("unresolved", []))
        raw_packages = raw_crate_map(data)
        selected_by_crate = used_license_map(data)
        for key, raw_crate in raw_packages.items():
            cargo_package = raw_crate.get("package", {})
            package = packages.setdefault(key, {
                "name": key[0],
                "version": key[1],
                "repository": cargo_package.get("repository"),
                "source": cargo_package.get("source") or "unknown source",
                "isWorkspace": False,
                "targets": [],
                "declaredLicense": raw_crate.get("license") or cargo_package.get("license"),
                "authors": cargo_package.get("authors", []),
                "selectedLicenseIds": [],
                "licenseTextSha256": [],
                "licenseSelection": cargo_package.get("licenseSelection"),
            })
            if target not in package["targets"]:
                package["targets"].append(target)
            for license_id, license_infos in selected_by_crate.get(key, {}).items():
                if license_id not in package["selectedLicenseIds"]:
                    package["selectedLicenseIds"].append(license_id)
                for license_info in license_infos:
                    actual_text = license_info.get("text")
                    if not isinstance(actual_text, str) or not actual_text.strip():
                        unresolved.append({"crate": f"{key[0]}@{key[1]}", "spdxId": license_id, "reason": "selected license has no text"})
                        continue
                    digest = sha256(actual_text)
                    if digest not in package["licenseTextSha256"]:
                        package["licenseTextSha256"].append(digest)
                    notice_key = (license_id, digest)
                    notice = notices.setdefault(notice_key, {
                        "spdxId": license_id,
                        "text": actual_text,
                        "sha256": digest,
                        "crates": [],
                        "sourcePath": license_info.get("source_path"),
                        "sources": [],
                    })
                    for use in license_info.get("used_by", []):
                        crate_ref = use.get("crate", {})
                        crate = {
                            "name": crate_ref.get("name"),
                            "version": crate_ref.get("version"),
                            "repository": crate_ref.get("repository"),
                            "targets": [target],
                        }
                        if use.get("path"):
                            crate["licensePath"] = use["path"]
                        if use.get("license_file_sha256"):
                            crate["licenseFileSha256"] = use["license_file_sha256"]
                        if use.get("source_url"):
                            crate["licenseSourceUrl"] = use["source_url"]
                        previous = next((item for item in notice["crates"] if (
                            item.get("name"), item.get("version"), item.get("repository"), item.get("licensePath")
                        ) == (
                            crate.get("name"), crate.get("version"), crate.get("repository"), crate.get("licensePath")
                        )), None)
                        if previous is None:
                            notice["crates"].append(crate)
                        elif target not in previous["targets"]:
                            previous["targets"].append(target)
                        if use.get("path") and use["path"] not in notice["sources"]:
                            notice["sources"].append(use["path"])
                    if len(notice["sources"]) > 1:
                        notice["sourcePath"] = None
        # all target records are already dependency-closure filtered by cargo-about.

    for package in packages.values():
        package["targets"].sort()
        package["selectedLicenseIds"].sort()
        package["licenseTextSha256"].sort()
        if not package.get("repository"):
            package["repository"] = None
    for notice in notices.values():
        notice["crates"].sort(key=lambda item: (item["name"], item["version"]))
        for crate in notice["crates"]:
            crate["targets"].sort()
        notice["sources"].sort()
    return {
        "packages": sorted(packages.values(), key=lambda item: (item["name"], item["version"])),
        "licenseTexts": sorted(notices.values(), key=lambda item: (item["spdxId"], item["sha256"])),
        "unresolved": unresolved,
    }


def selected_license_expression_mentions_or(expression: str | None) -> bool:
    return bool(expression and re.search(r"\bOR\b", expression))


def render_audit(data: dict[str, Any]) -> str:
    targets = data["scope"]["targets"]
    packages = data["packages"]
    lines = [
        "# Rust Third-Party License Audit",
        "",
        f"Collected with cargo-about {data['tool']['version']} using `--locked --workspace --all-features --fail` for the release targets: {', '.join(targets)}.",
        "",
        f"The target closure contains {len(packages)} distinct third-party crate versions. The three unpublished workspace crates ({', '.join(sorted(EXPECTED_WORKSPACE_IGNORED))}) are excluded by the `private = {{ ignore = true }}` cargo-about policy.",
        "",
        "Build and development dependencies are retained (`ignore-build-dependencies = false`, `ignore-dev-dependencies = false`). Cargo-about target filtering supplies the reachable graph for each triple; `cargo metadata --locked --all-features` supplies authors, repository/source identifiers, and package paths. The helper does not treat the flat metadata package list as dependency reachability.",
        "",
        "Every emitted license body is read byte-for-byte from a package license file or a checksum-verified upstream recovery file under `third-party/license-texts/`. cargo-about's synthesized SPDX fallback text is never emitted as package text. SHA-256 values pin the exact UTF-8 file bytes.",
        "",
        "## Scope by target",
        "",
        "| Target | Third-party crate versions |",
        "| --- | ---: |",
    ]
    for target in targets:
        count = sum(target in package["targets"] for package in packages)
        lines.append(f"| `{target}` | {count} |")
    lines.extend(["", "## Selected license branches", "", "`AND` expressions retain every required branch. Cargo-about branch choices are matched to complete package or checksum-verified upstream files. If cargo-about selected a synthesized SPDX body absent from the package while another declared `OR` branch has complete source text, the collector records that source-backed branch and explains the choice below. The audit output preserves the package's selected SPDX branch and source evidence.", ""])
    for package in packages:
        if selected_license_expression_mentions_or(package.get("declaredLicense")):
            lines.append(
                f"- `{package['name']}@{package['version']}` declares `{package['declaredLicense']}`; selected `{', '.join(package['selectedLicenseIds'])}` for {', '.join(package['targets'])}{'; ' + package['licenseSelection'] if package.get('licenseSelection') else ''}."
            )
    lines.extend(["", "## Provenance and text hashes", "", "| Crate | Declared expression | Selected SPDX IDs | License SHA-256 | Source |", "| --- | --- | --- | --- | --- |"])
    for package in packages:
        hashes = ", ".join(f"`{digest}`" for digest in package["licenseTextSha256"])
        source = package.get("repository") or package.get("source") or "not listed"
        lines.append(
            f"| `{package['name']}@{package['version']}` | `{package.get('declaredLicense') or 'not listed'}` | `{', '.join(package['selectedLicenseIds'])}` | {hashes} | {source} |"
        )
    lines.extend(["", "## Collected license texts", "", f"The output contains {len(data['licenseTexts'])} distinct (SPDX ID, exact-text hash) entries. The generator integrates these entries into `THIRD_PARTY_LICENSES`, preserving their copyright, source-file, and upstream repository information.", ""])
    for notice in data["licenseTexts"]:
        crate_names = ", ".join(f"{crate['name']}@{crate['version']}" for crate in notice["crates"])
        lines.append(f"- `{notice['spdxId']}` — SHA-256 `{notice['sha256']}`; applies to {crate_names}.")
    lines.extend(["", "## Supplemental upstream notices", ""])
    if data.get("supplementalNotices"):
        lines.append("These package files preserve notices and copyright attribution that are separate from selected SPDX branches.")
        for notice in data["supplementalNotices"]:
            purpose = f"; {notice['purpose']}" if notice.get("purpose") else ""
            lines.append(f"- `{notice['name']}@{notice['version']}` `{notice['path']}` ({notice['type']}; SHA-256 `{notice['sha256']}`){purpose}.")
    else:
        lines.append("No separate upstream notices or copyright files were found.")
    if data["unresolved"]:
        lines.extend(["", "## Unresolved records", ""])
        for item in data["unresolved"]:
            lines.append(f"- `{item.get('crate', 'unknown')}` {item.get('spdxId', '')}: {item['reason']}")
    else:
        lines.extend(["", "No unresolved Rust package or license-text records remain.", ""])
    return "\n".join(lines) + "\n"


def validate_allow_list(data: dict[str, Any]) -> None:
    deny = (ROOT / "deny.toml").read_text(encoding="utf-8")
    section = re.search(r"\[licenses\]([\s\S]*?)(?=\n\[[^\]]+\]|$)", deny)
    allow = re.search(r"\ballow\s*=\s*\[([\s\S]*?)\]", section.group(1) if section else "")
    if not allow:
        raise CollectionError("cannot parse deny.toml [licenses].allow list")
    expected = set(re.findall(r'"([^"]+)"', allow.group(1)))
    actual = set()
    for package in data["packages"]:
        actual.update(package["selectedLicenseIds"])
    unexpected = sorted(actual - expected)
    if unexpected:
        raise CollectionError(f"selected Rust licenses are not allowed by deny.toml: {', '.join(unexpected)}")


def validate_policy_configs() -> None:
    try:
        about = tomllib.loads(ABOUT_CONFIG.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise CollectionError(f"about.toml is invalid: {error}") from error
    deny = (ROOT / "deny.toml").read_text(encoding="utf-8")
    section = re.search(r"\[licenses\]([\s\S]*?)(?=\n\[[^\]]+\]|$)", deny)
    allow = re.search(r"\ballow\s*=\s*\[([\s\S]*?)\]", section.group(1) if section else "")
    deny_ids = set(re.findall(r'"([^"]+)"', allow.group(1) if allow else ""))
    about_ids = set(about.get("accepted", []))
    if not deny_ids or about_ids != deny_ids:
        raise CollectionError(f"about.toml accepted SPDX IDs must exactly match deny.toml: about={sorted(about_ids)}, deny={sorted(deny_ids)}")
    if about.get("targets") != TARGETS:
        raise CollectionError(f"about.toml target list must exactly match the collector targets: {about.get('targets')}")


def collect_supplemental_notices(
    packages: list[dict[str, Any]],
    metadata_packages: dict[tuple[str, str], dict[str, Any]],
    fallbacks: dict[tuple[str, str], dict[str, Any]],
    license_texts: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    selected_paths = {
        (crate.get("name"), crate.get("version"), crate.get("licensePath"))
        for notice in license_texts
        for crate in notice.get("crates", [])
    }
    result: dict[tuple[str, str, str], dict[str, Any]] = {}

    def add_notice(
        package: dict[str, Any],
        display_path: str,
        raw: bytes,
        *,
        source_url: str | None = None,
        git_blob_sha: str | None = None,
        purpose: str | None = None,
    ) -> None:
        try:
            body = raw.decode("utf-8")
        except UnicodeDecodeError:
            return
        if not body.strip() or len(raw) > 1024 * 1024:
            return
        ids = [spdx_id for spdx_id in LICENSE_FILENAMES if body_matches_license(spdx_id, body)]
        name, version = package["name"], package["version"]
        result[(name, version, display_path)] = {
            "name": name,
            "version": version,
            "repository": package.get("repository"),
            "authors": package.get("authors", []),
            "path": display_path,
            "spdxId": ids[0] if len(ids) == 1 else None,
            "type": "license text" if ids else "notice",
            "text": body,
            "sha256": sha256(raw),
            "sourceUrl": source_url,
            "gitBlobSha": git_blob_sha,
            "purpose": purpose,
        }

    for package in packages:
        name, version = package["name"], package["version"]
        metadata = metadata_packages.get((name, version), {})
        manifest = metadata.get("manifest_path")
        if manifest:
            package_root = Path(manifest).resolve().parent
            for base, dirs, filenames in os.walk(package_root):
                depth = len(Path(base).relative_to(package_root).parts)
                dirs[:] = sorted(d for d in dirs if d not in {".git", "target", "vendor"}) if depth < 6 else []
                for filename in filenames:
                    upper = filename.upper()
                    if not (
                        upper.startswith(("LICENSE", "LICENCE", "COPYING", "NOTICE"))
                        or "COPYRIGHT" in upper
                    ):
                        continue
                    path = Path(base) / filename
                    display_path = f"{name}@{version}/{path.relative_to(package_root).as_posix()}"
                    if (name, version, display_path) in selected_paths:
                        continue
                    try:
                        raw = path.read_bytes()
                    except OSError:
                        continue
                    add_notice(package, display_path, raw)

        fallback = fallbacks.get((name, version))
        policy = fallback.get("repositoryEvidence", {}).get("upstreamPolicyNotice") if fallback else None
        if policy:
            local_path = ROOT / "third-party/license-texts" / f"{name}-{version}" / policy["path"]
            display_path = (Path("third-party/license-texts") / f"{name}-{version}" / policy["path"]).as_posix()
            add_notice(
                package,
                display_path,
                local_path.read_bytes(),
                source_url=policy.get("sourceUrl"),
                git_blob_sha=policy.get("gitBlobSha"),
                purpose=policy.get("purpose"),
            )

    return sorted(result.values(), key=lambda item: (item["name"], item["version"], item["path"]))


def collect() -> dict[str, Any]:
    tool_version = cargo_about_version()
    validate_policy_configs()
    metadata = cargo_metadata()
    metadata_packages, workspace_ids = metadata_maps(metadata)
    ignored_workspace_names = {
        package["name"] for package in metadata.get("packages", []) if package.get("id") in workspace_ids
    }
    if ignored_workspace_names != EXPECTED_WORKSPACE_IGNORED:
        raise CollectionError(
            "workspace exclusion policy no longer matches the expected three crates: "
            + ", ".join(sorted(ignored_workspace_names))
        )

    fallbacks = load_fallback_ledger()
    config = temporary_about_config(metadata_packages)
    try:
        by_target = {
            target: normalize_target_data(
                cargo_about_data(config, target), target, metadata_packages, workspace_ids, fallbacks
            )
            for target in TARGETS
        }
        merged = merge_target_data(by_target)
        # Keep raw cargo-about JSON text separate from actual package-file bytes:
        # Handlebars can reflow indentation while rendering the configured union.
        about_template_texts = cargo_about_union_license_texts(config)
        # This invokes cargo-about's real handlebars renderer with the checked-in template.
        about_markdown = render_about_template(config)
    finally:
        config.unlink(missing_ok=True)

    result = {
        "schemaVersion": 1,
        "tool": {"name": "cargo-about", "version": tool_version},
        "scope": {
            "targets": TARGETS,
            "workspaceCratesIgnored": sorted(ignored_workspace_names),
            "buildDependencies": True,
            "devDependencies": True,
            "features": "all",
            "locked": True,
            "reachableTargetGraphs": True,
        },
        "packages": merged["packages"],
        "licenseTexts": merged["licenseTexts"],
        "supplementalNotices": collect_supplemental_notices(
            merged["packages"], metadata_packages, fallbacks, merged["licenseTexts"]
        ),
        "unresolved": merged["unresolved"],
        "aboutMarkdown": about_markdown,
        "aboutMarkdownTargets": TARGETS,
        "aboutTemplateLicenseTexts": about_template_texts,
    }
    validate_allow_list(result)
    return result


def write_audit_artifacts(result: dict[str, Any]) -> None:
    ARTIFACTS_DIR.mkdir(parents=True, exist_ok=True)
    (ARTIFACTS_DIR / "rust-licenses.json").write_text(
        json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    (ARTIFACTS_DIR / "RUST_LICENSE_AUDIT.md").write_text(render_audit(result), encoding="utf-8")


def main() -> int:
    try:
        result = collect()
        write_audit_artifacts(result)
        # Keep stdout a single JSON document; progress and failure diagnostics go to stderr.
        sys.stdout.write(json.dumps(result, ensure_ascii=False, separators=(",", ":")) + "\n")
        return 0
    except (CollectionError, OSError, KeyError, TypeError, ValueError) as error:
        print(f"Rust license collector: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
