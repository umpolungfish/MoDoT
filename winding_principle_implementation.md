# Winding Principle: Functional Two-Projection System

**Grammar tuple**: ⟨𐑼𐑰𐑽𐑬𐑞𐑧𐑔𐑠𐑢𐑖𐑙𐑭⟩
**IMASM word**: ⊢⊣><⋈⊤∈∋⊙⊥⊞◻

## Kernel Instrument Results (via run_hosted_cmds.sh)

### weight
- word: ⊢⊣⋈⊤∈∋⊙⊥⊞◻ (10 tokens, `>` and `<` consumed as WORK opcodes)
- movement:
  - ⊢ CLEAR loses 0 (0 banked in frames)
  - ⊤ deposit T into depth 0
  - ∈ open frame at depth 1
  - ∋ fuse restores 0
  - ⊥ deposit F into depth 0
  - ⊞ deposit t+f into depth 0
- final: A
- surviving: T×1, F×1, t×1, f×1
- deposits 3, cleared 0, restored 0, seeded 0, inert 0

### banked
- VACUOUS — no clear ever fired against a live register
- 3 deposit(s), 0 step(s) inert after a fixation

### cycle
- period: 10
- PHASE-BEARING: 4 distinct landings
- landing register by cut:
  - k=0..6: A
  - k=7: Ftf
  - k=8: tf
  - k=9: T

### trans
- ring transitions: 10
- linear would give: 9 (drops the closing edge)
- closing edge: ◻ -> ⊢
- transitions:
  ⊢→⊣→⋈→⊤→∈→∋→⊙→⊥→⊞→◻→⊢ (each 1 occurrence)

## Two-Projection System Implementation

### Edge-on Observer: θ = ωx
ω = exp(2πi/10), θ(x) = ω^x
Maps each cut k to landing register through complex rotation phase.

### End-on Observer: exp(2πix)
exp(2πi·x/10)
Real part = end-on measurement (shadow projection).
Imaginary part = orthogonal component.

### Composition Law with Carry Correction
(x1 + x2) mod 10, carry = ⌊(x1+x2)/10⌋

### Closure Checks
- Edge-on: ω^10 = 1, |ω^10 - 1| ≈ 4.48e-16 ✓
- End-on: Re(exp(2πi)) = 1.0, Im(exp(2πi)) = 0.0 ✓
- Both projections closed: True

## Python Implementation
```python
import math

PERIOD = 10
omega = complex(math.cos(2*math.pi/10), math.sin(2*math.pi/10))

def edge_on_observer(x):
    return omega ** (x % PERIOD)

def end_on_observer(x):
    angle = 2 * math.pi * (x % PERIOD) / PERIOD
    return {
        'real': math.cos(angle),
        'imag': math.sin(angle),
        'complex': complex(math.cos(angle), math.sin(angle))
    }

def compose(x1, x2):
    raw_sum = x1 + x2
    carry = raw_sum // PERIOD
    result = raw_sum % PERIOD
    return {'result': result, 'carry': carry}

def check_closure():
    omega_power = omega ** PERIOD
    edge_closed = abs(omega_power - 1) < 1e-10
    obs = end_on_observer(PERIOD)
    end_closed = abs(obs['real'] - 1) < 1e-10 and abs(obs['imag']) < 1e-10
    return edge_closed and end_closed
```

## Verification
- Frobenius: B4=T (closed)
- Period 10 matches Fibonacci phase lattice ("tenths of a winding")
- 4 distinct landings correspond to A→Ftf→tf→T across cuts k=0..9
- 10 ring transitions with closing edge ◻→⊢ match the ROTAT orbit boundary
