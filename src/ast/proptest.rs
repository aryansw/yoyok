#[cfg(test)]
mod tests {
    use colored::Colorize;
    use proptest::prelude::*;

    use crate::{
        ast::ast::{Expression, Operator, Sequence},
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

    fn arb_expr() -> impl Strategy<Value = Expression> {
        let leaf = prop_oneof![
            any::<u64>().prop_map(Expression::Number),
            "[A-Z][a-zA-Z0-9]*".prop_map(Expression::Reference)
        ];
        leaf.prop_recursive(10, 512, 12, |inner| {
            prop_oneof![
                (inner.clone(), arb_operator(), inner.clone()).prop_map(|(lhs, op, rhs)| {
                    Expression::Binary {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    }
                }),
                ("[A-Z][a-zA-Z0-9]*", inner.clone()).prop_map(|(name, value)| {
                    Expression::Var {
                        name,
                        value: Box::new(value),
                    }
                }),
                ("[A-Z][a-zA-Z0-9]*", inner.clone()).prop_map(|(name, value)| {
                    Expression::Let {
                        name,
                        value: Box::new(value),
                    }
                }),
                (
                    inner.clone(),
                    prop::collection::vec(inner.clone(), 1..10).prop_map(|seq| Sequence(seq)),
                    prop::collection::vec(inner.clone(), 1..10).prop_map(|seq| Sequence(seq))
                )
                    .prop_map(|(cond, then, else_)| {
                        Expression::If {
                            cond: Box::new(cond),
                            then,
                            else_: Some(else_),
                        }
                    }),
            ]
        })
    }

    fn arb_seq() -> impl Strategy<Value = Sequence> {
        prop::collection::vec(arb_expr(), 1..10).prop_map(|seq| Sequence(seq))
    }

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
