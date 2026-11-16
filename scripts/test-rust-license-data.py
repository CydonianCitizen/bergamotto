import importlib.util
import pathlib
import tempfile
import unittest


SCRIPT = pathlib.Path(__file__).with_name("rust-license-data.py")
SPEC = importlib.util.spec_from_file_location("rust_license_data", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class MergeTargetDataTests(unittest.TestCase):
    def test_merges_same_license_text_and_groups_crate_versions(self):
        first = {
            "crates": [
                {
                    "package": {
                        "name": "sample-crate",
                        "version": "1.0.0",
                        "license": "MIT OR Apache-2.0",
                        "repository": "https://example.invalid/sample-crate",
                        "source": "registry+https://github.com/rust-lang/crates.io-index",
                        "id": "registry+sample-crate@1.0.0",
                        "authors": ["Example Author"],
                    },
                    "license": "MIT OR Apache-2.0",
                }
            ],
            "licenses": [
                {
                    "id": "MIT",
                    "name": "MIT License",
                    "text": "Copyright Example Author\n\nMIT terms\n",
                    "source_path": "/tmp/LICENSE-MIT",
                    "used_by": [
                        {
                            "crate": {
                                "name": "sample-crate",
                                "version": "1.0.0",
                                "license": "MIT OR Apache-2.0",
                                "repository": "https://example.invalid/sample-crate",
                                "source": "registry+https://github.com/rust-lang/crates.io-index",
                                "id": "registry+sample-crate@1.0.0",
                                "authors": ["Example Author"],
                            },
                            "path": None,
                        }
                    ],
                }
            ],
        }
        second = {
            "crates": [
                {
                    "package": {
                        "name": "sample-crate",
                        "version": "1.1.0",
                        "license": "MIT OR Apache-2.0",
                        "repository": "https://example.invalid/sample-crate",
                        "source": "registry+https://github.com/rust-lang/crates.io-index",
                        "id": "registry+sample-crate@1.1.0",
                        "authors": ["Example Author"],
                    },
                    "license": "MIT OR Apache-2.0",
                }
            ],
            "licenses": [
                {
                    "id": "MIT",
                    "name": "MIT License",
                    "text": "Copyright Example Author\n\nMIT terms\n",
                    "source_path": "/tmp/LICENSE-MIT",
                    "used_by": [
                        {
                            "crate": {
                                "name": "sample-crate",
                                "version": "1.1.0",
                                "license": "MIT OR Apache-2.0",
                                "repository": "https://example.invalid/sample-crate",
                                "source": "registry+https://github.com/rust-lang/crates.io-index",
                                "id": "registry+sample-crate@1.1.0",
                                "authors": ["Example Author"],
                            },
                            "path": None,
                        }
                    ],
                }
            ],
        }

        result = MODULE.merge_target_data({"target-a": first, "target-b": second})

        self.assertEqual(len(result["licenseTexts"]), 1)
        self.assertEqual(
            result["licenseTexts"][0]["crates"],
            [
                {
                    "name": "sample-crate",
                    "version": "1.0.0",
                    "repository": "https://example.invalid/sample-crate",
                    "targets": ["target-a"],
                },
                {
                    "name": "sample-crate",
                    "version": "1.1.0",
                    "repository": "https://example.invalid/sample-crate",
                    "targets": ["target-b"],
                },
            ],
        )
        self.assertEqual(len(result["packages"]), 2)
        self.assertEqual(result["packages"][0]["selectedLicenseIds"], ["MIT"])
        self.assertEqual(result["packages"][1]["selectedLicenseIds"], ["MIT"])

    def test_preserves_multiple_source_files_for_one_selected_spdx_branch(self):
        crate = {
            "name": "ring",
            "version": "0.17.14",
            "license": "Apache-2.0 AND ISC",
            "repository": "https://example.invalid/ring",
            "source": "registry+https://github.com/rust-lang/crates.io-index",
            "id": "registry+ring@0.17.14",
            "authors": ["Example Author"],
        }
        data = {
            "crates": [{"package": crate, "license": crate["license"]}],
            "licenses": [
                {
                    "id": "Apache-2.0",
                    "name": "Apache License 2.0",
                    "text": "Copyright One\nApache License body from BoringSSL\n",
                    "source_path": "ring@0.17.14/LICENSE-BoringSSL",
                    "used_by": [{"crate": crate, "path": "ring@0.17.14/LICENSE-BoringSSL", "license_file_sha256": "a" * 64}],
                },
                {
                    "id": "Apache-2.0",
                    "name": "Apache License 2.0",
                    "text": "Copyright Two\nApache License body from once_cell\n",
                    "source_path": "ring@0.17.14/src/polyfill/once_cell/LICENSE-APACHE",
                    "used_by": [{"crate": crate, "path": "ring@0.17.14/src/polyfill/once_cell/LICENSE-APACHE", "license_file_sha256": "b" * 64}],
                },
            ],
        }

        result = MODULE.merge_target_data({"target-a": data})

        self.assertEqual(result["packages"][0]["selectedLicenseIds"], ["Apache-2.0"])
        self.assertEqual(len(result["licenseTexts"]), 2)
        self.assertEqual(
            {crate["licensePath"] for notice in result["licenseTexts"] for crate in notice["crates"]},
            {
                "ring@0.17.14/LICENSE-BoringSSL",
                "ring@0.17.14/src/polyfill/once_cell/LICENSE-APACHE",
            },
        )


class SourceLicenseResolutionTests(unittest.TestCase):
    def test_full_package_branch_replaces_synthetic_cargo_about_or_choice(self):
        bsd_text = (
            "Copyright 2016 Example Authors. All rights reserved.\n\n"
            "Redistribution and use in source and binary forms, with or without modification, are permitted.\n"
            "Neither the name of the copyright holder nor the names of its contributors may be used to endorse.\n"
            "THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS AS IS.\n"
            "IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY DAMAGES.\n"
        )
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="sample"\nversion="1.0.0"\n')
            (root / "LICENSE").write_text(bsd_text)
            package = {
                "name": "sample",
                "version": "1.0.0",
                "license": "BSD-3-Clause/MIT",
                "repository": "https://example.invalid/sample",
                "source": "registry+https://github.com/rust-lang/crates.io-index",
                "id": "registry+sample@1.0.0",
                "authors": ["Example Author"],
                "manifest_path": str(manifest),
            }
            raw = {
                "crates": [{"package": {**package, "license": "MIT"}, "license": "MIT"}],
                "licenses": [{
                    "id": "MIT",
                    "name": "MIT License",
                    "text": "synthetic SPDX MIT fallback that must not be emitted",
                    "source_path": None,
                    "used_by": [{"crate": {**package, "license": "MIT"}, "path": None}],
                }],
            }

            result = MODULE.normalize_target_data(
                raw,
                "target-a",
                {("sample", "1.0.0"): package},
                set(),
                {},
            )

        self.assertEqual(result["unresolved"], [])
        self.assertEqual([license["id"] for license in result["licenses"]], ["BSD-3-Clause"])
        self.assertEqual(result["licenses"][0]["text"], bsd_text)
        self.assertIn("selected BSD-3-Clause", result["crates"][0]["package"]["licenseSelection"])

    def test_cc0_classifier_accepts_package_cc0_without_optional_spdx_preamble(self):
        cc0_text = (
            "Creative Commons CC0 1.0 Universal\n\n"
            "CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE LEGAL SERVICES.\n"
            "The person associated with this work hereby dedicates the work to the public domain.\n"
            "Affirmer understands that the work is provided as-is without warranties.\n"
        )
        self.assertTrue(MODULE.body_matches_license("CC0-1.0", cc0_text))
        self.assertEqual(
            MODULE.license_ids_in_expression("BSD-3-Clause/MIT", ["MIT", "BSD-3-Clause"]),
            ["MIT", "BSD-3-Clause"],
        )


class AboutTemplateInputTests(unittest.TestCase):
    def test_exposes_deduplicated_raw_union_text_without_source_paths(self):
        raw = {
            "licenses": [
                {"id": "MIT", "text": "Raw cargo-about MIT text\n", "source_path": "/machine/a/LICENSE"},
                {"id": "MIT", "text": "Raw cargo-about MIT text\n", "source_path": "/machine/b/LICENSE"},
                {"id": "Apache-2.0", "text": "Raw Apache text\n", "source_path": None},
            ]
        }

        result = MODULE.about_template_license_texts(raw)

        self.assertEqual([(item["spdxId"], item["text"]) for item in result], [
            ("Apache-2.0", "Raw Apache text\n"),
            ("MIT", "Raw cargo-about MIT text\n"),
        ])
        self.assertEqual(
            result[1]["sha256"], MODULE.sha256("Raw cargo-about MIT text\n")
        )
        self.assertNotIn("source_path", result[0])
        self.assertNotIn("source_path", result[1])


if __name__ == "__main__":
    unittest.main()
