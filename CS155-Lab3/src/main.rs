use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr{
        ArithExpr(arith_expr) => IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr)  => BoolValue(eval_bool_expr(bool_expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr{
        BinArithExpr{left, right, op} => {
            let lnum = eval_arith_expr(*left);
            let rnum = eval_arith_expr(*right);
            match op{
                AddOp => lnum + rnum,
                SubOp => lnum - rnum,
                MulOp => lnum * rnum,
                IntDivOp => lnum / rnum,
            }
        },
        IntLit(num) => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr{
        ArithCmpExpr{left, right, op} => {
            let lnum = eval_arith_expr(*left);
            let rnum = eval_arith_expr(*right);
            match op {
                LtOp => lnum < rnum,
                LteOp => lnum <= rnum,
                GtOp => lnum > rnum,
                GteOp => lnum >= rnum,
                ArithEqOp => lnum == rnum,
                ArithNeqOp => lnum != rnum,
            }
        },
        BinBoolExpr{left, right, op} => {
            let lbool = eval_bool_expr(*left);
            let rbool = eval_bool_expr(*right);
            match op{
                AndOp => lbool && rbool,
                OrOp => lbool || rbool,
                BoolEqOp => lbool == rbool,
                BoolNeqOp => lbool != rbool,
            }
        },
        NotExpr(expr) => !eval_bool_expr(*expr),
        BoolLit(boole) => boole,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithexp1() {
        let mut expr = BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: BinArithOp::AddOp};
        assert_eq!(eval_arith_expr(expr), 6);  
        expr = BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: BinArithOp::SubOp};
        assert_eq!(eval_arith_expr(expr), 2);
        expr = BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: BinArithOp::MulOp};
        assert_eq!(eval_arith_expr(expr), 8);
        expr = BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: BinArithOp::IntDivOp};
        assert_eq!(eval_arith_expr(expr), 2);

        let expr2 = ArithExpr(IntLit(10));
        let answer = IntValue(10);
        assert_eq!(eval(expr2), answer);  
    }
    
    #[test]
    fn test_boolexp1() {
        let mut expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::LtOp};
        assert_eq!(eval_bool_expr(expr), false);  
        expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::LteOp};
        assert_eq!(eval_bool_expr(expr), false);
        expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::GtOp};
        assert_eq!(eval_bool_expr(expr), true);
        expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::GteOp};
        assert_eq!(eval_bool_expr(expr), true);
        expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::ArithEqOp};
        assert_eq!(eval_bool_expr(expr), false);
        expr = ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithCmpOp::ArithNeqOp};
        assert_eq!(eval_bool_expr(expr), true);

        expr = BinBoolExpr {left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BinLogicOp::AndOp};
        assert_eq!(eval_bool_expr(expr), false);
        expr = BinBoolExpr {left: Box::new(BoolLit(false)), right: Box::new(BoolLit(true)), op: BinLogicOp::OrOp};
        assert_eq!(eval_bool_expr(expr), true);
        expr = BinBoolExpr {left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BinLogicOp::BoolEqOp};
        assert_eq!(eval_bool_expr(expr), false);
        expr = BinBoolExpr {left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BinLogicOp::BoolNeqOp};
        assert_eq!(eval_bool_expr(expr), true);

        expr = NotExpr(Box::new(BoolExpr::BoolLit(true)));
        assert_eq!(eval_bool_expr(expr), false);
    }

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}
