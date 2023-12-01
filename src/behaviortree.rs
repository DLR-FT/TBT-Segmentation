// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{stl::Stl, table::Table, Trace};
use std::{collections::HashMap, time::SystemTime};

type SubtreeIdx = usize;
pub type Segmentation<'a> = Vec<(&'a TbtNode, usize, usize, f32)>;
static mut NODECOUNT: SubtreeIdx = 0;
// get formula count
fn gnc() -> SubtreeIdx {
    unsafe {
        NODECOUNT += 1;
        NODECOUNT - 1
    }
}
pub fn tbt_node_reset_count() {
    unsafe { NODECOUNT = 0 }
}
/*******************************
 * Tbt
 *******************************/

#[derive(Clone)]
#[allow(dead_code)]
pub struct Tbt {
    pub next_nodes: HashMap<usize, Vec<usize>>,
    pub tree: TbtNode,
}

#[allow(dead_code)]
impl Tbt {
    pub fn new(tree: TbtNode) -> Self {
        let mut next_nodes = HashMap::new();
        let mut stack = Vec::new();
        Tbt::init_next_nodes_map(&tree, &mut stack, &mut next_nodes);
        println!("Next nodes map:\n   {:?}", next_nodes);
        Tbt { next_nodes, tree }
    }

    fn init_next_nodes_map<'a>(
        tbt_node: &'a TbtNode,
        stack_sequence: &mut Vec<(&'a TbtNode, usize)>,
        map: &mut HashMap<usize, Vec<usize>>,
    ) {
        match tbt_node {
            TbtNode::Fallback(_, children) | TbtNode::Parallel(_, _, children) => {
                for (index, child) in children.iter().enumerate() {
                    stack_sequence.push((tbt_node, index));
                    Tbt::init_next_nodes_map(child, stack_sequence, map);
                    stack_sequence.pop();
                }
            }
            TbtNode::Kleene(_, _, _, child) | TbtNode::Timeout(_, _, child) => {
                stack_sequence.push((tbt_node, 0));
                Tbt::init_next_nodes_map(child, stack_sequence, map)
            }
            TbtNode::Sequence(_, l_child, r_child) => {
                stack_sequence.push((tbt_node, 0));
                Tbt::init_next_nodes_map(l_child, stack_sequence, map);
                stack_sequence.pop();
                stack_sequence.push((tbt_node, 1));
                Tbt::init_next_nodes_map(r_child, stack_sequence, map);
                stack_sequence.pop();
            }
            TbtNode::Leaf(index, _, _) => {
                let mut stack_copy = stack_sequence.clone();
                while !stack_copy.is_empty() {
                    let (parent, last_idx) = stack_copy.pop().unwrap();
                    if let TbtNode::Sequence(_, _, r_child) = parent {
                        if last_idx == 0 {
                            let next_leaves = Tbt::get_first_leaf(r_child);
                            map.insert(*index, next_leaves);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn get_first_leaf(tbt_node: &TbtNode) -> Vec<usize> {
        match tbt_node {
            TbtNode::Leaf(index, _, _) => vec![*index],
            TbtNode::Fallback(_, children) | TbtNode::Parallel(_, _, children) => {
                let mut vec_next = Vec::new();
                for child in children {
                    vec_next.append(&mut Tbt::get_first_leaf(child));
                }
                vec_next
            }
            TbtNode::Sequence(_, child, _)
            | TbtNode::Timeout(_, _, child)
            | TbtNode::Kleene(_, _, _, child) => Tbt::get_first_leaf(child),
        }
    }

    pub fn get_number_nodes() -> usize {
        unsafe { NODECOUNT }
    }
}
/*******************************
 * TbtNode
 *******************************/
#[derive(Clone)]
#[allow(dead_code)]
pub enum TbtNode {
    Leaf(SubtreeIdx, Stl, String),
    Fallback(SubtreeIdx, Vec<TbtNode>),
    Parallel(SubtreeIdx, usize, Vec<TbtNode>),
    Sequence(SubtreeIdx, Box<TbtNode>, Box<TbtNode>),
    Timeout(SubtreeIdx, usize, Box<TbtNode>),
    Kleene(SubtreeIdx, usize, Option<Box<TbtNode>>, Box<TbtNode>),
}

#[allow(dead_code)]
impl TbtNode {
    fn get_index(&self) -> SubtreeIdx {
        match self {
            TbtNode::Leaf(index, _, _)
            | TbtNode::Fallback(index, _)
            | TbtNode::Parallel(index, _, _)
            | TbtNode::Sequence(index, _, _)
            | TbtNode::Timeout(index, _, _)
            | TbtNode::Kleene(index, _, _, _) => *index,
        }
    }

    fn get_leaf(&self, leaf_index: usize) -> Option<&TbtNode> {
        match self {
            TbtNode::Leaf(index, _, _) => {
                if leaf_index == *index {
                    Some(self)
                } else {
                    None
                }
            }
            TbtNode::Fallback(_, children) | TbtNode::Parallel(_, _, children) => {
                for child in children {
                    let leaf = child.get_leaf(leaf_index);
                    if leaf.is_some() {
                        return leaf;
                    }
                }
                None
            }
            TbtNode::Sequence(_, l_child, r_child) => {
                let leaf = l_child.get_leaf(leaf_index);
                if leaf.is_some() {
                    return leaf;
                }
                let leaf = r_child.get_leaf(leaf_index);
                if leaf.is_some() {
                    return leaf;
                }
                None
            }
            TbtNode::Timeout(_, _, child) | TbtNode::Kleene(_, _, _, child) => {
                child.get_leaf(leaf_index)
            }
        }
    }

    pub fn leaf(formula: Stl, name: String) -> Self {
        TbtNode::Leaf(gnc(), formula, name)
    }

    pub fn fallback(formulas: Vec<TbtNode>) -> Self {
        TbtNode::Fallback(gnc(), formulas)
    }

    pub fn parallel(m: usize, formulas: Vec<TbtNode>) -> Self {
        TbtNode::Parallel(gnc(), m, formulas)
    }

    pub fn sequence(left_child: TbtNode, right_child: TbtNode) -> Self {
        TbtNode::Sequence(gnc(), Box::new(left_child), Box::new(right_child))
    }

    pub fn timeout(t: usize, child: TbtNode) -> Self {
        TbtNode::Timeout(gnc(), t, Box::new(child))
    }

    pub fn kleene(n: usize, child: TbtNode) -> Self {
        let index = gnc();
        let kleene_next = if n > 0 {
            Some(Box::new(TbtNode::kleene(n - 1, child.clone())))
        } else {
            None
        };
        TbtNode::Kleene(index, n, kleene_next, Box::new(child))
    }

    pub fn kleene_inf(child: TbtNode, trace_length: usize) -> Self {
        let n = usize::max(1, trace_length);
        let mut formulas = vec![];
        let mut kleene = TbtNode::kleene(n, child);
        formulas.push(kleene.clone());
        while let TbtNode::Kleene(_, n, Some(kleene_minus_one), _) = kleene {
            if n == 1 {
                break;
            }
            kleene = *kleene_minus_one;
            formulas.push(kleene.clone());
        }
        assert_eq!(n, formulas.len());
        TbtNode::parallel(1, formulas)
    }
}

/*******************************
 * Progress and print functions
 *******************************/
#[allow(dead_code)]
impl TbtNode {
    /******************
     * Helper functions
     ******************/
    pub fn pretty_print(&self, with_children: bool, line_shift: usize) -> String {
        let indent_num = line_shift + 2;
        let indent = " ".repeat(line_shift);
        match self {
            TbtNode::Leaf(index, formula, name) => {
                if with_children {
                    format!("{}Leaf({index} {name})[{}]", indent, formula.pretty_print())
                } else {
                    format!("{}Leaf({index} {name})", indent)
                }
            }
            TbtNode::Fallback(index, subtrees) => {
                if with_children {
                    let mut string_children = String::from("\n");
                    for subtree in subtrees {
                        string_children.push_str(&subtree.pretty_print(with_children, indent_num));
                        string_children.push_str(",\n");
                    }
                    format!("{}Fallback({index})[{string_children}{}]", indent, indent)
                } else {
                    format!("{}Fallback({index})", indent)
                }
            }
            TbtNode::Parallel(index, m, subtrees) => {
                if with_children {
                    let mut string_children = String::from("\n");
                    for subtree in subtrees {
                        string_children.push_str(&subtree.pretty_print(with_children, indent_num));
                        string_children.push_str(",\n");
                    }
                    format!(
                        "{}Parallel({index},m={m})[\n{string_children}{}]",
                        indent, indent
                    )
                } else {
                    format!("{}Parallel({index},m={m})", indent)
                }
            }
            TbtNode::Sequence(index, l_child, r_child) => {
                if with_children {
                    let mut children_string = l_child.pretty_print(with_children, indent_num);
                    children_string.push_str(",\n");
                    children_string.push_str(&r_child.pretty_print(with_children, indent_num));
                    children_string.push_str(",\n");
                    format!("{}Sequence({index})[\n{children_string}{}]", indent, indent)
                } else {
                    format!("{}Sequence({index})", indent)
                }
            }
            TbtNode::Timeout(index, t, child) => {
                if with_children {
                    let child_string = child.pretty_print(with_children, indent_num);
                    format!("{indent}Timeout({index}, t={t})[\n{child_string}{indent}]")
                } else {
                    format!("{indent}Timeout({index}, t={t})")
                }
            }
            TbtNode::Kleene(index, n, _, child) => {
                if with_children {
                    let child_string = child.pretty_print(with_children, indent_num);
                    format!("{indent}Kleene({index}, n={n})[\n{child_string}{indent}]")
                } else {
                    format!("{indent}Kleene({index}, n={n})")
                }
            }
        }
    }

    pub fn get_leaf_formula(&self, look_for_index: usize) -> Option<&Stl> {
        match self {
            TbtNode::Leaf(index, formula, _) => {
                if look_for_index == *index {
                    Some(formula)
                } else {
                    None
                }
            }
            TbtNode::Fallback(_, children) | TbtNode::Parallel(_, _, children) => {
                for child in children {
                    let found = child.get_leaf_formula(look_for_index);
                    if found.is_some() {
                        return found;
                    }
                }
                None
            }
            TbtNode::Sequence(_, l_child, r_child) => {
                let found = l_child.get_leaf_formula(look_for_index);
                if found.is_some() {
                    return found;
                }
                let found = r_child.get_leaf_formula(look_for_index);
                found
            }
            TbtNode::Timeout(_, _, child) | TbtNode::Kleene(_, _, _, child) => {
                child.get_leaf_formula(look_for_index)
            }
        }
    }

    pub fn get_atomics(&self) -> Vec<&Stl> {
        match self {
            TbtNode::Leaf(_, formula, _) => formula.get_atomics(),
            TbtNode::Fallback(_, children) | TbtNode::Parallel(_, _, children) => {
                let mut atomics = Vec::new();
                for child in children {
                    atomics.append(&mut child.get_atomics());
                }
                atomics
            }
            TbtNode::Sequence(_, l_child, r_child) => {
                let mut atomics = l_child.get_atomics();
                atomics.append(&mut r_child.get_atomics());
                atomics
            }
            TbtNode::Timeout(_, _, child) | TbtNode::Kleene(_, _, _, child) => child.get_atomics(),
        }
    }

    /***********************
     * Standard Evaluation
     ***********************/
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        &self,
        depth_manager_tree: &mut HashMap<usize, (usize, usize, f32)>,
        tree_table: &mut Table,
        formula_table: &mut Table,
        trace: &Trace,
        lower: usize,
        upper: usize,
        system_time: &SystemTime,
        debug: bool,
        lazy_eval: bool,
    ) -> f32 {
        // Display progress
        if debug {
            progress(tree_table, formula_table, system_time);
        }
        // Lookup table
        let res = if lower <= upper {
            match self {
                TbtNode::Leaf(index, _, _)
                | TbtNode::Fallback(index, _)
                | TbtNode::Parallel(index, _, _)
                | TbtNode::Sequence(index, _, _)
                | TbtNode::Timeout(index, _, _)
                | TbtNode::Kleene(index, _, _, _) => tree_table.lookup(*index, lower, upper),
            }
        } else {
            None
        };
        // Return previous computed result or start computation otherwise
        if let Some(value) = res {
            value
        } else {
            let (v, index) = match self {
                TbtNode::Leaf(index, formula, _) => {
                    let v = formula.evaluate(formula_table, trace, lower, upper, lazy_eval);
                    (v, *index)
                }
                TbtNode::Fallback(index, subtrees) => {
                    let (l, u, mut v) = if lazy_eval {
                        match depth_manager_tree.get(index) {
                            Some((last_l, last_u, last_v)) => (*last_l, *last_u, *last_v),
                            None => (lower, upper, f32::NEG_INFINITY),
                        }
                    } else {
                        (lower, upper, f32::NEG_INFINITY)
                    };
                    for i in l..(u + 1) {
                        for subtree in subtrees {
                            let s_v = subtree.evaluate(
                                depth_manager_tree,
                                tree_table,
                                formula_table,
                                trace,
                                i,
                                upper,
                                system_time,
                                debug,
                                lazy_eval,
                            );
                            v = f32::max(s_v, v);
                            if lazy_eval && v > 0.0 {
                                depth_manager_tree.insert(*index, (i + 1, u, v));
                                break;
                            }
                        }
                    }
                    (v, *index)
                }
                TbtNode::Parallel(index, m, subtrees) => {
                    let mut v_vec = vec![];
                    for subtree in subtrees {
                        v_vec.push(subtree.evaluate(
                            depth_manager_tree,
                            tree_table,
                            formula_table,
                            trace,
                            lower,
                            upper,
                            system_time,
                            debug,
                            lazy_eval,
                        ));
                    }
                    v_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
                    let mth_v_value = v_vec[m - 1];
                    (mth_v_value, *index)
                }
                TbtNode::Sequence(index, left_child, right_child) => {
                    let (l, u, mut v) = if lazy_eval {
                        match depth_manager_tree.get(index) {
                            Some((last_l, last_u, last_v)) => (*last_l, *last_u, *last_v),
                            None => (lower, upper, f32::NEG_INFINITY),
                        }
                    } else {
                        (lower, upper, f32::NEG_INFINITY)
                    };
                    for i in l..(u + 1) {
                        let t1_v = left_child.evaluate(
                            depth_manager_tree,
                            tree_table,
                            formula_table,
                            trace,
                            lower,
                            i,
                            system_time,
                            debug,
                            lazy_eval,
                        );
                        let t2_v = right_child.evaluate(
                            depth_manager_tree,
                            tree_table,
                            formula_table,
                            trace,
                            i + 1,
                            upper,
                            system_time,
                            debug,
                            lazy_eval,
                        );
                        v = f32::max(v, f32::min(t1_v, t2_v));
                        if lazy_eval && v > 0.0 {
                            depth_manager_tree.insert(*index, (i + 1, u, v));
                            break;
                        }
                    }
                    (v, *index)
                }
                TbtNode::Timeout(index, t, subtree) => {
                    let v = subtree.evaluate(
                        depth_manager_tree,
                        tree_table,
                        formula_table,
                        trace,
                        lower,
                        usize::min(upper, lower + t - 1),
                        system_time,
                        debug,
                        lazy_eval,
                    );
                    (v, *index)
                }
                TbtNode::Kleene(index, n, kleene_next, subtree) => {
                    let (l, u, mut v) = if lazy_eval {
                        match depth_manager_tree.get(index) {
                            Some((last_l, last_u, last_v)) => (*last_l, *last_u, *last_v),
                            None => (lower, upper, f32::NEG_INFINITY),
                        }
                    } else {
                        (lower, upper, f32::NEG_INFINITY)
                    };
                    if l <= u && *n > 0 {
                        let kleene_n_minus_1 = kleene_next.as_ref().unwrap();
                        // Copy of Sequence start!
                        for i in l..(u + 1) {
                            let t1_v = subtree.evaluate(
                                depth_manager_tree,
                                tree_table,
                                formula_table,
                                trace,
                                lower,
                                i,
                                system_time,
                                debug,
                                lazy_eval,
                            );
                            let t2_v = kleene_n_minus_1.evaluate(
                                depth_manager_tree,
                                tree_table,
                                formula_table,
                                trace,
                                i + 1,
                                upper,
                                system_time,
                                debug,
                                lazy_eval,
                            );
                            v = f32::max(v, f32::min(t1_v, t2_v));
                            if lazy_eval && v > 0.0 {
                                depth_manager_tree.insert(*index, (i + 1, u, v));
                                break;
                            }
                        }
                        (v, *index)
                        // Copy of Sequence end!
                    } else if *n == 0 && l <= u {
                        let v = subtree.evaluate(
                            depth_manager_tree,
                            tree_table,
                            formula_table,
                            trace,
                            l,
                            u,
                            system_time,
                            debug,
                            lazy_eval,
                        );
                        (v, *index)
                    } else {
                        return f32::INFINITY;
                    }
                }
            };
            // Store result in table for next access
            if lower <= upper {
                tree_table.set(index, lower, upper, v);
            }
            v
        }
    }

    /*******************
     * Segmentation
     *******************/
    pub fn get_segmentation(
        &self,
        tree_table: &mut Table,
        formula_table: &mut Table,
        trace: &Trace,
        lower: usize,
        upper: usize,
        is_lazy: bool,
    ) -> Segmentation {
        match self {
            TbtNode::Leaf(index, formula, _) => {
                let v = if lower > upper {
                    formula.evaluate(formula_table, trace, lower, upper, is_lazy)
                } else {
                    match tree_table.lookup(*index, lower, upper) {
                        Some(v) => v,
                        None => formula.evaluate(formula_table, trace, lower, upper, is_lazy),
                    }
                };
                vec![(self, lower, upper, v)]
            }
            TbtNode::Fallback(_, subtrees) => {
                let (mut v, mut begin, mut end, mut st) = (f32::NEG_INFINITY, lower, upper, None);
                for i in lower..(upper + 1) {
                    for subtree in subtrees {
                        let s_v = match tree_table.lookup_segmentation_tree(subtree, i, upper) {
                            Some(v) => v,
                            None => {
                                if is_lazy {
                                    continue;
                                } else {
                                    panic!("unexpected")
                                }
                            }
                        };
                        if s_v > v {
                            v = s_v;
                            begin = i;
                            end = upper;
                            st = Some(subtree)
                        }
                    }
                }
                let mut self_segmentation = vec![(self, lower, upper, v)];
                let mut child_segmentation = st.unwrap().get_segmentation(
                    tree_table,
                    formula_table,
                    trace,
                    begin,
                    end,
                    is_lazy,
                );
                self_segmentation.append(&mut child_segmentation);
                self_segmentation
            }
            TbtNode::Parallel(_, m, subtrees) => {
                let mut v_vec = vec![];
                for subtree in subtrees {
                    let s_v = match tree_table.lookup_segmentation_tree(subtree, lower, upper) {
                        Some(v) => v,
                        None => {
                            if is_lazy {
                                continue;
                            } else {
                                f32::NEG_INFINITY // is ignored
                            }
                        }
                    };
                    v_vec.push((s_v, subtree));
                }
                v_vec.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
                let (mth_v_value, _) = v_vec[m - 1];
                let mut self_segmentation = vec![(self, lower, upper, mth_v_value)];
                for (_, st) in v_vec.iter().take(*m) {
                    let mut child_segmentation = st.get_segmentation(
                        tree_table,
                        formula_table,
                        trace,
                        lower,
                        upper,
                        is_lazy,
                    );
                    self_segmentation.append(&mut child_segmentation);
                }
                self_segmentation
            }
            TbtNode::Sequence(_, left_child, right_child) => {
                let (mut v, mut begin, mut change) = (f32::NEG_INFINITY, lower, upper);
                for u in lower..(upper + 1) {
                    let t1_v = match tree_table.lookup_segmentation_tree(left_child, lower, u) {
                        Some(v) => v,
                        None => {
                            if is_lazy {
                                continue;
                            } else {
                                panic!("unexpected")
                            }
                        }
                    };
                    let t2_v = if u + 1 > upper {
                        f32::NEG_INFINITY
                    } else {
                        match tree_table.lookup_segmentation_tree(right_child, u + 1, upper) {
                            Some(v) => v,
                            None => {
                                if is_lazy {
                                    continue;
                                } else {
                                    f32::NEG_INFINITY
                                }
                            }
                        }
                    };
                    if t1_v < t2_v {
                        if t1_v > v {
                            v = t1_v;
                            begin = lower;
                            change = u;
                        }
                    } else if t2_v > v {
                        v = t2_v;
                        begin = lower;
                        change = u;
                    }
                }
                let mut self_segmentation = vec![(self, lower, upper, v)];
                let mut child_segmentation = left_child.get_segmentation(
                    tree_table,
                    formula_table,
                    trace,
                    begin,
                    change,
                    is_lazy,
                );
                self_segmentation.append(&mut child_segmentation);
                let mut child_segmentation = right_child.get_segmentation(
                    tree_table,
                    formula_table,
                    trace,
                    change + 1,
                    upper,
                    is_lazy,
                );
                self_segmentation.append(&mut child_segmentation);
                self_segmentation
            }
            TbtNode::Timeout(_, t, child) => {
                let v = if lower > usize::min(upper, lower + t - 1) {
                    f32::NEG_INFINITY
                } else {
                    tree_table
                        .lookup_segmentation_tree(child, lower, usize::min(upper, lower + t - 1))
                        .unwrap()
                };
                let mut self_segmentation = vec![(self, lower, upper, v)];
                let mut child_segmentation = child.get_segmentation(
                    tree_table,
                    formula_table,
                    trace,
                    lower,
                    usize::min(upper, lower + t - 1),
                    is_lazy,
                );
                self_segmentation.append(&mut child_segmentation);
                self_segmentation
            }
            TbtNode::Kleene(_, n, kleene_next, child) => {
                if lower <= upper && *n > 0 {
                    let kleene_n_minus_1 = kleene_next.as_ref().unwrap();
                    let left_child = child;
                    let right_child = kleene_n_minus_1;
                    // Copy of Sequence start!
                    let (mut v, mut begin, mut change) = (f32::NEG_INFINITY, lower, upper);
                    for u in lower..(upper + 1) {
                        let t1_v = match tree_table.lookup_segmentation_tree(left_child, lower, u) {
                            Some(v) => v,
                            None => {
                                if is_lazy {
                                    continue;
                                } else {
                                    panic!("unexpected")
                                }
                            }
                        };
                        let t2_v = if u + 1 > upper {
                            f32::NEG_INFINITY
                        } else {
                            match tree_table.lookup_segmentation_tree(right_child, u + 1, upper) {
                                Some(v) => v,
                                None => {
                                    if is_lazy {
                                        continue;
                                    } else {
                                        f32::NEG_INFINITY
                                    }
                                }
                            }
                        };
                        if t1_v < t2_v {
                            if t1_v > v {
                                v = t1_v;
                                begin = lower;
                                change = u;
                            }
                        } else if t2_v > v {
                            v = t2_v;
                            begin = lower;
                            change = u;
                        }
                    }
                    let mut self_segmentation = vec![(self, lower, upper, v)];
                    let mut child_segmentation = left_child.get_segmentation(
                        tree_table,
                        formula_table,
                        trace,
                        begin,
                        change,
                        is_lazy,
                    );
                    self_segmentation.append(&mut child_segmentation);
                    let mut child_segmentation = right_child.get_segmentation(
                        tree_table,
                        formula_table,
                        trace,
                        change + 1,
                        upper,
                        is_lazy,
                    );
                    self_segmentation.append(&mut child_segmentation);
                    self_segmentation
                    // Copy of Sequence end!
                } else if *n == 0 && lower < upper {
                    let v = tree_table
                        .lookup_segmentation_tree(child, lower, upper)
                        .unwrap();
                    let mut self_segmentation = vec![(self, lower, upper, v)];
                    self_segmentation.append(&mut child.get_segmentation(
                        tree_table,
                        formula_table,
                        trace,
                        lower,
                        upper,
                        is_lazy,
                    ));
                    self_segmentation
                } else {
                    vec![(self, lower, upper, f32::INFINITY)]
                }
            }
        }
    }

    fn get_tau_dif(
        &self,
        lower: usize,
        upper: usize,
        segmentations: &Vec<Segmentation>,
    ) -> Option<usize> {
        let mut found = None;
        for segmentation in segmentations {
            for (st, l, u, _) in segmentation {
                let res = match (self, st) {
                    (TbtNode::Leaf(si, _, _), TbtNode::Leaf(ss, _, _))
                    | (TbtNode::Fallback(si, _), TbtNode::Fallback(ss, _))
                    | (TbtNode::Parallel(si, _, _), TbtNode::Parallel(ss, _, _))
                    | (TbtNode::Sequence(si, _, _), TbtNode::Sequence(ss, _, _))
                    | (TbtNode::Timeout(si, _, _), TbtNode::Timeout(ss, _, _))
                    | (TbtNode::Kleene(si, _, _, _), TbtNode::Kleene(ss, _, _, _)) => {
                        if si == ss {
                            let lower_dif = if lower > *l { lower - l } else { l - lower };
                            let upper_dif = if upper > *u { upper - u } else { u - upper };
                            Some(lower_dif + upper_dif)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                found = match (found, res) {
                    (None, None) => None,
                    (None, Some(v)) => Some(v),
                    (Some(v), None) => Some(v),
                    (Some(v), Some(w)) => {
                        if v < w {
                            Some(v)
                        } else {
                            Some(w)
                        }
                    }
                }
            }
        }
        found
    }

    #[allow(clippy::too_many_arguments)]
    fn get_segmentation_under_restriction(
        &self,
        tree_table: &mut Table,
        formula_table: &mut Table,
        trace: &Trace,
        lower: usize,
        upper: usize,
        tau_dif: usize,
        rho_dif: f32,
        segmentations: &Vec<Segmentation>,
    ) -> (usize, Segmentation) {
        match self {
            TbtNode::Leaf(index, formula, _) => {
                let v = if lower > upper {
                    formula.evaluate(formula_table, trace, lower, upper, false)
                } else {
                    match tree_table.lookup(*index, lower, upper) {
                        Some(v) => v,
                        None => formula.evaluate(formula_table, trace, lower, upper, false),
                    }
                };
                match self.get_tau_dif(lower, upper, segmentations) {
                    Some(actual_tau_dif) => (actual_tau_dif, vec![(self, lower, upper, v)]),
                    None => (usize::MAX, vec![(self, lower, upper, v)]),
                }
            }
            TbtNode::Fallback(_, subtrees) => {
                let mut candidates = Vec::<(f32, usize, usize, &TbtNode)>::new();
                for i in lower..(upper + 1) {
                    for subtree in subtrees {
                        let s_v = tree_table
                            .lookup_segmentation_tree(subtree, i, upper)
                            .unwrap();
                        candidates.push((s_v, i, upper, subtree));
                    }
                }
                candidates
                    .sort_by(|(rho1, _, _, _), (rho2, _, _, _)| rho2.partial_cmp(rho1).unwrap());
                candidates.retain(|&(rho, _, _, _)| rho > rho_dif);

                let mut best_candidate: (usize, Segmentation, f32) =
                    (usize::MIN, Vec::new(), f32::MIN);
                for (v, begin, end, subtree) in candidates {
                    let mut candidate = subtree.get_segmentation_under_restriction(
                        tree_table,
                        formula_table,
                        trace,
                        begin,
                        end,
                        tau_dif,
                        rho_dif,
                        segmentations,
                    );
                    if candidate.0 > tau_dif {
                        let mut self_segmentation = vec![(self, lower, upper, v)];
                        self_segmentation.append(&mut candidate.1);
                        return (candidate.0, self_segmentation);
                    } else if candidate.0 > best_candidate.0 {
                        best_candidate.0 = candidate.0;
                        best_candidate.1 = candidate.1;
                        best_candidate.2 = v;
                    }
                }
                let mut self_segmentation = vec![(self, lower, upper, best_candidate.2)];
                self_segmentation.append(&mut best_candidate.1);
                (best_candidate.0, self_segmentation)
            }
            TbtNode::Parallel(_, m, subtrees) => {
                let mut candidates = vec![];
                for subtree in subtrees {
                    let s_v = match tree_table.lookup_segmentation_tree(subtree, lower, upper) {
                        Some(v) => v,
                        None => f32::NEG_INFINITY, // is ignored,
                    };
                    candidates.push((s_v, subtree));
                }
                candidates.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
                candidates.retain(|&(rho, _)| rho > rho_dif);
                let (mth_v_value, _) = candidates[m - 1];
                let mut self_segmentation = vec![(self, lower, upper, mth_v_value)];
                let mut sum_tau_dif = 0;
                for (_, st) in candidates.iter().take(*m) {
                    let mut child_segmentation = st.get_segmentation_under_restriction(
                        tree_table,
                        formula_table,
                        trace,
                        lower,
                        upper,
                        tau_dif,
                        rho_dif,
                        segmentations,
                    );
                    sum_tau_dif += child_segmentation.0;
                    self_segmentation.append(&mut child_segmentation.1);
                }
                (sum_tau_dif, self_segmentation)
            }
            TbtNode::Sequence(_, left_child, right_child) => {
                let mut candidates = Vec::<(
                    f32,
                    (f32, usize, usize, &TbtNode),
                    (f32, usize, usize, &TbtNode),
                )>::new();
                for u in lower..(upper + 1) {
                    let t1_v = tree_table
                        .lookup_segmentation_tree(left_child, lower, u)
                        .unwrap();
                    let t2_v = if u + 1 > upper {
                        Some(f32::NEG_INFINITY) // ignores t2_v when taking minimum within next if statement
                    } else {
                        tree_table.lookup_segmentation_tree(right_child, u + 1, upper)
                    };
                    if let Some(t2_v) = t2_v {
                        let min_v = f32::min(t1_v, t2_v);
                        candidates.push((
                            min_v,
                            (t1_v, lower, u, left_child),
                            (t2_v, u + 1, upper, right_child),
                        ));
                    } else {
                        candidates.push((
                            t1_v,
                            (t1_v, lower, u, left_child),
                            (f32::INFINITY, u + 1, upper, right_child),
                        ));
                    }
                }
                candidates.sort_by(|(a, _, _), (b, _, _)| b.partial_cmp(a).unwrap());
                candidates.retain(|&(rho, _, _)| rho > rho_dif);
                let mut best_segmentation = (usize::MIN, Vec::new(), Vec::new(), f32::MIN);
                for (v, left_child, right_child) in candidates {
                    let mut left_child_segmentation =
                        left_child.3.get_segmentation_under_restriction(
                            tree_table,
                            formula_table,
                            trace,
                            left_child.1,
                            left_child.2,
                            tau_dif,
                            rho_dif,
                            segmentations,
                        );
                    let mut right_child_segmentation =
                        right_child.3.get_segmentation_under_restriction(
                            tree_table,
                            formula_table,
                            trace,
                            right_child.1,
                            right_child.2,
                            tau_dif,
                            rho_dif,
                            segmentations,
                        );
                    let sum_tau_dif = match left_child_segmentation
                        .0
                        .checked_add(right_child_segmentation.0)
                    {
                        Some(result) => result,
                        None => usize::MAX,
                    };
                    if sum_tau_dif > tau_dif {
                        let mut self_segmentation = vec![(self, lower, upper, v)];
                        self_segmentation.append(&mut left_child_segmentation.1);
                        self_segmentation.append(&mut right_child_segmentation.1);
                        return (sum_tau_dif, self_segmentation);
                    }
                    if sum_tau_dif > best_segmentation.0 {
                        best_segmentation.0 =
                            left_child_segmentation.0 + right_child_segmentation.0;
                        best_segmentation.1 = left_child_segmentation.1;
                        best_segmentation.2 = right_child_segmentation.1;
                        best_segmentation.3 = v;
                    }
                }
                let mut self_segmentation = vec![(self, lower, upper, best_segmentation.3)];
                self_segmentation.append(&mut best_segmentation.1);
                self_segmentation.append(&mut best_segmentation.2);
                (best_segmentation.0, self_segmentation)
            }
            TbtNode::Timeout(_, t, child) => {
                let v = tree_table
                    .lookup_segmentation_tree(child, lower, upper)
                    .unwrap();
                let mut self_segmentation = vec![(self, lower, upper, v)];
                let mut child_segmentation = child.get_segmentation_under_restriction(
                    tree_table,
                    formula_table,
                    trace,
                    lower,
                    usize::min(upper, lower + t - 1),
                    tau_dif,
                    rho_dif,
                    segmentations,
                );
                self_segmentation.append(&mut child_segmentation.1);
                (child_segmentation.0, self_segmentation)
            }
            TbtNode::Kleene(_, n, kleene_next, child) => {
                if lower <= upper && *n > 0 {
                    let kleene_n_minus_1 = kleene_next.as_ref().unwrap();
                    let left_child = child;
                    let right_child = kleene_n_minus_1;
                    // Copy of Sequence start!
                    let mut candidates = Vec::<(
                        f32,
                        (f32, usize, usize, &TbtNode),
                        (f32, usize, usize, &TbtNode),
                    )>::new();
                    for u in lower..(upper + 1) {
                        let t1_v = tree_table
                            .lookup_segmentation_tree(left_child, lower, u)
                            .unwrap();
                        let t2_v = if u + 1 > upper {
                            Some(f32::NEG_INFINITY) // ignores t2_v when taking minimum within next if statement
                        } else {
                            tree_table.lookup_segmentation_tree(right_child, u + 1, upper)
                        };
                        if let Some(t2_v) = t2_v {
                            let min_v = f32::min(t1_v, t2_v);
                            candidates.push((
                                min_v,
                                (t1_v, lower, u, left_child),
                                (t2_v, u + 1, upper, right_child),
                            ));
                        } else {
                            candidates.push((
                                t1_v,
                                (t1_v, lower, u, left_child),
                                (f32::INFINITY, u + 1, upper, right_child),
                            ));
                        }
                    }
                    candidates.sort_by(|(a, _, _), (b, _, _)| b.partial_cmp(a).unwrap());
                    candidates.retain(|&(rho, _, _)| rho > rho_dif);
                    let mut best_segmentation = (usize::MIN, Vec::new(), Vec::new(), f32::MIN);
                    for (v, left_child, right_child) in candidates {
                        let mut left_child_segmentation =
                            left_child.3.get_segmentation_under_restriction(
                                tree_table,
                                formula_table,
                                trace,
                                left_child.1,
                                left_child.2,
                                tau_dif,
                                rho_dif,
                                segmentations,
                            );
                        let mut right_child_segmentation =
                            right_child.3.get_segmentation_under_restriction(
                                tree_table,
                                formula_table,
                                trace,
                                right_child.1,
                                right_child.2,
                                tau_dif,
                                rho_dif,
                                segmentations,
                            );
                        let sum_tau_dif = left_child_segmentation.0 + right_child_segmentation.0;
                        if sum_tau_dif > tau_dif {
                            let mut self_segmentation = vec![(self, lower, upper, v)];
                            self_segmentation.append(&mut left_child_segmentation.1);
                            self_segmentation.append(&mut right_child_segmentation.1);
                            return (sum_tau_dif, self_segmentation);
                        }
                        if sum_tau_dif > best_segmentation.0 {
                            best_segmentation.0 =
                                left_child_segmentation.0 + right_child_segmentation.0;
                            best_segmentation.1 = left_child_segmentation.1;
                            best_segmentation.2 = right_child_segmentation.1;
                            best_segmentation.3 = v;
                        }
                    }
                    let mut self_segmentation = vec![(self, lower, upper, best_segmentation.3)];
                    self_segmentation.append(&mut best_segmentation.1);
                    self_segmentation.append(&mut best_segmentation.2);
                    (best_segmentation.0, self_segmentation)
                    // Copy of Sequence end!
                } else if *n == 0 && lower < upper {
                    let v = tree_table
                        .lookup_segmentation_tree(child, lower, upper)
                        .unwrap();
                    let mut child_segmentation = child.get_segmentation_under_restriction(
                        tree_table,
                        formula_table,
                        trace,
                        lower,
                        upper,
                        tau_dif,
                        rho_dif,
                        segmentations,
                    );
                    let mut self_segmentation = vec![(self, lower, upper, v)];
                    self_segmentation.append(&mut child_segmentation.1);
                    (child_segmentation.0, self_segmentation)
                } else {
                    (0, vec![(self, lower, upper, f32::INFINITY)])
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_alternative_segmentation(
        &self,
        tree_table: &mut Table,
        formula_table: &mut Table,
        trace: &Trace,
        lower: usize,
        upper: usize,
        best_segmentation: &Segmentation,
        tau_dif: usize,
        rho_dif: f32,
        number: usize,
        print_leaf_segments_only: bool,
    ) -> Vec<(usize, Segmentation)> {
        println!("\n\nAlternatives:");
        let mut res_segmentation = Vec::new();
        let mut segmentations = vec![best_segmentation.to_vec()];
        for i in 0..number {
            println!("Got {}/{} alternative segmentations.", i, number);
            let segmentation = self.get_segmentation_under_restriction(
                tree_table,
                formula_table,
                trace,
                lower,
                upper,
                tau_dif,
                rho_dif,
                &segmentations,
            );
            let (robustness_value, segmentation_str) =
                print_segmentation(&segmentation.1, print_leaf_segments_only, false);
            println!("Segmentation with remaining tau difference of {} and robustness of {robustness_value} is:\n{segmentation_str}", segmentation.0);
            segmentations.push(segmentation.1.to_vec());
            res_segmentation.push(segmentation);
        }
        res_segmentation
    }
}

/*******************************
 * Progress and print functions
 *******************************/

fn progress(tree_table: &mut Table, formula_table: &mut Table, system_time: &SystemTime) {
    let (tree_set_calls, tree_total) = tree_table.progress();
    let (formula_set_calls, formula_total) = formula_table.progress();
    if tree_set_calls % 10000 == 0 || formula_set_calls % 10000 == 0 {
        print!("\r");
        print!(
            "Status after {} seconds, tree: {} % ({} / {}),  formula: {} % ({} / {})",
            system_time.elapsed().unwrap().as_secs(),
            (((tree_set_calls as f32) / (tree_total as f32)) * 100.0) as u32,
            tree_set_calls,
            tree_total,
            (((formula_set_calls as f32) / (formula_total as f32)) * 100.0) as u32,
            formula_set_calls,
            formula_total
        );
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}

pub fn print_segmentation(
    segmentation: &Segmentation,
    only_leaves: bool,
    is_lazy: bool,
) -> (f32, String) {
    let mut robustness = f32::INFINITY;
    let mut seg_string = String::new();
    for (tbt_node, lower, upper, value) in segmentation {
        if only_leaves {
            match tbt_node {
                TbtNode::Leaf(_, _, _) => (),
                _ => continue,
            }
        }
        seg_string += &format!(
            "lower: {:10}   upper: {:10}   value: {:15}  segment: {}\n",
            lower,
            upper,
            value,
            tbt_node.pretty_print(false, 0),
        );
        if value < &robustness {
            robustness = *value;
        }
    }
    if is_lazy && robustness < 0.0 {
        (f32::NEG_INFINITY, seg_string)
    } else {
        (robustness, seg_string)
    }
}
