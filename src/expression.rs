use crate::Context;

use std::clone::Clone;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

/// 表达式  核心对象
/// 一切语法都是表达式
pub trait Expression: Debug {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value>;
}

#[derive(Debug)]
pub struct Subtract {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Subtract {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Int(l_int - r_int)),
            (_, _) => panic!("不是数字不能做求余数运算"),
        }
    }
}

#[derive(Debug)]
pub struct Multiply {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Multiply {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Int(l_int * r_int)),
            (_, _) => panic!("不是数字不能做求余数运算"),
        }
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Divide {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Int(l_int / r_int)),
            (_, _) => panic!("不是数字不能做求余数运算"),
        }
    }
}

#[derive(Debug)]
pub struct NotEquals {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for NotEquals {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        return Some(Value::Bool(l != r));
    }
}

/// 小于
#[derive(Debug)]
pub struct Not {
    pub expr: Box<dyn Expression>,
}

impl Expression for Not {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let res = self.expr.evaluate(ctx).unwrap();
        match res {
            Value::Bool(b) => Some(Value::Bool(!b)),
            _ => panic!("逻辑运算符只能用在 bool 类型上"),
        }
    }
}

/// 小于
#[derive(Debug)]
pub struct LT {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for LT {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Bool(l_int < r_int)),
            (_, _) => panic!("不是数字不能做比较运算"),
        }
    }
}

/// 小于等于
#[derive(Debug)]
pub struct LTE {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for LTE {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Bool(l_int <= r_int)),
            (_, _) => panic!("不是数字不能做比较运算"),
        }
    }
}

/// 大于等于
#[derive(Debug)]
pub struct GTE {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for GTE {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Bool(l_int >= r_int)),
            (_, _) => panic!("不是数字不能做比较运算"),
        }
    }
}

/// 大于
#[derive(Debug)]
pub struct GT {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for GT {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Bool(l_int > r_int)),
            (_, _) => panic!("不是数字不能做比较运算"),
        }
    }
}

/// && 与
#[derive(Debug)]
pub struct And {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for And {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Bool(l_b)), Some(Value::Bool(r_b))) => Some(Value::Bool(l_b && r_b)),
            (_, _) => panic!("不是bool类型不能做逻辑运算"),
        }
    }
}

/// || 或
#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Or {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Bool(l_b)), Some(Value::Bool(r_b))) => Some(Value::Bool(l_b || r_b)),
            (_, _) => panic!("不是bool类型不能做逻辑运算"),
        }
    }
}

/// 括号表达式
#[derive(Debug)]
pub struct Paren {
    pub inner: Box<dyn Expression>,
}

impl Expression for Paren {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        self.inner.evaluate(ctx)
    }
}

#[derive(Debug)]
pub struct Equals {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Equals {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        return Some(Value::Bool(l == r));
    }
}

#[derive(Debug)]
pub struct Mod {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Mod {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Int(l_int % r_int)),
            (_, _) => panic!("不是数字不能做求余数运算"),
        }
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Add {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let l = self.left.evaluate(ctx);
        let r = self.right.evaluate(ctx);
        match (l, r) {
            (Some(Value::Int(l_int)), Some(Value::Int(r_int))) => Some(Value::Int(l_int + r_int)),
            (_, _) => panic!("不是数字不能做加运算"),
        }
    }
}

#[derive(Debug)]
pub struct Println {
    pub expression: Box<dyn Expression>,
}

impl Expression for Println {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let res = self.expression.evaluate(ctx).unwrap();
        ctx.output.push(format!("{}\n", res.to_string()));
        None
    }
}

#[derive(Debug)]
pub struct Print {
    pub expression: Box<dyn Expression>,
}

impl Expression for Print {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let res = self.expression.evaluate(ctx).unwrap();
        ctx.output.push(res.to_string());
        None
    }
}

// 赋值语句
#[derive(Debug)]
pub struct Var {
    pub left: String,
    pub right: Box<dyn Expression>,
}

impl Expression for Var {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let e = &self.right;
        //        dbg!(&e);
        let res = e.evaluate(ctx).unwrap().clone();
        ctx.variables.insert((&self.left).clone(), res);
        None
    }
}

pub type Command = Box<VecDeque<Box<dyn Expression>>>;

impl Expression for Command {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        let mut res = None;
        for expr in self.iter() {
            res = expr.evaluate(ctx);
        }
        res
    }
}

#[derive(Debug)]
pub struct Loop {
    pub predict: Box<dyn Expression>,
    pub cmd: Command,
}

impl Expression for Loop {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        loop {
            match self.predict.evaluate(ctx) {
                Some(Value::Bool(false)) => {
                    break;
                }
                Some(Value::Bool(true)) => {
                    self.cmd.evaluate(ctx);
                }
                _ => {
                    panic!("循环判断条件 返回值 只能是 bool 类型");
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct If {
    pub predict: Box<dyn Expression>,
    pub cmd: Command,
}

impl Expression for If {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        match self.predict.evaluate(ctx) {
            Some(Value::Bool(false)) => {}
            Some(Value::Bool(true)) => {
                self.cmd.evaluate(ctx);
            }
            _ => panic!("if 语句条件只能是 bool 类型"),
        }
        None
    }
}

pub enum Element {
    /// 变量
    Variable(Variable),
    /// 常量
    Value(Value),
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self {
            Element::Value(v) => v.fmt(f),
            Element::Variable(v) => v.fmt(f),
        }
    }
}

impl Expression for Element {
    fn evaluate(&self, ctx: &mut Context) -> Option<Value> {
        match &self {
            Element::Value(v) => v.evaluate(ctx),
            Element::Variable(v) => v.evaluate(ctx),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable {
    pub name: String,
}

impl Expression for Variable {
    fn evaluate(&self, context: &mut Context) -> Option<Value> {
        let val = context.variables.get(&self.name);
        assert!(
            val.is_some(),
            "不能获取一个未定义的变量 {}",
            self.name
        );
        return Some(val.unwrap().clone());
    }
}

/// ----------------------------------------
/// 常数类型
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Value {
    // 仅支持 int  bool类型
    Int(i32),
    Bool(bool),
    Void,
    String(String),
}

impl Expression for Value {
    fn evaluate(&self, _: &mut Context) -> Option<Value> {
        Some(self.clone())
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(int) => (*int).to_string(),
            Value::Bool(b) => (*b).to_string(),
            Value::Void => String::new(),
            Value::String(s) => s.clone(),
            //            Value::Float(f) => f.to_string(),
        }
    }
}
//-----------------------------------------

#[cfg(test)]
mod tests {
    use crate::expression::Element::Value;
    use crate::expression::Value::{Bool, Int};
    use crate::expression::{Add, Expression, Subtract};
    use crate::Context;

    #[test]
    fn test_sub_int_int() {
        let mut ctx = Context::default();
        let add = Subtract {
            left: box Value(Int(1)),
            right: box Value(Int(1)),
        };
        assert_eq!(add.evaluate(&mut ctx), Some(Int(0)));
    }

    #[should_panic]
    #[test]
    fn test_sub_bool_int() {
        let mut ctx = Context::default();
        let add: Box<dyn Expression> = box Add {
            left: box Value(Bool(false)),
            right: box Value(Int(1)),
        };
        add.evaluate(&mut ctx);
    }

    #[test]
    fn test_add_int_int() {
        let mut ctx = Context::default();
        let add = Add {
            left: box Value(Int(1)),
            right: box Value(Int(1)),
        };
        assert_eq!(add.evaluate(&mut ctx), Some(Int(2)));
    }

    #[should_panic]
    #[test]
    fn test_add_bool_int() {
        let mut ctx = Context::default();
        let add = Add {
            left: box Value(Bool(false)),
            right: box Value(Int(1)),
        };
        add.evaluate(&mut ctx);
    }
}
