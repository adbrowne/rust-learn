use std::fmt;
use proptest::prelude::*;

#[derive(Debug, Clone)]
pub struct IntegerValue {
    value: i32
}

impl fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Debug, Clone)]
pub enum Expression{
    Integer(IntegerValue),
    Function(FunctionCall),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Integer(IntegerValue{value}) => write!(f, "int({})", value),
            Expression::Function(call) => write!(f,"{}", call),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall{
    name : String,
    arguments : Vec<Expression>,
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}(", self.name)?;
        for (count, v) in self.arguments.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f,")")
    }
}

fn interpret(expr: &Expression) -> i32 {
    match expr {
        Expression::Integer(IntegerValue{value}) => *value,
        Expression::Function(FunctionCall{name, arguments}) =>
            match name.as_str() {
                "plus" => interpret(&arguments[0]) + interpret(&arguments[1]),
                x => panic!("unknown function {}", x),
            },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_integer_expression_evaluates_to_itself() {
        let result = interpret(&Expression::Integer(IntegerValue { value: 1 }));
        assert_eq!(result, 1);
    }

    #[test]
    fn plus_function_is_evaluated() {
        let result = interpret(&Expression::Function(
                FunctionCall { 
                    name: String::from("plus"), 
                    arguments: vec![
                        Expression::Integer(IntegerValue { value: 1 }),
                        Expression::Integer(IntegerValue { value: 1 }),
                    ] 
                }
                )
            );
        assert_eq!(result, 2);
    }

    prop_compose! {
        fn arb_integer_value()(value in 0i32..10000) -> IntegerValue { 
            IntegerValue{value}
        }
    }

    fn expression_strategy() -> impl Strategy<Value = Expression> {
      let leaf = 
      prop_oneof![
        arb_integer_value().prop_map(Expression::Integer),
      ];

      leaf.prop_recursive(
          8,
          256,
          10,
          |left| prop_oneof![
            prop::collection::vec(left.clone(), 2).prop_map(|arguments| Expression::Function(FunctionCall { name : String::from("plus"), arguments }) )
          ]
      )

    }


    proptest! {
    #[test]
    fn doesnt_crash(e in expression_strategy()) {
        interpret(&e);
    }
}

}

