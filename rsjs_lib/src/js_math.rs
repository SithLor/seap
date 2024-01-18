
//from https://github.com/TheAlgorithms/Rust/blob/master/src/math/ceil.rs
//mit license
pub fn ceil(x: f64) -> f64 {
    let x_rounded_towards_zero = x as i32 as f64;
    if x < 0. || x_rounded_towards_zero == x {
        x_rounded_towards_zero
    } else {
        x_rounded_towards_zero + 1_f64
    }
}


#[inline(always)]
fn E() -> f64 {
    2.718281828459045
}

pub enum MathSign {
    Positive,
    Negative,
    Zero,
}
/**
 * Returns the result of 32-bit multiplication of two numbers.
 * @param x First number
 * @param y Second number
 */
pub fn imul(x: f64, y: f64) -> f64 {
    //return the result of 32-bit multiplication of two numbers.
    return x * y;
}
/**
 * Returns the number of leading zero bits in the 32-bit binary representation of a number.
 * @param x A numeric expression.
 */
pub fn clz32(x: f64) -> f64{
    //return the number of leading zero bits in the 32-bit binary representation of a number.
    return x
}
/**
 * Returns the sign of the x, indicating whether x is positive, negative or zero.
 * @param x The numeric expression to test
 */
pub fn sign(x: f64) -> MathSign {
    //get the sign of a number
    if x > 0.0 {
        MathSign::Positive
    } else if x < 0.0 {
        MathSign::Negative
    } else {
        MathSign::Zero
    }
}
/**
 * Returns the base 10 logarithm of a number.
 * @param x A numeric expression.
 */
pub fn log10(x: f64) -> f64 {
    //return the base 10 logarithm of a number
    return x.log10();
}

/**
 * Returns the base 2 logarithm of a number.
 * @param x A numeric expression.
 */
pub fn log2(x: f64) -> f64 {
    //return the base 2 logarithm of a number
    return x.log2();
}

/**
 * Returns the natural logarithm of 1 + x.
 * @param x A numeric expression.
 */
//pub fn log1p(x: f64){
//return the natural logarithm of 1 + x

// }

/**
 * Returns the result of (e^x - 1), which is an implementation-dependent approximation to
 * subtracting 1 from the exponential function of x (e raised to the power of x, where e
 * is the base of the natural logarithms).
 * @param x A numeric expression.
 */
//pub fn expm1(x: f64){}

/**
 * Returns the hyperbolic cosine of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn cosh(x: f64) -> f64 {
    //return the hyperbolic cosine of a number
    return x.cosh();
}

/**
 * Returns the hyperbolic sine of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn sinh(x: f64) -> f64 {
    //return the hyperbolic sine of a number
    return x.sinh();
}
/**
 * Returns the hyperbolic tangent of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn tanh(x: f64) -> f64 {
    //return the hyperbolic tangent of a number
    return x.tanh();
}

/**
 * Returns the inverse hyperbolic cosine of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn acosh(x: f64) -> f64 {
    //return the inverse hyperbolic cosine of a number
    return x.acosh();
}

/**
 * Returns the inverse hyperbolic sine of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn asinh(x: f64) -> f64 {
    //return the inverse hyperbolic sine of a number
    return x.asinh();
}

/**
 * Returns the inverse hyperbolic tangent of a number.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn atanh(x: f64) -> f64 {
    //return the inverse hyperbolic tangent of a number
    return x.atanh();
}

/**
 * Returns the integral part of the a numeric expression, x, removing any fractional digits.
 * If x is already an integer, the result is x.
 * @param x A numeric expression.
 */
pub fn trunc(x: f64) -> f64 {
    //return the integral part of the a numeric expression, x, removing any fractional digits.
    //If x is already an integer, the result is x.
    return x.trunc();
}

/**
 * Returns the nearest single precision float representation of a number.
 * @param x A numeric expression.
 */
pub fn fround(x: f64) -> f64 {
    //return the nearest single precision float representation of a number.
    return x.round();
}

/**
 * Returns an implementation-dependent approximation to the cube root of number.
 * @param x A numeric expression.
 */
pub fn cbrt(x: f64) -> f64 {
    //return an implementation-dependent approximation to the cube root of number.
    return x.cbrt();
}

//------es5---/

/**
 * Returns the absolute value of a f64 (the value without regard to whether it is positive or negative).
 * For example, the absolute value of -5 is the same as the absolute value of 5.
 * @param x A numeric expression for which the absolute value is needed.
 */
pub fn abs(x: f64) -> f64 {
    //return the absolute value of a f64 (the value without regard to whether it is positive or negative).
    //For example, the absolute value of -5 is the same as the absolute value of 5.
    return x.abs();
}
/**
 * Returns the arc cosine (or inverse cosine) of a f64.
 * @param x A numeric expression.
 */
pub fn acos(x: f64) -> f64 {
    //return the arc cosine (or inverse cosine) of a f64.
    return x.acos();
}
/**
 * Returns the arcsine of a f64.
 * @param x A numeric expression.
 */
pub fn asin(x: f64) -> f64 {
    //return the arcsine of a f64.
    return x.asin();
}
/**
 * Returns the arctangent of a f64.
 * @param x A numeric expression for which the arctangent is needed.
 */
pub fn atan(x: f64) -> f64 {
    //return the arctangent of a f64.
    return x.atan();
}
/**
 * Returns the angle (in radians) from the X axis to a point.
 * @param y A numeric expression representing the cartesian y-coordinate.
 * @param x A numeric expression representing the cartesian x-coordinate.
 */
pub fn atan2(y: f64, x: f64) -> f64 {
    //return the angle (in radians) from the X axis to a point.
    return y.atan2(x);
}
/**
 * Returns the smallest integer greater than or equal to its numeric argument.
 * @param x A numeric expression.
 */
pub fn ceil(x: f64) -> f64 {
    //return the smallest integer greater than or equal to its numeric argument.
    return x.ceil();
}
/**
 * Returns the cosine of a f64.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn cos(x: f64) -> f64 {
    //return the cosine of a f64.
    return x.cos();
}
/**
 * Returns e (the base of natural logarithms) raised to a power.
 * @param x A numeric expression representing the power of e.
 */
pub fn exp(x: f64) -> f64 {
    //return e (the base of natural logarithms) raised to a power.
    return x.exp();
}
/**
 * Returns the greatest integer less than or equal to its numeric argument.
 * @param x A numeric expression.
 */
pub fn floor(x: f64) -> f64 {
    //return the greatest integer less than or equal to its numeric argument.
    return x.floor();
}
/**
 * Returns the natural logarithm (base e) of a f64.
 * @param x A numeric expression.
 */
pub fn log(x: f64) -> f64 {
    //return the natural logarithm (base e) of a f64.
    return x.log(E());
}
/**
 * Returns the larger of a set of supplied numeric expressions.
 * @param values Numeric expressions to be evaluated.
 */
//fn max(values: vec![]){return values.max();}
/**
 * Returns the smaller of a set of supplied numeric expressions.
 * @param values Numeric expressions to be evaluated.
 */

//fn min(values: vec![]){}
/**
 * Returns the value of a base expression taken to a specified power.
 * @param x The base value of the expression.
 * @param y The exponent value of the expression.
 */
pub fn pow(x: f64, y: f64) -> f64 {
    //return the value of a base expression taken to a specified power.
    return x.powf(y);
}
/** Returns a pseudorandom f64 between 0 and 1. */
pub fn random() -> f64 {
    //return a pseudorandom f64 between 0 and 1.
    return 0.0;
}
/**
 * Returns a supplied numeric expression rounded to the nearest integer.
 * @param x The value to be rounded to the nearest integer.
 */
pub fn round(x: f64) -> f64 {
    //return a supplied numeric expression rounded to the nearest integer.
    return x.round();
}
/**
 * Returns the sine of a f64.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn sin(x: f64) -> f64 {
    //return the sine of a f64.
    return x.sin();
}
/**
 * Returns the square root of a f64.
 * @param x A numeric expression.
 */
pub fn sqrt(x: f64) -> f64 {
    //return the square root of a f64.
    return x.sqrt();
}
/**
 * Returns the tangent of a f64.
 * @param x A numeric expression that contains an angle measured in radians.
 */
pub fn tan(x: f64) -> f64 {
    //return the tangent of a f64.
    return x.tan();
}
