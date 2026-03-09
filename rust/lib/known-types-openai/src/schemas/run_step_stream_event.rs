// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RunStepStreamEvent {
    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
    /// created.
    ThreadRunStepCreated(RunStepObject),

    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object)
    /// moves to an `in_progress` state.
    ThreadRunStepInProgress(RunStepObject),

    /// Occurs when parts of a [run
    /// step](/docs/api-reference/run-steps/step-object) are being streamed.
    ThreadRunStepDelta(RunStepDeltaObject),

    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
    /// completed.
    ThreadRunStepCompleted(RunStepObject),

    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object)
    /// fails.
    ThreadRunStepFailed(RunStepObject),

    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
    /// cancelled.
    ThreadRunStepCancelled(RunStepObject),

    /// Occurs when a [run step](/docs/api-reference/run-steps/step-object)
    /// expires.
    ThreadRunStepExpired(RunStepObject),
}
