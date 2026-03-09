// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThreadStreamEvent {
    /// Occurs when a new [thread](/docs/api-reference/threads/object) is
    /// created.
    ThreadCreated(ThreadObject),
}
