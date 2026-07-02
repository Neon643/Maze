/// Common contract for maze path solving algorithms.
///
/// `Solvation` represents already assembled solving strategy.
/// Client code should only call `eval`.
pub trait Solvation {
    /// Result type produced by concrete solvation strategy.
    type Result;

    /// Evaluates solvation strategy and returns solving result.
    fn eval(self) -> Self::Result;
}
