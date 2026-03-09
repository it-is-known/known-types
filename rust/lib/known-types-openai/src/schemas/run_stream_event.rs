// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RunStreamEvent {
    /// Occurs when a new [run](/docs/api-reference/runs/object) is created.
    ThreadRunCreated(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) moves to a `queued`
    /// status.
    ThreadRunQueued(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) moves to an
    /// `in_progress` status.
    ThreadRunInProgress(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) moves to a
    /// `requires_action` status.
    ThreadRunRequiresAction(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) is completed.
    ThreadRunCompleted(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) ends with status
    /// `incomplete`.
    ThreadRunIncomplete(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) fails.
    ThreadRunFailed(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) moves to a
    /// `cancelling` status.
    ThreadRunCancelling(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) is cancelled.
    ThreadRunCancelled(RunObject),

    /// Occurs when a [run](/docs/api-reference/runs/object) expires.
    ThreadRunExpired(RunObject),
}
