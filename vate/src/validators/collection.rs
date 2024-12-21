//! Collection validators.

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

use crate::core::*;

/// Forwards the length of the target to the inner validator.
pub struct Length<V>(pub V);

impl Length<()> {
    pub const VEC_VTAG: ValidationTag = "m=collection;v=Length;t=Vec";
    pub const VEC_TARGET_LENGTH_DIDX: usize = 0;

    pub const VEC_DEQUE_VTAG: ValidationTag = "m=collection;v=Length;t=VecDeque";
    pub const VEC_DEQUE_TARGET_LENGTH_DIDX: usize = 0;

    pub const LINKED_LIST_VTAG: ValidationTag = "m=collection;v=Length;t=LinkedList";
    pub const LINKED_LIST_TARGET_LENGTH_DIDX: usize = 0;

    pub const B_TREE_SET_VTAG: ValidationTag = "m=collection;v=Length;t=BTreeSet";
    pub const B_TREE_SET_TARGET_LENGTH_DIDX: usize = 0;

    pub const B_TREE_MAP_VTAG: ValidationTag = "m=collection;v=Length;t=BTreeMap";
    pub const B_TREE_MAP_TARGET_LENGTH_DIDX: usize = 0;

    pub const HASH_SET_VTAG: ValidationTag = "m=collection;v=Length;t=HashSet";
    pub const HASH_SET_TARGET_LENGTH_DIDX: usize = 0;

    pub const HASH_MAP_VTAG: ValidationTag = "m=collection;v=Length;t=HashMap";
    pub const HASH_MAP_TARGET_LENGTH_DIDX: usize = 0;

    pub const BINARY_HEAP_VTAG: ValidationTag = "m=collection;v=Length;t=BinaryHeap";
    pub const BINARY_HEAP_TARGET_LENGTH_DIDX: usize = 0;
}

impl<T, C, E, V> Validator<&Vec<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &Vec<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::VEC_VTAG,
                Detailer::default().set_detail(Length::VEC_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<T, C, E, V> Validator<&VecDeque<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &VecDeque<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::VEC_DEQUE_VTAG,
                Detailer::default().set_detail(Length::VEC_DEQUE_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<T, C, E, V> Validator<&LinkedList<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &LinkedList<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::LINKED_LIST_VTAG,
                Detailer::default().set_detail(Length::LINKED_LIST_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<T, C, E, V> Validator<&BTreeSet<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &BTreeSet<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::B_TREE_SET_VTAG,
                Detailer::default().set_detail(Length::B_TREE_SET_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<Key, Value, C, E, V> Validator<&BTreeMap<Key, Value>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &BTreeMap<Key, Value>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::B_TREE_MAP_VTAG,
                Detailer::default().set_detail(Length::B_TREE_MAP_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<T, C, E, V> Validator<&HashSet<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &HashSet<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::HASH_SET_VTAG,
                Detailer::default().set_detail(Length::HASH_SET_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<Key, Value, C, E, V> Validator<&HashMap<Key, Value>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &HashMap<Key, Value>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::HASH_MAP_VTAG,
                Detailer::default().set_detail(Length::HASH_MAP_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

impl<T, C, E, V> Validator<&BinaryHeap<T>, C, E> for Length<V>
where
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: &BinaryHeap<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let len = target.len();

        self.0.run(
            &len,
            context,
            invalid.push_validation(
                Length::BINARY_HEAP_VTAG,
                Detailer::default().set_detail(Length::BINARY_HEAP_TARGET_LENGTH_DIDX, &len),
            ),
            interpreter,
            data,
            report,
        )
    }
}

/// Forwards each iterated item of the target to the inner validator.
pub struct ForEach<V>(pub V);

impl ForEach<()> {
    pub const VEC_VTAG: ValidationTag = "m=collection;v=ForEach;t=Vec";
    pub const VEC_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const VEC_DEQUE_VTAG: ValidationTag = "m=collection;v=ForEach;t=VecDeque";
    pub const VEC_DEQUE_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const LINKED_LIST_VTAG: ValidationTag = "m=collection;v=ForEach;t=LinkedList";
    pub const LINKED_LIST_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const B_TREE_SET_VTAG: ValidationTag = "m=collection;v=ForEach;t=BTreeSet";
    pub const B_TREE_SET_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const B_TREE_MAP_VTAG: ValidationTag = "m=collection;v=ForEach;t=BTreeMap";
    pub const B_TREE_MAP_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const HASH_SET_VTAG: ValidationTag = "m=collection;v=ForEach;t=HashSet";
    pub const HASH_SET_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const HASH_MAP_VTAG: ValidationTag = "m=collection;v=ForEach;t=HashMap";
    pub const HASH_MAP_TARGET_ITEM_INDEX_DIDX: usize = 0;

    pub const BINARY_HEAP_VTAG: ValidationTag = "m=collection;v=ForEach;t=BinaryHeap";
    pub const BINARY_HEAP_TARGET_ITEM_INDEX_DIDX: usize = 0;
}

impl<T, C, E, V> Validator<&Vec<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &Vec<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::VEC_VTAG,
                    Detailer::default().set_detail(ForEach::VEC_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<T, C, E, V> Validator<&VecDeque<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &VecDeque<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::VEC_DEQUE_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::VEC_DEQUE_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<T, C, E, V> Validator<&LinkedList<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &LinkedList<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::LINKED_LIST_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::LINKED_LIST_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<T, C, E, V> Validator<&BTreeSet<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &BTreeSet<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::B_TREE_SET_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::B_TREE_SET_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<Key, Value, C, E, V> Validator<&BTreeMap<Key, Value>, C, E> for ForEach<V>
where
    for<'a> V: Validator<(&'a Key, &'a Value), C, E>,
{
    fn run<D, R>(
        &self,
        target: &BTreeMap<Key, Value>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, (key, value)) in target.iter().enumerate() {
            let cf = self.0.run(
                (key, value),
                context,
                invalid.clone().push_validation(
                    ForEach::B_TREE_MAP_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::B_TREE_MAP_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<T, C, E, V> Validator<&HashSet<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &HashSet<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::HASH_SET_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::HASH_SET_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<Key, Value, C, E, V> Validator<&HashMap<Key, Value>, C, E> for ForEach<V>
where
    for<'a> V: Validator<(&'a Key, &'a Value), C, E>,
{
    fn run<D, R>(
        &self,
        target: &HashMap<Key, Value>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, (key, value)) in target.iter().enumerate() {
            let cf = self.0.run(
                (key, value),
                context,
                invalid.clone().push_validation(
                    ForEach::HASH_MAP_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::HASH_MAP_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

impl<T, C, E, V> Validator<&BinaryHeap<T>, C, E> for ForEach<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        &self,
        target: &BinaryHeap<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        for (index, target_item) in target.iter().enumerate() {
            let cf = self.0.run(
                target_item,
                context,
                invalid.clone().push_validation(
                    ForEach::BINARY_HEAP_VTAG,
                    Detailer::default()
                        .set_detail(ForEach::BINARY_HEAP_TARGET_ITEM_INDEX_DIDX, &index),
                ),
                interpreter,
                data,
                report,
            )?;

            if matches!(cf, ControlFlow::Exit) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }
}

/// Validates the target contains the value at field `0`.
pub struct Contains<T>(pub T);

impl Contains<()> {
    pub const VEC_VTAG: ValidationTag = "m=collection;v=Contains;t=Vec";
    pub const VEC_NEEDLE_VALUE_DIDX: usize = 0;

    pub const VEC_DEQUE_VTAG: ValidationTag = "m=collection;v=Contains;t=VecDeque";
    pub const VEC_DEQUE_NEEDLE_VALUE_DIDX: usize = 0;

    pub const LINKED_LIST_VTAG: ValidationTag = "m=collection;v=Contains;t=LinkedList";
    pub const LINKED_NEEDLE_VALUE_DIDX: usize = 0;

    pub const B_TREE_SET_VTAG: ValidationTag = "m=collection;v=Contains;t=BTreeSet";
    pub const B_TREE_SET_NEEDLE_VALUE_DIDX: usize = 0;

    pub const B_TREE_MAP_VTAG: ValidationTag = "m=collection;v=Contains;t=BTreeMap";
    pub const B_TREE_MAP_NEEDLE_VALUE_DIDX: usize = 0;

    pub const HASH_SET_VTAG: ValidationTag = "m=collection;v=Contains;t=HashSet";
    pub const HASH_SET_NEEDLE_VALUE_DIDX: usize = 0;

    pub const HASH_MAP_VTAG: ValidationTag = "m=collection;v=Contains;t=HashMap";
    pub const HASH_MAP_NEEDLE_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<&Vec<T>, C, E> for Contains<T>
where
    T: PartialEq + ToString,
{
    fn run<D, R>(
        &self,
        target: &Vec<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::VEC_VTAG,
                    Detailer::default().set_detail(Contains::VEC_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E> Validator<&VecDeque<T>, C, E> for Contains<T>
where
    T: PartialEq + ToString,
{
    fn run<D, R>(
        &self,
        target: &VecDeque<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::VEC_DEQUE_VTAG,
                    Detailer::default().set_detail(Contains::VEC_DEQUE_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E> Validator<&LinkedList<T>, C, E> for Contains<T>
where
    T: PartialEq + ToString,
{
    fn run<D, R>(
        &self,
        target: &LinkedList<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::LINKED_LIST_VTAG,
                    Detailer::default().set_detail(Contains::LINKED_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E> Validator<&BTreeSet<T>, C, E> for Contains<T>
where
    T: PartialEq + ToString + Ord,
{
    fn run<D, R>(
        &self,
        target: &BTreeSet<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::B_TREE_SET_VTAG,
                    Detailer::default().set_detail(Contains::B_TREE_SET_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<Key, Value, C, E> Validator<&BTreeMap<Key, Value>, C, E> for Contains<Key>
where
    Key: PartialEq + ToString + Ord,
{
    fn run<D, R>(
        &self,
        target: &BTreeMap<Key, Value>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains_key(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::B_TREE_MAP_VTAG,
                    Detailer::default().set_detail(Contains::B_TREE_MAP_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E> Validator<&HashSet<T>, C, E> for Contains<T>
where
    T: PartialEq + ToString + Hash + Eq,
{
    fn run<D, R>(
        &self,
        target: &HashSet<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::HASH_SET_VTAG,
                    Detailer::default().set_detail(Contains::HASH_SET_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<Key, Value, C, E> Validator<&HashMap<Key, Value>, C, E> for Contains<Key>
where
    Key: PartialEq + ToString + Hash + Eq,
{
    fn run<D, R>(
        &self,
        target: &HashMap<Key, Value>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let needle = &self.0;

        if target.contains_key(needle) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Contains::HASH_MAP_VTAG,
                    Detailer::default().set_detail(Contains::HASH_MAP_NEEDLE_VALUE_DIDX, needle),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is among the slice in field `0`.
pub struct Among<T>(pub T);

impl Among<()> {
    pub const SLICE_VTAG: ValidationTag = "m=collection;v=Among;t=Slice";

    // TODO: Implement the `Among` validator for other collections, like `&Vec`, `&HashSet`, etc.
}

impl<T, C, E> Validator<T, C, E> for Among<&[T]>
where
    T: PartialEq,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let haystack = self.0;

        if haystack.contains(&target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(Among::SLICE_VTAG, Detailer::default()),
                interpreter,
                data,
            ))
        }
    }
}
