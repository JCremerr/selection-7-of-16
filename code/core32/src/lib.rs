//! selcore (crate selcore32, code/core32): core library for exact selection
//! complexity on posets with n <= 19 (port of code/core).
//!
//! * [`poset`]  - `Poset` (u32 bitsets above/below, incremental transitive closure),
//!                `Problem` (P, i) with reduction, solved test, duality.
//! * [`canon`]  - exact canonical keys of problems up to isomorphism and duality
//!                (individualization-refinement with automorphism pruning).
//! * [`count`]  - compatible-solution count |C(P,i)| and static lower bounds.
//! * [`solver`] - exact solver: memoized AND/OR search with (lb, ub) bounds.
//! * [`bits`]   - bit helpers, hasher, RNG.

pub mod bits;
pub mod cert;
pub mod dfpn;
pub mod canon;
pub mod count;
pub mod lbbest;
pub mod poset;
pub mod solver;

pub use canon::{decode_key, Canonizer, Key};
pub use count::{ceil_log2, compat_count, static_lb};
pub use poset::{Poset, Problem, MAXN};
