# References

Just a list of references and helpful notes for this project.

## Helpful links
- https://drops.dagstuhl.de/opus/volltexte/2018/9208/pdf/LIPIcs-ECOOP-2018-3.pdf
- https://cs.ioc.ee/ewscs/2010/mycroft/linear-2up.pdf
- https://www.cs.purdue.edu/homes/suresh/papers/pldi11.pdf
- http://hacksoflife.blogspot.com/2021/06/we-never-needed-stackfull-coroutines.html
- https://publications.lib.chalmers.se/records/fulltext/219133/219133.pdf
- https://isocpp.org/files/papers/p1492r0.pdf
- https://blog.varunramesh.net/posts/stackless-vs-stackful-coroutines/
- https://faultlore.com/blah/linear-rust/

## Linear/Affine Types
 - [Towards Formal Semantics for Rust (Theses)](https://digitalcommons.calpoly.edu/cgi/viewcontent.cgi?article=3804&context=theses)
 - [**Katina: Formalization of Rust**](https://dada.cs.washington.edu/research/tr/2015/03/UW-CSE-15-03-02.pdf)
 - [**Termination of Borrow Checking in Rust**](https://whileydave.com/publications/PPS22_NFM_preprint.pdf)
 - [RustBelt: Securing foundations of Rust](https://people.mpi-sws.org/~dreyer/papers/rustbelt/paper.pdf)
 - [**Krust Formal Semantics for Rust**](https://arxiv.org/pdf/1804.10806.pdf)
 - [Oxide: The Essence of Rust](https://arxiv.org/pdf/1903.00982.pdf)
   - Sequel to: [Rust Distilled](https://arxiv.org/pdf/1806.02693.pdf)
 - [A lightweight formalism of Lifetimes and Borrowing](https://dl.acm.org/doi/pdf/10.1145/3443420)
 - [Practical Affine Types](https://users.cs.northwestern.edu/~jesse/pubs/alms/tovpucella-alms.pdf)
 - [Linear Types can change the world](https://cs.ioc.ee/ewscs/2010/mycroft/linear-2up.pdf)
 - [Linear Typing of CPS](https://core.ac.uk/download/pdf/30696825.pdf)

## Rust Verification
 - [Static Verifier for Rust](https://www.research-collection.ethz.ch/bitstream/handle/20.500.11850/155723/eth-49222-01.pdf?sequence=1&isAllowed=y)
 - [KRust source code](https://faculty.sist.shanghaitech.edu.cn/faculty/songfu/Projects/KRust/)

## Typing Coroutines 
  - [Typing Coroutines](https://proglang.informatik.uni-freiburg.de/projects/coroutines/TFP2010-ext.pdf)
  - [**Implementing Stackful Typesafe Routines in Scala**](https://drops.dagstuhl.de/opus/volltexte/2018/9208/pdf/LIPIcs-ECOOP-2018-3.pdf)
    - First-class, Symmetric, Stackful, Typesafe Coroutines
    - Snapshots to clone coroutines whenever necessary
    - [Video](https://www.youtube.com/watch?v=B3hKOUtc4e0)
  - [Soundness of Coroutines with Snapshots](https://arxiv.org/pdf/1806.01405.pdf)

## Coroutines Explained
  - [**Revisiting Coroutines**](https://citeseer.ist.psu.edu/viewdoc/download;jsessionid=13883E22B46E5495E3BC3600A3895DA5?doi=10.1.1.58.4017&rep=rep1&type=pdf)
    - Classifies different types of coroutines, makes a case for assymmetric stackful first-class coroutines, i.e, Full Coroutines.

## Coroutine Implementation
  - [Unifying syntax for Stackless and Stackful Coroutines](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2015/n4398.pdf)
  - [Coroutines with High Order functions](https://arxiv.org/pdf/1812.08278.pdf)
  - [Context Crate](https://docs.rs/context/latest/context/)
    - [Coroutine-rs](https://github.com/rustcc/coroutine-rs)
      - Lacks some important features, particularly with regards to working with complex types, perhaps this + symmetric stackful coroutines are places to look into.
      - Coroutines are first class though, which is great.
  - [Corosensei](https://github.com/Amanieu/corosensei)
  - [Boost Coroutines](https://www.boost.org/doc/libs/1_75_0/libs/coroutine/doc/html/index.html)

## Course Materials
  - [CS 164: Hack your Language](https://sites.google.com/a/bodik.org/cs164/)
  - [CS 242: Linear Types](https://stanford-cs242.github.io/f19/assignments/assign6/)

## Random Scribbles

Concurrent ML

- (Similar to Coroutines)
- Asynchronous CML (Reasoned about Coroutine like Behaviour)
  - Unbuffered accessing
- Look at CML and ACML
- Linear Types are important for Ownership (Affine-typing)
  - Combinators: Linear Types for CML
  - Type systems for Concurrent Programs
