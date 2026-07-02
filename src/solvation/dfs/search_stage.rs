/// Executable stage of DFS solvation pipeline.
///
/// Each stage may either build intermediate DFS state or produce final
/// solvation result. Stages are composed by wrapping one stage into another.
pub trait SearchStage {
    /// Value produced by this pipeline stage.
    type Output;
    /// Runs this stage and returns its output.
    fn eval(self) -> Self::Output;
}
