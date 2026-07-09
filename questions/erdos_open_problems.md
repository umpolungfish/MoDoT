# Erdős Problems: Complete List of All Open Problems

**Generated on:** 2026-07-09 00:37:30 UTC

**Primary Sources:**
- Website: [https://www.erdosproblems.com](https://www.erdosproblems.com) — Community database of Paul Erdős's problems maintained by Thomas Bloom.
- Structured Data: [https://github.com/teorth/erdosproblems](https://github.com/teorth/erdosproblems) `data/problems.yaml` (community-maintained ground truth metadata).

## Statistics

- **Total problems** in the database: **1217**
- **Solved problems** (per homepage): **558** (46%)
- **Unresolved problems**: 659
- **Problems marked 'open'** in the structured database: **616**

> **Note on 'Open' status:** The 'open' designation in the database reflects problems for which the site maintainer is not aware of a published solution. It does not guarantee the problem is unsolved in the mathematical literature; some 'open' problems may have been resolved in obscure papers. The site encourages literature searches. Other unresolved statuses (e.g., 'verifiable', 'falsifiable', 'independent') exist but are fewer in number.

## Methodology

This document was compiled by:
1. Retrieving the complete `problems.yaml` from the GitHub repository (containing structured metadata for all 1217 problems).
2. Filtering for entries where `status.state == 'open'`.
3. Enriching with page structure understanding from direct scraping of sample problem pages on erdosproblems.com.
4. Providing direct hyperlinks to each problem's full page, which contains the mathematical statement, extensive commentary, references, history, OEIS links, and discussion forum.

**Full problem statements are not duplicated here** to keep the document focused and of manageable size; they are best viewed on the official site where math is properly rendered and context (references, updates) is complete. Sample full extraction is provided below for illustration.

## Sample: Full Problem Extraction (Problem #1)

**Title/Heading:** 1 | Erdős Problems

**Prize:** $500

**Problem Statement:**

> If $A\subseteq \{1,\ldots,N\}$ with $\lvert A\rvert=n$ is such that the subset sums $\sum_{a\in S}a$ are distinct for all $S\subseteq A$ then
> \[N \gg 2^{n}.\]

**Status:** OPEN (This is open, and cannot be resolved with a finite computation.)

**Last edited on site:** 06 April 2026

**Key Commentary & History (excerpted):**

Erdős called this 'perhaps my first serious problem' (dating to 1931). The powers of 2 show that $2^n$ would be best possible (upper bound $N \leq 2^{n-1}$). Improved by Conway and Guy to $N \leq 2^{n-2}$ for large n. Best known upper bound $N \leq 0.22002 \cdot 2^n$ due to Bohman.

Trivial lower bound $N \gg 2^n / n$. Erdős and Moser proved $N \geq (1/4 - o(1)) 2^n / \sqrt{n}$. Current best constant $\sqrt{2/\pi}$ achieved by Dubroff, Fox, and Xu, who prove the exact bound involving binomial coefficient.

Equivalent formulation: maximal size $F(x)$ of dissociated set in [1,x] satisfies $F(x) < \log_2 x + O(1)$.

Generalization to reals with subset sums differing by at least 1 also proposed.

Appears in Erdős-Spencer book 'The kitchen sink'. Sequence of minimal N: [A276661](https://oeis.org/A276661) in OEIS.

See also Problem 350.

**References:** Many, including Er56, Er57, ..., DFX21, Gu04, etc. (full list on site).

**History note:** The open status reflects the current belief of the website owner. There may be literature solving it of which the maintainer is unaware. Please search literature yourself.

---

## All Open Problems (616 total)

Problems are sorted by number. Click the link for the **full statement, detailed commentary, references, and updates** on the official site.

| # | Prize | Tags | OEIS | Formalized (Lean) | Last Update | Link |
|---|-------|------|------|-------------------|-------------|------|
| 1 | $500 | number theory, additive combinatorics | A276661 | Yes | 2025-08-31 | [#1](https://www.erdosproblems.com/1) |
| 3 | $5000 | number theory, additive combinatorics, arithmetic progressions | A003002, A003003, A003004, A003005 | Yes | 2025-08-31 | [#3](https://www.erdosproblems.com/3) |
| 5 | None | number theory, primes | A001223 | No | 2025-08-31 | [#5](https://www.erdosproblems.com/5) |
| 9 | None | number theory, additive basis, primes | A006286 | Yes | 2025-08-31 | [#9](https://www.erdosproblems.com/9) |
| 10 | None | number theory, additive basis, primes | A387053 | Yes | 2025-08-31 | [#10](https://www.erdosproblems.com/10) |
| 11 | None | number theory, additive basis | A001220, A377587 | Yes | 2026-03-14 | [#11](https://www.erdosproblems.com/11) |
| 12 | None | number theory | — | Yes | 2025-08-31 | [#12](https://www.erdosproblems.com/12) |
| 14 | None | number theory, sidon sets, additive combinatorics | A143824, possible | Yes | 2025-08-31 | [#14](https://www.erdosproblems.com/14) |
| 15 | None | number theory, primes | — | Yes | 2025-08-31 | [#15](https://www.erdosproblems.com/15) |
| 17 | None | number theory, primes | A038133 | Yes | 2025-08-31 | [#17](https://www.erdosproblems.com/17) |
| 18 | None | number theory, divisors, factorials | A005153 | Yes | 2025-08-31 | [#18](https://www.erdosproblems.com/18) |
| 20 | $1000 | combinatorics | A332077 | Yes | 2025-08-31 | [#20](https://www.erdosproblems.com/20) |
| 25 | None | number theory | — | Yes | 2025-08-31 | [#25](https://www.erdosproblems.com/25) |
| 28 | $500 | number theory, additive basis | — | Yes | 2025-08-31 | [#28](https://www.erdosproblems.com/28) |
| 30 | $1000 | number theory, sidon sets, additive combinatorics | A143824, A227590, A003022 | Yes | 2025-08-31 | [#30](https://www.erdosproblems.com/30) |
| 32 | None | number theory, additive basis | — | Yes | 2025-08-31 | [#32](https://www.erdosproblems.com/32) |
| 33 | None | number theory, additive basis | — | Yes | 2025-08-31 | [#33](https://www.erdosproblems.com/33) |
| 36 | None | number theory, additive combinatorics | A393584, possible | Yes | 2025-08-31 | [#36](https://www.erdosproblems.com/36) |
| 39 | $500 | number theory, sidon sets, additive combinatorics | — | Yes | 2025-08-31 | [#39](https://www.erdosproblems.com/39) |
| 40 | $500 | number theory, additive basis | — | Yes | 2025-08-31 | [#40](https://www.erdosproblems.com/40) |
| 41 | $500 | number theory, sidon sets, additive combinatorics | — | Yes | 2025-08-31 | [#41](https://www.erdosproblems.com/41) |
| 44 | None | number theory, sidon sets, additive combinatorics | — | Yes | 2025-08-31 | [#44](https://www.erdosproblems.com/44) |
| 50 | $250 | number theory | — | Yes | 2025-08-31 | [#50](https://www.erdosproblems.com/50) |
| 51 | None | number theory | A002202, A014197 | Yes | 2025-08-31 | [#51](https://www.erdosproblems.com/51) |
| 52 | $250 | number theory, additive combinatorics | A263996 | Yes | 2026-05-28 | [#52](https://www.erdosproblems.com/52) |
| 60 | None | graph theory, cycles | A006855 | No | 2025-08-31 | [#60](https://www.erdosproblems.com/60) |
| 61 | None | graph theory | — | Yes | 2025-08-31 | [#61](https://www.erdosproblems.com/61) |
| 62 | None | graph theory | — | No | 2025-08-31 | [#62](https://www.erdosproblems.com/62) |
| 65 | None | graph theory, cycles | — | No | 2025-08-31 | [#65](https://www.erdosproblems.com/65) |
| 66 | $500 | number theory, additive basis | — | Yes | 2025-08-31 | [#66](https://www.erdosproblems.com/66) |
| 68 | None | number theory, irrationality | A331373 | Yes | 2025-08-31 | [#68](https://www.erdosproblems.com/68) |
| 70 | None | graph theory, ramsey theory, set theory | — | Yes | 2025-08-31 | [#70](https://www.erdosproblems.com/70) |
| 74 | $500 | graph theory, chromatic number, cycles | — | Yes | 2025-08-31 | [#74](https://www.erdosproblems.com/74) |
| 75 | None | graph theory, chromatic number | — | Yes | 2025-08-31 | [#75](https://www.erdosproblems.com/75) |
| 77 | $250 | graph theory, ramsey theory | A059442 | No | 2025-08-31 | [#77](https://www.erdosproblems.com/77) |
| 78 | $100 | graph theory, ramsey theory | A059442 | No | 2025-08-31 | [#78](https://www.erdosproblems.com/78) |
| 80 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#80](https://www.erdosproblems.com/80) |
| 81 | None | graph theory | possible | No | 2025-08-31 | [#81](https://www.erdosproblems.com/81) |
| 82 | None | graph theory | A120414, A390256, A390257, A390919, A392636, A394400, A394462, A394539, A394563, A394564, A394573, A394574, A394930, A394933 | Yes | 2025-08-31 | [#82](https://www.erdosproblems.com/82) |
| 84 | None | graph theory, cycles | possible | No | 2025-08-31 | [#84](https://www.erdosproblems.com/84) |
| 85 | None | graph theory | A006672, possible | Yes | 2026-03-14 | [#85](https://www.erdosproblems.com/85) |
| 86 | $100 | graph theory | A245762 | No | 2025-08-31 | [#86](https://www.erdosproblems.com/86) |
| 87 | None | graph theory, ramsey theory | A059442, possible | No | 2025-08-31 | [#87](https://www.erdosproblems.com/87) |
| 89 | $500 | geometry, distances | A186704, A131628 | Yes | 2025-08-31 | [#89](https://www.erdosproblems.com/89) |
| 91 | None | geometry, distances | A186704, possible | Yes | 2025-08-31 | [#91](https://www.erdosproblems.com/91) |
| 96 | None | geometry, distances, convex | possible | Yes | 2025-08-31 | [#96](https://www.erdosproblems.com/96) |
| 98 | None | geometry, distances | possible | Yes | 2025-08-31 | [#98](https://www.erdosproblems.com/98) |
| 99 | $100 | geometry, distances | — | Yes | 2025-08-31 | [#99](https://www.erdosproblems.com/99) |
| 100 | None | geometry, distances | — | Yes | 2025-08-31 | [#100](https://www.erdosproblems.com/100) |
| 101 | $100 | geometry | A006065, possible | Yes | 2025-08-31 | [#101](https://www.erdosproblems.com/101) |
| 102 | None | geometry | — | No | 2025-08-31 | [#102](https://www.erdosproblems.com/102) |
| 103 | None | geometry, distances | possible | No | 2025-08-31 | [#103](https://www.erdosproblems.com/103) |
| 104 | $100 | geometry | A003829 | No | 2025-08-31 | [#104](https://www.erdosproblems.com/104) |
| 108 | None | graph theory, chromatic number, cycles | possible | Yes | 2025-08-31 | [#108](https://www.erdosproblems.com/108) |
| 111 | None | graph theory, chromatic number, set theory | — | No | 2025-08-31 | [#111](https://www.erdosproblems.com/111) |
| 112 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#112](https://www.erdosproblems.com/112) |
| 117 | None | group theory | possible | No | 2025-08-31 | [#117](https://www.erdosproblems.com/117) |
| 119 | $100 | analysis, polynomials | — | Yes | 2025-08-31 | [#119](https://www.erdosproblems.com/119) |
| 120 | $100 | combinatorics | — | Yes | 2025-08-31 | [#120](https://www.erdosproblems.com/120) |
| 122 | None | number theory | — | No | 2025-08-31 | [#122](https://www.erdosproblems.com/122) |
| 123 | $250 | number theory | — | Yes | 2025-08-31 | [#123](https://www.erdosproblems.com/123) |
| 124 | None | number theory, base representations | — | Yes | 2025-08-31 | [#124](https://www.erdosproblems.com/124) |
| 126 | $250 | number theory | possible | Yes | 2025-08-31 | [#126](https://www.erdosproblems.com/126) |
| 129 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#129](https://www.erdosproblems.com/129) |
| 130 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#130](https://www.erdosproblems.com/130) |
| 131 | None | number theory | A068063 | No | 2025-08-31 | [#131](https://www.erdosproblems.com/131) |
| 132 | $100 | distances | — | No | 2025-08-31 | [#132](https://www.erdosproblems.com/132) |
| 137 | None | number theory | — | Yes | 2025-08-31 | [#137](https://www.erdosproblems.com/137) |
| 138 | $500 | additive combinatorics | A005346 | Yes | 2025-08-31 | [#138](https://www.erdosproblems.com/138) |
| 141 | None | additive combinatorics, primes, arithmetic progressions | A006560 | Yes | 2025-08-31 | [#141](https://www.erdosproblems.com/141) |
| 142 | $10000 | additive combinatorics, arithmetic progressions | A003002, A003003, A003004, A003005 | Yes | 2025-08-31 | [#142](https://www.erdosproblems.com/142) |
| 143 | $500 | primitive sets | — | Yes | 2025-08-31 | [#143](https://www.erdosproblems.com/143) |
| 145 | None | number theory | A005117 | Yes | 2025-08-31 | [#145](https://www.erdosproblems.com/145) |
| 146 | $500 | graph theory, turan number | — | No | 2025-08-31 | [#146](https://www.erdosproblems.com/146) |
| 148 | None | number theory, unit fractions | A076393, A006585 | No | 2025-08-31 | [#148](https://www.erdosproblems.com/148) |
| 149 | None | graph theory | — | No | 2025-08-31 | [#149](https://www.erdosproblems.com/149) |
| 151 | None | graph theory | possible | No | 2025-08-31 | [#151](https://www.erdosproblems.com/151) |
| 153 | None | sidon sets | — | Yes | 2025-08-31 | [#153](https://www.erdosproblems.com/153) |
| 155 | None | additive combinatorics, sidon sets | A143824, A227590, A003022 | Yes | 2025-08-31 | [#155](https://www.erdosproblems.com/155) |
| 156 | None | sidon sets | A382397 | Yes | 2025-08-31 | [#156](https://www.erdosproblems.com/156) |
| 158 | None | sidon sets | — | Yes | 2025-08-31 | [#158](https://www.erdosproblems.com/158) |
| 159 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#159](https://www.erdosproblems.com/159) |
| 160 | None | additive combinatorics, arithmetic progressions | possible | Yes | 2025-08-31 | [#160](https://www.erdosproblems.com/160) |
| 161 | $500 | combinatorics, ramsey theory, discrepancy | — | No | 2025-08-31 | [#161](https://www.erdosproblems.com/161) |
| 162 | None | combinatorics, ramsey theory, discrepancy | — | No | 2025-08-31 | [#162](https://www.erdosproblems.com/162) |
| 165 | $250 | graph theory, ramsey theory | A000791 | No | 2025-08-31 | [#165](https://www.erdosproblems.com/165) |
| 168 | None | additive combinatorics | A004059, A057561, A094708, A386439 | Yes | 2025-08-31 | [#168](https://www.erdosproblems.com/168) |
| 169 | None | additive combinatorics, arithmetic progressions | A005346 | No | 2025-08-31 | [#169](https://www.erdosproblems.com/169) |
| 170 | None | additive combinatorics | A046693 | Yes | 2025-08-31 | [#170](https://www.erdosproblems.com/170) |
| 172 | None | additive combinatorics, ramsey theory | — | Yes | 2025-08-31 | [#172](https://www.erdosproblems.com/172) |
| 173 | None | geometry, ramsey theory | — | No | 2025-08-31 | [#173](https://www.erdosproblems.com/173) |
| 174 | None | geometry, ramsey theory | — | No | 2025-08-31 | [#174](https://www.erdosproblems.com/174) |
| 176 | None | additive combinatorics, arithmetic progressions, discrepancy | possible | No | 2025-08-31 | [#176](https://www.erdosproblems.com/176) |
| 177 | None | discrepancy, arithmetic progressions | — | No | 2025-08-31 | [#177](https://www.erdosproblems.com/177) |
| 180 | None | graph theory, turan number | — | No | 2025-08-31 | [#180](https://www.erdosproblems.com/180) |
| 181 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#181](https://www.erdosproblems.com/181) |
| 183 | $250 | graph theory, ramsey theory | A003323 | No | 2025-08-31 | [#183](https://www.erdosproblems.com/183) |
| 184 | None | graph theory, cycles | possible | Yes | 2025-08-31 | [#184](https://www.erdosproblems.com/184) |
| 187 | None | additive combinatorics, ramsey theory | — | No | 2025-08-31 | [#187](https://www.erdosproblems.com/187) |
| 188 | None | geometry, ramsey theory | — | Yes | 2025-08-31 | [#188](https://www.erdosproblems.com/188) |
| 193 | None | geometry | A231255 | Yes | 2025-08-31 | [#193](https://www.erdosproblems.com/193) |
| 195 | None | arithmetic progressions | — | Yes | 2025-08-31 | [#195](https://www.erdosproblems.com/195) |
| 196 | None | arithmetic progressions | — | Yes | 2025-08-31 | [#196](https://www.erdosproblems.com/196) |
| 197 | None | arithmetic progressions | — | Yes | 2025-08-31 | [#197](https://www.erdosproblems.com/197) |
| 200 | None | primes, arithmetic progressions | A005115 | Yes | 2025-08-31 | [#200](https://www.erdosproblems.com/200) |
| 201 | None | additive combinatorics, arithmetic progressions | A003002, A003003, A003004, A003005, possible | No | 2025-08-31 | [#201](https://www.erdosproblems.com/201) |
| 203 | None | primes, covering systems | — | Yes | 2025-08-31 | [#203](https://www.erdosproblems.com/203) |
| 208 | None | number theory | A005117, A076259 | Yes | 2025-08-31 | [#208](https://www.erdosproblems.com/208) |
| 212 | None | geometry, distances | — | Yes | 2025-08-31 | [#212](https://www.erdosproblems.com/212) |
| 213 | None | geometry, distances | — | Yes | 2025-08-31 | [#213](https://www.erdosproblems.com/213) |
| 217 | None | geometry, distances | possible | No | 2025-08-31 | [#217](https://www.erdosproblems.com/217) |
| 218 | None | number theory, primes | A333230, A333231, A064113 | Yes | 2025-08-31 | [#218](https://www.erdosproblems.com/218) |
| 222 | None | number theory, squares | A256435 | No | 2025-08-31 | [#222](https://www.erdosproblems.com/222) |
| 233 | None | number theory, primes | A074741 | Yes | 2025-08-31 | [#233](https://www.erdosproblems.com/233) |
| 234 | None | number theory, primes | — | Yes | 2025-08-31 | [#234](https://www.erdosproblems.com/234) |
| 236 | None | number theory, primes | A039669, A109925 | Yes | 2025-08-31 | [#236](https://www.erdosproblems.com/236) |
| 238 | None | number theory, primes | — | Yes | 2025-08-31 | [#238](https://www.erdosproblems.com/238) |
| 241 | $100 | additive combinatorics, sidon sets | A387704 | Yes | 2025-08-31 | [#241](https://www.erdosproblems.com/241) |
| 243 | None | number theory | A000058 | Yes | 2025-08-31 | [#243](https://www.erdosproblems.com/243) |
| 244 | None | number theory, primes | — | Yes | 2025-08-31 | [#244](https://www.erdosproblems.com/244) |
| 247 | None | number theory, irrationality | — | Yes | 2025-08-31 | [#247](https://www.erdosproblems.com/247) |
| 249 | None | number theory, irrationality | A256936 | Yes | 2025-08-31 | [#249](https://www.erdosproblems.com/249) |
| 251 | None | number theory, irrationality | A098990 | Yes | 2025-08-31 | [#251](https://www.erdosproblems.com/251) |
| 252 | None | number theory, irrationality | A227988, A227989, possible | Yes | 2025-08-31 | [#252](https://www.erdosproblems.com/252) |
| 254 | None | number theory | — | Yes | 2025-08-31 | [#254](https://www.erdosproblems.com/254) |
| 256 | None | analysis | — | No | 2025-08-31 | [#256](https://www.erdosproblems.com/256) |
| 257 | None | irrationality | — | Yes | 2025-08-31 | [#257](https://www.erdosproblems.com/257) |
| 260 | None | irrationality | — | Yes | 2025-08-31 | [#260](https://www.erdosproblems.com/260) |
| 261 | None | number theory | — | No | 2025-08-31 | [#261](https://www.erdosproblems.com/261) |
| 263 | None | irrationality | — | Yes | 2025-08-31 | [#263](https://www.erdosproblems.com/263) |
| 264 | None | irrationality | — | Yes | 2025-08-31 | [#264](https://www.erdosproblems.com/264) |
| 265 | None | irrationality | — | No | 2025-08-31 | [#265](https://www.erdosproblems.com/265) |
| 267 | None | irrationality | — | Yes | 2025-08-31 | [#267](https://www.erdosproblems.com/267) |
| 269 | None | irrationality | — | Yes | 2025-08-31 | [#269](https://www.erdosproblems.com/269) |
| 271 | None | additive combinatorics, arithmetic progressions | A005487 | No | 2025-08-31 | [#271](https://www.erdosproblems.com/271) |
| 272 | None | additive combinatorics, arithmetic progressions | possible | Yes | 2025-08-31 | [#272](https://www.erdosproblems.com/272) |
| 273 | None | number theory, covering systems | — | Yes | 2025-08-31 | [#273](https://www.erdosproblems.com/273) |
| 274 | None | group theory, covering systems | — | Yes | 2025-08-31 | [#274](https://www.erdosproblems.com/274) |
| 276 | None | number theory, covering systems | — | Yes | 2025-08-31 | [#276](https://www.erdosproblems.com/276) |
| 278 | None | number theory, covering systems | — | No | 2025-08-31 | [#278](https://www.erdosproblems.com/278) |
| 279 | None | number theory, covering systems, primes | — | Yes | 2025-08-31 | [#279](https://www.erdosproblems.com/279) |
| 282 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#282](https://www.erdosproblems.com/282) |
| 288 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#288](https://www.erdosproblems.com/288) |
| 289 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#289](https://www.erdosproblems.com/289) |
| 291 | None | number theory, unit fractions | A110566 | Yes | 2025-08-31 | [#291](https://www.erdosproblems.com/291) |
| 293 | None | number theory, unit fractions | possible | No | 2025-08-31 | [#293](https://www.erdosproblems.com/293) |
| 295 | None | number theory, unit fractions | A192881 | Yes | 2025-08-31 | [#295](https://www.erdosproblems.com/295) |
| 301 | None | number theory, unit fractions | A390394 | No | 2025-08-31 | [#301](https://www.erdosproblems.com/301) |
| 302 | None | number theory, unit fractions | A390395 | No | 2025-08-31 | [#302](https://www.erdosproblems.com/302) |
| 304 | None | number theory, unit fractions | A097847, A097849 | Yes | 2025-08-31 | [#304](https://www.erdosproblems.com/304) |
| 306 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#306](https://www.erdosproblems.com/306) |
| 311 | None | number theory, unit fractions | — | No | 2025-08-31 | [#311](https://www.erdosproblems.com/311) |
| 312 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#312](https://www.erdosproblems.com/312) |
| 313 | None | number theory, unit fractions | A054377 | Yes | 2025-08-31 | [#313](https://www.erdosproblems.com/313) |
| 317 | None | number theory, unit fractions | — | Yes | 2025-08-31 | [#317](https://www.erdosproblems.com/317) |
| 319 | None | number theory, unit fractions | possible | Yes | 2025-08-31 | [#319](https://www.erdosproblems.com/319) |
| 320 | None | number theory, unit fractions | A072207 | No | 2025-08-31 | [#320](https://www.erdosproblems.com/320) |
| 321 | None | number theory, unit fractions | A384927, A391592 | Yes | 2025-08-31 | [#321](https://www.erdosproblems.com/321) |
| 322 | None | number theory, powers | A025456, A025418 | No | 2025-08-31 | [#322](https://www.erdosproblems.com/322) |
| 323 | None | number theory, powers | possible | Yes | 2025-08-31 | [#323](https://www.erdosproblems.com/323) |
| 324 | None | number theory, powers | — | Yes | 2025-08-31 | [#324](https://www.erdosproblems.com/324) |
| 325 | None | number theory, powers | possible | Yes | 2025-08-31 | [#325](https://www.erdosproblems.com/325) |
| 326 | None | number theory, additive basis | — | Yes | 2025-08-31 | [#326](https://www.erdosproblems.com/326) |
| 327 | None | number theory, unit fractions | A384927 | No | 2025-08-31 | [#327](https://www.erdosproblems.com/327) |
| 329 | None | number theory | possible | Yes | 2025-08-31 | [#329](https://www.erdosproblems.com/329) |
| 332 | None | number theory | — | Yes | 2025-08-31 | [#332](https://www.erdosproblems.com/332) |
| 334 | None | number theory | A062241, A045535 | No | 2025-08-31 | [#334](https://www.erdosproblems.com/334) |
| 335 | None | number theory, additive combinatorics | — | No | 2025-08-31 | [#335](https://www.erdosproblems.com/335) |
| 336 | None | number theory, additive basis | possible | No | 2025-08-31 | [#336](https://www.erdosproblems.com/336) |
| 338 | None | number theory, additive basis | — | No | 2025-08-31 | [#338](https://www.erdosproblems.com/338) |
| 340 | None | number theory, additive combinatorics, sidon sets | A080200, A005282 | Yes | 2025-08-31 | [#340](https://www.erdosproblems.com/340) |
| 341 | None | number theory | — | Yes | 2025-08-31 | [#341](https://www.erdosproblems.com/341) |
| 342 | None | number theory | A002858 | Yes | 2025-08-31 | [#342](https://www.erdosproblems.com/342) |
| 345 | None | number theory, complete sequences | A001661 | No | 2025-08-31 | [#345](https://www.erdosproblems.com/345) |
| 346 | None | number theory, complete sequences | — | Yes | 2025-08-31 | [#346](https://www.erdosproblems.com/346) |
| 348 | None | number theory, complete sequences | — | Yes | 2025-08-31 | [#348](https://www.erdosproblems.com/348) |
| 349 | None | number theory, complete sequences | — | Yes | 2025-08-31 | [#349](https://www.erdosproblems.com/349) |
| 352 | None | geometry | — | Yes | 2025-08-31 | [#352](https://www.erdosproblems.com/352) |
| 354 | None | number theory | — | Yes | 2025-08-31 | [#354](https://www.erdosproblems.com/354) |
| 357 | None | number theory | A364132, A364153, possible | Yes | 2025-08-31 | [#357](https://www.erdosproblems.com/357) |
| 359 | None | number theory | A002048 | Yes | 2025-08-31 | [#359](https://www.erdosproblems.com/359) |
| 361 | None | number theory | possible | Yes | 2025-08-31 | [#361](https://www.erdosproblems.com/361) |
| 365 | None | number theory | A060355, A060859, A175155 | No | 2025-09-20 | [#365](https://www.erdosproblems.com/365) |
| 367 | None | number theory | A057521 | No | 2025-08-31 | [#367](https://www.erdosproblems.com/367) |
| 368 | None | number theory | A074399 | No | 2025-08-31 | [#368](https://www.erdosproblems.com/368) |
| 371 | None | number theory | A070089 | Yes | 2025-08-31 | [#371](https://www.erdosproblems.com/371) |
| 373 | None | number theory, factorials | A003135 | Yes | 2025-08-31 | [#373](https://www.erdosproblems.com/373) |
| 374 | None | number theory | A388851, A387184, A389117, A389148 | No | 2025-08-31 | [#374](https://www.erdosproblems.com/374) |
| 376 | None | number theory, binomial coefficients, base representations | A030979 | Yes | 2025-08-31 | [#376](https://www.erdosproblems.com/376) |
| 377 | None | number theory, binomial coefficients | — | Yes | 2025-08-31 | [#377](https://www.erdosproblems.com/377) |
| 382 | None | number theory | A388850 | No | 2025-08-31 | [#382](https://www.erdosproblems.com/382) |
| 383 | None | number theory | — | Yes | 2025-08-31 | [#383](https://www.erdosproblems.com/383) |
| 385 | None | number theory | A322292 | Yes | 2025-08-31 | [#385](https://www.erdosproblems.com/385) |
| 386 | None | number theory, binomial coefficients | A280992 | Yes | 2025-08-31 | [#386](https://www.erdosproblems.com/386) |
| 388 | None | number theory | — | No | 2025-08-31 | [#388](https://www.erdosproblems.com/388) |
| 389 | None | number theory | A375071 | Yes | 2025-08-31 | [#389](https://www.erdosproblems.com/389) |
| 390 | None | number theory, factorials | A193429 | Yes | 2025-08-31 | [#390](https://www.erdosproblems.com/390) |
| 393 | None | number theory, factorials | A388302 | No | 2025-08-31 | [#393](https://www.erdosproblems.com/393) |
| 394 | None | number theory | A344005 | Yes | 2025-10-28 | [#394](https://www.erdosproblems.com/394) |
| 396 | None | number theory, binomial coefficients | A375077 | Yes | 2025-08-31 | [#396](https://www.erdosproblems.com/396) |
| 400 | None | number theory, factorials | possible | Yes | 2025-08-31 | [#400](https://www.erdosproblems.com/400) |
| 404 | None | number theory, factorials | — | No | 2025-08-31 | [#404](https://www.erdosproblems.com/404) |
| 406 | None | number theory, base representations | — | Yes | 2025-08-31 | [#406](https://www.erdosproblems.com/406) |
| 408 | None | number theory, iterated functions | A049108 | No | 2025-08-31 | [#408](https://www.erdosproblems.com/408) |
| 409 | None | number theory, iterated functions | A039651, A229487 | Yes | 2025-08-31 | [#409](https://www.erdosproblems.com/409) |
| 410 | None | number theory, iterated functions | A007497, possible | Yes | 2025-08-31 | [#410](https://www.erdosproblems.com/410) |
| 411 | None | number theory, iterated functions | A383044, possible | No | 2025-08-31 | [#411](https://www.erdosproblems.com/411) |
| 412 | None | number theory, iterated functions | A007497, A051572 | Yes | 2025-08-31 | [#412](https://www.erdosproblems.com/412) |
| 413 | None | number theory, iterated functions | A005236 | Yes | 2025-08-31 | [#413](https://www.erdosproblems.com/413) |
| 414 | None | number theory, iterated functions | A064491, possible | Yes | 2025-08-31 | [#414](https://www.erdosproblems.com/414) |
| 415 | None | number theory | possible | No | 2025-08-31 | [#415](https://www.erdosproblems.com/415) |
| 416 | None | number theory | A264810 | Yes | 2025-08-31 | [#416](https://www.erdosproblems.com/416) |
| 417 | None | number theory | A264810, A061070 | Yes | 2025-08-31 | [#417](https://www.erdosproblems.com/417) |
| 420 | None | number theory | — | No | 2025-08-31 | [#420](https://www.erdosproblems.com/420) |
| 421 | None | number theory | A389544, A390848 | Yes | 2025-08-31 | [#421](https://www.erdosproblems.com/421) |
| 422 | None | number theory | A005185 | Yes | 2025-08-31 | [#422](https://www.erdosproblems.com/422) |
| 423 | None | number theory | A005243 | No | 2025-08-31 | [#423](https://www.erdosproblems.com/423) |
| 424 | None | number theory | A005244 | Yes | 2025-08-31 | [#424](https://www.erdosproblems.com/424) |
| 425 | None | number theory, sidon sets | possible | No | 2025-08-31 | [#425](https://www.erdosproblems.com/425) |
| 428 | None | number theory, primes | — | Yes | 2025-08-31 | [#428](https://www.erdosproblems.com/428) |
| 430 | None | number theory | possible | No | 2025-08-31 | [#430](https://www.erdosproblems.com/430) |
| 431 | None | number theory, primes | — | No | 2025-08-31 | [#431](https://www.erdosproblems.com/431) |
| 432 | None | number theory | — | No | 2025-08-31 | [#432](https://www.erdosproblems.com/432) |
| 436 | None | number theory | A000445, possible | No | 2025-08-31 | [#436](https://www.erdosproblems.com/436) |
| 445 | None | number theory | — | Yes | 2025-08-31 | [#445](https://www.erdosproblems.com/445) |
| 450 | None | number theory, divisors | — | No | 2025-08-31 | [#450](https://www.erdosproblems.com/450) |
| 451 | None | number theory | A386620 | No | 2025-08-31 | [#451](https://www.erdosproblems.com/451) |
| 452 | None | number theory | possible | No | 2025-08-31 | [#452](https://www.erdosproblems.com/452) |
| 454 | None | number theory, primes | A389676, A389677 | Yes | 2025-08-31 | [#454](https://www.erdosproblems.com/454) |
| 455 | None | number theory | — | Yes | 2025-08-31 | [#455](https://www.erdosproblems.com/455) |
| 456 | None | number theory | possible | Yes | 2025-08-31 | [#456](https://www.erdosproblems.com/456) |
| 460 | None | number theory | — | No | 2025-08-31 | [#460](https://www.erdosproblems.com/460) |
| 461 | None | number theory, primes | possible | No | 2025-08-31 | [#461](https://www.erdosproblems.com/461) |
| 462 | None | number theory, primes | A032742, possible | No | 2025-08-31 | [#462](https://www.erdosproblems.com/462) |
| 463 | None | number theory, primes | possible | Yes | 2025-08-31 | [#463](https://www.erdosproblems.com/463) |
| 467 | None | number theory | — | No | 2025-08-31 | [#467](https://www.erdosproblems.com/467) |
| 468 | None | number theory, divisors | A167485, A387502, A387503 | No | 2025-08-31 | [#468](https://www.erdosproblems.com/468) |
| 469 | None | number theory, divisors | A006036, A119425, possible | Yes | 2025-08-31 | [#469](https://www.erdosproblems.com/469) |
| 470 | $10 | number theory, divisors | A006037, A002975 | Yes | 2025-08-31 | [#470](https://www.erdosproblems.com/470) |
| 472 | None | number theory | A389713, possible | No | 2025-08-31 | [#472](https://www.erdosproblems.com/472) |
| 477 | None | number theory | — | Yes | 2025-08-31 | [#477](https://www.erdosproblems.com/477) |
| 478 | None | number theory, factorials | A210184 | No | 2025-08-31 | [#478](https://www.erdosproblems.com/478) |
| 479 | None | number theory | A036236, A015919, A050259, A015921, A006521, A006517, A015940 | Yes | 2025-08-31 | [#479](https://www.erdosproblems.com/479) |
| 483 | None | number theory, additive combinatorics, ramsey theory | A030126 | No | 2025-08-31 | [#483](https://www.erdosproblems.com/483) |
| 486 | None | number theory, primitive sets | — | Yes | 2025-08-31 | [#486](https://www.erdosproblems.com/486) |
| 489 | None | number theory | — | Yes | 2025-08-31 | [#489](https://www.erdosproblems.com/489) |
| 495 | None | diophantine approximation, number theory | — | Yes | 2025-08-31 | [#495](https://www.erdosproblems.com/495) |
| 500 | $500 | graph theory, hypergraphs, turan number | A140462 | No | 2025-08-31 | [#500](https://www.erdosproblems.com/500) |
| 501 | None | combinatorics, set theory | — | Yes | 2025-08-31 | [#501](https://www.erdosproblems.com/501) |
| 503 | None | geometry, distances | A175769 | Yes | 2025-08-31 | [#503](https://www.erdosproblems.com/503) |
| 507 | None | geometry | — | Yes | 2025-08-31 | [#507](https://www.erdosproblems.com/507) |
| 508 | None | geometry, ramsey theory | — | Yes | 2025-08-31 | [#508](https://www.erdosproblems.com/508) |
| 509 | None | analysis | — | Yes | 2025-08-31 | [#509](https://www.erdosproblems.com/509) |
| 510 | None | analysis | — | Yes | 2025-08-31 | [#510](https://www.erdosproblems.com/510) |
| 513 | None | analysis | — | Yes | 2025-08-31 | [#513](https://www.erdosproblems.com/513) |
| 514 | None | analysis | — | No | 2025-08-31 | [#514](https://www.erdosproblems.com/514) |
| 517 | None | analysis | — | Yes | 2025-08-31 | [#517](https://www.erdosproblems.com/517) |
| 520 | None | number theory, probability | — | Yes | 2025-08-31 | [#520](https://www.erdosproblems.com/520) |
| 521 | None | analysis, polynomials, probability | — | No | 2025-08-31 | [#521](https://www.erdosproblems.com/521) |
| 522 | None | analysis, polynomials, probability | — | Yes | 2025-12-08 | [#522](https://www.erdosproblems.com/522) |
| 524 | None | analysis, probability, polynomials | — | No | 2025-08-31 | [#524](https://www.erdosproblems.com/524) |
| 528 | None | geometry | A387897, A156816 | No | 2025-08-31 | [#528](https://www.erdosproblems.com/528) |
| 529 | None | geometry, probability | — | No | 2025-08-31 | [#529](https://www.erdosproblems.com/529) |
| 530 | None | number theory, sidon sets | A143824, possible | No | 2025-08-31 | [#530](https://www.erdosproblems.com/530) |
| 531 | None | number theory, ramsey theory | possible | No | 2025-08-31 | [#531](https://www.erdosproblems.com/531) |
| 535 | None | number theory | possible | Yes | 2025-08-31 | [#535](https://www.erdosproblems.com/535) |
| 536 | None | number theory | possible | Yes | 2025-08-31 | [#536](https://www.erdosproblems.com/536) |
| 538 | None | number theory | — | No | 2025-08-31 | [#538](https://www.erdosproblems.com/538) |
| 539 | None | number theory | possible | Yes | 2025-08-31 | [#539](https://www.erdosproblems.com/539) |
| 544 | None | graph theory, ramsey theory | A000791 | No | 2025-08-31 | [#544](https://www.erdosproblems.com/544) |
| 545 | None | graph theory, ramsey theory | A059442, possible | No | 2025-12-02 | [#545](https://www.erdosproblems.com/545) |
| 550 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#550](https://www.erdosproblems.com/550) |
| 552 | None | graph theory, ramsey theory | A006672 | No | 2025-08-31 | [#552](https://www.erdosproblems.com/552) |
| 554 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#554](https://www.erdosproblems.com/554) |
| 555 | None | graph theory, ramsey theory | A389313, possible | No | 2025-08-31 | [#555](https://www.erdosproblems.com/555) |
| 557 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#557](https://www.erdosproblems.com/557) |
| 558 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#558](https://www.erdosproblems.com/558) |
| 560 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#560](https://www.erdosproblems.com/560) |
| 561 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#561](https://www.erdosproblems.com/561) |
| 562 | None | graph theory, ramsey theory, hypergraphs | possible | Yes | 2025-08-31 | [#562](https://www.erdosproblems.com/562) |
| 563 | None | graph theory, ramsey theory, hypergraphs | — | No | 2025-08-31 | [#563](https://www.erdosproblems.com/563) |
| 564 | $500 | graph theory, ramsey theory, hypergraphs | possible | Yes | 2025-08-31 | [#564](https://www.erdosproblems.com/564) |
| 566 | None | graph theory, ramsey theory | — | Yes | 2025-08-31 | [#566](https://www.erdosproblems.com/566) |
| 567 | None | graph theory, ramsey theory | — | Yes | 2025-08-31 | [#567](https://www.erdosproblems.com/567) |
| 568 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#568](https://www.erdosproblems.com/568) |
| 569 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#569](https://www.erdosproblems.com/569) |
| 571 | None | graph theory, turan number | — | No | 2025-08-31 | [#571](https://www.erdosproblems.com/571) |
| 572 | None | graph theory, turan number | possible | No | 2025-08-31 | [#572](https://www.erdosproblems.com/572) |
| 573 | None | graph theory, turan number | A006856 | No | 2025-08-31 | [#573](https://www.erdosproblems.com/573) |
| 575 | None | graph theory, turan number | — | No | 2025-08-31 | [#575](https://www.erdosproblems.com/575) |
| 576 | None | graph theory, turan number | possible | No | 2025-08-31 | [#576](https://www.erdosproblems.com/576) |
| 579 | None | graph theory, turan number | — | Yes | 2025-08-31 | [#579](https://www.erdosproblems.com/579) |
| 584 | None | graph theory, cycles | — | No | 2025-08-31 | [#584](https://www.erdosproblems.com/584) |
| 585 | None | graph theory, cycles | possible | No | 2025-08-31 | [#585](https://www.erdosproblems.com/585) |
| 588 | $100 | geometry | A006065, A008997 | No | 2025-08-31 | [#588](https://www.erdosproblems.com/588) |
| 589 | None | geometry | possible | No | 2025-08-31 | [#589](https://www.erdosproblems.com/589) |
| 592 | $1000 | set theory, ramsey theory | — | Yes | 2025-08-31 | [#592](https://www.erdosproblems.com/592) |
| 593 | $500 | set theory, graph theory, hypergraphs, chromatic number | — | Yes | 2025-08-31 | [#593](https://www.erdosproblems.com/593) |
| 595 | $250 | graph theory, set theory | — | Yes | 2025-08-31 | [#595](https://www.erdosproblems.com/595) |
| 596 | None | graph theory, ramsey theory, set theory | — | Yes | 2025-08-31 | [#596](https://www.erdosproblems.com/596) |
| 597 | None | graph theory, ramsey theory, set theory | — | No | 2025-08-31 | [#597](https://www.erdosproblems.com/597) |
| 598 | None | set theory, ramsey theory | — | Yes | 2025-08-31 | [#598](https://www.erdosproblems.com/598) |
| 600 | None | graph theory | possible | No | 2025-08-31 | [#600](https://www.erdosproblems.com/600) |
| 601 | $500 | graph theory, set theory | — | No | 2025-08-31 | [#601](https://www.erdosproblems.com/601) |
| 602 | None | combinatorics, set theory | — | Yes | 2025-08-31 | [#602](https://www.erdosproblems.com/602) |
| 604 | $500 | geometry, distances | possible | No | 2025-08-31 | [#604](https://www.erdosproblems.com/604) |
| 609 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#609](https://www.erdosproblems.com/609) |
| 611 | None | graph theory | — | No | 2025-08-31 | [#611](https://www.erdosproblems.com/611) |
| 612 | None | graph theory | — | No | 2025-08-31 | [#612](https://www.erdosproblems.com/612) |
| 614 | None | graph theory | possible | No | 2025-08-31 | [#614](https://www.erdosproblems.com/614) |
| 616 | None | graph theory | — | No | 2025-08-31 | [#616](https://www.erdosproblems.com/616) |
| 620 | None | graph theory | possible | No | 2025-08-31 | [#620](https://www.erdosproblems.com/620) |
| 623 | None | set theory | — | Yes | 2025-08-31 | [#623](https://www.erdosproblems.com/623) |
| 624 | None | combinatorics | possible | Yes | 2025-08-31 | [#624](https://www.erdosproblems.com/624) |
| 625 | $1000 | graph theory, chromatic number | — | No | 2025-08-31 | [#625](https://www.erdosproblems.com/625) |
| 626 | None | graph theory, chromatic number, cycles | possible | No | 2025-08-31 | [#626](https://www.erdosproblems.com/626) |
| 627 | None | graph theory, chromatic number | possible | No | 2025-08-31 | [#627](https://www.erdosproblems.com/627) |
| 629 | None | graph theory, chromatic number | possible | No | 2025-08-31 | [#629](https://www.erdosproblems.com/629) |
| 634 | $25 | geometry | possible | No | 2025-08-31 | [#634](https://www.erdosproblems.com/634) |
| 635 | None | number theory | — | No | 2026-01-30 | [#635](https://www.erdosproblems.com/635) |
| 638 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#638](https://www.erdosproblems.com/638) |
| 640 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#640](https://www.erdosproblems.com/640) |
| 642 | None | graph theory, cycles | possible | No | 2025-08-31 | [#642](https://www.erdosproblems.com/642) |
| 643 | None | graph theory, hypergraphs | possible | No | 2025-08-31 | [#643](https://www.erdosproblems.com/643) |
| 644 | None | combinatorics | possible | No | 2025-08-31 | [#644](https://www.erdosproblems.com/644) |
| 653 | None | geometry, distances | possible | Yes | 2025-08-31 | [#653](https://www.erdosproblems.com/653) |
| 654 | None | geometry, distances | possible | No | 2025-08-31 | [#654](https://www.erdosproblems.com/654) |
| 655 | None | geometry, distances | possible | Yes | 2025-08-31 | [#655](https://www.erdosproblems.com/655) |
| 657 | None | geometry, distances | possible | No | 2025-08-31 | [#657](https://www.erdosproblems.com/657) |
| 660 | None | geometry, distances, convex | possible | No | 2025-08-31 | [#660](https://www.erdosproblems.com/660) |
| 661 | $50 | geometry, distances | possible | No | 2025-08-31 | [#661](https://www.erdosproblems.com/661) |
| 662 | None | geometry, distances | — | No | 2025-08-31 | [#662](https://www.erdosproblems.com/662) |
| 663 | None | number theory | A391668 | No | 2025-08-31 | [#663](https://www.erdosproblems.com/663) |
| 665 | None | combinatorics | — | No | 2025-08-31 | [#665](https://www.erdosproblems.com/665) |
| 667 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#667](https://www.erdosproblems.com/667) |
| 668 | None | geometry, distances | A385657 | No | 2025-08-31 | [#668](https://www.erdosproblems.com/668) |
| 669 | None | geometry | A003035, A006065, A008997, possible | No | 2025-08-31 | [#669](https://www.erdosproblems.com/669) |
| 670 | None | geometry, distances | — | No | 2026-04-16 | [#670](https://www.erdosproblems.com/670) |
| 671 | $250 | analysis | — | No | 2025-08-31 | [#671](https://www.erdosproblems.com/671) |
| 675 | None | number theory | — | No | 2025-08-31 | [#675](https://www.erdosproblems.com/675) |
| 676 | None | number theory | A390181, in progress | No | 2025-08-31 | [#676](https://www.erdosproblems.com/676) |
| 677 | None | number theory | possible | Yes | 2025-08-31 | [#677](https://www.erdosproblems.com/677) |
| 679 | None | number theory | — | No | 2025-08-31 | [#679](https://www.erdosproblems.com/679) |
| 680 | None | number theory, primes | — | Yes | 2025-08-31 | [#680](https://www.erdosproblems.com/680) |
| 681 | None | number theory, primes | A389680 | Yes | 2025-08-31 | [#681](https://www.erdosproblems.com/681) |
| 683 | None | number theory, primes, binomial coefficients | A006530, A074399, A121359, possible | Yes | 2025-09-04 | [#683](https://www.erdosproblems.com/683) |
| 684 | None | number theory, primes, binomial coefficients | A392019, possible | No | 2025-08-31 | [#684](https://www.erdosproblems.com/684) |
| 685 | None | number theory, primes, binomial coefficients | — | No | 2025-08-31 | [#685](https://www.erdosproblems.com/685) |
| 686 | None | number theory | — | Yes | 2025-08-31 | [#686](https://www.erdosproblems.com/686) |
| 687 | $1000 | number theory | A048670, A058989 | No | 2025-08-31 | [#687](https://www.erdosproblems.com/687) |
| 688 | None | number theory | — | Yes | 2025-08-31 | [#688](https://www.erdosproblems.com/688) |
| 689 | None | number theory | — | Yes | 2025-08-31 | [#689](https://www.erdosproblems.com/689) |
| 691 | None | number theory | — | No | 2025-08-31 | [#691](https://www.erdosproblems.com/691) |
| 693 | None | number theory, divisors | A391118, possible | No | 2025-08-31 | [#693](https://www.erdosproblems.com/693) |
| 695 | None | number theory | A061092 | Yes | 2025-08-31 | [#695](https://www.erdosproblems.com/695) |
| 700 | None | number theory, binomial coefficients | A091963, possible | Yes | 2025-08-31 | [#700](https://www.erdosproblems.com/700) |
| 701 | None | combinatorics, intersecting family | — | Yes | 2025-08-31 | [#701](https://www.erdosproblems.com/701) |
| 704 | None | graph theory, geometry, chromatic number | — | No | 2025-08-31 | [#704](https://www.erdosproblems.com/704) |
| 706 | None | graph theory, chromatic number | possible | No | 2025-08-31 | [#706](https://www.erdosproblems.com/706) |
| 708 | $100 | number theory | possible | No | 2025-08-31 | [#708](https://www.erdosproblems.com/708) |
| 709 | None | number theory | possible | No | 2025-08-31 | [#709](https://www.erdosproblems.com/709) |
| 710 | ₹2000 | number theory | A390246 | No | 2025-08-31 | [#710](https://www.erdosproblems.com/710) |
| 711 | ₹1000 | number theory | possible | No | 2025-08-31 | [#711](https://www.erdosproblems.com/711) |
| 712 | $500 | graph theory, turan number, hypergraphs | possible | No | 2025-08-31 | [#712](https://www.erdosproblems.com/712) |
| 713 | $500 | graph theory, turan number | — | No | 2025-08-31 | [#713](https://www.erdosproblems.com/713) |
| 714 | None | graph theory, turan number | possible | No | 2025-08-31 | [#714](https://www.erdosproblems.com/714) |
| 719 | None | graph theory, hypergraphs | possible | No | 2025-08-31 | [#719](https://www.erdosproblems.com/719) |
| 724 | None | combinatorics | A001438 | No | 2025-08-31 | [#724](https://www.erdosproblems.com/724) |
| 725 | None | combinatorics | A001009 | No | 2025-08-31 | [#725](https://www.erdosproblems.com/725) |
| 726 | None | number theory | — | Yes | 2025-08-31 | [#726](https://www.erdosproblems.com/726) |
| 727 | None | number theory, factorials | A002503, A343507, A389396 | Yes | 2025-08-31 | [#727](https://www.erdosproblems.com/727) |
| 730 | None | number theory, binomial coefficients, base representations | A129515 | Yes | 2025-08-31 | [#730](https://www.erdosproblems.com/730) |
| 731 | None | number theory, binomial coefficients | A006197 | No | 2025-08-31 | [#731](https://www.erdosproblems.com/731) |
| 734 | None | combinatorics | possible | No | 2025-08-31 | [#734](https://www.erdosproblems.com/734) |
| 738 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#738](https://www.erdosproblems.com/738) |
| 740 | None | graph theory, chromatic number | — | Yes | 2025-08-31 | [#740](https://www.erdosproblems.com/740) |
| 749 | None | additive combinatorics | — | Yes | 2025-08-31 | [#749](https://www.erdosproblems.com/749) |
| 750 | None | graph theory, chromatic number | — | Yes | 2025-08-31 | [#750](https://www.erdosproblems.com/750) |
| 757 | None | geometry, distances, sidon sets | possible | Yes | 2025-08-31 | [#757](https://www.erdosproblems.com/757) |
| 761 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#761](https://www.erdosproblems.com/761) |
| 766 | None | graph theory, turan number | possible | No | 2025-08-31 | [#766](https://www.erdosproblems.com/766) |
| 768 | None | number theory | A001034, A352287 | No | 2025-08-31 | [#768](https://www.erdosproblems.com/768) |
| 769 | None | number theory, geometry | A014544, possible | No | 2025-08-31 | [#769](https://www.erdosproblems.com/769) |
| 770 | None | number theory | A263647, possible | Yes | 2025-08-31 | [#770](https://www.erdosproblems.com/770) |
| 773 | None | number theory, sidon sets, squares | A390813 | No | 2025-08-31 | [#773](https://www.erdosproblems.com/773) |
| 774 | None | number theory | — | Yes | 2025-08-31 | [#774](https://www.erdosproblems.com/774) |
| 776 | None | combinatorics | possible | No | 2025-08-31 | [#776](https://www.erdosproblems.com/776) |
| 778 | None | graph theory | — | No | 2025-08-31 | [#778](https://www.erdosproblems.com/778) |
| 782 | None | number theory | — | No | 2025-08-31 | [#782](https://www.erdosproblems.com/782) |
| 786 | None | number theory | A143301, possible | Yes | 2025-08-31 | [#786](https://www.erdosproblems.com/786) |
| 787 | None | additive combinatorics | possible | No | 2025-08-31 | [#787](https://www.erdosproblems.com/787) |
| 788 | None | additive combinatorics | possible | No | 2025-08-31 | [#788](https://www.erdosproblems.com/788) |
| 789 | None | additive combinatorics | possible | Yes | 2025-08-31 | [#789](https://www.erdosproblems.com/789) |
| 790 | None | additive combinatorics | possible | No | 2025-08-31 | [#790](https://www.erdosproblems.com/790) |
| 791 | None | additive combinatorics | A066063 | No | 2025-08-31 | [#791](https://www.erdosproblems.com/791) |
| 792 | None | additive combinatorics | possible | No | 2025-08-31 | [#792](https://www.erdosproblems.com/792) |
| 793 | None | number theory | possible | No | 2025-08-31 | [#793](https://www.erdosproblems.com/793) |
| 796 | None | number theory | possible | No | 2025-08-31 | [#796](https://www.erdosproblems.com/796) |
| 802 | None | graph theory | — | No | 2025-08-31 | [#802](https://www.erdosproblems.com/802) |
| 805 | None | graph theory | possible | No | 2025-08-31 | [#805](https://www.erdosproblems.com/805) |
| 809 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#809](https://www.erdosproblems.com/809) |
| 810 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#810](https://www.erdosproblems.com/810) |
| 811 | None | graph theory, ramsey theory | possible | No | 2025-08-31 | [#811](https://www.erdosproblems.com/811) |
| 812 | None | graph theory, ramsey theory | A059442 | Yes | 2025-08-31 | [#812](https://www.erdosproblems.com/812) |
| 813 | None | graph theory | possible | No | 2025-08-31 | [#813](https://www.erdosproblems.com/813) |
| 817 | None | additive combinatorics | possible | Yes | 2025-08-31 | [#817](https://www.erdosproblems.com/817) |
| 819 | None | additive combinatorics | possible | No | 2025-08-31 | [#819](https://www.erdosproblems.com/819) |
| 820 | None | number theory | A263647 | No | 2025-08-31 | [#820](https://www.erdosproblems.com/820) |
| 821 | None | number theory | A014197 | Yes | 2025-08-31 | [#821](https://www.erdosproblems.com/821) |
| 824 | None | number theory | possible | No | 2025-08-31 | [#824](https://www.erdosproblems.com/824) |
| 826 | None | number theory | — | Yes | 2025-08-31 | [#826](https://www.erdosproblems.com/826) |
| 827 | None | geometry | possible | No | 2025-08-31 | [#827](https://www.erdosproblems.com/827) |
| 828 | None | number theory | — | Yes | 2025-08-31 | [#828](https://www.erdosproblems.com/828) |
| 829 | None | number theory | possible | Yes | 2025-08-31 | [#829](https://www.erdosproblems.com/829) |
| 830 | None | number theory | A259180 | Yes | 2025-08-31 | [#830](https://www.erdosproblems.com/830) |
| 831 | None | geometry | possible | No | 2025-08-31 | [#831](https://www.erdosproblems.com/831) |
| 836 | None | graph theory, hypergraphs, chromatic number | — | No | 2025-08-31 | [#836](https://www.erdosproblems.com/836) |
| 837 | None | graph theory, hypergraphs | — | No | 2025-08-31 | [#837](https://www.erdosproblems.com/837) |
| 838 | None | geometry, convex | possible | No | 2025-08-31 | [#838](https://www.erdosproblems.com/838) |
| 839 | None | number theory | — | No | 2025-08-31 | [#839](https://www.erdosproblems.com/839) |
| 840 | None | additive combinatorics, sidon sets | — | No | 2025-08-31 | [#840](https://www.erdosproblems.com/840) |
| 849 | None | number theory, binomial coefficients | A003016, A003015, A059233, A098565, A090162, A180058, A182237 | Yes | 2025-08-31 | [#849](https://www.erdosproblems.com/849) |
| 850 | None | number theory, primes | A343101 | Yes | 2025-08-31 | [#850](https://www.erdosproblems.com/850) |
| 852 | None | number theory, primes | A001223, A053597, A078515 | No | 2025-08-31 | [#852](https://www.erdosproblems.com/852) |
| 853 | None | number theory, primes | A001223, A390769 | Yes | 2025-08-31 | [#853](https://www.erdosproblems.com/853) |
| 854 | None | number theory | A389839, A048670 | No | 2025-08-31 | [#854](https://www.erdosproblems.com/854) |
| 855 | None | number theory, primes | A023193 | Yes | 2026-03-14 | [#855](https://www.erdosproblems.com/855) |
| 856 | None | number theory | possible | No | 2025-08-31 | [#856](https://www.erdosproblems.com/856) |
| 857 | None | combinatorics | possible | Yes | 2025-08-31 | [#857](https://www.erdosproblems.com/857) |
| 859 | None | number theory, divisors | — | Yes | 2025-08-31 | [#859](https://www.erdosproblems.com/859) |
| 860 | None | number theory, primes | A048670, A058989 | No | 2025-08-31 | [#860](https://www.erdosproblems.com/860) |
| 864 | None | number theory, sidon sets, additive combinatorics | A389182 | No | 2025-08-31 | [#864](https://www.erdosproblems.com/864) |
| 866 | None | number theory, additive combinatorics | possible | No | 2025-08-31 | [#866](https://www.erdosproblems.com/866) |
| 870 | None | number theory, additive basis | — | No | 2025-08-31 | [#870](https://www.erdosproblems.com/870) |
| 872 | None | number theory, primitive sets | possible | Yes | 2025-08-31 | [#872](https://www.erdosproblems.com/872) |
| 873 | None | number theory | — | Yes | 2025-08-31 | [#873](https://www.erdosproblems.com/873) |
| 875 | None | additive combinatorics | — | No | 2025-08-31 | [#875](https://www.erdosproblems.com/875) |
| 876 | None | additive combinatorics | — | No | 2025-08-31 | [#876](https://www.erdosproblems.com/876) |
| 878 | None | number theory | A339378, possible | No | 2025-08-31 | [#878](https://www.erdosproblems.com/878) |
| 879 | None | number theory | A186736 | No | 2025-08-31 | [#879](https://www.erdosproblems.com/879) |
| 881 | None | number theory, additive basis | — | Yes | 2025-08-31 | [#881](https://www.erdosproblems.com/881) |
| 883 | None | number theory, graph theory | — | No | 2025-08-31 | [#883](https://www.erdosproblems.com/883) |
| 885 | None | number theory, divisors | — | Yes | 2025-08-31 | [#885](https://www.erdosproblems.com/885) |
| 886 | None | number theory, divisors | — | Yes | 2025-08-31 | [#886](https://www.erdosproblems.com/886) |
| 887 | None | number theory, divisors | — | Yes | 2025-08-31 | [#887](https://www.erdosproblems.com/887) |
| 889 | None | number theory | possible | Yes | 2025-08-31 | [#889](https://www.erdosproblems.com/889) |
| 890 | None | number theory, primes | — | Yes | 2025-08-31 | [#890](https://www.erdosproblems.com/890) |
| 891 | None | number theory | — | Yes | 2025-08-31 | [#891](https://www.erdosproblems.com/891) |
| 892 | None | number theory, primitive sets | — | No | 2025-08-31 | [#892](https://www.erdosproblems.com/892) |
| 893 | None | number theory, divisors | A046801, possible | Yes | 2025-08-31 | [#893](https://www.erdosproblems.com/893) |
| 901 | None | combinatorics, hypergraphs | possible | No | 2025-08-31 | [#901](https://www.erdosproblems.com/901) |
| 902 | None | graph theory | A362137 | No | 2025-08-31 | [#902](https://www.erdosproblems.com/902) |
| 906 | None | analysis, iterated functions | — | Yes | 2025-08-31 | [#906](https://www.erdosproblems.com/906) |
| 911 | None | graph theory, ramsey theory | — | No | 2025-08-31 | [#911](https://www.erdosproblems.com/911) |
| 912 | None | number theory, factorials | A071626 | Yes | 2025-08-31 | [#912](https://www.erdosproblems.com/912) |
| 913 | None | number theory | A359747 | Yes | 2025-08-31 | [#913](https://www.erdosproblems.com/913) |
| 917 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#917](https://www.erdosproblems.com/917) |
| 918 | None | graph theory, chromatic number | — | Yes | 2025-08-31 | [#918](https://www.erdosproblems.com/918) |
| 919 | None | graph theory, chromatic number | — | No | 2025-08-31 | [#919](https://www.erdosproblems.com/919) |
| 920 | None | graph theory, chromatic number | possible | Yes | 2025-08-31 | [#920](https://www.erdosproblems.com/920) |
| 928 | None | number theory | A006530 | No | 2025-09-04 | [#928](https://www.erdosproblems.com/928) |
| 929 | None | number theory | possible | No | 2025-08-31 | [#929](https://www.erdosproblems.com/929) |
| 930 | None | number theory | — | Yes | 2025-08-31 | [#930](https://www.erdosproblems.com/930) |
| 931 | None | number theory | — | Yes | 2025-08-31 | [#931](https://www.erdosproblems.com/931) |
| 932 | None | number theory | A387864 | Yes | 2025-08-31 | [#932](https://www.erdosproblems.com/932) |
| 933 | None | number theory | possible | Yes | 2025-08-31 | [#933](https://www.erdosproblems.com/933) |
| 934 | None | graph theory | possible | No | 2025-08-31 | [#934](https://www.erdosproblems.com/934) |
| 935 | None | number theory | A057521, A389244, possible | No | 2025-09-04 | [#935](https://www.erdosproblems.com/935) |
| 936 | None | number theory | A146968, possible | Yes | 2025-08-31 | [#936](https://www.erdosproblems.com/936) |
| 938 | None | number theory | A001694, possible | Yes | 2025-08-31 | [#938](https://www.erdosproblems.com/938) |
| 939 | None | number theory | possible | Yes | 2025-08-31 | [#939](https://www.erdosproblems.com/939) |
| 940 | None | number theory | possible | Yes | 2025-08-31 | [#940](https://www.erdosproblems.com/940) |
| 942 | None | number theory | possible | Yes | 2025-08-31 | [#942](https://www.erdosproblems.com/942) |
| 943 | None | number theory | possible | Yes | 2025-08-31 | [#943](https://www.erdosproblems.com/943) |
| 944 | None | graph theory, chromatic number | — | Yes | 2025-08-31 | [#944](https://www.erdosproblems.com/944) |
| 945 | None | number theory, divisors | possible, A048892 | Yes | 2025-08-31 | [#945](https://www.erdosproblems.com/945) |
| 949 | None | ramsey theory | — | Yes | 2025-08-31 | [#949](https://www.erdosproblems.com/949) |
| 950 | None | number theory, primes | — | Yes | 2025-08-31 | [#950](https://www.erdosproblems.com/950) |
| 951 | None | number theory | — | Yes | 2025-08-31 | [#951](https://www.erdosproblems.com/951) |
| 952 | None | number theory | — | Yes | 2025-08-31 | [#952](https://www.erdosproblems.com/952) |
| 953 | None | geometry, distances | — | No | 2025-08-31 | [#953](https://www.erdosproblems.com/953) |
| 954 | None | number theory | A390642 | No | 2025-08-31 | [#954](https://www.erdosproblems.com/954) |
| 955 | None | number theory | possible | No | 2025-08-31 | [#955](https://www.erdosproblems.com/955) |
| 956 | None | geometry, distances, convex | possible | No | 2025-08-31 | [#956](https://www.erdosproblems.com/956) |
| 959 | None | geometry, distances | — | No | 2025-08-31 | [#959](https://www.erdosproblems.com/959) |
| 961 | None | number theory | A213253 | Yes | 2025-08-31 | [#961](https://www.erdosproblems.com/961) |
| 962 | None | number theory | A327909 | Yes | 2025-08-31 | [#962](https://www.erdosproblems.com/962) |
| 963 | None | number theory | possible | No | 2025-08-31 | [#963](https://www.erdosproblems.com/963) |
| 968 | None | number theory | A387591 | Yes | 2025-08-31 | [#968](https://www.erdosproblems.com/968) |
| 969 | None | number theory | A013928 | No | 2025-08-31 | [#969](https://www.erdosproblems.com/969) |
| 970 | None | number theory | A048669 | No | 2025-08-31 | [#970](https://www.erdosproblems.com/970) |
| 971 | None | number theory | A226521 | Yes | 2025-08-31 | [#971](https://www.erdosproblems.com/971) |
| 972 | None | number theory | — | Yes | 2025-08-31 | [#972](https://www.erdosproblems.com/972) |
| 973 | None | analysis | — | Yes | 2025-08-31 | [#973](https://www.erdosproblems.com/973) |
| 975 | None | number theory, divisors | A147807, possible | Yes | 2025-08-31 | [#975](https://www.erdosproblems.com/975) |
| 976 | None | number theory | — | No | 2025-08-31 | [#976](https://www.erdosproblems.com/976) |
| 978 | None | number theory | possible | Yes | 2025-08-31 | [#978](https://www.erdosproblems.com/978) |
| 979 | None | number theory | A385316, possible | Yes | 2025-08-31 | [#979](https://www.erdosproblems.com/979) |
| 983 | None | number theory | possible | No | 2025-08-31 | [#983](https://www.erdosproblems.com/983) |
| 985 | None | number theory | A002233, A219429, A103309, possible | Yes | 2025-08-31 | [#985](https://www.erdosproblems.com/985) |
| 995 | None | analysis, discrepancy | — | No | 2025-09-07 | [#995](https://www.erdosproblems.com/995) |
| 996 | None | analysis | — | Yes | 2025-09-07 | [#996](https://www.erdosproblems.com/996) |
| 1002 | None | analysis, diophantine approximation | — | Yes | 2025-09-07 | [#1002](https://www.erdosproblems.com/1002) |
| 1003 | None | number theory | A001274 | Yes | 2025-09-08 | [#1003](https://www.erdosproblems.com/1003) |
| 1004 | None | number theory | possible | Yes | 2025-09-07 | [#1004](https://www.erdosproblems.com/1004) |
| 1005 | None | number theory | A386893 | No | 2025-09-09 | [#1005](https://www.erdosproblems.com/1005) |
| 1011 | None | graph theory | possible | No | 2025-09-10 | [#1011](https://www.erdosproblems.com/1011) |
| 1013 | None | graph theory | A292528 | No | 2025-09-10 | [#1013](https://www.erdosproblems.com/1013) |
| 1016 | None | graph theory, cycles | A105206 | No | 2025-09-10 | [#1016](https://www.erdosproblems.com/1016) |
| 1017 | None | graph theory | possible | No | 2025-09-12 | [#1017](https://www.erdosproblems.com/1017) |
| 1029 | $100 | graph theory, ramsey theory | A059442 | No | 2025-09-13 | [#1029](https://www.erdosproblems.com/1029) |
| 1030 | None | graph theory, ramsey theory | A000791, A059442 | No | 2025-09-13 | [#1030](https://www.erdosproblems.com/1030) |
| 1032 | None | graph theory, chromatic number | possible | No | 2025-09-13 | [#1032](https://www.erdosproblems.com/1032) |
| 1033 | None | graph theory | possible | No | 2025-12-12 | [#1033](https://www.erdosproblems.com/1033) |
| 1035 | None | graph theory | possible | No | 2025-12-26 | [#1035](https://www.erdosproblems.com/1035) |
| 1038 | None | analysis | — | Yes | 2025-09-15 | [#1038](https://www.erdosproblems.com/1038) |
| 1039 | None | analysis | — | No | 2025-09-15 | [#1039](https://www.erdosproblems.com/1039) |
| 1040 | None | analysis | — | No | 2025-09-15 | [#1040](https://www.erdosproblems.com/1040) |
| 1045 | None | analysis | — | No | 2026-03-14 | [#1045](https://www.erdosproblems.com/1045) |
| 1049 | None | irrationality | — | Yes | 2025-09-28 | [#1049](https://www.erdosproblems.com/1049) |
| 1052 | $10 | number theory | A002827 | Yes | 2025-09-28 | [#1052](https://www.erdosproblems.com/1052) |
| 1053 | None | number theory | A007539 | No | 2025-09-28 | [#1053](https://www.erdosproblems.com/1053) |
| 1054 | None | number theory, divisors | A167485 | Yes | 2025-09-28 | [#1054](https://www.erdosproblems.com/1054) |
| 1055 | None | number theory, primes | A005113 | Yes | 2025-09-28 | [#1055](https://www.erdosproblems.com/1055) |
| 1056 | None | number theory | A060427 | Yes | 2025-09-28 | [#1056](https://www.erdosproblems.com/1056) |
| 1057 | None | number theory | A006931 | Yes | 2025-09-28 | [#1057](https://www.erdosproblems.com/1057) |
| 1059 | None | number theory, primes | A064152 | Yes | 2025-09-28 | [#1059](https://www.erdosproblems.com/1059) |
| 1060 | None | number theory | A327153 | Yes | 2025-09-28 | [#1060](https://www.erdosproblems.com/1060) |
| 1061 | None | number theory | A110177, possible | Yes | 2025-09-28 | [#1061](https://www.erdosproblems.com/1061) |
| 1062 | None | number theory | A038372 | Yes | 2025-09-28 | [#1062](https://www.erdosproblems.com/1062) |
| 1063 | None | number theory | A389360 | Yes | 2025-10-01 | [#1063](https://www.erdosproblems.com/1063) |
| 1065 | None | number theory | A074781, A339465 | Yes | 2025-10-01 | [#1065](https://www.erdosproblems.com/1065) |
| 1066 | None | graph theory, planar graphs | possible | No | 2025-10-01 | [#1066](https://www.erdosproblems.com/1066) |
| 1068 | None | graph theory, set theory, chromatic number | — | Yes | 2025-10-01 | [#1068](https://www.erdosproblems.com/1068) |
| 1070 | None | geometry | possible | No | 2025-10-05 | [#1070](https://www.erdosproblems.com/1070) |
| 1072 | None | number theory | A073944, A072937, A154554 | Yes | 2025-10-05 | [#1072](https://www.erdosproblems.com/1072) |
| 1073 | None | number theory | A256519 | Yes | 2025-10-05 | [#1073](https://www.erdosproblems.com/1073) |
| 1074 | None | number theory | A063980, A064164 | Yes | 2025-10-05 | [#1074](https://www.erdosproblems.com/1074) |
| 1075 | None | hypergraphs | — | No | 2025-10-05 | [#1075](https://www.erdosproblems.com/1075) |
| 1083 | None | geometry, distances | A186704, possible | No | 2025-10-17 | [#1083](https://www.erdosproblems.com/1083) |
| 1084 | None | geometry, distances | A045945, possible | Yes | 2025-10-17 | [#1084](https://www.erdosproblems.com/1084) |
| 1085 | None | geometry, distances | A186705, possible | Yes | 2025-10-17 | [#1085](https://www.erdosproblems.com/1085) |
| 1086 | None | geometry, distances | possible | No | 2025-10-17 | [#1086](https://www.erdosproblems.com/1086) |
| 1087 | None | geometry, distances | possible | No | 2025-10-17 | [#1087](https://www.erdosproblems.com/1087) |
| 1088 | None | geometry | possible | No | 2025-10-17 | [#1088](https://www.erdosproblems.com/1088) |
| 1093 | None | number theory, binomial coefficients | — | Yes | 2025-10-18 | [#1093](https://www.erdosproblems.com/1093) |
| 1094 | None | number theory, binomial coefficients | — | Yes | 2025-10-18 | [#1094](https://www.erdosproblems.com/1094) |
| 1095 | None | number theory, binomial coefficients | A003458 | Yes | 2025-10-18 | [#1095](https://www.erdosproblems.com/1095) |
| 1097 | None | number theory, additive combinatorics | — | Yes | 2025-10-18 | [#1097](https://www.erdosproblems.com/1097) |
| 1100 | None | number theory, divisors | A325864, possible | No | 2025-10-19 | [#1100](https://www.erdosproblems.com/1100) |
| 1101 | None | number theory | — | Yes | 2025-10-19 | [#1101](https://www.erdosproblems.com/1101) |
| 1103 | None | number theory | A392164 | No | 2025-10-19 | [#1103](https://www.erdosproblems.com/1103) |
| 1104 | None | graph theory, chromatic number | A292528 | Yes | 2025-10-26 | [#1104](https://www.erdosproblems.com/1104) |
| 1106 | None | number theory | A194259, A194260 | Yes | 2025-11-17 | [#1106](https://www.erdosproblems.com/1106) |
| 1107 | None | number theory, powerful | A056828, A392342, A392343, possible | Yes | 2025-11-17 | [#1107](https://www.erdosproblems.com/1107) |
| 1108 | None | number theory, factorials | A051761, A115645, A025494 | Yes | 2025-11-17 | [#1108](https://www.erdosproblems.com/1108) |
| 1109 | None | number theory | A392164, A392165 | No | 2025-12-03 | [#1109](https://www.erdosproblems.com/1109) |
| 1110 | None | number theory | possible | No | 2025-12-07 | [#1110](https://www.erdosproblems.com/1110) |
| 1111 | None | graph theory | possible | No | 2025-12-07 | [#1111](https://www.erdosproblems.com/1111) |
| 1112 | None | additive combinatorics | — | No | 2025-12-28 | [#1112](https://www.erdosproblems.com/1112) |
| 1113 | None | number theory, covering systems | A076336 | Yes | 2025-12-28 | [#1113](https://www.erdosproblems.com/1113) |
| 1117 | None | analysis | — | No | 2025-12-29 | [#1117](https://www.erdosproblems.com/1117) |
| 1120 | None | analysis | — | No | 2025-12-29 | [#1120](https://www.erdosproblems.com/1120) |
| 1122 | None | number theory | — | No | 2025-12-30 | [#1122](https://www.erdosproblems.com/1122) |
| 1131 | None | analysis, polynomials | — | No | 2026-01-01 | [#1131](https://www.erdosproblems.com/1131) |
| 1132 | None | analysis, polynomials | — | No | 2026-01-01 | [#1132](https://www.erdosproblems.com/1132) |
| 1133 | None | analysis, polynomials | — | Yes | 2026-01-01 | [#1133](https://www.erdosproblems.com/1133) |
| 1135 | $500 | number theory | A006370, A008908 | Yes | 2026-01-11 | [#1135](https://www.erdosproblems.com/1135) |
| 1137 | None | number theory, primes | A083550, A005250 | Yes | 2026-01-23 | [#1137](https://www.erdosproblems.com/1137) |
| 1139 | None | number theory, primes | possible | Yes | 2026-01-23 | [#1139](https://www.erdosproblems.com/1139) |
| 1142 | None | number theory, primes | A039669 | Yes | 2026-01-23 | [#1142](https://www.erdosproblems.com/1142) |
| 1143 | None | number theory, primes | — | No | 2026-01-23 | [#1143](https://www.erdosproblems.com/1143) |
| 1144 | None | number theory, probability | — | No | 2026-01-23 | [#1144](https://www.erdosproblems.com/1144) |
| 1145 | None | additive combinatorics, additive basis | — | Yes | 2026-01-23 | [#1145](https://www.erdosproblems.com/1145) |
| 1146 | None | number theory | possible | Yes | 2026-01-23 | [#1146](https://www.erdosproblems.com/1146) |
| 1150 | None | analysis, polynomials | — | Yes | 2026-01-23 | [#1150](https://www.erdosproblems.com/1150) |
| 1151 | None | analysis, polynomials | — | No | 2026-01-23 | [#1151](https://www.erdosproblems.com/1151) |
| 1152 | None | analysis, polynomials | — | No | 2026-01-23 | [#1152](https://www.erdosproblems.com/1152) |
| 1155 | None | graph theory | — | No | 2026-01-23 | [#1155](https://www.erdosproblems.com/1155) |
| 1156 | None | graph theory, chromatic number | — | No | 2026-01-23 | [#1156](https://www.erdosproblems.com/1156) |
| 1157 | None | hypergraphs, turan number | possible | No | 2026-01-23 | [#1157](https://www.erdosproblems.com/1157) |
| 1158 | None | hypergraphs, turan number | possible | No | 2026-01-23 | [#1158](https://www.erdosproblems.com/1158) |
| 1159 | None | combinatorics | — | No | 2026-01-23 | [#1159](https://www.erdosproblems.com/1159) |
| 1160 | None | group theory | A000001 | No | 2026-01-23 | [#1160](https://www.erdosproblems.com/1160) |
| 1162 | None | group theory | possible | No | 2026-01-23 | [#1162](https://www.erdosproblems.com/1162) |
| 1163 | None | group theory | — | No | 2026-01-23 | [#1163](https://www.erdosproblems.com/1163) |
| 1167 | None | set theory, probability | — | Yes | 2026-01-23 | [#1167](https://www.erdosproblems.com/1167) |
| 1168 | None | set theory, ramsey theory | — | No | 2026-01-23 | [#1168](https://www.erdosproblems.com/1168) |
| 1170 | None | set theory, ramsey theory | — | No | 2026-01-23 | [#1170](https://www.erdosproblems.com/1170) |
| 1171 | None | set theory, ramsey theory | — | No | 2026-01-23 | [#1171](https://www.erdosproblems.com/1171) |
| 1172 | None | set theory, ramsey theory | — | No | 2026-01-23 | [#1172](https://www.erdosproblems.com/1172) |
| 1173 | None | set theory, combinatorics | — | No | 2026-01-23 | [#1173](https://www.erdosproblems.com/1173) |
| 1175 | None | set theory, chromatic number | — | Yes | 2026-01-23 | [#1175](https://www.erdosproblems.com/1175) |
| 1177 | None | set theory, chromatic number, hypergraphs | — | No | 2026-01-23 | [#1177](https://www.erdosproblems.com/1177) |
| 1178 | None | graph theory, hypergraphs | — | No | 2026-01-25 | [#1178](https://www.erdosproblems.com/1178) |
| 1181 | None | number theory | possible | No | 2026-03-07 | [#1181](https://www.erdosproblems.com/1181) |
| 1182 | None | graph theory, ramsey theory | possible | No | 2026-03-07 | [#1182](https://www.erdosproblems.com/1182) |
| 1183 | None | combinatorics, ramsey theory | possible | No | 2026-03-07 | [#1183](https://www.erdosproblems.com/1183) |
| 1184 | None | number theory, primes | possible | No | 2026-04-04 | [#1184](https://www.erdosproblems.com/1184) |
| 1186 | None | additive combinatorics, arithmetic progressions | possible | No | 2026-04-04 | [#1186](https://www.erdosproblems.com/1186) |
| 1188 | None | number theory, covering systems | possible | No | 2026-04-04 | [#1188](https://www.erdosproblems.com/1188) |
| 1189 | None | number theory, covering systems | possible | No | 2026-04-04 | [#1189](https://www.erdosproblems.com/1189) |
| 1191 | $1000 | additive combinatorics, sidon sets | possible | No | 2026-04-04 | [#1191](https://www.erdosproblems.com/1191) |
| 1192 | None | additive combinatorics, additive basis | possible | Yes | 2026-04-04 | [#1192](https://www.erdosproblems.com/1192) |
| 1194 | None | additive combinatorics, additive basis, sidon sets | possible | No | 2026-04-04 | [#1194](https://www.erdosproblems.com/1194) |
| 1199 | None | additive combinatorics, ramsey theory | possible | Yes | 2026-04-04 | [#1199](https://www.erdosproblems.com/1199) |
| 1200 | None | number theory, primes | possible | No | 2026-04-04 | [#1200](https://www.erdosproblems.com/1200) |
| 1201 | None | number theory, primes | possible | Yes | 2026-04-04 | [#1201](https://www.erdosproblems.com/1201) |
| 1203 | None | number theory | possible | Yes | 2026-04-04 | [#1203](https://www.erdosproblems.com/1203) |
| 1204 | None | number theory | A008407, A023193, A135311, possible | No | 2026-04-04 | [#1204](https://www.erdosproblems.com/1204) |
| 1206 | None | number theory, sidon sets | possible | No | 2026-04-04 | [#1206](https://www.erdosproblems.com/1206) |
| 1207 | None | geometry, distances | possible | No | 2026-04-04 | [#1207](https://www.erdosproblems.com/1207) |
| 1208 | None | geometry, distances | A193838, A271490, possible | No | 2026-04-04 | [#1208](https://www.erdosproblems.com/1208) |
| 1209 | None | number theory | possible | Yes | 2026-04-04 | [#1209](https://www.erdosproblems.com/1209) |
| 1210 | None | number theory | possible | Yes | 2026-04-04 | [#1210](https://www.erdosproblems.com/1210) |
| 1212 | None | number theory, primes | possible | Yes | 2026-04-04 | [#1212](https://www.erdosproblems.com/1212) |

---

## Additional Insights

### Problems with Prizes among Open Ones
Many open problems carry Erdős prizes (e.g., $100, $500, $1000, $5000, $10000). Check the table above or filter the JSON data.

### Formalized in Lean
A growing number of problems (including some open ones) have formal statements in Lean theorem prover. See the 'Yes' in the Formalized column.

### Tags
Dominant areas among open problems include number theory, additive combinatorics, graph theory, Ramsey theory, primes, etc. (visible in the Tags column).

## How to Contribute or Stay Updated

- Visit individual problem pages to add comments or corrections.
- Contribute to the GitHub repo: update metadata, link OEIS, note formalizations, or report solutions found in literature.
- The site has a forum for each problem.
- Recent progress is tracked on the homepage ('OPEN -> SOLVED' section).

## Disclaimer

This compiled list is for reference and convenience. Always verify the latest status and full details directly on [erdosproblems.com](https://www.erdosproblems.com) as the database is actively maintained and updated. The 'open' status is not a guarantee of unsolvability.

Data current as of the YAML snapshot used (many entries last updated 2025-08-31 or later into 2026).

**End of document.** Full, untruncated list of all 616 open problems included above.




p3 momonados_agent.py --ask ./q1.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q2.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q3.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q4.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q5.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q6.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q7.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q8.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q9.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q10.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q11.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q12.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q14.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q15.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q17.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q18.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q20.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q25.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q28.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q30.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q32.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q33.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q36.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q39.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q40.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q41.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q44.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q50.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q51.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q52.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q60.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q61.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q62.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q65.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q66.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q68.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q70.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q74.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q75.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q77.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q78.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q80.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q81.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q82.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q84.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q85.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q86.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q87.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q89.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q91.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q96.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q98.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q99.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q100.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q101.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q102.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q103.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q104.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q108.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q111.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q112.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q117.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q119.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q120.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q122.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q123.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q124.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q126.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q129.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q130.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q131.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q132.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q137.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q138.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q141.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q142.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q143.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q145.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q146.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q148.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q149.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q151.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q153.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q155.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q156.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q158.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q159.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q160.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q161.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q162.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q165.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q168.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q169.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q170.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q172.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q173.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q174.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q176.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q177.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q180.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q181.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q183.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q184.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q187.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q188.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q193.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q195.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q196.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q197.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q200.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q201.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q203.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q208.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q212.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q213.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q217.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q218.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q222.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q233.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q234.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q236.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q238.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q241.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q243.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q244.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q247.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q249.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q251.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q252.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q254.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q256.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q257.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q260.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q261.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q263.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q264.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q265.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q267.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q269.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q271.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q272.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q273.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q274.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q276.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q278.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q279.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q282.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q288.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q289.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q291.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q293.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q295.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q301.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q302.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q304.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q306.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q311.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q312.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q313.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q317.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q319.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q320.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q321.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q322.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q323.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q324.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q325.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q326.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q327.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q329.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q332.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q334.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q335.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q336.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q338.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q340.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q341.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q342.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q345.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q346.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q348.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q349.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q352.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q354.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q357.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q359.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q361.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q365.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q367.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q368.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q371.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q373.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q374.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q376.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q377.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q382.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q383.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q385.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q386.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q388.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q389.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q390.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q393.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q394.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q396.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q400.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q404.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q406.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q408.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q409.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q410.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q411.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q412.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q413.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q414.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q415.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q416.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q417.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q420.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q421.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q422.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q423.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q424.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q425.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q428.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q430.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q431.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q432.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q436.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q445.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q450.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q451.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q452.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q454.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q455.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q456.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q460.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q461.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q462.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q463.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q467.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q468.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q469.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q470.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q472.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q477.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q478.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q479.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q483.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q486.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q489.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q495.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q500.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q501.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q503.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q507.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q508.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q509.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q510.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q513.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q514.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q517.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q520.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q521.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q522.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q524.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q528.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q529.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q530.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q531.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q535.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q536.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q538.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q539.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q544.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q545.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q550.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q552.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q554.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q555.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q557.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q558.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q560.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q561.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q562.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q563.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q564.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q566.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q567.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q568.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q569.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q571.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q572.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q573.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q575.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q576.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q579.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q584.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q585.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q588.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q589.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q592.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q593.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q595.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q596.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q597.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q598.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q600.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q601.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q602.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q604.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q609.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q611.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q612.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q614.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q616.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q620.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q623.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q624.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q625.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q626.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q627.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q629.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q634.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q635.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q638.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q640.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q642.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q643.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q644.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q653.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q654.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q655.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q657.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q660.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q661.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q662.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q663.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q665.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q667.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q668.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q669.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q670.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q671.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q675.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q676.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q677.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q679.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q680.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q681.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q683.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q684.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q685.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q686.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q687.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q688.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q689.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q691.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q693.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q695.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q700.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q701.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q704.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q706.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q708.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q709.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q710.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q711.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q712.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q713.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q714.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q719.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q724.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q725.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q726.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q727.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q730.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q731.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q734.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q738.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q740.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q749.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q750.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q757.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q761.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q766.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q768.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q769.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q770.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q773.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q774.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q776.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q778.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q782.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q786.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q787.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q788.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q789.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q790.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q791.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q792.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q793.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q796.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q802.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q805.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q809.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q810.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q811.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q812.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q813.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q817.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q819.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q820.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q821.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q824.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q826.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q827.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q828.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q829.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q830.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q831.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q836.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q837.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q838.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q839.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q840.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q849.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q850.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q852.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q853.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q854.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q855.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q856.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q857.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q859.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q860.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q864.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q866.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q870.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q872.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q873.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q875.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q876.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q878.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q879.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q881.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q883.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q885.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q886.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q887.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q889.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q890.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q891.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q892.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q893.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q901.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q902.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q906.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q911.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q912.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q913.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q917.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q918.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q919.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q920.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q928.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q929.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q930.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q931.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q932.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q933.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q934.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q935.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q936.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q938.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q939.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q940.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q942.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q943.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q944.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q945.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q946.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q949.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q950.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q951.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q952.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q953.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q954.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q955.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q956.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q959.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q961.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q962.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q963.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q968.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q969.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q970.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q971.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q972.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q973.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q975.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q976.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q978.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q979.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q983.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q985.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q995.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q996.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1002.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1003.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1004.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1005.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1011.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1013.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1016.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1017.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1029.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1030.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1032.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1033.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1035.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1038.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1039.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1040.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1045.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1049.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1052.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1053.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1054.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1055.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1056.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1057.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1059.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1060.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1061.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1062.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1063.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1065.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1066.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1068.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1070.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1072.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1073.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1074.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1075.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1083.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1084.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1085.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1086.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1087.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1088.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1093.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1094.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1095.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1097.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1100.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1101.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1103.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1104.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1106.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1107.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1108.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1109.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1110.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1111.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1112.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1113.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1117.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1120.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1122.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1131.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1132.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1133.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1135.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1137.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1139.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1142.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1143.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1144.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1145.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1146.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1150.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1151.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1152.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1155.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1156.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1157.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1158.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1159.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1160.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1162.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1163.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1167.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1168.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1170.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1171.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1172.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1173.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1175.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1177.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1178.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1181.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1182.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1183.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1184.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1186.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1188.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1189.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1191.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1192.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1194.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1199.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1200.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1201.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1203.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1204.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1206.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1207.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1208.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1209.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1210.txt --model google/gemini-3-flash-preview
p3 momonados_agent.py --ask ./q1212.txt --model google/gemini-3-flash-preview