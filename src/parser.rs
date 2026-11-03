// SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
// SPDX-License-Identifier: Apache-2.0

use crate::{behaviortree::TbtNode, functions::Func, stl::Stl};
use core::panic;
use pest::{error::ErrorVariant, Parser};
use pest_derive::Parser;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Parser)]
#[grammar = "tbt.pest"]
struct TBTParser;

pub fn parse(
    formula: String,
    events_per_second: usize,
    use_prints: bool,
) -> Result<TbtNode, pest::error::Error<Rule>> {
    let mut pairs = TBTParser::parse(Rule::Specification, &formula)?;
    if use_prints {
        println!("Parsing successful!");
    }
    let spec_pair = pairs.next().unwrap();
    assert!(spec_pair.as_rule() == Rule::Specification);
    let mut spec_iter = spec_pair.into_inner();
    let constant_map = parse_constants(spec_iter.next().unwrap())?;
    if use_prints {
        println!("Constants: {:#?}", constant_map);
    }
    parse_tbt(
        spec_iter.next().unwrap(),
        events_per_second,
        &constant_map,
        use_prints,
    )
}

fn parse_constants(
    constants_pair: pest::iterators::Pair<'_, Rule>,
) -> Result<HashMap<String, f32>, pest::error::Error<Rule>> {
    assert!(constants_pair.as_rule() == Rule::Constants);
    let mut map = HashMap::<String, f32>::new();
    for constant_pair in constants_pair.into_inner() {
        let mut c_pair = constant_pair.into_inner();
        let name = c_pair.next().unwrap().as_str().to_string();
        let val = c_pair
            .next()
            .unwrap()
            .as_str()
            .trim()
            .parse::<f32>()
            .unwrap();
        map.insert(name, val);
    }
    Ok(map)
}

fn parse_tbt(
    tbt_pair: pest::iterators::Pair<'_, Rule>,
    events_per_second: usize,
    constant_map: &HashMap<String, f32>,
    use_prints: bool,
) -> Result<TbtNode, pest::error::Error<Rule>> {
    match tbt_pair.as_rule() {
        Rule::Leaf => {
            let mut leaf_tokens = tbt_pair.into_inner();
            let mut p = leaf_tokens.next().unwrap();
            let mut name = "";
            if p.as_rule() == Rule::Word {
                name = p.as_str();
                p = leaf_tokens.next().unwrap();
            }
            assert!(p.as_rule() == Rule::STL);
            Ok(TbtNode::leaf(
                parse_stl(p, events_per_second, constant_map, use_prints)?,
                name.to_string(),
            ))
        }
        Rule::Fallback => {
            // read arguments
            let p = tbt_pair.into_inner().next().unwrap();
            assert!(p.as_rule() == Rule::TBTList);
            let tbt_list = parse_tbt_list(p, events_per_second, constant_map, use_prints)?;
            // build TBT Node
            Ok(TbtNode::fallback(tbt_list))
        }
        Rule::Parallel => {
            let mut arguments = tbt_pair.into_inner();
            // read m
            let p = arguments.next().unwrap();
            assert!(p.as_rule() == Rule::Number);
            let m = p.as_str().trim().parse::<usize>().unwrap();
            // read arguments
            let p = arguments.next().unwrap();
            assert!(p.as_rule() == Rule::TBTList);
            let tbt_list = parse_tbt_list(p, events_per_second, constant_map, use_prints)?;
            // build TBT Node
            Ok(TbtNode::parallel(m, tbt_list))
        }
        Rule::Sequence => {
            // read arguments
            let p = tbt_pair.into_inner().next().unwrap();
            assert!(p.as_rule() == Rule::TBTList);
            // build TBT Node by stacking sequence nodes
            let mut tbt_list = parse_tbt_list(p, events_per_second, constant_map, use_prints)?;
            let mut tbt_node = tbt_list[tbt_list.len() - 1].clone();
            tbt_list.reverse();
            for tbt in &tbt_list[1..] {
                tbt_node = TbtNode::sequence(tbt.clone(), tbt_node)
            }
            Ok(tbt_node)
        }
        Rule::Timeout => {
            let mut arguments = tbt_pair.into_inner();
            // read m
            let p = arguments.next().unwrap();
            assert!(p.as_rule() == Rule::Number);
            let t = p.as_str().trim().parse::<usize>().unwrap() * events_per_second;
            // read arguments
            let p = arguments.next().unwrap();
            assert!(
                p.as_rule() == Rule::Leaf
                    || p.as_rule() == Rule::Fallback
                    || p.as_rule() == Rule::Sequence
                    || p.as_rule() == Rule::Timeout
                    || p.as_rule() == Rule::Kleene
            );
            let child = parse_tbt(p, events_per_second, constant_map, use_prints)?;
            // build TBT Node
            Ok(TbtNode::timeout(t, child))
        }
        Rule::Kleene => {
            let mut arguments = tbt_pair.into_inner();
            // read m
            let p = arguments.next().unwrap();
            assert!(p.as_rule() == Rule::Number);
            let n = p.as_str().trim().parse::<usize>().unwrap();
            // read arguments
            let p = arguments.next().unwrap();
            assert!(
                p.as_rule() == Rule::Leaf
                    || p.as_rule() == Rule::Fallback
                    || p.as_rule() == Rule::Sequence
                    || p.as_rule() == Rule::Timeout
                    || p.as_rule() == Rule::Kleene
            );
            let child = parse_tbt(p, events_per_second, constant_map, use_prints)?;
            // build TBT Node
            Ok(TbtNode::kleene(n, child))
        }
        Rule::STL => Ok(TbtNode::leaf(
            parse_stl(tbt_pair, events_per_second, constant_map, use_prints)?,
            "".to_string(),
        )), // easy workaround to parse TBT list (parse_tbtlist() already accesses inner and calls parse_tbt which accesses inner again)
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: vec![
                    Rule::Leaf,
                    Rule::Fallback,
                    Rule::Parallel,
                    Rule::Sequence,
                    Rule::Timeout,
                    Rule::Kleene,
                ],
                negatives: vec![tbt_pair.as_rule()],
            },
            tbt_pair.as_span(),
        )),
    }
}

fn parse_tbt_list(
    tbt_pair: pest::iterators::Pair<'_, Rule>,
    events_per_second: usize,
    constant_map: &HashMap<String, f32>,
    use_prints: bool,
) -> Result<Vec<TbtNode>, pest::error::Error<Rule>> {
    let mut tbt_list = vec![];
    for pair in tbt_pair.into_inner() {
        let tbt = parse_tbt(pair, events_per_second, constant_map, use_prints)?;
        tbt_list.push(tbt);
    }
    assert!(!tbt_list.is_empty());
    Ok(tbt_list)
}

fn parse_stl(
    stl_pair: pest::iterators::Pair<'_, Rule>,
    events_per_second: usize,
    constant_map: &HashMap<String, f32>,
    use_prints: bool,
) -> Result<Stl, pest::error::Error<Rule>> {
    assert!(stl_pair.as_rule() == Rule::STL);
    // Precedence
    let mut precedence_map = HashMap::new(); // value: precedence value where smaller is stronger and second is true iff left-associative
    let string_conjunction = Stl::get_string_conjunction();
    let string_disjunction = Stl::get_string_disjunction();
    let string_until = Stl::get_string_until();
    let string_until_interval = Stl::get_string_until_interval();
    precedence_map.insert(&string_conjunction, (3, true));
    precedence_map.insert(&string_disjunction, (4, true));
    precedence_map.insert(&string_until, (5, false));
    precedence_map.insert(&string_until_interval, (5, false));

    // Flattening
    let mut p_iter = stl_pair.into_inner();

    let mut children = Vec::<Stl>::new();
    let mut ops = Vec::<(String, usize, usize)>::new();
    let mut is_op = false;
    while p_iter.peek().is_some() {
        let element = p_iter.next().unwrap();
        // for element in p_iter.by_ref() {
        if is_op {
            if element.as_rule() == Rule::Until_interval {
                let mut p = element.into_inner();
                let lower =
                    p.next().unwrap().as_str().trim().parse::<usize>().unwrap() * events_per_second;
                let upper =
                    p.next().unwrap().as_str().trim().parse::<usize>().unwrap() * events_per_second;
                ops.push((Stl::get_string_until_interval(), lower, upper));
            } else {
                ops.push((element.as_str().to_owned(), 1, 0));
            }
        } else {
            children.push(parse_stl_element(
                element,
                events_per_second,
                constant_map,
                use_prints,
            )?);
        }
        is_op = !is_op;
    }
    // similar to shunting yard algorithm but very inefficient todo: implement O(N), currently O(N^2)
    while !ops.is_empty() {
        let mut next = (i32::MAX, 0);
        for (i, op) in ops.iter().enumerate() {
            // Go once through
            let (val, is_left_associative) = precedence_map.get(&op.0).unwrap();
            if *val < next.0 || (*val <= next.0 && !is_left_associative) {
                next = (*val, i);
            }
        }
        // Do the update
        let (op, lower, upper) = ops.remove(next.1);
        let left_child = children.remove(next.1);
        let right_child = children.remove(next.1);
        let new_op = if op == string_conjunction {
            Stl::conjunction(left_child, right_child)
        } else if op == string_disjunction {
            Stl::disjunction(left_child, right_child)
        } else if op == string_until {
            Stl::until(left_child, right_child)
        } else if op == string_until_interval {
            Stl::until_interval(lower, upper, left_child, right_child)
        } else {
            panic!("{}", format!("Unexpected operator: {op}"))
        };
        // Add new child
        children.insert(next.1, new_op);
    }
    let parsed_func = children[0].clone();
    Ok(parsed_func)
}

fn parse_stl_element(
    stl_element_pair: pest::iterators::Pair<'_, Rule>,
    events_per_second: usize,
    constant_map: &HashMap<String, f32>,
    use_prints: bool,
) -> Result<Stl, pest::error::Error<Rule>> {
    assert!(stl_element_pair.as_rule() == Rule::STL_element);
    let pair = stl_element_pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::Atomic => {
            let mut p = pair.into_inner();
            // read word, arguments, and function
            let function_name = p.next().unwrap().as_str();
            let mut value_names = Vec::<String>::new();
            let arguments = p.next().unwrap();
            let arguments_rule = arguments.as_rule();
            let arguments_span = arguments.as_span();
            for word in arguments.into_inner() {
                value_names.push(word.as_str().to_string());
            }
            // check if arguments are unique
            let string_set: HashSet<_> = value_names.iter().cloned().collect();
            if string_set.len() != value_names.len() {
                println!("Atomic function needs to provide unique arguments!");
                Err(pest::error::Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![
                            Rule::STLNeg,
                            Rule::Next,
                            Rule::Eventually,
                            Rule::Globally,
                            Rule::Eventually_interval,
                            Rule::Globally_interval,
                        ],
                        negatives: vec![arguments_rule],
                    },
                    arguments_span,
                ))
            } else {
                // build proposition
                assert!(p.peek().unwrap().as_rule() == Rule::Function);
                let func = parse_fn(p.next().unwrap(), &value_names, constant_map)?;
                let print_fn = func.clone();
                let value_names_cpy = value_names.clone();
                let ap = Stl::atomic_expr(value_names, func);
                if let Stl::Atomic(i, _, _) = ap {
                    if use_prints {
                        println!(
                            "AP({i}) {function_name} ({:?}): {}\n",
                            value_names_cpy, print_fn
                        );
                    }
                }
                Ok(ap)
            }
        }
        Rule::STL_unary_op => {
            let mut p_iter = pair.into_inner();
            let operation = p_iter.next().unwrap();
            let stl_child = parse_stl(
                p_iter.next().unwrap(),
                events_per_second,
                constant_map,
                use_prints,
            )?;
            match operation.as_rule() {
                Rule::STLNeg => Ok(Stl::negation(stl_child)),
                Rule::Next => Ok(Stl::next(stl_child)),
                Rule::Eventually => Ok(Stl::eventually(stl_child)),
                Rule::Globally => Ok(Stl::globally(stl_child)),
                Rule::Eventually_interval => {
                    let mut p = operation.into_inner();
                    let lower = p.next().unwrap().as_str().trim().parse::<usize>().unwrap()
                        * events_per_second;
                    let upper = p.next().unwrap().as_str().trim().parse::<usize>().unwrap()
                        * events_per_second;
                    Ok(Stl::eventually_interval(lower, upper, stl_child))
                }
                Rule::Globally_interval => {
                    let mut p = operation.into_inner();
                    let lower = p.next().unwrap().as_str().trim().parse::<usize>().unwrap()
                        * events_per_second;
                    let upper = p.next().unwrap().as_str().trim().parse::<usize>().unwrap()
                        * events_per_second;
                    Ok(Stl::globally_interval(lower, upper, stl_child))
                }
                _ => Err(pest::error::Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![
                            Rule::STLNeg,
                            Rule::Next,
                            Rule::Eventually,
                            Rule::Globally,
                            Rule::Eventually_interval,
                            Rule::Globally_interval,
                        ],
                        negatives: vec![operation.as_rule()],
                    },
                    operation.as_span(),
                )),
            }
        }
        Rule::Brackets => parse_stl(
            pair.into_inner().next().unwrap(),
            events_per_second,
            constant_map,
            use_prints,
        ),
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: vec![Rule::Atomic, Rule::Brackets, Rule::STL_unary_op],
                negatives: vec![pair.as_rule()],
            },
            pair.as_span(),
        )),
    }
}

fn parse_fn(
    pair: pest::iterators::Pair<'_, Rule>,
    arguments: &Vec<String>,
    constant_map: &HashMap<String, f32>,
) -> Result<Func, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::Function);
    // Precedence according to https://en.wikipedia.org/wiki/Operators_in_C_and_C++
    let mut precedence_map = HashMap::new(); // value: precedence value where smaller is stronger and second is true iff left-associative
    let string_add = Func::get_string_add();
    let string_sub = Func::get_string_sub();
    let string_mul = Func::get_string_mul();
    let string_div = Func::get_string_div();
    let string_mod = Func::get_string_mod();
    let string_pow = Func::get_string_pow();
    precedence_map.insert(&string_add, (6, true));
    precedence_map.insert(&string_sub, (6, true));
    precedence_map.insert(&string_mul, (5, true));
    precedence_map.insert(&string_div, (5, true));
    precedence_map.insert(&string_mod, (5, true));
    precedence_map.insert(&string_pow, (4, false)); // not in precedence but expected to be stronger

    // Flattening
    let mut p_iter = pair.into_inner();

    let mut children = Vec::<Func>::new();
    let mut ops = Vec::<String>::new();
    let mut is_op = false;
    for element in p_iter.by_ref() {
        if is_op {
            ops.push(element.as_str().to_owned());
        } else {
            children.push(parse_function_element(element, arguments, constant_map)?);
        }
        is_op = !is_op;
    }
    // similar to shunting yard algorithm but very inefficient todo: implement O(N), currently O(N^2)
    while !ops.is_empty() {
        let mut next = (i32::MAX, 0);
        for (i, op) in ops.iter().enumerate() {
            // Go once through
            let (val, is_left_associative) = precedence_map.get(&op).unwrap();
            if *val < next.0 || (*val <= next.0 && !is_left_associative) {
                next = (*val, i);
            }
        }
        // Do the update
        let op = ops.remove(next.1);
        let left_child = children.remove(next.1);
        let right_child = children.remove(next.1);
        let new_op = if op == string_add {
            Func::add(left_child, right_child)
        } else if op == string_sub {
            Func::sub(left_child, right_child)
        } else if op == string_mul {
            Func::mul(left_child, right_child)
        } else if op == string_div {
            Func::div(left_child, right_child)
        } else if op == string_mod {
            Func::modulo(left_child, right_child)
        } else if op == string_pow {
            Func::pow(left_child, right_child)
        } else {
            panic!("{}", format!("Unexpected operator: {op}"))
        };
        // Add new child
        children.insert(next.1, new_op);
    }
    let parsed_func = children[0].clone();
    Ok(parsed_func)
}

fn parse_function_element(
    pair: pest::iterators::Pair<'_, Rule>,
    arguments: &Vec<String>,
    constant_map: &HashMap<String, f32>,
) -> Result<Func, pest::error::Error<Rule>> {
    let mut p_iter = pair.into_inner();
    let element = p_iter.next().unwrap();
    match element.as_rule() {
        Rule::Word => {
            let w = element.as_str().trim();
            if let Some(index) = arguments.iter().position(|x| x == w) {
                Ok(Func::Acc(index, arguments.clone()))
            } else {
                let access = constant_map.get(w);
                if let Some(a) = access {
                    Ok(Func::Number(*a))
                } else {
                    println!(
                        "Argument '{w}' was not found in arguments vector {:?}",
                        arguments
                    );
                    Err(pest::error::Error::new_from_span(
                        ErrorVariant::ParsingError {
                            positives: vec![Rule::Word],
                            negatives: vec![element.as_rule()],
                        },
                        element.as_span(),
                    ))
                }
            }
        }
        Rule::Float => {
            let number = element.as_str().trim().parse::<f32>().unwrap();
            Ok(Func::number(number))
        }
        Rule::FnBrackets => parse_fn(
            element.into_inner().next().unwrap(),
            arguments,
            constant_map,
        ),
        Rule::FunctionCall => {
            let mut p_iter = element.into_inner();
            let fn_call = p_iter.next().unwrap().into_inner().next().unwrap();
            let child = parse_fn(p_iter.next().unwrap(), arguments, constant_map)?;
            match fn_call.as_rule() {
                Rule::Root => Ok(Func::sqrt(child)),
                Rule::Abs => Ok(Func::abs(child)),
                Rule::RadToDeg => Ok(Func::rad_to_deg(child)),
                Rule::DegToRad => Ok(Func::deg_to_rad(child)),
                Rule::Cos => Ok(Func::cos(child)),
                Rule::Sin => Ok(Func::sin(child)),
                Rule::Min => {
                    let r_child = parse_fn(p_iter.next().unwrap(), arguments, constant_map)?;
                    Ok(Func::min(child, r_child))
                }
                Rule::Max => {
                    let r_child = parse_fn(p_iter.next().unwrap(), arguments, constant_map)?;
                    Ok(Func::max(child, r_child))
                }
                _ => Err(pest::error::Error::new_from_span(
                    ErrorVariant::ParsingError {
                        positives: vec![
                            Rule::Root,
                            Rule::Abs,
                            Rule::RadToDeg,
                            Rule::DegToRad,
                            Rule::Cos,
                            Rule::Sin,
                            Rule::Min,
                            Rule::Max,
                        ],
                        negatives: vec![fn_call.as_rule()],
                    },
                    fn_call.as_span(),
                )),
            }
        }
        _ => Err(pest::error::Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: vec![
                    Rule::Word,
                    Rule::Float,
                    Rule::FnBrackets,
                    Rule::FunctionCall,
                ],
                negatives: vec![element.as_rule()],
            },
            element.as_span(),
        )),
    }
}
