// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{table::Table, ApF, Trace};
use std::collections::HashMap;

type SubformulaIdx = usize;

pub static mut FORMULACOUNT: SubformulaIdx = 0;
// get formula count
fn gfc() -> SubformulaIdx {
    unsafe {
        FORMULACOUNT += 1;
        FORMULACOUNT - 1
    }
}
pub fn stl_reset_count() {
    unsafe { FORMULACOUNT = 0 }
}
#[derive(Clone)]
#[allow(dead_code)]
pub enum Stl {
    Atomic(SubformulaIdx, Vec<String>, ApF),
    Conjunction(SubformulaIdx, Box<Stl>, Box<Stl>),
    Disjunction(SubformulaIdx, Box<Stl>, Box<Stl>),
    Neg(SubformulaIdx, Box<Stl>),
    Next(SubformulaIdx, Box<Stl>),
    Eventually(SubformulaIdx, Box<Stl>),
    Globally(SubformulaIdx, Box<Stl>),
    Until(SubformulaIdx, Box<Stl>, Box<Stl>),
    EventuallyInterval(SubformulaIdx, usize, usize, Box<Stl>),
    GloballyInterval(SubformulaIdx, usize, usize, Box<Stl>),
    UntilInterval(SubformulaIdx, usize, usize, Box<Stl>, Box<Stl>),
}

// Constructors
#[allow(dead_code)]
impl Stl {
    pub fn get_number_formulas() -> usize {
        unsafe { FORMULACOUNT }
    }
    pub fn atomic(name: Vec<String>, f: ApF) -> Self {
        Stl::Atomic(gfc(), name, f)
    }
    pub fn conjunction(left_child: Stl, right_child: Stl) -> Self {
        Stl::Conjunction(gfc(), Box::new(left_child), Box::new(right_child))
    }
    pub fn disjunction(left_child: Stl, right_child: Stl) -> Self {
        Stl::Disjunction(gfc(), Box::new(left_child), Box::new(right_child))
    }
    pub fn neg(child: Stl) -> Self {
        Stl::Neg(gfc(), Box::new(child))
    }
    pub fn next(child: Stl) -> Self {
        Stl::Next(gfc(), Box::new(child))
    }
    pub fn eventually(child: Stl) -> Self {
        Stl::Eventually(gfc(), Box::new(child))
    }
    pub fn globally(child: Stl) -> Self {
        Stl::Globally(gfc(), Box::new(child))
    }
    pub fn until(left_child: Stl, right_child: Stl) -> Self {
        Stl::Until(gfc(), Box::new(left_child), Box::new(right_child))
    }
    pub fn eventually_interval(lower: usize, upper: usize, child: Stl) -> Self {
        Stl::EventuallyInterval(gfc(), lower, upper, Box::new(child))
    }
    pub fn globally_interval(lower: usize, upper: usize, child: Stl) -> Self {
        Stl::GloballyInterval(gfc(), lower, upper, Box::new(child))
    }
    pub fn until_interval(lower: usize, upper: usize, left_child: Stl, right_child: Stl) -> Self {
        Stl::UntilInterval(
            gfc(),
            lower,
            upper,
            Box::new(left_child),
            Box::new(right_child),
        )
    }
}

// Functions
impl Stl {
    pub fn get_atomics(&self) -> Vec<&Stl> {
        match self {
            Stl::Atomic(_, _, _) => {
                vec![self]
            }
            Stl::Conjunction(_, l_child, r_child)
            | Stl::Until(_, l_child, r_child)
            | Stl::UntilInterval(_, _, _, l_child, r_child)
            | Stl::Disjunction(_, l_child, r_child) => {
                let mut atomics = l_child.get_atomics();
                atomics.append(&mut r_child.get_atomics());
                atomics
            }
            Stl::Neg(_, child)
            | Stl::Next(_, child)
            | Stl::Eventually(_, child)
            | Stl::Globally(_, child)
            | Stl::EventuallyInterval(_, _, _, child)
            | Stl::GloballyInterval(_, _, _, child) => child.get_atomics(),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Stl::Atomic(index, _, _) => {
                // let mut s_concat = String::from("");
                // for parameter in s {
                //     s_concat.push_str(parameter);
                //     s_concat.push(',');
                // }
                format!("AP({})", index)
            }
            Stl::Conjunction(_, l_child, r_child) => format!(
                "({} and {})",
                l_child.pretty_print(),
                r_child.pretty_print()
            ),
            Stl::Disjunction(_, l_child, r_child) => {
                format!("({} or {})", l_child.pretty_print(), r_child.pretty_print())
            }
            Stl::Neg(_, child) => format!("!({})", child.pretty_print()),
            Stl::Next(_, child) => format!("X({})", child.pretty_print()),
            Stl::Eventually(_, child) => format!("F({})", child.pretty_print()),
            Stl::Globally(_, child) => format!("G({})", child.pretty_print()),
            Stl::Until(_, l_child, r_child) => {
                format!("({} U {})", l_child.pretty_print(), r_child.pretty_print())
            }
            Stl::EventuallyInterval(_, l, u, child) => {
                format!("F[{l},{u}]({})", child.pretty_print())
            }
            Stl::GloballyInterval(_, l, u, child) => {
                format!("G[{l},{u}]({})", child.pretty_print())
            }
            Stl::UntilInterval(_, l, u, l_child, r_child) => {
                format!(
                    "({} U[{l},{u}] {})",
                    l_child.pretty_print(),
                    r_child.pretty_print()
                )
            }
        }
    }
}

impl Stl {
    pub fn evaluate_fnc(
        &self,
        names: &Vec<String>,
        trace: &(usize, HashMap<String, Vec<f32>>),
        lower: usize,
        function: &ApF,
    ) -> f32 {
        let mut values = Vec::<f32>::new();
        for name in names {
            let trace_for_atomic = trace.1.get(name).unwrap();
            let v = match trace_for_atomic.get(lower) {
                Some(v) => *v,
                None => f32::NEG_INFINITY,
            };
            values.push(v);
        }
        function(&values)
    }

    pub fn evaluate(
        &self,
        table: &mut Table,
        trace: &Trace,
        lower: usize,
        upper: usize,
        is_lazy: bool,
    ) -> f32 {
        // Lookup table
        let res = if lower <= upper {
            match self {
                Stl::Atomic(index, _, _)
                | Stl::Conjunction(index, _, _)
                | Stl::Disjunction(index, _, _)
                | Stl::Neg(index, _)
                | Stl::Next(index, _)
                | Stl::Eventually(index, _)
                | Stl::Globally(index, _)
                | Stl::Until(index, _, _)
                | Stl::EventuallyInterval(index, _, _, _)
                | Stl::GloballyInterval(index, _, _, _)
                | Stl::UntilInterval(index, _, _, _, _) => table.lookup(*index, lower, upper),
            }
        } else {
            None
        };
        // Return previous computed result or start computation otherwise
        if let Some(value) = res {
            value
        } else {
            let (v, index) = match self {
                Stl::Atomic(index, names, function) => {
                    let v = if lower <= upper {
                        self.evaluate_fnc(names, trace, lower, function)
                    } else {
                        f32::NEG_INFINITY
                    };
                    (v, *index)
                }
                Stl::Conjunction(index, l_child, r_child) => {
                    let v = f32::min(
                        (l_child).evaluate(table, trace, lower, upper, is_lazy),
                        (r_child).evaluate(table, trace, lower, upper, is_lazy),
                    );
                    (v, *index)
                }
                Stl::Disjunction(index, l_child, r_child) => {
                    let v = f32::max(
                        (l_child).evaluate(table, trace, lower, upper, is_lazy),
                        (r_child).evaluate(table, trace, lower, upper, is_lazy),
                    );
                    (v, *index)
                }
                Stl::Neg(index, child) => {
                    let v = -1.0 * child.evaluate(table, trace, lower, upper, is_lazy);
                    (v, *index)
                }
                Stl::Next(index, child) => {
                    let v = child.evaluate(table, trace, lower + 1, upper, is_lazy);
                    (v, *index)
                }
                Stl::Eventually(index, child) => {
                    let mut v = f32::NEG_INFINITY;
                    for i in lower..(upper + 1) {
                        v = f32::max(v, child.evaluate(table, trace, i, upper, is_lazy));
                        if is_lazy && v > 0.0 {
                            break;
                        }
                    }
                    (v, *index)
                }
                Stl::Globally(index, child) => {
                    let mut v = f32::INFINITY;
                    for i in lower..(upper + 1) {
                        v = f32::min(v, child.evaluate(table, trace, i, upper, is_lazy));
                        if is_lazy && v < 0.0 {
                            break;
                        }
                    }
                    (v, *index)
                }
                Stl::Until(index, l_child, r_child) => {
                    let mut v: f32 = f32::NEG_INFINITY;
                    for i in lower..(upper + 1) {
                        let mut min_v = r_child.evaluate(table, trace, i, upper, is_lazy);
                        for j in lower..i {
                            let l_v = l_child.evaluate(table, trace, j, upper, is_lazy);
                            min_v = f32::min(min_v, l_v);
                        }
                        v = f32::max(v, min_v);
                        if is_lazy && v > 0.0 {
                            break;
                        }
                    }
                    (v, *index)
                }
                Stl::EventuallyInterval(index, l, u, child) => {
                    let mut v = f32::NEG_INFINITY;
                    let u = usize::min(upper, *u);
                    for i in *l..(u + 1) {
                        let child_robustness =
                            child.evaluate(table, trace, lower + i, upper, is_lazy);
                        v = f32::max(v, child_robustness);
                        if is_lazy && v > 0.0 {
                            break;
                        }
                    }
                    (v, *index)
                }
                Stl::GloballyInterval(index, l, u, child) => {
                    let mut v = f32::INFINITY;
                    let u = usize::min(upper, *u);
                    if *l > u {
                        v = f32::NEG_INFINITY;
                    } else {
                        for i in *l..(u + 1) {
                            let child_robustness =
                                child.evaluate(table, trace, lower + i, upper, is_lazy);
                            v = f32::min(v, child_robustness);
                            if is_lazy && v < 0.0 {
                                break;
                            }
                        }
                    }
                    (v, *index)
                }
                Stl::UntilInterval(index, l, u, l_child, r_child) => {
                    let mut v: f32 = f32::NEG_INFINITY;
                    let u = usize::min(upper, *u);
                    for i in *l..(u + 1) {
                        let mut min_v = r_child.evaluate(table, trace, lower + i, upper, is_lazy);
                        for j in *l..i {
                            let l_v = l_child.evaluate(table, trace, lower + j, upper, is_lazy);
                            min_v = f32::min(min_v, l_v);
                        }
                        v = f32::max(v, min_v);
                        if is_lazy && v > 0.0 {
                            break;
                        }
                    }
                    (v, *index)
                }
            };
            // Store result in table for next access
            if lower <= upper {
                table.set(index, lower, upper, v);
            }
            v
        }
    }
}
