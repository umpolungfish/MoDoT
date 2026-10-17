#!/usr/bin/env python3
"""Executable argument/exit fixture for the local binary bridge."""
import json
import os
import sys

print(json.dumps(sys.argv[1:], ensure_ascii=False))
sys.exit(int(os.environ.get("MODO_TEST_BACKEND_EXIT", "0")))
