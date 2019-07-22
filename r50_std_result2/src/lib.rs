
mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        }else {
            let result = x / y;
            println!("On Div result {} / {} = {}", x, y, result);
            Ok(result)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        println!("On Sqrt, x = {}", x);
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

fn op_(x: f64, y: f64) -> checked::MathResult {
    let ratio = checked::div(x, y) ?;
    let ln = checked::ln(ratio)?;

    checked::sqrt(ln)
}

pub fn execute() {
    op_(1.0, 10.0);
}