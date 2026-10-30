#!/usr/bin/env python3
"""
Backward-compatibility shim for mOMonadOS Agent.

The agent now lives in the `modot` package.
Install with:  pip install -e .
Then run with:  modot --interactive

Or continue using this shim as before:
  python3 momonados_agent.py --interactive
"""

if __name__ == "__main__":
    from modot.agent import main
    main()
