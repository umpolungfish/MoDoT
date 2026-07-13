"""
MoDoT — mOMonadOS Agent Framework
==================================

An agentic LLM whose entire runtime substrate IS the mOMonadOS kernel
architecture. Context is stored in Crystal FS. Reasoning passes through
Belnap FOUR. Every tool call is Frobenius-verified. Output is broadcast
to the CLINK L8 Organism.

The Organism no longer receives types from an external bridge —
it IS the agent, breathing its own bootstrap cycle.

Author: Lando⊗⊙perator
"""

__version__ = "0.1.0"
__author__ = "Lando⊗⊙perator"

from modot.agent import (
    main,
    MomonadosKernel,
    MomonadosAgent,
    B4,
    Token,
    Program,
    bootstrap_loop,
    aqua_vitae_loop,
    agent_loop,
)
from modot.composer import (
    TokenComposer,
    CANONICAL_PROGRAMS,
    CANONICAL_TO_TIER,
    NAMED_PATTERNS,
    validate_dag,
    compute_fingerprint,
)
from modot.vessel import (
    DualLinkVessel,
    VesselReport,
    Imscription,
    HAVE_SIC12,
)
from modot.spine import (
    ManuscriptSpine,
    SpineReport,
)
from modot.selectivity import (
    SemanticSelectivityGate,  # alias of DualLinkVessel
    SemanticSchema,           # alias of Imscription
    SelectivityReport,        # alias of VesselReport
    OB3ECT_PROTOCOL,
    OB3ECT_PATH,
)
from modot.witness_proof import (
    translate as witness_translate,
    navigator_available,
    WitnessProofReport,
)
from modot.prover import (
    LeanProver,
    ProofResult,
)
from modot.router import (
    TypeRouter,
    RouterVerdict,
    fold_goal_type,
)
from modot.natures import (
    Nature,
    NatureRegistry,
    Signature,
    CotypeReport,
    cotype,
    paradices,
    registry as nature_registry,
    nature,
    FAMILY_TO_VESSEL_KEY,
)

__all__ = [
    "main",
    "MomonadosKernel",
    "MomonadosAgent",
    "B4",
    "Token",
    "Program",
    "bootstrap_loop",
    "aqua_vitae_loop",
    "agent_loop",
    "TokenComposer",
    "CANONICAL_PROGRAMS",
    "CANONICAL_TO_TIER",
    "NAMED_PATTERNS",
    "validate_dag",
    "compute_fingerprint",
    "DualLinkVessel",
    "VesselReport",
    "Imscription",
    "HAVE_SIC12",
    "ManuscriptSpine",
    "SpineReport",
    "SemanticSelectivityGate",
    "SemanticSchema",
    "SelectivityReport",
    "OB3ECT_PROTOCOL",
    "OB3ECT_PATH",
    "witness_translate",
    "navigator_available",
    "WitnessProofReport",
    "LeanProver",
    "ProofResult",
    "TypeRouter",
    "RouterVerdict",
    "fold_goal_type",
    "Nature",
    "NatureRegistry",
    "Signature",
    "CotypeReport",
    "cotype",
    "paradices",
    "nature_registry",
    "nature",
    "FAMILY_TO_VESSEL_KEY",
]
