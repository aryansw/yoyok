use proptest::{option, prelude::*};

use crate::ast::ast::{Expression, Operator, Sequence, Size, Type, Value};

use super::ast::{Function, Program};

fn arb_binary() -> impl Strategy<Value = Operator> {
    prop_oneof![
        Just(Operator::Add),
        Just(Operator::Sub),
        Just(Operator::Mul),
        Just(Operator::Div),
        Just(Operator::And),
        Just(Operator::Or),
        Just(Operator::Assign),
        Just(Operator::Eq),
        Just(Operator::Neq),
        Just(Operator::Lt),
        Just(Operator::Lte),
        Just(Operator::Gt),
        Just(Operator::Gte),
    ]
}

fn arb_unary() -> impl Strategy<Value = Operator> {
    prop_oneof![Just(Operator::Not), Just(Operator::Sub)]
}

fn arb_size() -> impl Strategy<Value = Size> {
    prop_oneof![
        Just(Size::ThirtyTwo),
    ]
}

fn arb_type() -> impl Strategy<Value = Type> {
    let leaf = prop_oneof![
        arb_size().prop_map(Type::Signed),
        Just(Type::Bool),
        Just(Type::Char),
    ];
    leaf.prop_recursive(5, 22, 12, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 1..4).prop_map(|seq| Type::Tuple(seq)),
            (inner.clone(), any::<usize>()).prop_map(|(ty, size)| Type::Array(Box::new(ty), size)),
            (prop::collection::vec(inner.clone(), 1..4), inner.clone()).prop_map(|(args, ret)| {
                Type::Function {
                    args: args,
                    ret: Box::new(ret),
                }
            }),
        ]
    })
}

fn arb_opt_type() -> impl Strategy<Value = Option<Type>> {
    option::weighted(0.8, arb_type())
}

fn arb_expr() -> impl Strategy<Value = Expression> {
    let leaf = prop_oneof![
        any::<u64>().prop_map(|x| Expression::Value(x.into())),
        "[A-Z][a-zA-Z0-9]*".prop_map(Expression::Reference),
        any::<bool>().prop_map(|x| Expression::Value(x.into())),
        "[A-Z][a-zA-Z0-9]*".prop_map(|x| Expression::Value(Value::String(x))),
        "[A-Z][a-zA-Z0-9]+".prop_map(|x| Expression::Value(Value::Char(x.chars().next().unwrap()))),
    ];
    leaf.prop_recursive(12, 512, 12, |inner| {
        prop_oneof![
            (arb_unary(), inner.clone()).prop_map(|(op, rhs)| Expression::Unary {
                op,
                rhs: Box::new(rhs)
            }),
            (inner.clone(), arb_binary(), inner.clone()).prop_map(|(lhs, op, rhs)| {
                Expression::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                }
            }),
            (
                "[A-Z][a-zA-Z0-9]*",
                arb_opt_type(),
                inner.clone(),
                any::<bool>()
            )
                .prop_map(|(name, ty, value, mutable)| {
                    Expression::Let {
                        name,
                        ty,
                        value: Box::new(value),
                        mutable,
                    }
                }),
            (
                inner.clone(),
                prop::collection::vec(inner.clone(), 1..10).prop_map(Sequence),
                prop::collection::vec(inner.clone(), 1..10).prop_map(Sequence)
            )
                .prop_map(|(cond, then, else_)| {
                    Expression::If {
                        cond: Box::new(cond),
                        then,
                        else_: Some(else_),
                    }
                }),
            prop::collection::vec(inner.clone(), 0..10).prop_map(Expression::Array),
            prop::collection::vec(inner.clone(), 0..10).prop_map(Expression::Tuple),
            (inner.clone(), prop::collection::vec(inner.clone(), 0..10)).prop_map(
                |(func, args)| {
                    Expression::Call {
                        func: Box::new(func),
                        args,
                    }
                }
            ),
            (
                inner.clone(),
                prop::collection::vec(inner.clone(), 1..10).prop_map(Sequence)
            )
                .prop_map(|(cond, body)| {
                    Expression::While {
                        cond: Box::new(cond),
                        body,
                    }
                }),
        ]
    })
}

fn arb_seq() -> impl Strategy<Value = Sequence> {
    prop::collection::vec(arb_expr(), 1..10).prop_map(|seq| Sequence(seq))
}

fn arb_func() -> impl Strategy<Value = Function> {
    // name, args, ret and body
    (
        "[A-Z][a-zA-Z0-9]*",
        (
            prop::collection::vec("[A-Z][a-zA-Z0-9]*", 1..10),
            arb_type(),
        )
            .prop_map(|(names, ty)| {
                names
                    .into_iter()
                    .map(|name| (name, ty.clone()))
                    .collect::<Vec<_>>()
            }),
        arb_type(),
        arb_seq(),
    )
        .prop_map(|(name, args, ret, body)| Function {
            name,
            args,
            ret,
            body,
        })
}

pub fn arb_prgm() -> impl Strategy<Value = Program> {
    prop::collection::vec(arb_func(), 1..10).prop_map(Program)
}

#[cfg(test)]
mod tests {
    use crate::{ast::proptest::arb_prgm, parse};
    use colored::Colorize;
    use proptest::prelude::*;

    proptest! {
        // Any generated AST should be parsable (i.e. round-trip)
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn proptest_parse(prgm in arb_prgm()){
            let src = format!("{}", prgm);
            let _parse = parse(&src).inspect_err(|_e| println!("\n{}\n{}", format!("Test input:").bright_red(), src))?;
        }

    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn proptest_parse2(prgm in arb_prgm()){
            let ast = parse(&format!("{}", prgm))?;
            // Round-trip (ignore first one because it is not guaranteed to be the same)
            let ast1 = parse(&format!("{}", ast))?;
            let ast2 = parse(&format!("{}", ast1))?;
            prop_assert_eq!(ast1, ast2, "Round-trip failed");
        }
    }
}
