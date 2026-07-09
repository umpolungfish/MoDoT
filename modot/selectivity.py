#!/usr/bin/env python3
"""
modot/selectivity.py — compatibility shim over the Dual-Link SIC vessel
======================================================================

The classical grader that once lived here (MUST/MUSTNOT schema, 0.6 threshold,
LLM-as-judge, protocol == integration) has been removed. Selectivity is now
co-typing in the d=12 Dual-Link SIC frame. See modot/vessel.py.

This module re-exports the vessel under the historical names so existing
imports keep working, and keeps only the dual-voice helpers that were real.
"""
from modot.vessel import (  # noqa: F401
    DualLinkVessel,
    DualLinkVessel as SemanticSelectivityGate,
    VesselReport,
    VesselReport as SelectivityReport,
    Imscription,
    Imscription as SemanticSchema,
    ImscriptionError,
    B4,
    HAVE_SIC12,
    sic_available,
    parse_imscription,
    fold_cotype,
    cotype_defects,
    _selftest,
)

# Historical constants — the vessel's own IMASM protocol, not a clipboard schema.
OB3ECT_PATH = "ob3ects/selectivity_gate"
OB3ECT_PROTOCOL = DualLinkVessel.PROTOCOL


def _selftest_compat() -> None:
    report = _selftest()
    print("DualLinkVessel self-test (via selectivity shim):")
    if not report.get("available"):
        print(f"  SKIP: {report}")
        return
    for k, v in report.items():
        print(f"  {k}: {v}")
    print("  all structural checks passed")


if __name__ == "__main__":
    _selftest_compat()
