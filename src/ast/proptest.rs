use proptest::{option, prelude::*};

use crate::{
    ast::ast::{Expression, Operator, Sequence, Size, Type, Value},
    parser::parser::parse,
};
fn arb_operator() -> impl Strategy<Value = Operator> {
    prop_oneof![
        Just(Operator::Add),
        Just(Operator::Sub),
        Just(Operator::Mul),
        Just(Operator::Div),
        Just(Operator::Assign),
        Just(Operator::Gt),
    ]
}

fn arb_size() -> impl Strategy<Value = Size> {
    prop_oneof![
        Just(Size::Eight),
        Just(Size::Sixteen),
        Just(Size::ThirtyTwo),
        Just(Size::SixtyFour),
    ]
}

fn arb_type() -> impl Strategy<Value = Type> {
    let leaf = prop_oneof![
        arb_size().prop_map(Type::Signed),
        arb_size().prop_map(Type::Unsigned),
        arb_size().prop_map(Type::Float),
        Just(Type::Bool),
        Just(Type::Char),
    ];
    leaf.prop_recursive(5, 22, 12, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 1..4).prop_map(|seq| Type::Tuple(seq)),
            (inner.clone(), any::<u64>()).prop_map(|(ty, size)| Type::Array(Box::new(ty), size)),
            (inner.clone(), inner.clone()).prop_map(|(args, ret)| Type::Function {
                args: Box::new(args),
                ret: Box::new(ret)
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
    leaf.prop_recursive(12, 512, 10, |inner| {
        prop_oneof![
            (inner.clone(), arb_operator(), inner.clone()).prop_map(|(lhs, op, rhs)| {
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
        ]
    })
}

pub fn arb_seq() -> impl Strategy<Value = Sequence> {
    prop::collection::vec(arb_expr(), 1..10).prop_map(|seq| Sequence(seq))
}

#[cfg(test)]
mod tests {
    use crate::{ast::proptest::arb_seq, parse};
    use colored::Colorize;
    use proptest::prelude::*;

    proptest! {
        // Any generated AST should be parsable (i.e. round-trip)
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn proptest_parse(expr in arb_seq()){
            let src = format!("{}", expr);
            let _parse = parse(&src).inspect_err(|_e| println!("\n{}\n{}", format!("Test input:").bright_red(), src))?;
        }
    }
}
