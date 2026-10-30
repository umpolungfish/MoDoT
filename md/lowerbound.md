### Cleaned Proof: Lower Bound for Distinct Subset Sums

**Problem Statement.**  
Let  
$$
A=\{a_1,a_2,\dots,a_n\}\subseteq \{1,2,\dots,N\}
$$  
be a set of \(n\) distinct positive integers such that all subset sums  
$$
\sum_{a\in S} a,\qquad S\subseteq A,
$$  
are distinct. We prove the lower bound  
$$
N \gg \frac{2^n}{\sqrt{n}}.
$$

---

**Proof.**  
The distinctness condition means that the map  
$$
S \mapsto \sum_{a\in S} a
$$  
from the power set \(\mathcal{P}(A)\) into the integers is injective. Hence there are exactly \(2^n\) distinct subset sums.

---

**Step 1: Weak pigeonhole bound.**  
The largest possible subset sum is  
$$
S_{\max}=\sum_{i=1}^n a_i \le nN.
$$  
All subset sums are nonnegative integers lying in the interval \([0,S_{\max}]\), which contains at most \(nN+1\) integers. Therefore,  
$$
2^n \le nN+1,
$$  
giving the weak estimate  
$$
N \ge \frac{2^n-1}{n}.
$$  
We now improve this using concentration of measure.

---

**Step 2: Random subset sum.**  
Let  
$$
X=\sum_{i=1}^n \varepsilon_i a_i,
$$  
where \(\varepsilon_1,\dots,\varepsilon_n\) are independent Bernoulli\((1/2)\) random variables, i.e.  
$$
\Pr(\varepsilon_i=1)=\Pr(\varepsilon_i=0)=\frac12.
$$  
Thus \(X\) is uniformly distributed over the \(2^n\) distinct subset sums.

Its mean is  
$$
\mathbb{E}[X]=\frac12\sum_{i=1}^n a_i,
$$  
and its variance is  
$$
\operatorname{Var}(X)
=\sum_{i=1}^n \operatorname{Var}(\varepsilon_i a_i)
=\frac14\sum_{i=1}^n a_i^2.
$$  
Since \(a_i\le N\) for every \(i\), we have  
$$
\operatorname{Var}(X)\le \frac14\,nN^2.
$$  
Hence the standard deviation satisfies  
$$
\sigma:=\sqrt{\operatorname{Var}(X)}
\le \frac{\sqrt{n}\,N}{2}.
$$

---

**Step 3: Concentration interval.**  
By Chebyshev's inequality, for any \(k>0\),  
$$
\Pr\!\bigl(|X-\mathbb{E}[X]|<k\sigma\bigr)\ge 1-\frac1{k^2}.
$$  
Choose \(k=\sqrt{2}\). Then  
$$
\Pr\!\bigl(|X-\mathbb{E}[X]|<\sqrt{2}\,\sigma\bigr)\ge \frac12.
$$  
Therefore, at least half of the \(2^n\) subset sums lie in the interval  
$$
I:=\left[\mathbb{E}[X]-\sqrt{2}\,\sigma,\;
       \mathbb{E}[X]+\sqrt{2}\,\sigma\right].
$$  
The length of \(I\) is  
$$
2\sqrt{2}\,\sigma
\le 2\sqrt{2}\left(\frac{\sqrt{n}\,N}{2}\right)
= \sqrt{2n}\,N.
$$  
The number of integers contained in an interval of length \(L\) is at most \(L+1\). Thus the interval \(I\) contains at most  
$$
\sqrt{2n}\,N+1
$$  
integers.

---

**Step 4: Counting and the lower bound.**  
Because all subset sums are distinct, the number of subset sums lying inside \(I\) cannot exceed the total number of integers in \(I\). Hence  
$$
\frac12 \cdot 2^n \le \sqrt{2n}\,N+1.
$$  
Equivalently,  
$$
2^{n-1} \le \sqrt{2n}\,N+1.
$$  
Rearranging gives the explicit bound  
$$
N \ge \frac{2^{n-1}-1}{\sqrt{2n}}.
$$  
Therefore, asymptotically,  
$$
N \gg \frac{2^n}{\sqrt{n}}.
$$

---

**Conclusion.**  
The proof establishes the claimed lower bound. The stronger conjecture of Erdős—that \(N > c\,2^n\) for an absolute constant \(c>0\)—remains open and is not resolved by this second-moment method. \(\square\)

---