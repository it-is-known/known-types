// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageStreamEvent {
    /// Occurs when a [message](/docs/api-reference/messages/object) is created.
    ThreadMessageCreated(MessageObject),

    /// Occurs when a [message](/docs/api-reference/messages/object) moves to an
    /// `in_progress` state.
    ThreadMessageInProgress(MessageObject),

    /// Occurs when parts of a [Message](/docs/api-reference/messages/object)
    /// are being streamed.
    ThreadMessageDelta(MessageDeltaObject),

    /// Occurs when a [message](/docs/api-reference/messages/object) is
    /// completed.
    ThreadMessageCompleted(MessageObject),

    /// Occurs when a [message](/docs/api-reference/messages/object) ends before
    /// it is completed.
    ThreadMessageIncomplete(MessageObject),
}
