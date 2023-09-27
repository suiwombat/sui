//-
// Copyright 2017, 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Strategies for generating values by taking samples of collections.
//!
//! Note that the strategies in this module are not native combinators; that
//! is, the input collection is not itself a strategy, but is rather fixed when
//! the strategy is created.

use crate::std_facade::{Arc, Cow, Vec};
use core::fmt;
use core::mem;
use core::ops::Range;
use core::u64;

use rand::Rng;

use crate::bits::{self, BitSetValueTree, SampledBitSetStrategy, VarBitSet};
use crate::num;
use crate::strategy::*;
use crate::test_runner::*;

/// Re-exported to make usage more ergonomic.
pub use crate::collection::{size_range, SizeRange};

/// Sample subsequences whose size are within `size` from the given collection
/// `values`.
///
/// A subsequence is a subset of the elements in a collection in the order they
/// occur in that collection. The elements are not chosen to be contiguous.
///
/// This is roughly analogous to `rand::sample`, except that it guarantees that
/// the order is preserved.
///
/// `values` may be a static slice or a `Vec`.
///
/// ## Panics
///
/// Panics if the maximum size implied by `size` is larger than the size of
/// `values`.
///
/// Panics if `size` is a zero-length range.
pub fn subsequence<T: Clone + 'static>(
    values: impl Into<Cow<'static, [T]>>,
    size: impl Into<SizeRange>,
) -> Subsequence<T> {
    let values = values.into();
    let len = values.len();
    let size = size.into();

    size.assert_nonempty();
    assert!(
        size.end_incl() <= len,
        "Maximum size of subsequence {} exceeds length of input {}",
        size.end_incl(),
        len
    );
    Subsequence {
        values: Arc::new(values),
        bit_strategy: bits::varsize::sampled(size, 0..len),
    }
}

/// Strategy to generate `Vec`s by sampling a subsequence from another
/// collection.
///
/// This is created by the `subsequence` function in the same module.
#[derive(Debug, Clone)]
#[must_use = "strategies do nothing unless used"]
pub struct Subsequence<T: Clone + 'static> {
    values: Arc<Cow<'static, [T]>>,
    bit_strategy: SampledBitSetStrategy<VarBitSet>,
}

impl<T: fmt::Debug + Clone + 'static> Strategy for Subsequence<T> {
    type Tree = SubsequenceValueTree<T>;
    type Value = Vec<T>;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(SubsequenceValueTree {
            values: Arc::clone(&self.values),
            inner: self.bit_strategy.new_tree(runner)?,
        })
    }
}

/// `ValueTree` type for `Subsequence`.
#[derive(Debug, Clone)]
pub struct SubsequenceValueTree<T: Clone + 'static> {
    values: Arc<Cow<'static, [T]>>,
    inner: BitSetValueTree<VarBitSet>,
}

impl<T: fmt::Debug + Clone + 'static> ValueTree for SubsequenceValueTree<T> {
    type Value = Vec<T>;

    fn current(&self) -> Self::Value {
        let inner = self.inner.current();
        let ret = inner.iter().map(|ix| self.values[ix].clone()).collect();
        ret
    }

    fn simplify(&mut self) -> bool {
        self.inner.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.inner.complicate()
    }
}

#[derive(Debug, Clone)]
struct SelectMapFn<T: Clone + 'static>(Arc<Cow<'static, [T]>>);

impl<T: fmt::Debug + Clone + 'static> statics::MapFn<usize> for SelectMapFn<T> {
    type Output = T;

    fn apply(&self, ix: usize) -> T {
        self.0[ix].clone()
    }
}

opaque_strategy_wrapper! {
    /// Strategy to produce one value from a fixed collection of options.
    ///
    /// Created by the `select()` in the same module.
    #[derive(Clone, Debug)]
    pub struct Select[<T>][where T : Clone + fmt::Debug + 'static](
        statics::Map<Range<usize>, SelectMapFn<T>>)
        -> SelectValueTree<T>;
    /// `ValueTree` corresponding to `Select`.
    #[derive(Clone, Debug)]
    pub struct SelectValueTree[<T>][where T : Clone + fmt::Debug + 'static](
        statics::Map<num::usize::BinarySearch, SelectMapFn<T>>)
        -> T;
}

/// Create a strategy which uniformly selects one value from `values`.
///
/// `values` should be a `&'static [T]` or a `Vec<T>`, or potentially another
/// type that can be coerced to `Cow<'static,[T]>`.
///
/// This is largely equivalent to making a `Union` of a bunch of `Just`
/// strategies, but is substantially more efficient and shrinks by binary
/// search.
///
/// If `values` is also to be generated by a strategy, see
/// [`Index`](struct.Index.html) for a more efficient way to select values than
/// using `prop_flat_map()`.
pub fn select<T: Clone + fmt::Debug + 'static>(
    values: impl Into<Cow<'static, [T]>>,
) -> Select<T> {
    let cow = values.into();

    Select(statics::Map::new(0..cow.len(), SelectMapFn(Arc::new(cow))))
}

/// A stand-in for an index into a slice or similar collection or conceptually
/// similar things.
///
/// At the lowest level, `Index` is a mechanism for generating `usize` values
/// in the range [0..N), for some N whose value is not known until it is
/// needed. (Contrast with using `0..N` itself as a strategy, where you need to
/// know N when you define the strategy.)
///
/// For any upper bound, the actual index produced by an `Index` is the same no
/// matter how many times it is used. Different upper bounds will produce
/// different but not independent values.
///
/// Shrinking will cause the index to binary search through the underlying
/// collection(s) it is used to sample.
///
/// Note that `Index` _cannot_ currently be used as a slice index (e.g.,
/// `slice[index]`) due to the trait coherence rules.
///
/// ## Example
///
/// If the collection itself being indexed is itself generated by a strategy,
/// you can make separately define that strategy and a strategy generating one
/// or more `Index`es and then join the two after input generation, avoiding a
/// call to `prop_flat_map()`.
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///     # /*
///     #[test]
///     # */
///     fn my_test(
///         names in prop::collection::vec("[a-z]+", 10..20),
///         indices in prop::collection::vec(any::<prop::sample::Index>(), 5..10)
///     ) {
///         // We now have Vec<String> of ten to twenty names, and a Vec<Index>
///         // of five to ten indices and can combine them however we like.
///         for index in &indices {
///             println!("Accessing item by index: {}", names[index.index(names.len())]);
///             println!("Accessing item by convenience method: {}", index.get(&names));
///         }
///         // Test stuff...
///     }
/// }
/// #
/// # fn main() { my_test(); }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Index(usize);

impl Index {
    /// Return the real index that would be used to index a collection of size `size`.
    ///
    /// ## Panics
    ///
    /// Panics if `size == 0`.
    pub fn index(&self, size: usize) -> usize {
        assert!(size > 0, "Attempt to use `Index` with 0-size collection");

        // No platforms currently have `usize` wider than 64 bits, so `u128` is
        // sufficient to hold the result of a full multiply, letting us do a
        // simple fixed-point multiply.
        ((size as u128) * (self.0 as u128) >> (mem::size_of::<usize>() * 8))
            as usize
    }

    /// Return a reference to the element in `slice` that this `Index` refers to.
    ///
    /// A shortcut for `&slice[index.index(slice.len())]`.
    pub fn get<'a, T>(&self, slice: &'a [T]) -> &'a T {
        &slice[self.index(slice.len())]
    }

    /// Return a mutable reference to the element in `slice` that this `Index`
    /// refers to.
    ///
    /// A shortcut for `&mut slice[index.index(slice.len())]`.
    pub fn get_mut<'a, T>(&self, slice: &'a mut [T]) -> &'a mut T {
        let ix = self.index(slice.len());
        &mut slice[ix]
    }
}

mapfn! {
    [] fn UsizeToIndex[](raw: usize) -> Index {
        Index(raw)
    }
}

opaque_strategy_wrapper! {
    /// Strategy to create `Index`es.
    ///
    /// Created via `any::<Index>()`.
    #[derive(Clone, Debug)]
    pub struct IndexStrategy[][](
        statics::Map<num::usize::Any, UsizeToIndex>)
        -> IndexValueTree;
    /// `ValueTree` corresponding to `IndexStrategy`.
    #[derive(Clone, Debug)]
    pub struct IndexValueTree[][](
        statics::Map<num::usize::BinarySearch,UsizeToIndex>)
        -> Index;
}

impl IndexStrategy {
    pub(crate) fn new() -> Self {
        IndexStrategy(statics::Map::new(num::usize::ANY, UsizeToIndex))
    }
}

/// A value for picking random values out of iterators.
///
/// This is, in a sense, a more flexible variant of
/// [`Index`](struct.Index.html) in that it can operate on arbitrary
/// `IntoIterator` values.
///
/// Initially, the selection is roughly uniform, with a very slight bias
/// towards items earlier in the iterator.
///
/// Shrinking causes the selection to move toward items earlier in the
/// iterator, ultimately settling on the very first, but this currently happens
/// in a very haphazard way that may fail to find the earliest failing input.
///
/// ## Example
///
/// Generate a non-indexable collection and a value to pick out of it.
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///     # /*
///     #[test]
///     # */
///     fn my_test(
///         names in prop::collection::hash_set("[a-z]+", 10..20),
///         selector in any::<prop::sample::Selector>()
///     ) {
///         println!("Selected name: {}", selector.select(&names));
///         // Test stuff...
///     }
/// }
/// #
/// # fn main() { my_test(); }
/// ```
#[derive(Clone, Debug)]
pub struct Selector {
    rng: TestRng,
    bias_increment: u64,
}

/// Strategy to create `Selector`s.
///
/// Created via `any::<Selector>()`.
#[derive(Debug)]
pub struct SelectorStrategy {
    _nonexhaustive: (),
}

/// `ValueTree` corresponding to `SelectorStrategy`.
#[derive(Debug)]
pub struct SelectorValueTree {
    rng: TestRng,
    reverse_bias_increment: num::u64::BinarySearch,
}

impl SelectorStrategy {
    pub(crate) fn new() -> Self {
        SelectorStrategy { _nonexhaustive: () }
    }
}

impl Strategy for SelectorStrategy {
    type Tree = SelectorValueTree;
    type Value = Selector;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(SelectorValueTree {
            rng: runner.new_rng(),
            reverse_bias_increment: num::u64::BinarySearch::new(u64::MAX),
        })
    }
}

impl ValueTree for SelectorValueTree {
    type Value = Selector;

    fn current(&self) -> Selector {
        Selector {
            rng: self.rng.clone(),
            bias_increment: u64::MAX - self.reverse_bias_increment.current(),
        }
    }

    fn simplify(&mut self) -> bool {
        self.reverse_bias_increment.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.reverse_bias_increment.complicate()
    }
}

impl Selector {
    /// Pick a random element from iterable `it`.
    ///
    /// The selection is unaffected by the elements themselves, and is
    /// dependent only on the actual length of `it`.
    ///
    /// `it` is always iterated completely.
    ///
    /// ## Panics
    ///
    /// Panics if `it` has no elements.
    pub fn select<T: IntoIterator>(&self, it: T) -> T::Item {
        self.try_select(it).expect("select from empty iterator")
    }

    /// Pick a random element from iterable `it`.
    ///
    /// Returns `None` if `it` is empty.
    ///
    /// The selection is unaffected by the elements themselves, and is
    /// dependent only on the actual length of `it`.
    ///
    /// `it` is always iterated completely.
    pub fn try_select<T: IntoIterator>(&self, it: T) -> Option<T::Item> {
        let mut bias = 0u64;
        let mut min_score = 0;
        let mut best = None;
        let mut rng = self.rng.clone();

        for item in it {
            let score = bias.saturating_add(rng.gen());
            if best.is_none() || score < min_score {
                best = Some(item);
                min_score = score;
            }

            bias = bias.saturating_add(self.bias_increment);
        }

        best
    }
}

#[cfg(test)]
mod test {
    use crate::std_facade::BTreeSet;

    use super::*;
    use crate::arbitrary::any;

    #[test]
    fn sample_slice() {
        static VALUES: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7];
        let mut size_counts = [0; 8];
        let mut value_counts = [0; 8];

        let mut runner = TestRunner::deterministic();
        let input = subsequence(VALUES, 3..7);

        for _ in 0..2048 {
            let value = input.new_tree(&mut runner).unwrap().current();
            // Generated the correct number of items
            assert!(value.len() >= 3 && value.len() < 7);
            // Chose distinct items
            assert_eq!(
                value.len(),
                value.iter().cloned().collect::<BTreeSet<_>>().len()
            );
            // Values are in correct order
            let mut sorted = value.clone();
            sorted.sort();
            assert_eq!(sorted, value);

            size_counts[value.len()] += 1;

            for value in value {
                value_counts[value] += 1;
            }
        }

        for i in 3..7 {
            assert!(
                size_counts[i] >= 256 && size_counts[i] < 1024,
                "size {} was chosen {} times",
                i,
                size_counts[i]
            );
        }

        for (ix, &v) in value_counts.iter().enumerate() {
            assert!(
                v >= 1024 && v < 1500,
                "Value {} was chosen {} times",
                ix,
                v
            );
        }
    }

    #[test]
    fn sample_vec() {
        // Just test that the types work out
        let values = vec![0, 1, 2, 3, 4];

        let mut runner = TestRunner::deterministic();
        let input = subsequence(values, 1..3);

        let _ = input.new_tree(&mut runner).unwrap().current();
    }

    #[test]
    fn test_select() {
        let values = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut counts = [0; 8];

        let mut runner = TestRunner::deterministic();
        let input = select(values);

        for _ in 0..1024 {
            counts[input.new_tree(&mut runner).unwrap().current()] += 1;
        }

        for (ix, &count) in counts.iter().enumerate() {
            assert!(
                count >= 64 && count < 256,
                "Generated value {} {} times",
                ix,
                count
            );
        }
    }

    #[test]
    fn test_sample_sanity() {
        check_strategy_sanity(subsequence(vec![0, 1, 2, 3, 4], 1..3), None);
    }

    #[test]
    fn test_select_sanity() {
        check_strategy_sanity(select(vec![0, 1, 2, 3, 4]), None);
    }

    #[test]
    fn subseq_empty_vec_works() {
        let mut runner = TestRunner::deterministic();
        let input = subsequence(Vec::<()>::new(), 0..1);
        assert_eq!(
            Vec::<()>::new(),
            input.new_tree(&mut runner).unwrap().current()
        );
    }

    #[test]
    fn subseq_full_vec_works() {
        let v = vec![1u32, 2u32, 3u32];
        let mut runner = TestRunner::deterministic();
        let input = subsequence(v.clone(), 3);
        assert_eq!(v, input.new_tree(&mut runner).unwrap().current());
    }

    #[test]
    fn index_works() {
        let mut runner = TestRunner::deterministic();
        let input = any::<Index>();
        let col = vec!["foo", "bar", "baz"];
        let mut seen = BTreeSet::new();

        for _ in 0..16 {
            let mut tree = input.new_tree(&mut runner).unwrap();
            seen.insert(*tree.current().get(&col));

            while tree.simplify() {}

            assert_eq!("foo", *tree.current().get(&col));
        }

        assert_eq!(col.into_iter().collect::<BTreeSet<_>>(), seen);
    }

    #[test]
    fn selector_works() {
        let mut runner = TestRunner::deterministic();
        let input = any::<Selector>();
        let col: BTreeSet<&str> =
            vec!["foo", "bar", "baz"].into_iter().collect();
        let mut seen = BTreeSet::new();

        for _ in 0..16 {
            let mut tree = input.new_tree(&mut runner).unwrap();
            seen.insert(*tree.current().select(&col));

            while tree.simplify() {}

            assert_eq!("bar", *tree.current().select(&col));
        }

        assert_eq!(col, seen);
    }
}
