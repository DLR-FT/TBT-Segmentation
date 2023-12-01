// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::behaviortree::TbtNode;

/*
    Formula:
        1st index: subformula
        2nd index: lower
        3rd index: upper

    Behavior Tree:
        1st index: phi
        2nd index: lower
        3rd index: upper
*/
type Values<T> = Vec<Vec<Vec<T>>>;

pub struct Table {
    table: Box<Values<Option<f32>>>,
    amount_first_idx: usize,
    trace_length: usize,
    pub total_lookups: usize,
    pub total_set_calls: usize,
    pub total_entries: usize,
}

impl Table {
    pub fn new(amount_first_idx: usize, trace_length: usize) -> Table {
        let number_entries = amount_first_idx * ((trace_length * (trace_length + 1)) / 2);
        let mut table = Box::new(Values::<Option<f32>>::with_capacity(amount_first_idx));
        let mut created = 0;
        for _ in 0..amount_first_idx {
            let mut lower = Box::new(Vec::<Vec<Option<f32>>>::with_capacity(trace_length));
            for l in 0..trace_length {
                let mut upper = Box::new(Vec::<Option<f32>>::with_capacity(trace_length - l));
                for _ in 0..(trace_length - l) {
                    created += 1;
                    upper.push(None);
                }
                lower.push(*upper);
            }
            table.push(*lower);
        }
        assert_eq!(created, number_entries);
        Table {
            table,
            amount_first_idx,
            trace_length,
            total_lookups: 0,
            total_set_calls: 0,
            total_entries: number_entries,
        }
    }

    #[allow(clippy::collapsible_match)]
    pub fn lookup(
        &mut self,
        first_index: usize,
        lower_index: usize,
        upper_index: usize,
    ) -> Option<f32> {
        let res = match self.table.get(first_index) {
            Some(entry) => match entry.get(lower_index) {
                Some(entry) => match entry.get(upper_index - lower_index) {
                    Some(entry) => match entry {
                        Some(value) => {
                            self.total_lookups += 1;
                            Some(*value)
                        }
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        };
        res
    }

    pub fn lookup_segmentation_tree(
        &mut self,
        tree: &TbtNode,
        lower_index: usize,
        upper_index: usize,
    ) -> Option<f32> {
        match tree {
            TbtNode::Leaf(index, _, _)
            | TbtNode::Fallback(index, _)
            | TbtNode::Parallel(index, _, _)
            | TbtNode::Sequence(index, _, _)
            | TbtNode::Timeout(index, _, _)
            | TbtNode::Kleene(index, _, _, _) => self.lookup(*index, lower_index, upper_index),
        }
    }

    pub fn set(&mut self, first_index: usize, lower_index: usize, upper_index: usize, value: f32) {
        if lower_index <= upper_index
            && lower_index < self.trace_length
            && upper_index < self.trace_length
            && first_index < self.amount_first_idx
        {
            self.total_set_calls += 1;
            self.table[first_index][lower_index][upper_index - lower_index] = Some(value);
        } else {
            println!(
                "\nOut of bounds, should not happen! Index: {first_index} Lower: {lower_index} Upper: {upper_index}"
            );
            panic!()
        }
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.total_set_calls, self.total_entries)
    }
}
