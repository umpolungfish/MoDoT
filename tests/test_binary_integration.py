"""Exercise the installed native interface and Python bridge without inference."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("ig_tools", ROOT / "modot/ig_tools.py")
BRIDGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRIDGE)
SOURCE = "217728509307789974617049079437290125283"


class BinaryIntegration(unittest.TestCase):
    def native(self, *args, ok=True):
        result = subprocess.run([str(ROOT / "ask"), *args], cwd=ROOT.parent,
                                capture_output=True, text=True, timeout=30)
        self.assertEqual(result.returncode == 0, ok, result.stderr)
        return result.stdout + result.stderr

    def test_native_backends_and_codec_roundtrip(self):
        self.assertIn("help", self.native("--tool", "gm", "help"))
        self.assertIn("vox", self.native("--tool", "vox", "help"))
        encoded = self.native("--tool", "godel_encode", SOURCE)
        word = next(line.split(None, 1)[1] for line in encoded.splitlines()
                    if line.startswith("word "))
        self.assertIn(SOURCE, self.native("--tool", "godel_decode", word))
        self.assertIn("help", self.native("--kernel", "help"))

    def test_python_backends_and_dict_calls(self):
        for name in ("gm", "vox", "godel"):
            result = BRIDGE.call(name, {"command": "help"})
            self.assertEqual(result["status"], "ok", result)
            self.assertTrue(result["stdout"] or result["stderr"])
        result = BRIDGE.call("godel_encode", {"n": SOURCE})
        self.assertEqual(result["status"], "ok", result)
        word = next(line.split(None, 1)[1] for line in result["stdout"].splitlines()
                    if line.startswith("word "))
        decoded = BRIDGE.call("godel_decode", {"word": word})
        self.assertIn(SOURCE, decoded["stdout"])

    def test_child_failures_and_argument_boundaries(self):
        self.native("--tool", "godel", "missing-command", ok=False)
        result = BRIDGE.parse_and_call("godel missing-command")
        self.assertEqual(result["status"], "error")
        self.assertNotEqual(result["returncode"], 0)
        # A single argument containing spaces must arrive as one argument.
        direct = BRIDGE.call_godel("encode", ["invalid value"])
        cli = subprocess.run(["python3", "-m", "modot.ig_tools", "call",
                              "godel", "encode", "invalid value"], cwd=ROOT,
                             capture_output=True, text=True, timeout=30)
        self.assertNotEqual(cli.returncode, 0)
        self.assertEqual(json.loads(cli.stdout)["stderr"], direct["stderr"])

    def test_native_overrides_preserve_opaque_arguments(self):
        fixture = str(ROOT / "tests/argv_backend.py")
        args = ["phase", "(a,b)", "a value with spaces", "--flag", "⊢⊙⊣"]
        for tool, key in [("gm", "GMOMONADOS_BIN"), ("vox", "VOX_BIN"),
                          ("godel", "GODEL_BIN"), ("m3", "M3IOSIS_BIN")]:
            with patch.dict(os.environ, {key: fixture}):
                output = self.native("--tool", tool, *args)
                self.assertEqual(json.loads(output), args)
        with patch.dict(os.environ, {"GMOMONADOS_BIN": fixture,
                                     "MODO_TEST_BACKEND_EXIT": "7"}):
            self.native("--tool", "gm", "help", ok=False)
            self.native("--kernel", "help", ok=False)

    def test_m3iosis_computation_aliases_and_errors(self):
        for tool in ("m3", "m3iosis"):
            output = self.native("--tool", tool, "fib", "--fusion", "tau", "tau")
            self.assertIn("tau x tau = 1 + tau", output)
            result = BRIDGE.call(tool, {"command": "fib", "args": ["--fusion", "tau", "tau"]})
            self.assertEqual(result["status"], "ok", result)
            self.assertIn("tau x tau = 1 + tau", result["stdout"])
        self.assertIn("Meta-Mathematical", self.native("--tool", "m3"))
        self.native("--tool", "m3", "missing-command", ok=False)
        result = BRIDGE.parse_and_call("m3 missing-command")
        self.assertEqual(result["status"], "error")
        self.assertNotEqual(result["returncode"], 0)
        fixture = str(ROOT / "tests/argv_backend.py")
        with patch.dict(os.environ, {"M3IOSIS_BIN": fixture}):
            result = BRIDGE.call("m3", {"command": "sim", "args": ["--word", "-1", "a value with spaces"]})
            self.assertEqual(json.loads(result["stdout"]), ["sim", "--word", "-1", "a value with spaces"])


if __name__ == "__main__":
    unittest.main()
