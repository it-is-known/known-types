// This is free and unencumbered software released into the public domain.

//! **OpenAI API: Streaming**
//!
//! Stream the result of executing a Run or resuming a Run after submitting tool
//! outputs.
//! You can stream events from the [Create Thread and
//! Run][/docs/api-reference/runs/createThreadAndRun],
//! [Create Run][/docs/api-reference/runs/createRun], and [Submit Tool
//! Outputs][/docs/api-reference/runs/submitToolOutputs]
//! endpoints by passing `"stream": true`. The response will be a [Server-Sent
//! events](https://html.spec.whatwg.org/multipage/server-sent-events.html#server-sent-events)
//! stream.
//! Our Node and Python SDKs provide helpful utilities to make streaming easy.
//! Reference the
//! [Assistants API quickstart][/docs/assistants/overview] to learn more.
//!
//! [/docs/api-reference/runs/createThreadAndRun]: https://platform.openai.com/docs/api-reference/runs/createThreadAndRun
//! [/docs/api-reference/runs/createRun]: https://platform.openai.com/docs/api-reference/runs/createRun
//! [/docs/api-reference/runs/submitToolOutputs]: https://platform.openai.com/docs/api-reference/runs/submitToolOutputs
//! [/docs/assistants/overview]: https://platform.openai.com/docs/assistants/overview
