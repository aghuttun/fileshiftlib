"""Smoke tests for the Python bindings public surface.

These tests verify that the main symbols exposed by the extension module are
available and importable in a local environment.
"""

import unittest

import fileshiftlib


class SmokeTests(unittest.TestCase):
    """Validate minimal public API symbols exposed by the package."""

    def test_public_api_symbols_exist(self):
        """Ensure Python consumers can access the core `SFTP` class."""
        self.assertTrue(hasattr(fileshiftlib, "SFTP"))

    def test_sftp_methods_exist(self):
        """Ensure expected SFTP methods are available on the binding class."""
        required_methods = [
            "reconnect",
            "is_connected",
            "list_dir",
            "change_dir",
            "delete_file",
            "download_file",
            "upload_file",
        ]

        for method_name in required_methods:
            self.assertTrue(hasattr(fileshiftlib.SFTP, method_name), msg=f"Missing method: {method_name}")


if __name__ == "__main__":
    unittest.main()
