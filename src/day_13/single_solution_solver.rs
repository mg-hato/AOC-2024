use std::ops::BitXor;


use crate::helper::display::vector_display;

use super::model::{ClawMachine, Position};

/// `SingleSolutionFinder` is essentially a solver for a system with two equations
/// two unknowns, limited in solution space to non-negative integers.
pub struct SingleSolutionSolver;

impl SingleSolutionSolver {

    /// Greates common divisor
    fn gcd(x: u64, y: u64) -> Option<u64> {
        if x == 0 || y == 0 { return None; }
        let mut left = x;
        let mut right = y;
        loop {
            if left < right {
                left = left.bitxor(right);
                right = right.bitxor(left);
                left = left.bitxor(right);
            }
            
            // GCD found
            if left % right == 0 {
                return Some(right);
            } else {
                left %= right;
            }
        }
    }

    /// Gets ratio of `x` and `y` as a pair `(L,R)` where `L` and `R` are coprime, representing `x/y` 
    /// - if `y` is 0, ratio is undefined: returns `None`
    /// - if `x` is 0, it returns `Some((0, 1))`
    /// - otherwise, returns coprime ratio `Some((L, R))`
    fn ratio(x: u64, y: u64) -> (u64, u64) {
        let gcd = Self::gcd(x, y).unwrap();
        (x / gcd, y / gcd)
    }

    /// Checks whether equation is supported. Equation is supported if
    /// 1. Button A and B do not have zero movements in X or Y component
    /// 2. Button A's gradient is not equal to button's B gradient.
    /// 
    /// Button's gradient is defined as follows. Let's say that pressing button
    /// will move the claw by `x` and `y` in X and Y-axis respectively.
    /// The gradient of that button is defined as `x / y`. 
    fn verify_assumptions(equation: ClawMachine) -> Result<(), String> {
        let ClawMachine {
            button_a: Position { x: ax, y: ay },
            button_b: Position { x: bx, y: by},
            ..
        } = equation;
        if [ax, ay, bx, by].iter().any(|&coefficient|coefficient == 0) {
            Err(format!("Equation with zero coefficients not supported."))
        } else if Self::ratio(ax, ay) == Self::ratio(bx, by) {
            Err(vector_display(&vec![
                format!("Ratio of x and y factor of button A and B is equal: '{:?}'.", Self::ratio(ax, ay)),
                format!("This equation is not supported."),
            ], " "))
        } else {
            Ok(())
        }
    }

    /// Multiplies each component of `(x, y)` pair by `mul`.
    /// Returns `Some(_)` if no multiplication overflow occurs.
    fn multiply(mul: u64, xy: (u64, u64)) -> Option<(u64, u64)> {
        let (x, y) = xy;
        if let (Some(x_res), Some(y_res)) = (mul.checked_mul(x), mul.checked_mul(y)) {
            Some((x_res, y_res))
        } else { None }
    }

    /// Attempts to subtract two equations of the form:
    /// - `fst_eq: X * m1 = r1`
    /// - `snd_eq: X * m2 = r2`
    /// into a desired form: `X * m = r` where `m` and `r` are greater or equal to zero. 
    /// Only `X` is the unknown in the above equations. It tries to do this by either
    /// subtracting first from the second equation or other way around.
    /// If neither yields the desired form, it returns `None` otherwise it returns `Some(m, r)`
    fn try_subtract_equations(fst_eq: (u64, u64), snd_eq: (u64, u64)) -> Option<(u64, u64)> {
        let (m1, r1) = fst_eq;
        let (m2, r2) = snd_eq;
        if (m1 >= m2 && r1 >= r2) || (m1 <= m2 && r1 <= r2) {
            Some((m1.abs_diff(m2), r1.abs_diff(r2)))
        } else { None }
    }

    /// Returns `Some(x / y)` if `x` is divisible by `y`. Otherwise, `None`
    fn div(x: u64, y: u64) -> Option<u64> {
        if x % y == 0 {
            Some(x / y)
        } else { None }
    }

    /// Returns:
    /// - `Some((a,b))` where `a` and `b` is unique solution: `a` and `b` presses of button A and B, respectively,
    /// in order to reach exactly `prize` coordinates.
    /// - `None` if it cannot find the solution in the constrained scope.
    /// - `Err(e)` an error described with `e`. For example, any overflows during calculation would be reported through error.
    /// Or if the system of equations does not satisfy assumptions, that would also be conveyed through error.
    pub fn solve(equation: ClawMachine) -> Result<Option<(u64, u64)>, String> {
        // We make certaint assumptions while solving the system of two equations.
        // Before starting, let's verify those assumptions
        if let Err(e) = Self::verify_assumptions(equation) { return Err(e); }
        let ClawMachine {
            button_a: Position { x: ax, y: ay},
            button_b: Position { x: bx, y: by },
            prize:    Position { x: px, y: py }, 
        } = equation;

        // We have equations where A and B are unknown values:
        // 1. A * ax + B * bx = px
        // 2. A * ay + B * by = py
        // Firstly, we multiply each equation by appropriate number so that
        // A's coefficient in both equations is the same. This step should not fail,
        // and its failure can only mean that we have overflowed.
        let fst_eq = Self::multiply(ay, (bx, px));
        let snd_eq = Self::multiply(ax, (by, py));
        if fst_eq.is_none() || snd_eq.is_none() {
            return Err(format!("Multiplication overflow occurred"));
        }

        let (fst_eq, snd_eq) = (fst_eq.unwrap(), snd_eq.unwrap());
        
        // Now we are going to subtract first from the second equation (or vice versa)  to eliminate unknown A.
        // The produced equation should be of the form `B * mul = res` with `mul` and `res` greater or equal to zero.
        let new_eq = Self::try_subtract_equations(fst_eq, snd_eq);
        
        // If the subtraction gave us the desired form, we know the `mul` is non-zero (distinct gradient assumption).
        // So we work out `B` which is the number of button B presses
        let b = new_eq.and_then(|(mul, res)|Self::div(res, mul));
        if b.is_none() { return Ok(None); }
        
        // At this point, we have successfully solved the unknown B within our solution scope.
        let b = b.unwrap();
        
        // Now we go back to solving for A: A = (px - B * bx) / ax; (as well as the second equation for Y-axis)
        // If any operation along the way overflows it indicates that A is not in our solution scope (e.g. negative A)
        let a_x = bx.checked_mul(b).and_then(|p|px.checked_sub(p)).and_then(|p|Self::div(p, ax));
        let a_y = by.checked_mul(b).and_then(|p|py.checked_sub(p)).and_then(|p|Self::div(p, ay));

        // Solving for A from the two equations should give us the same result. We dilligently check that
        let a = if a_x == a_y { a_x } else { None };

        Ok(a.map(|a|(a,b)))
    }
}