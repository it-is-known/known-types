// This is free and unencumbered software released into the public domain.

//! OpenAI API schemas

#![allow(non_camel_case_types)]

use crate::prelude::{String, Vec};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddUploadPartRequest {
    /// The chunk of bytes for this Part.
    pub r#data: String,
}

/// Represents an individual Admin API key in an org.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminApiKey {
    /// The object type, which is always `organization.admin_api_key`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the API key
    pub r#name: String,

    /// The redacted value of the API key
    pub r#redacted_value: String,

    /// The value of the API key.
    pub r#value: Option<String>,

    /// The Unix timestamp (in seconds) of when the API key was created
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the API key was last used
    pub r#last_used_at: Option<i64>,

    pub r#owner: AdminApiKey_Owner,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminApiKey_Owner {
    /// Always `user`
    pub r#type: Option<String>,

    /// The object type, which is always organization.user
    pub r#object: Option<String>,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: Option<String>,

    /// The name of the user
    pub r#name: Option<String>,

    /// The Unix timestamp (in seconds) of when the user was created
    pub r#created_at: Option<i64>,

    /// Always `owner`
    pub r#role: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Annotation {
    FileCitationBody(FileCitationBody),

    UrlCitationBody(UrlCitationBody),

    FilePath(FilePath),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApiKeyList {
    pub r#object: Option<String>,

    pub r#data: Option<Vec<AdminApiKey>>,

    pub r#has_more: Option<bool>,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproximateLocation {
    /// The type of location approximation.
    pub r#type: String,

    pub r#country: Option<ApproximateLocation_Country>,

    pub r#region: Option<ApproximateLocation_Region>,

    pub r#city: Option<ApproximateLocation_City>,

    pub r#timezone: Option<ApproximateLocation_Timezone>,
}

pub type ApproximateLocation_City = Option<String>;

pub type ApproximateLocation_Country = Option<String>;

pub type ApproximateLocation_Region = Option<String>;

pub type ApproximateLocation_Timezone = Option<String>;

/// Represents an `assistant` that can call the model and use tools.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `assistant`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub r#created_at: i64,

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// ID of the model to use.
    pub r#model: String,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    pub r#tools: Vec<AssistantObject_Tools>,

    pub r#tool_resources: Option<AssistantObject_ToolResources>,

    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// A set of resources that are used by the assistant's tools.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantObject_ToolResources {
    pub r#code_interpreter: Option<AssistantObject_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<AssistantObject_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantObject_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter`` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantObject_ToolResources_FileSearch {
    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// attached to this assistant.
    pub r#vector_store_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssistantObject_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

/// Represents an event emitted when streaming a Run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssistantStreamEvent {
    ThreadStreamEvent(ThreadStreamEvent),

    RunStreamEvent(RunStreamEvent),

    RunStepStreamEvent(RunStepStreamEvent),

    MessageStreamEvent(MessageStreamEvent),

    ErrorEvent(ErrorEvent),

    DoneEvent(DoneEvent),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssistantSupportedModels {
    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1"))]
    Gpt41,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1-mini"))]
    Gpt41Mini,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1-nano"))]
    Gpt41Nano,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1-2025-04-14"))]
    Gpt4120250414,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1-mini-2025-04-14"))]
    Gpt41Mini20250414,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.1-nano-2025-04-14"))]
    Gpt41Nano20250414,

    #[cfg_attr(feature = "serde", serde(rename = "o3-mini"))]
    O3Mini,

    #[cfg_attr(feature = "serde", serde(rename = "o3-mini-2025-01-31"))]
    O3Mini20250131,

    #[cfg_attr(feature = "serde", serde(rename = "o1"))]
    O1,

    #[cfg_attr(feature = "serde", serde(rename = "o1-2024-12-17"))]
    O120241217,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o"))]
    Gpt4o,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o-2024-11-20"))]
    Gpt4o20241120,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o-2024-08-06"))]
    Gpt4o20240806,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o-2024-05-13"))]
    Gpt4o20240513,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o-mini"))]
    Gpt4oMini,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4o-mini-2024-07-18"))]
    Gpt4oMini20240718,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.5-preview"))]
    Gpt45Preview,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4.5-preview-2025-02-27"))]
    Gpt45Preview20250227,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-turbo"))]
    Gpt4Turbo,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-turbo-2024-04-09"))]
    Gpt4Turbo20240409,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-0125-preview"))]
    Gpt40125Preview,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-turbo-preview"))]
    Gpt4TurboPreview,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-1106-preview"))]
    Gpt41106Preview,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-vision-preview"))]
    Gpt4VisionPreview,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4"))]
    Gpt4,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-0314"))]
    Gpt40314,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-0613"))]
    Gpt40613,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-32k"))]
    Gpt432k,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-32k-0314"))]
    Gpt432k0314,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-4-32k-0613"))]
    Gpt432k0613,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo"))]
    Gpt35Turbo,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo-16k"))]
    Gpt35Turbo16k,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo-0613"))]
    Gpt35Turbo0613,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo-1106"))]
    Gpt35Turbo1106,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo-0125"))]
    Gpt35Turbo0125,

    #[cfg_attr(feature = "serde", serde(rename = "gpt-3.5-turbo-16k-0613"))]
    Gpt35Turbo16k0613,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsCode {
    /// The type of tool being defined: `code_interpreter`
    pub r#type: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFileSearch {
    /// The type of tool being defined: `file_search`
    pub r#type: String,

    pub r#file_search: Option<AssistantToolsFileSearch_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFileSearchTypeOnly {
    /// The type of tool being defined: `file_search`
    pub r#type: String,
}

/// Overrides for the file search tool.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFileSearch_FileSearch {
    /// The maximum number of results the file search tool should output.
    pub r#max_num_results: Option<i64>,

    pub r#ranking_options: Option<FileSearchRankingOptions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFunction {
    /// The type of tool being defined: `function`
    pub r#type: String,

    pub r#function: FunctionObject,
}

/// Specifies the format that the model must output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssistantsApiResponseFormatOption {
    /// `auto` is the default value
    Text(String),

    ResponseFormatText(ResponseFormatText),

    ResponseFormatJsonObject(ResponseFormatJsonObject),

    ResponseFormatJsonSchema(ResponseFormatJsonSchema),
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssistantsApiToolChoiceOption {
    /// `none` means the model will not call any tools and instead generates a
    /// message.
    Text(String),

    AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}

/// Specifies a tool the model should use.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantsNamedToolChoice {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: Option<AssistantsNamedToolChoice_Function>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantsNamedToolChoice_Function {
    /// The name of the function to call.
    pub r#name: String,
}

/// The format of the output, in one of these options: `json`, `text`, `srt`,
/// `verbose_json`, or `vtt`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AudioResponseFormat {
    #[cfg_attr(feature = "serde", serde(rename = "json"))]
    Json,

    #[cfg_attr(feature = "serde", serde(rename = "text"))]
    Text,

    #[cfg_attr(feature = "serde", serde(rename = "srt"))]
    Srt,

    #[cfg_attr(feature = "serde", serde(rename = "verbose_json"))]
    VerboseJson,

    #[cfg_attr(feature = "serde", serde(rename = "vtt"))]
    Vtt,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// A log of a user action or configuration change within this organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog {
    /// The ID of this log.
    pub r#id: String,

    pub r#type: AuditLogEventType,

    /// The Unix timestamp (in seconds) of the event.
    pub r#effective_at: i64,

    pub r#project: Option<AuditLog_Project>,

    pub r#actor: AuditLogActor,

    #[cfg_attr(feature = "serde", serde(rename = "api_key.created"))]
    pub r#api_key_created: Option<AuditLog_ApiKeyCreated>,

    #[cfg_attr(feature = "serde", serde(rename = "api_key.updated"))]
    pub r#api_key_updated: Option<AuditLog_ApiKeyUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "api_key.deleted"))]
    pub r#api_key_deleted: Option<AuditLog_ApiKeyDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.created"))]
    pub r#checkpoint_permission_created: Option<AuditLog_CheckpointPermissionCreated>,

    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.deleted"))]
    pub r#checkpoint_permission_deleted: Option<AuditLog_CheckpointPermissionDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "invite.sent"))]
    pub r#invite_sent: Option<AuditLog_InviteSent>,

    #[cfg_attr(feature = "serde", serde(rename = "invite.accepted"))]
    pub r#invite_accepted: Option<AuditLog_InviteAccepted>,

    #[cfg_attr(feature = "serde", serde(rename = "invite.deleted"))]
    pub r#invite_deleted: Option<AuditLog_InviteDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "login.failed"))]
    pub r#login_failed: Option<AuditLog_LoginFailed>,

    #[cfg_attr(feature = "serde", serde(rename = "logout.failed"))]
    pub r#logout_failed: Option<AuditLog_LogoutFailed>,

    #[cfg_attr(feature = "serde", serde(rename = "organization.updated"))]
    pub r#organization_updated: Option<AuditLog_OrganizationUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "project.created"))]
    pub r#project_created: Option<AuditLog_ProjectCreated>,

    #[cfg_attr(feature = "serde", serde(rename = "project.updated"))]
    pub r#project_updated: Option<AuditLog_ProjectUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "project.archived"))]
    pub r#project_archived: Option<AuditLog_ProjectArchived>,

    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.updated"))]
    pub r#rate_limit_updated: Option<AuditLog_RateLimitUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.deleted"))]
    pub r#rate_limit_deleted: Option<AuditLog_RateLimitDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.created"))]
    pub r#service_account_created: Option<AuditLog_ServiceAccountCreated>,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.updated"))]
    pub r#service_account_updated: Option<AuditLog_ServiceAccountUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.deleted"))]
    pub r#service_account_deleted: Option<AuditLog_ServiceAccountDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "user.added"))]
    pub r#user_added: Option<AuditLog_UserAdded>,

    #[cfg_attr(feature = "serde", serde(rename = "user.updated"))]
    pub r#user_updated: Option<AuditLog_UserUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "user.deleted"))]
    pub r#user_deleted: Option<AuditLog_UserDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "certificate.created"))]
    pub r#certificate_created: Option<AuditLog_CertificateCreated>,

    #[cfg_attr(feature = "serde", serde(rename = "certificate.updated"))]
    pub r#certificate_updated: Option<AuditLog_CertificateUpdated>,

    #[cfg_attr(feature = "serde", serde(rename = "certificate.deleted"))]
    pub r#certificate_deleted: Option<AuditLog_CertificateDeleted>,

    #[cfg_attr(feature = "serde", serde(rename = "certificates.activated"))]
    pub r#certificates_activated: Option<AuditLog_CertificatesActivated>,

    #[cfg_attr(feature = "serde", serde(rename = "certificates.deactivated"))]
    pub r#certificates_deactivated: Option<AuditLog_CertificatesDeactivated>,
}

/// The actor who performed the audit logged action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActor {
    /// The type of actor.
    pub r#type: Option<String>,

    pub r#session: Option<AuditLogActorSession>,

    pub r#api_key: Option<AuditLogActorApiKey>,
}

/// The API Key used to perform the audit logged action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorApiKey {
    /// The tracking id of the API key.
    pub r#id: Option<String>,

    /// The type of API key.
    pub r#type: Option<String>,

    pub r#user: Option<AuditLogActorUser>,

    pub r#service_account: Option<AuditLogActorServiceAccount>,
}

/// The service account that performed the audit logged action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorServiceAccount {
    /// The service account id.
    pub r#id: Option<String>,
}

/// The session in which the audit logged action was performed.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorSession {
    pub r#user: Option<AuditLogActorUser>,

    /// The IP address from which the action was performed.
    pub r#ip_address: Option<String>,
}

/// The user who performed the audit logged action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorUser {
    /// The user id.
    pub r#id: Option<String>,

    /// The user email.
    pub r#email: Option<String>,
}

/// The event type.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuditLogEventType {
    #[cfg_attr(feature = "serde", serde(rename = "api_key.created"))]
    ApiKeyCreated,

    #[cfg_attr(feature = "serde", serde(rename = "api_key.updated"))]
    ApiKeyUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "api_key.deleted"))]
    ApiKeyDeleted,

    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.created"))]
    CheckpointPermissionCreated,

    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.deleted"))]
    CheckpointPermissionDeleted,

    #[cfg_attr(feature = "serde", serde(rename = "invite.sent"))]
    InviteSent,

    #[cfg_attr(feature = "serde", serde(rename = "invite.accepted"))]
    InviteAccepted,

    #[cfg_attr(feature = "serde", serde(rename = "invite.deleted"))]
    InviteDeleted,

    #[cfg_attr(feature = "serde", serde(rename = "login.succeeded"))]
    LoginSucceeded,

    #[cfg_attr(feature = "serde", serde(rename = "login.failed"))]
    LoginFailed,

    #[cfg_attr(feature = "serde", serde(rename = "logout.succeeded"))]
    LogoutSucceeded,

    #[cfg_attr(feature = "serde", serde(rename = "logout.failed"))]
    LogoutFailed,

    #[cfg_attr(feature = "serde", serde(rename = "organization.updated"))]
    OrganizationUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "project.created"))]
    ProjectCreated,

    #[cfg_attr(feature = "serde", serde(rename = "project.updated"))]
    ProjectUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "project.archived"))]
    ProjectArchived,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.created"))]
    ServiceAccountCreated,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.updated"))]
    ServiceAccountUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "service_account.deleted"))]
    ServiceAccountDeleted,

    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.updated"))]
    RateLimitUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.deleted"))]
    RateLimitDeleted,

    #[cfg_attr(feature = "serde", serde(rename = "user.added"))]
    UserAdded,

    #[cfg_attr(feature = "serde", serde(rename = "user.updated"))]
    UserUpdated,

    #[cfg_attr(feature = "serde", serde(rename = "user.deleted"))]
    UserDeleted,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ApiKeyCreated {
    /// The tracking ID of the API key.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_ApiKeyCreated_Data>,
}

/// The payload used to create the API key.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ApiKeyCreated_Data {
    /// A list of scopes allowed for the API key, e.g.
    pub r#scopes: Option<Vec<String>>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ApiKeyDeleted {
    /// The tracking ID of the API key.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ApiKeyUpdated {
    /// The tracking ID of the API key.
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_ApiKeyUpdated_ChangesRequested>,
}

/// The payload used to update the API key.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ApiKeyUpdated_ChangesRequested {
    /// A list of scopes allowed for the API key, e.g.
    pub r#scopes: Option<Vec<String>>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificateCreated {
    /// The certificate ID.
    pub r#id: Option<String>,

    /// The name of the certificate.
    pub r#name: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificateDeleted {
    /// The certificate ID.
    pub r#id: Option<String>,

    /// The name of the certificate.
    pub r#name: Option<String>,

    /// The certificate content in PEM format.
    pub r#certificate: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificateUpdated {
    /// The certificate ID.
    pub r#id: Option<String>,

    /// The name of the certificate.
    pub r#name: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificatesActivated {
    pub r#certificates: Option<Vec<AuditLog_CertificatesActivated_Certificates>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificatesActivated_Certificates {
    /// The certificate ID.
    pub r#id: Option<String>,

    /// The name of the certificate.
    pub r#name: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificatesDeactivated {
    pub r#certificates: Option<Vec<AuditLog_CertificatesDeactivated_Certificates>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CertificatesDeactivated_Certificates {
    /// The certificate ID.
    pub r#id: Option<String>,

    /// The name of the certificate.
    pub r#name: Option<String>,
}

/// The project and fine-tuned model checkpoint that the checkpoint permission
/// was created for.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CheckpointPermissionCreated {
    /// The ID of the checkpoint permission.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_CheckpointPermissionCreated_Data>,
}

/// The payload used to create the checkpoint permission.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CheckpointPermissionCreated_Data {
    /// The ID of the project that the checkpoint permission was created for.
    pub r#project_id: Option<String>,

    /// The ID of the fine-tuned model checkpoint.
    pub r#fine_tuned_model_checkpoint: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_CheckpointPermissionDeleted {
    /// The ID of the checkpoint permission.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_InviteAccepted {
    /// The ID of the invite.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_InviteDeleted {
    /// The ID of the invite.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_InviteSent {
    /// The ID of the invite.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_InviteSent_Data>,
}

/// The payload used to create the invite.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_InviteSent_Data {
    /// The email invited to the organization.
    pub r#email: Option<String>,

    /// The role the email was invited to be.
    pub r#role: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_LoginFailed {
    /// The error code of the failure.
    pub r#error_code: Option<String>,

    /// The error message of the failure.
    pub r#error_message: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_LogoutFailed {
    /// The error code of the failure.
    pub r#error_code: Option<String>,

    /// The error message of the failure.
    pub r#error_message: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_OrganizationUpdated {
    /// The organization ID.
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_OrganizationUpdated_ChangesRequested>,
}

/// The payload used to update the organization settings.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_OrganizationUpdated_ChangesRequested {
    /// The organization title.
    pub r#title: Option<String>,

    /// The organization description.
    pub r#description: Option<String>,

    /// The organization name.
    pub r#name: Option<String>,

    pub r#settings: Option<AuditLog_OrganizationUpdated_ChangesRequested_Settings>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_OrganizationUpdated_ChangesRequested_Settings {
    /// Visibility of the threads page which shows messages created with the
    /// Assistants API and Playground.
    pub r#threads_ui_visibility: Option<String>,

    /// Visibility of the usage dashboard which shows activity and costs for
    /// your organization.
    pub r#usage_dashboard_visibility: Option<String>,
}

/// The project that the action was scoped to.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_Project {
    /// The project ID.
    pub r#id: Option<String>,

    /// The project title.
    pub r#name: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ProjectArchived {
    /// The project ID.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ProjectCreated {
    /// The project ID.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_ProjectCreated_Data>,
}

/// The payload used to create the project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ProjectCreated_Data {
    /// The project name.
    pub r#name: Option<String>,

    /// The title of the project as seen on the dashboard.
    pub r#title: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ProjectUpdated {
    /// The project ID.
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_ProjectUpdated_ChangesRequested>,
}

/// The payload used to update the project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ProjectUpdated_ChangesRequested {
    /// The title of the project as seen on the dashboard.
    pub r#title: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_RateLimitDeleted {
    /// The rate limit ID
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_RateLimitUpdated {
    /// The rate limit ID
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_RateLimitUpdated_ChangesRequested>,
}

/// The payload used to update the rate limits.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_RateLimitUpdated_ChangesRequested {
    /// The maximum requests per minute.
    pub r#max_requests_per_1_minute: Option<i64>,

    /// The maximum tokens per minute.
    pub r#max_tokens_per_1_minute: Option<i64>,

    /// The maximum images per minute.
    pub r#max_images_per_1_minute: Option<i64>,

    /// The maximum audio megabytes per minute.
    pub r#max_audio_megabytes_per_1_minute: Option<i64>,

    /// The maximum requests per day.
    pub r#max_requests_per_1_day: Option<i64>,

    /// The maximum batch input tokens per day.
    pub r#batch_1_day_max_input_tokens: Option<i64>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ServiceAccountCreated {
    /// The service account ID.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_ServiceAccountCreated_Data>,
}

/// The payload used to create the service account.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ServiceAccountCreated_Data {
    /// The role of the service account.
    pub r#role: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ServiceAccountDeleted {
    /// The service account ID.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ServiceAccountUpdated {
    /// The service account ID.
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_ServiceAccountUpdated_ChangesRequested>,
}

/// The payload used to updated the service account.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_ServiceAccountUpdated_ChangesRequested {
    /// The role of the service account.
    pub r#role: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_UserAdded {
    /// The user ID.
    pub r#id: Option<String>,

    pub r#data: Option<AuditLog_UserAdded_Data>,
}

/// The payload used to add the user to the project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_UserAdded_Data {
    /// The role of the user.
    pub r#role: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_UserDeleted {
    /// The user ID.
    pub r#id: Option<String>,
}

/// The details for events with this `type`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_UserUpdated {
    /// The project ID.
    pub r#id: Option<String>,

    pub r#changes_requested: Option<AuditLog_UserUpdated_ChangesRequested>,
}

/// The payload used to update the user.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog_UserUpdated_ChangesRequested {
    /// The role of the user.
    pub r#role: Option<String>,
}

/// The default strategy.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AutoChunkingStrategyRequestParam {
    /// Always `auto`.
    pub r#type: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch {
    pub r#id: String,

    /// The object type, which is always `batch`.
    pub r#object: String,

    /// The OpenAI API endpoint used by the batch.
    pub r#endpoint: String,

    pub r#errors: Option<Batch_Errors>,

    /// The ID of the input file for the batch.
    pub r#input_file_id: String,

    /// The time frame within which the batch should be processed.
    pub r#completion_window: String,

    /// The current status of the batch.
    pub r#status: String,

    /// The ID of the file containing the outputs of successfully executed
    /// requests.
    pub r#output_file_id: Option<String>,

    /// The ID of the file containing the outputs of requests with errors.
    pub r#error_file_id: Option<String>,

    /// The Unix timestamp (in seconds) for when the batch was created.
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) for when the batch started processing.
    pub r#in_progress_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch will expire.
    pub r#expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    pub r#finalizing_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch was completed.
    pub r#completed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch failed.
    pub r#failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch expired.
    pub r#expired_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    pub r#cancelling_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    pub r#cancelled_at: Option<i64>,

    pub r#request_counts: Option<Batch_RequestCounts>,

    pub r#metadata: Option<Metadata>,
}

/// The per-line object of the batch input file
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestInput {
    /// A developer-provided per-request id that will be used to match outputs
    /// to inputs.
    pub r#custom_id: Option<String>,

    /// The HTTP method to be used for the request.
    pub r#method: Option<String>,

    /// The OpenAI API relative URL to be used for the request.
    pub r#url: Option<String>,
}

/// The per-line object of the batch output and error files
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestOutput {
    pub r#id: Option<String>,

    /// A developer-provided per-request id that will be used to match outputs
    /// to inputs.
    pub r#custom_id: Option<String>,

    pub r#response: Option<BatchRequestOutput_Response>,

    pub r#error: Option<BatchRequestOutput_Error>,
}

/// For requests that failed with a non-HTTP error, this will contain more
/// information on the cause of the failure.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestOutput_Error {
    /// A machine-readable error code.
    pub r#code: Option<String>,

    /// A human-readable error message.
    pub r#message: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestOutput_Response {
    /// The HTTP status code of the response
    pub r#status_code: Option<i64>,

    /// An unique identifier for the OpenAI API request.
    pub r#request_id: Option<String>,

    /// The JSON body of the response
    pub r#body: Option<BatchRequestOutput_Response_Body>,
}

/// The JSON body of the response
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestOutput_Response_Body;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch_Errors {
    /// The object type, which is always `list`.
    pub r#object: Option<String>,

    pub r#data: Option<Vec<Batch_Errors_Data>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch_Errors_Data {
    /// An error code identifying the error type.
    pub r#code: Option<String>,

    /// A human-readable message providing more details about the error.
    pub r#message: Option<String>,

    /// The name of the parameter that caused the error, if applicable.
    pub r#param: Option<String>,

    /// The line number of the input file where the error occurred, if
    /// applicable.
    pub r#line: Option<i64>,
}

/// The request counts for different statuses within the batch.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch_RequestCounts {
    /// Total number of requests in the batch.
    pub r#total: i64,

    /// Number of requests that have been completed successfully.
    pub r#completed: i64,

    /// Number of requests that have failed.
    pub r#failed: i64,
}

/// Represents an individual `certificate` uploaded to the organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Certificate {
    /// The object type.
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the certificate.
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    pub r#created_at: i64,

    pub r#certificate_details: Certificate_CertificateDetails,

    /// Whether the certificate is currently active at the specified scope.
    pub r#active: Option<bool>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Certificate_CertificateDetails {
    /// The Unix timestamp (in seconds) of when the certificate becomes valid.
    pub r#valid_at: Option<i64>,

    /// The Unix timestamp (in seconds) of when the certificate expires.
    pub r#expires_at: Option<i64>,

    /// The content of the certificate in PEM format.
    pub r#content: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionDeleted {
    /// The type of object being deleted.
    pub r#object: String,

    /// The ID of the chat completion that was deleted.
    pub r#id: String,

    /// Whether the chat completion was deleted.
    pub r#deleted: bool,
}

/// Specifying a particular function via `{"name": "my_function"}` forces the
/// model to call that function.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionFunctionCallOption {
    /// The name of the function to call.
    pub r#name: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionFunctions {
    /// A description of what the function does, used by the model to choose
    /// when and how to call the function.
    pub r#description: Option<String>,

    /// The name of the function to be called.
    pub r#name: String,

    pub r#parameters: Option<FunctionParameters>,
}

/// An object representing a list of Chat Completions.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionList {
    /// The type of this object.
    pub r#object: String,

    /// An array of chat completion objects.
    pub r#data: Vec<CreateChatCompletionResponse>,

    /// The identifier of the first chat completion in the data array.
    pub r#first_id: String,

    /// The identifier of the last chat completion in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more Chat Completions available.
    pub r#has_more: bool,
}

/// An object representing a list of chat completion messages.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageList {
    /// The type of this object.
    pub r#object: String,

    /// An array of chat completion message objects.
    pub r#data: Vec<ChatCompletionMessageList_Data>,

    /// The identifier of the first chat message in the data array.
    pub r#first_id: String,

    /// The identifier of the last chat message in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more chat messages available.
    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageList_Data {
    /// The contents of the message.
    pub r#content: Option<String>,

    /// The refusal message generated by the model.
    pub r#refusal: Option<String>,

    pub r#tool_calls: Option<ChatCompletionMessageToolCalls>,

    /// Annotations for the message, when applicable, as when using the [web
    /// search tool](/docs/guides/tools-web-search?api-mode=chat).
    pub r#annotations: Option<Vec<ChatCompletionResponseMessage_Annotations>>,

    /// The role of the author of this message.
    pub r#role: String,

    pub r#function_call: Option<ChatCompletionResponseMessage_FunctionCall>,

    pub r#audio: Option<ChatCompletionResponseMessage_Audio>,

    /// The identifier of the chat message.
    pub r#id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageList_Data_Variant2 {
    /// The identifier of the chat message.
    pub r#id: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of the tool.
    pub r#type: String,

    pub r#function: ChatCompletionMessageToolCall_Function,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCallChunk {
    pub r#index: i64,

    /// The ID of the tool call.
    pub r#id: Option<String>,

    /// The type of the tool.
    pub r#type: Option<String>,

    pub r#function: Option<ChatCompletionMessageToolCallChunk_Function>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCallChunk_Function {
    /// The name of the function to call.
    pub r#name: Option<String>,

    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    pub r#arguments: Option<String>,
}

/// The function that the model called.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCall_Function {
    /// The name of the function to call.
    pub r#name: String,

    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    pub r#arguments: String,
}

/// The tool calls generated by the model, such as function calls.
pub type ChatCompletionMessageToolCalls = Vec<ChatCompletionMessageToolCall>;

/// Output types that you would like the model to generate for this request.
pub type ChatCompletionModalities = Vec<String>;

/// Specifies a tool the model should use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionNamedToolChoice {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: ChatCompletionNamedToolChoice_Function,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionNamedToolChoice_Function {
    /// The name of the function to call.
    pub r#name: String,
}

/// Messages sent by the model in response to user messages.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestAssistantMessage {
    pub r#content: Option<ChatCompletionRequestAssistantMessage_Content>,

    /// The refusal message by the assistant.
    pub r#refusal: Option<String>,

    /// An optional name for the participant.
    pub r#name: Option<String>,

    pub r#audio: Option<ChatCompletionRequestAssistantMessage_Audio>,

    pub r#tool_calls: Option<ChatCompletionMessageToolCalls>,

    pub r#function_call: Option<ChatCompletionRequestAssistantMessage_FunctionCall>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),

    ChatCompletionRequestMessageContentPartRefusal(ChatCompletionRequestMessageContentPartRefusal),
}

/// Data about a previous audio response from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestAssistantMessage_Audio {
    /// Unique identifier for a previous audio response from the model.
    pub r#id: String,
}

/// The contents of the assistant message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestAssistantMessage_Content {
    /// The contents of the assistant message.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}

/// Deprecated and replaced by `tool_calls`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestAssistantMessage_FunctionCall {
    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    pub r#arguments: String,

    /// The name of the function to call.
    pub r#name: String,
}

/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestDeveloperMessage {
    pub r#content: ChatCompletionRequestDeveloperMessage_Content,

    /// An optional name for the participant.
    pub r#name: Option<String>,
}

/// The contents of the developer message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestDeveloperMessage_Content {
    /// The contents of the developer message.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestFunctionMessage {
    /// The contents of the function message.
    pub r#content: Option<String>,

    /// The name of the function to call.
    pub r#name: String,
}

include!("schemas/chat_completion_request_message.rs");

/// Learn about [audio inputs](/docs/guides/audio).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartAudio {
    /// The type of the content part.
    pub r#type: String,

    pub r#input_audio: ChatCompletionRequestMessageContentPartAudio_InputAudio,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartAudio_InputAudio {
    /// Base64 encoded audio data.
    pub r#data: String,

    /// The format of the encoded audio data.
    pub r#format: String,
}

/// Learn about [file inputs](/docs/guides/text) for text generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartFile {
    /// The type of the content part.
    pub r#type: String,

    pub r#file: ChatCompletionRequestMessageContentPartFile_File,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartFile_File {
    /// The name of the file, used when passing the file to the model as a
    /// string.
    pub r#filename: Option<String>,

    /// The base64 encoded file data, used when passing the file to the model as
    /// a string.
    pub r#file_data: Option<String>,

    /// The ID of an uploaded file to use as input.
    pub r#file_id: Option<String>,
}

/// Learn about [image inputs](/docs/guides/vision).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartImage {
    /// The type of the content part.
    pub r#type: String,

    pub r#image_url: ChatCompletionRequestMessageContentPartImage_ImageUrl,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartImage_ImageUrl {
    /// Either a URL of the image or the base64 encoded image data.
    pub r#url: String,

    /// Specifies the detail level of the image.
    pub r#detail: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    /// The type of the content part.
    pub r#type: String,

    /// The refusal message generated by the model.
    pub r#refusal: String,
}

/// Learn about [text inputs](/docs/guides/text-generation).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartText {
    /// The type of the content part.
    pub r#type: String,

    /// The text content.
    pub r#text: String,
}

/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestSystemMessage {
    pub r#content: ChatCompletionRequestSystemMessage_Content,

    /// An optional name for the participant.
    pub r#name: Option<String>,
}

/// The contents of the system message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestSystemMessage_Content {
    /// The contents of the system message.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestToolMessage {
    pub r#content: ChatCompletionRequestToolMessage_Content,

    /// Tool call that this message is responding to.
    pub r#tool_call_id: String,
}

/// The contents of the tool message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestToolMessage_Content {
    /// The contents of the tool message.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

/// Messages sent by an end user, containing prompts or additional context
/// information.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestUserMessage {
    pub r#content: ChatCompletionRequestUserMessage_Content,

    /// An optional name for the participant.
    pub r#name: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestUserMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),

    ChatCompletionRequestMessageContentPartImage(ChatCompletionRequestMessageContentPartImage),

    ChatCompletionRequestMessageContentPartAudio(ChatCompletionRequestMessageContentPartAudio),

    ChatCompletionRequestMessageContentPartFile(ChatCompletionRequestMessageContentPartFile),
}

/// The contents of the user message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionRequestUserMessage_Content {
    /// The text contents of the message.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestUserMessageContentPart>),
}

/// A chat completion message generated by the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage {
    /// The contents of the message.
    pub r#content: Option<String>,

    /// The refusal message generated by the model.
    pub r#refusal: Option<String>,

    pub r#tool_calls: Option<ChatCompletionMessageToolCalls>,

    /// Annotations for the message, when applicable, as when using the [web
    /// search tool](/docs/guides/tools-web-search?api-mode=chat).
    pub r#annotations: Option<Vec<ChatCompletionResponseMessage_Annotations>>,

    /// The role of the author of this message.
    pub r#role: String,

    pub r#function_call: Option<ChatCompletionResponseMessage_FunctionCall>,

    pub r#audio: Option<ChatCompletionResponseMessage_Audio>,
}

/// A URL citation when using web search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage_Annotations {
    /// The type of the URL citation.
    pub r#type: String,

    pub r#url_citation: ChatCompletionResponseMessage_Annotations_UrlCitation,
}

/// A URL citation when using web search.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage_Annotations_UrlCitation {
    /// The index of the last character of the URL citation in the message.
    pub r#end_index: i64,

    /// The index of the first character of the URL citation in the message.
    pub r#start_index: i64,

    /// The URL of the web resource.
    pub r#url: String,

    /// The title of the web resource.
    pub r#title: String,
}

/// If the audio output modality is requested, this object contains data about
/// the audio response from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage_Audio {
    /// Unique identifier for this audio response.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when this audio response will no
    /// longer be accessible on the server for use in multi-turn conversations.
    pub r#expires_at: i64,

    /// Base64 encoded audio bytes generated by the model, in the format
    /// specified in the request.
    pub r#data: String,

    /// Transcript of the audio generated by the model.
    pub r#transcript: String,
}

/// Deprecated and replaced by `tool_calls`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage_FunctionCall {
    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    pub r#arguments: String,

    /// The name of the function to call.
    pub r#name: String,
}

/// The role of the author of a message
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRole {
    #[cfg_attr(feature = "serde", serde(rename = "developer"))]
    Developer,

    #[cfg_attr(feature = "serde", serde(rename = "system"))]
    System,

    #[cfg_attr(feature = "serde", serde(rename = "user"))]
    User,

    #[cfg_attr(feature = "serde", serde(rename = "assistant"))]
    Assistant,

    #[cfg_attr(feature = "serde", serde(rename = "tool"))]
    Tool,

    #[cfg_attr(feature = "serde", serde(rename = "function"))]
    Function,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// Options for streaming response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionStreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]`
    /// message.
    pub r#include_usage: Option<bool>,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionStreamResponseDelta {
    /// The contents of the chunk message.
    pub r#content: Option<String>,

    pub r#function_call: Option<ChatCompletionStreamResponseDelta_FunctionCall>,

    pub r#tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,

    /// The role of the author of this message.
    pub r#role: Option<String>,

    /// The refusal message generated by the model.
    pub r#refusal: Option<String>,
}

/// Deprecated and replaced by `tool_calls`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionStreamResponseDelta_FunctionCall {
    /// The arguments to call the function with, as generated by the model in
    /// JSON format.
    pub r#arguments: Option<String>,

    /// The name of the function to call.
    pub r#name: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionTokenLogprob {
    /// The token.
    pub r#token: String,

    /// The log probability of this token, if it is within the top 20 most
    /// likely tokens.
    pub r#logprob: f64,

    /// A list of integers representing the UTF-8 bytes representation of the
    /// token.
    pub r#bytes: Option<Vec<i64>>,

    /// List of the most likely tokens and their log probability, at this token
    /// position.
    pub r#top_logprobs: Vec<ChatCompletionTokenLogprob_TopLogprobs>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionTokenLogprob_TopLogprobs {
    /// The token.
    pub r#token: String,

    /// The log probability of this token, if it is within the top 20 most
    /// likely tokens.
    pub r#logprob: f64,

    /// A list of integers representing the UTF-8 bytes representation of the
    /// token.
    pub r#bytes: Option<Vec<i64>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionTool {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: FunctionObject,
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChatCompletionToolChoiceOption {
    /// `none` means the model will not call any tool and instead generates a
    /// message.
    Text(String),

    ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}

/// The chunking strategy used to chunk the file(s).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChunkingStrategyRequestParam {
    AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),

    StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}

/// A click action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Click {
    /// Specifies the event type.
    pub r#type: String,

    /// Indicates which mouse button was pressed during the click.
    pub r#button: String,

    /// The x-coordinate where the click occurred.
    pub r#x: i64,

    /// The y-coordinate where the click occurred.
    pub r#y: i64,
}

/// The output of a code interpreter tool call that is a file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterFileOutput {
    /// The type of the code interpreter file output.
    pub r#type: String,

    pub r#files: Vec<CodeInterpreterFileOutput_Files>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterFileOutput_Files {
    /// The MIME type of the file.
    pub r#mime_type: String,

    /// The ID of the file.
    pub r#file_id: String,
}

/// The output of a code interpreter tool call that is text.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterTextOutput {
    /// The type of the code interpreter text output.
    pub r#type: String,

    /// The logs of the code interpreter tool call.
    pub r#logs: String,
}

/// A tool call to run code.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterToolCall {
    /// The unique ID of the code interpreter tool call.
    pub r#id: String,

    /// The type of the code interpreter tool call.
    pub r#type: String,

    /// The code to run.
    pub r#code: String,

    /// The status of the code interpreter tool call.
    pub r#status: String,

    /// The results of the code interpreter tool call.
    pub r#results: Vec<CodeInterpreterToolOutput>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CodeInterpreterToolOutput {
    CodeInterpreterTextOutput(CodeInterpreterTextOutput),

    CodeInterpreterFileOutput(CodeInterpreterFileOutput),
}

/// A filter used to compare a specified attribute key to a given value using a
/// defined comparison operation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComparisonFilter {
    /// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
    pub r#type: String,

    /// The key to compare against the value.
    pub r#key: String,

    pub r#value: ComparisonFilter_Value,
}

/// The value to compare against the attribute key; supports string, number, or
/// boolean types.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComparisonFilter_Value {
    Text(String),

    Number(f64),

    Boolean(bool),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompleteUploadRequest {
    /// The ordered list of Part IDs.
    pub r#part_ids: Vec<String>,

    /// The optional md5 checksum for the file contents to verify if the bytes
    /// uploaded matches what you expect.
    pub r#md5: Option<String>,
}

/// Usage statistics for the completion request.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompletionUsage {
    /// Number of tokens in the generated completion.
    pub r#completion_tokens: i64,

    /// Number of tokens in the prompt.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used in the request (prompt + completion).
    pub r#total_tokens: i64,

    pub r#completion_tokens_details: Option<CompletionUsage_CompletionTokensDetails>,

    pub r#prompt_tokens_details: Option<CompletionUsage_PromptTokensDetails>,
}

/// Breakdown of tokens used in a completion.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompletionUsage_CompletionTokensDetails {
    /// When using Predicted Outputs, the number of tokens in the prediction
    /// that appeared in the completion.
    pub r#accepted_prediction_tokens: Option<i64>,

    /// Audio input tokens generated by the model.
    pub r#audio_tokens: Option<i64>,

    /// Tokens generated by the model for reasoning.
    pub r#reasoning_tokens: Option<i64>,

    /// When using Predicted Outputs, the number of tokens in the prediction
    /// that did not appear in the completion.
    pub r#rejected_prediction_tokens: Option<i64>,
}

/// Breakdown of tokens used in the prompt.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompletionUsage_PromptTokensDetails {
    /// Audio input tokens present in the prompt.
    pub r#audio_tokens: Option<i64>,

    /// Cached tokens present in the prompt.
    pub r#cached_tokens: Option<i64>,
}

/// Combine multiple filters using `and` or `or`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompoundFilter {
    /// Type of operation: `and` or `or`.
    pub r#type: String,

    /// Array of filters to combine.
    pub r#filters: Vec<ComparisonFilter>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComputerAction {
    Click(Click),

    DoubleClick(DoubleClick),

    Drag(Drag),

    KeyPress(KeyPress),

    Move(Move),

    Screenshot(Screenshot),

    Scroll(Scroll),

    Type(Type),

    Wait(Wait),
}

/// The output of a computer tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerCallOutputItemParam {
    pub r#id: Option<ComputerCallOutputItemParam_Id>,

    /// The ID of the computer tool call that produced the output.
    pub r#call_id: String,

    /// The type of the computer tool call output.
    pub r#type: String,

    pub r#output: ComputerScreenshotImage,

    pub r#acknowledged_safety_checks: Option<ComputerCallOutputItemParam_AcknowledgedSafetyChecks>,

    pub r#status: Option<ComputerCallOutputItemParam_Status>,
}

pub type ComputerCallOutputItemParam_AcknowledgedSafetyChecks = Option<ComputerCallOutputItemParam_AcknowledgedSafetyChecks_1>;

/// The safety checks reported by the API that have been acknowledged by the
/// developer.
pub type ComputerCallOutputItemParam_AcknowledgedSafetyChecks_1 = Vec<ComputerCallSafetyCheckParam>;

pub type ComputerCallOutputItemParam_Id = Option<String>;

pub type ComputerCallOutputItemParam_Status = Option<String>;

/// A pending safety check for the computer call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerCallSafetyCheckParam {
    /// The ID of the pending safety check.
    pub r#id: String,

    pub r#code: Option<ComputerCallSafetyCheckParam_Code>,

    pub r#message: Option<ComputerCallSafetyCheckParam_Message>,
}

pub type ComputerCallSafetyCheckParam_Code = Option<String>;

pub type ComputerCallSafetyCheckParam_Message = Option<String>;

/// A computer screenshot image used with the computer use tool.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerScreenshotImage {
    /// Specifies the event type.
    pub r#type: String,

    /// The URL of the screenshot image.
    pub r#image_url: Option<String>,

    /// The identifier of an uploaded file that contains the screenshot.
    pub r#file_id: Option<String>,
}

/// A tool call to a computer use tool.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCall {
    /// The type of the computer call.
    pub r#type: String,

    /// The unique ID of the computer call.
    pub r#id: String,

    /// An identifier used when responding to the tool call with output.
    pub r#call_id: String,

    pub r#action: ComputerAction,

    /// The pending safety checks for the computer call.
    pub r#pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,

    /// The status of the item.
    pub r#status: String,
}

/// The output of a computer tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallOutput {
    /// The type of the computer tool call output.
    pub r#type: String,

    /// The ID of the computer tool call output.
    pub r#id: Option<String>,

    /// The ID of the computer tool call that produced the output.
    pub r#call_id: String,

    /// The safety checks reported by the API that have been acknowledged by the
    /// developer.
    pub r#acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,

    pub r#output: ComputerScreenshotImage,

    /// The status of the message input.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallOutputResource {
    /// The type of the computer tool call output.
    pub r#type: String,

    /// The unique ID of the computer call tool output.
    pub r#id: String,

    /// The ID of the computer tool call that produced the output.
    pub r#call_id: String,

    /// The safety checks reported by the API that have been acknowledged by the
    /// developer.
    pub r#acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,

    pub r#output: ComputerScreenshotImage,

    /// The status of the message input.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallOutputResource_Variant2 {
    /// The unique ID of the computer call tool output.
    pub r#id: String,
}

/// A pending safety check for the computer call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallSafetyCheck {
    /// The ID of the pending safety check.
    pub r#id: String,

    /// The type of the pending safety check.
    pub r#code: String,

    /// Details about the pending safety check.
    pub r#message: String,
}

/// A tool that controls a virtual computer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerUsePreviewTool {
    /// The type of the computer use tool.
    pub r#type: String,

    /// The type of computer environment to control.
    pub r#environment: String,

    /// The width of the computer display.
    pub r#display_width: i64,

    /// The height of the computer display.
    pub r#display_height: i64,
}

/// Multi-modal input and output contents.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Content {
    InputContent(InputContent),

    OutputContent(OutputContent),
}

/// An x/y coordinate pair, e.g.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coordinate {
    /// The x-coordinate.
    pub r#x: i64,

    /// The y-coordinate.
    pub r#y: i64,
}

/// The aggregated costs details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CostsResult {
    pub r#object: String,

    pub r#amount: Option<CostsResult_Amount>,

    /// When `group_by=line_item`, this field provides the line item of the
    /// grouped costs result.
    pub r#line_item: Option<String>,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped costs result.
    pub r#project_id: Option<String>,
}

/// The monetary value in its associated currency.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CostsResult_Amount {
    /// The numeric value of the cost.
    pub r#value: Option<f64>,

    /// Lowercase ISO-4217 currency e.g.
    pub r#currency: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest {
    pub r#model: CreateAssistantRequest_Model,

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// A list of tool enabled on the assistant.
    pub r#tools: Option<Vec<CreateAssistantRequest_Tools>>,

    pub r#tool_resources: Option<CreateAssistantRequest_ToolResources>,

    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// ID of the model to use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateAssistantRequest_Model {
    Text(String),

    AssistantSupportedModels(AssistantSupportedModels),
}

/// A set of resources that are used by the assistant's tools.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest_ToolResources {
    pub r#code_interpreter: Option<CreateAssistantRequest_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<CreateAssistantRequest_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest_ToolResources_FileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to
    /// this assistant.
    pub r#vector_store_ids: Option<Vec<String>>,

    /// A helper to create a [vector
    /// store](/docs/api-reference/vector-stores/object) with file_ids and
    /// attach it to this assistant.
    pub r#vector_stores: Option<Vec<CreateAssistantRequest_ToolResources_FileSearch_VectorStores>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest_ToolResources_FileSearch_VectorStores {
    /// A list of [file](/docs/api-reference/files) IDs to add to the vector
    /// store.
    pub r#file_ids: Option<Vec<String>>,

    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateAssistantRequest_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest {
    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,

    pub r#service_tier: Option<ServiceTier>,

    /// A list of messages comprising the conversation so far.
    pub r#messages: Vec<ChatCompletionRequestMessage>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: ModelIdsShared,

    pub r#modalities: Option<ResponseModalities>,

    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// An upper bound for the number of tokens that can be generated for a
    /// completion, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_completion_tokens: Option<i64>,

    /// Number between -2.0 and 2.0.
    pub r#frequency_penalty: Option<f64>,

    /// Number between -2.0 and 2.0.
    pub r#presence_penalty: Option<f64>,

    pub r#web_search_options: Option<CreateChatCompletionRequest_Variant2_WebSearchOptions>,

    /// An integer between 0 and 20 specifying the number of most likely tokens
    /// to return at each token position, each with an associated log
    /// probability.
    pub r#top_logprobs: Option<i64>,

    pub r#response_format: Option<CreateChatCompletionRequest_Variant2_ResponseFormat>,

    pub r#audio: Option<CreateChatCompletionRequest_Variant2_Audio>,

    /// Whether or not to store the output of this chat completion request for
    /// use in our [model distillation](/docs/guides/distillation) or
    /// [evals](/docs/guides/evals) products.
    pub r#store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,

    pub r#stop: Option<StopConfiguration>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub r#logit_bias: Option<CreateChatCompletionRequest_LogitBias>,

    /// Whether to return log probabilities of the output tokens or not.
    pub r#logprobs: Option<bool>,

    /// The maximum number of [tokens](/tokenizer) that can be generated in the
    /// chat completion.
    pub r#max_tokens: Option<i64>,

    /// How many chat completion choices to generate for each input message.
    pub r#n: Option<i64>,

    pub r#prediction: Option<PredictionContent>,

    /// This feature is in Beta.
    pub r#seed: Option<i64>,

    pub r#stream_options: Option<ChatCompletionStreamOptions>,

    /// A list of tools the model may call.
    pub r#tools: Option<Vec<ChatCompletionTool>>,

    pub r#tool_choice: Option<ChatCompletionToolChoiceOption>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,

    pub r#function_call: Option<CreateChatCompletionRequest_Variant2_FunctionCall>,

    /// Deprecated in favor of `tools`.
    pub r#functions: Option<Vec<ChatCompletionFunctions>>,
}

/// Modify the likelihood of specified tokens appearing in the completion.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_LogitBias;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_Variant2 {
    /// A list of messages comprising the conversation so far.
    pub r#messages: Vec<ChatCompletionRequestMessage>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: ModelIdsShared,

    pub r#modalities: Option<ResponseModalities>,

    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// An upper bound for the number of tokens that can be generated for a
    /// completion, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_completion_tokens: Option<i64>,

    /// Number between -2.0 and 2.0.
    pub r#frequency_penalty: Option<f64>,

    /// Number between -2.0 and 2.0.
    pub r#presence_penalty: Option<f64>,

    pub r#web_search_options: Option<CreateChatCompletionRequest_Variant2_WebSearchOptions>,

    /// An integer between 0 and 20 specifying the number of most likely tokens
    /// to return at each token position, each with an associated log
    /// probability.
    pub r#top_logprobs: Option<i64>,

    pub r#response_format: Option<CreateChatCompletionRequest_Variant2_ResponseFormat>,

    pub r#audio: Option<CreateChatCompletionRequest_Variant2_Audio>,

    /// Whether or not to store the output of this chat completion request for
    /// use in our [model distillation](/docs/guides/distillation) or
    /// [evals](/docs/guides/evals) products.
    pub r#store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,

    pub r#stop: Option<StopConfiguration>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub r#logit_bias: Option<CreateChatCompletionRequest_Variant2_LogitBias>,

    /// Whether to return log probabilities of the output tokens or not.
    pub r#logprobs: Option<bool>,

    /// The maximum number of [tokens](/tokenizer) that can be generated in the
    /// chat completion.
    pub r#max_tokens: Option<i64>,

    /// How many chat completion choices to generate for each input message.
    pub r#n: Option<i64>,

    pub r#prediction: Option<PredictionContent>,

    /// This feature is in Beta.
    pub r#seed: Option<i64>,

    pub r#stream_options: Option<ChatCompletionStreamOptions>,

    /// A list of tools the model may call.
    pub r#tools: Option<Vec<ChatCompletionTool>>,

    pub r#tool_choice: Option<ChatCompletionToolChoiceOption>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,

    pub r#function_call: Option<CreateChatCompletionRequest_Variant2_FunctionCall>,

    /// Deprecated in favor of `tools`.
    pub r#functions: Option<Vec<ChatCompletionFunctions>>,
}

/// Modify the likelihood of specified tokens appearing in the completion.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_Variant2_LogitBias;

/// Parameters for audio output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_Variant2_Audio {
    /// The voice the model uses to respond.
    pub r#voice: VoiceIdsShared,

    /// Specifies the output audio format.
    pub r#format: String,
}

/// Deprecated in favor of `tool_choice`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateChatCompletionRequest_Variant2_FunctionCall {
    /// `none` means the model will not call a function and instead generates a
    /// message.
    Text(String),

    ChatCompletionFunctionCallOption(ChatCompletionFunctionCallOption),
}

/// An object specifying the format that the model must output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateChatCompletionRequest_Variant2_ResponseFormat {
    ResponseFormatText(ResponseFormatText),

    ResponseFormatJsonSchema(ResponseFormatJsonSchema),

    ResponseFormatJsonObject(ResponseFormatJsonObject),
}

/// This tool searches the web for relevant results to use in a response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_Variant2_WebSearchOptions {
    pub r#user_location: Option<CreateChatCompletionRequest_Variant2_WebSearchOptions_UserLocation>,

    pub r#search_context_size: Option<WebSearchContextSize>,
}

/// Approximate location parameters for the search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest_Variant2_WebSearchOptions_UserLocation {
    /// The type of location approximation.
    pub r#type: String,

    pub r#approximate: WebSearchLocation,
}

/// Represents a chat completion response returned by model, based on the
/// provided input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub r#id: String,

    /// A list of chat completion choices.
    pub r#choices: Vec<CreateChatCompletionResponse_Choices>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub r#created: i64,

    /// The model used for the chat completion.
    pub r#model: String,

    pub r#service_tier: Option<ServiceTier>,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub r#object: String,

    pub r#usage: Option<CompletionUsage>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionResponse_Choices {
    /// The reason the model stopped generating tokens.
    pub r#finish_reason: String,

    /// The index of the choice in the list of choices.
    pub r#index: i64,

    pub r#message: ChatCompletionResponseMessage,

    pub r#logprobs: Option<CreateChatCompletionResponse_Choices_Logprobs>,
}

/// Log probability information for the choice.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionResponse_Choices_Logprobs {
    /// A list of message content tokens with log probability information.
    pub r#content: Option<Vec<ChatCompletionTokenLogprob>>,

    /// A list of message refusal tokens with log probability information.
    pub r#refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

/// Represents a streamed chunk of a chat completion response returned by the
/// model, based on the provided input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion.
    pub r#id: String,

    /// A list of chat completion choices.
    pub r#choices: Vec<CreateChatCompletionStreamResponse_Choices>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub r#created: i64,

    /// The model to generate the completion.
    pub r#model: String,

    pub r#service_tier: Option<ServiceTier>,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion.chunk`.
    pub r#object: String,

    /// An optional field that will only be present when you set
    /// `stream_options: {"include_usage": true}` in your request.
    pub r#usage: Option<CompletionUsage>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionStreamResponse_Choices {
    pub r#delta: ChatCompletionStreamResponseDelta,

    pub r#logprobs: Option<CreateChatCompletionStreamResponse_Choices_Logprobs>,

    /// The reason the model stopped generating tokens.
    pub r#finish_reason: Option<String>,

    /// The index of the choice in the list of choices.
    pub r#index: i64,
}

/// Log probability information for the choice.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionStreamResponse_Choices_Logprobs {
    /// A list of message content tokens with log probability information.
    pub r#content: Option<Vec<ChatCompletionTokenLogprob>>,

    /// A list of message refusal tokens with log probability information.
    pub r#refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionRequest {
    pub r#model: CreateCompletionRequest_Model,

    pub r#prompt: Option<CreateCompletionRequest_Prompt>,

    /// Generates `best_of` completions server-side and returns the "best" (the
    /// one with the highest log probability per token).
    pub r#best_of: Option<i64>,

    /// Echo back the prompt in addition to the completion
    pub r#echo: Option<bool>,

    /// Number between -2.0 and 2.0.
    pub r#frequency_penalty: Option<f64>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub r#logit_bias: Option<CreateCompletionRequest_LogitBias>,

    /// Include the log probabilities on the `logprobs` most likely output
    /// tokens, as well the chosen tokens.
    pub r#logprobs: Option<i64>,

    /// The maximum number of [tokens](/tokenizer) that can be generated in the
    /// completion.
    pub r#max_tokens: Option<i64>,

    /// How many completions to generate for each prompt.
    pub r#n: Option<i64>,

    /// Number between -2.0 and 2.0.
    pub r#presence_penalty: Option<f64>,

    /// If specified, our system will make a best effort to sample
    /// deterministically, such that repeated requests with the same `seed` and
    /// parameters should return the same result.
    pub r#seed: Option<i64>,

    pub r#stop: Option<StopConfiguration>,

    /// Whether to stream back partial progress.
    pub r#stream: Option<bool>,

    pub r#stream_options: Option<ChatCompletionStreamOptions>,

    /// The suffix that comes after a completion of inserted text.
    pub r#suffix: Option<String>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,
}

/// Modify the likelihood of specified tokens appearing in the completion.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionRequest_LogitBias;

/// ID of the model to use.
pub type CreateCompletionRequest_Model = String;

/// The prompt(s) to generate completions for, encoded as a string, array of
/// strings, array of tokens, or array of token arrays.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateCompletionRequest_Prompt {
    Text(String),

    TextArray(Vec<String>),

    TokenArray(Vec<i64>),

    TokenArrayArray(Vec<Vec<i64>>),
}

/// Represents a completion response from the API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionResponse {
    /// A unique identifier for the completion.
    pub r#id: String,

    /// The list of completion choices the model generated for the input prompt.
    pub r#choices: Vec<CreateCompletionResponse_Choices>,

    /// The Unix timestamp (in seconds) of when the completion was created.
    pub r#created: i64,

    /// The model used for completion.
    pub r#model: String,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: Option<String>,

    /// The object type, which is always "text_completion"
    pub r#object: String,

    pub r#usage: Option<CompletionUsage>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionResponse_Choices {
    /// The reason the model stopped generating tokens.
    pub r#finish_reason: String,

    pub r#index: i64,

    pub r#logprobs: Option<CreateCompletionResponse_Choices_Logprobs>,

    pub r#text: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionResponse_Choices_Logprobs {
    pub r#text_offset: Option<Vec<i64>>,

    pub r#token_logprobs: Option<Vec<f64>>,

    pub r#tokens: Option<Vec<String>>,

    pub r#top_logprobs: Option<Vec<CreateCompletionResponse_Choices_Logprobs_TopLogprobs>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionResponse_Choices_Logprobs_TopLogprobs;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEmbeddingRequest {
    pub r#input: CreateEmbeddingRequest_Input,

    pub r#model: CreateEmbeddingRequest_Model,

    /// The format to return the embeddings in.
    pub r#encoding_format: Option<String>,

    /// The number of dimensions the resulting output embeddings should have.
    pub r#dimensions: Option<i64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,
}

/// Input text to embed, encoded as a string or array of tokens.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEmbeddingRequest_Input {
    /// The string that will be turned into an embedding.
    Text(String),

    /// The array of strings that will be turned into an embedding.
    TextArray(Vec<String>),

    /// The array of integers that will be turned into an embedding.
    TokenArray(Vec<i64>),

    /// The array of arrays containing integers that will be turned into an
    /// embedding.
    TokenArrayArray(Vec<Vec<i64>>),
}

/// ID of the model to use.
pub type CreateEmbeddingRequest_Model = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEmbeddingResponse {
    /// The list of embeddings generated by the model.
    pub r#data: Vec<Embedding>,

    /// The name of the model used to generate the embedding.
    pub r#model: String,

    /// The object type, which is always "list".
    pub r#object: String,

    pub r#usage: CreateEmbeddingResponse_Usage,
}

/// The usage information for the request.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEmbeddingResponse_Usage {
    /// The number of tokens used by the prompt.
    pub r#prompt_tokens: i64,

    /// The total number of tokens used by the request.
    pub r#total_tokens: i64,
}

/// A CompletionsRunDataSource object describing a model sampling configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCompletionsRunDataSource {
    /// The type of run data source.
    pub r#type: String,

    pub r#input_messages: Option<CreateEvalCompletionsRunDataSource_InputMessages>,

    pub r#sampling_params: Option<CreateEvalCompletionsRunDataSource_SamplingParams>,

    /// The name of the model to use for generating completions (e.g.
    pub r#model: Option<String>,

    pub r#source: CreateEvalCompletionsRunDataSource_Source,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalCompletionsRunDataSource_InputMessages {
    CreateEvalCompletionsRunDataSource_InputMessages_Variant1(CreateEvalCompletionsRunDataSource_InputMessages_Variant1),

    CreateEvalCompletionsRunDataSource_InputMessages_Variant2(CreateEvalCompletionsRunDataSource_InputMessages_Variant2),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCompletionsRunDataSource_InputMessages_Variant1 {
    /// The type of input messages.
    pub r#type: String,

    /// A list of chat messages forming the prompt or context.
    pub r#template: Vec<CreateEvalCompletionsRunDataSource_InputMessages_Variant1_Template>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalCompletionsRunDataSource_InputMessages_Variant1_Template {
    EasyInputMessage(EasyInputMessage),

    EvalItem(EvalItem),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCompletionsRunDataSource_InputMessages_Variant2 {
    /// The type of input messages.
    pub r#type: String,

    /// A reference to a variable in the "item" namespace.
    pub r#item_reference: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCompletionsRunDataSource_SamplingParams {
    /// A higher temperature increases randomness in the outputs.
    pub r#temperature: Option<f64>,

    /// The maximum number of tokens in the generated output.
    pub r#max_completion_tokens: Option<i64>,

    /// An alternative to temperature for nucleus sampling; 1.0 includes all
    /// tokens.
    pub r#top_p: Option<f64>,

    /// A seed value to initialize the randomness, during sampling.
    pub r#seed: Option<i64>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalCompletionsRunDataSource_Source {
    EvalJsonlFileContentSource(EvalJsonlFileContentSource),

    EvalJsonlFileIdSource(EvalJsonlFileIdSource),

    EvalStoredCompletionsSource(EvalStoredCompletionsSource),
}

/// A CustomDataSourceConfig object that defines the schema for the data source
/// used for the evaluation runs.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCustomDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// The json schema for each row in the data source.
    pub r#item_schema: CreateEvalCustomDataSourceConfig_ItemSchema,

    /// Whether the eval should expect you to populate the sample namespace (ie,
    /// by generating responses off of your data source)
    pub r#include_sample_schema: Option<bool>,
}

/// The json schema for each row in the data source.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCustomDataSourceConfig_ItemSchema;

/// A chat message that makes up the prompt or context.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalItem {
    CreateEvalItem_Variant1(CreateEvalItem_Variant1),

    EvalItem(EvalItem),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalItem_Variant1 {
    /// The role of the message (e.g.
    pub r#role: String,

    /// The content of the message.
    pub r#content: String,
}

/// A JsonlRunDataSource object with that specifies a JSONL file that matches
/// the eval
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalJsonlRunDataSource {
    /// The type of data source.
    pub r#type: String,

    pub r#source: CreateEvalJsonlRunDataSource_Source,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalJsonlRunDataSource_Source {
    EvalJsonlFileContentSource(EvalJsonlFileContentSource),

    EvalJsonlFileIdSource(EvalJsonlFileIdSource),
}

/// A LabelModelGrader object which uses a model to assign labels to each item
/// in the evaluation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalLabelModelGrader {
    /// The object type, which is always `label_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    /// A list of chat messages forming the prompt or context.
    pub r#input: Vec<CreateEvalItem>,

    /// The labels to classify to each item in the evaluation.
    pub r#labels: Vec<String>,

    /// The labels that indicate a passing result.
    pub r#passing_labels: Vec<String>,
}

/// A data source config which specifies the metadata property of your stored
/// completions query.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalLogsDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// Metadata filters for the logs data source.
    pub r#metadata: Option<CreateEvalLogsDataSourceConfig_Metadata>,
}

/// Metadata filters for the logs data source.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalLogsDataSourceConfig_Metadata;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalRequest {
    /// The name of the evaluation.
    pub r#name: Option<String>,

    pub r#metadata: Option<Metadata>,

    pub r#data_source_config: CreateEvalRequest_DataSourceConfig,

    /// A list of graders for all eval runs in this group.
    pub r#testing_criteria: Vec<CreateEvalRequest_TestingCriteria>,
}

/// The configuration for the data source used for the evaluation runs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalRequest_DataSourceConfig {
    CreateEvalCustomDataSourceConfig(CreateEvalCustomDataSourceConfig),

    CreateEvalLogsDataSourceConfig(CreateEvalLogsDataSourceConfig),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalRequest_TestingCriteria {
    CreateEvalLabelModelGrader(CreateEvalLabelModelGrader),

    EvalStringCheckGrader(EvalStringCheckGrader),

    EvalTextSimilarityGrader(EvalTextSimilarityGrader),

    EvalPythonGrader(EvalPythonGrader),

    EvalScoreModelGrader(EvalScoreModelGrader),
}

/// A ResponsesRunDataSource object describing a model sampling configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource {
    /// The type of run data source.
    pub r#type: String,

    pub r#input_messages: Option<CreateEvalResponsesRunDataSource_InputMessages>,

    pub r#sampling_params: Option<CreateEvalResponsesRunDataSource_SamplingParams>,

    /// The name of the model to use for generating completions (e.g.
    pub r#model: Option<String>,

    pub r#source: CreateEvalResponsesRunDataSource_Source,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalResponsesRunDataSource_InputMessages {
    CreateEvalResponsesRunDataSource_InputMessages_Variant1(CreateEvalResponsesRunDataSource_InputMessages_Variant1),

    CreateEvalResponsesRunDataSource_InputMessages_Variant2(CreateEvalResponsesRunDataSource_InputMessages_Variant2),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource_InputMessages_Variant1 {
    /// The type of input messages.
    pub r#type: String,

    /// A list of chat messages forming the prompt or context.
    pub r#template: Vec<CreateEvalResponsesRunDataSource_InputMessages_Variant1_Template>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalResponsesRunDataSource_InputMessages_Variant1_Template {
    CreateEvalResponsesRunDataSource_InputMessages_Variant1_Template_Variant1(CreateEvalResponsesRunDataSource_InputMessages_Variant1_Template_Variant1),

    EvalItem(EvalItem),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource_InputMessages_Variant1_Template_Variant1 {
    /// The role of the message (e.g.
    pub r#role: String,

    /// The content of the message.
    pub r#content: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource_InputMessages_Variant2 {
    /// The type of input messages.
    pub r#type: String,

    /// A reference to a variable in the "item" namespace.
    pub r#item_reference: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource_SamplingParams {
    /// A higher temperature increases randomness in the outputs.
    pub r#temperature: Option<f64>,

    /// The maximum number of tokens in the generated output.
    pub r#max_completion_tokens: Option<i64>,

    /// An alternative to temperature for nucleus sampling; 1.0 includes all
    /// tokens.
    pub r#top_p: Option<f64>,

    /// A seed value to initialize the randomness, during sampling.
    pub r#seed: Option<i64>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalResponsesRunDataSource_Source {
    EvalJsonlFileContentSource(EvalJsonlFileContentSource),

    EvalJsonlFileIdSource(EvalJsonlFileIdSource),

    EvalResponsesSource(EvalResponsesSource),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalRunRequest {
    /// The name of the run.
    pub r#name: Option<String>,

    pub r#metadata: Option<Metadata>,

    pub r#data_source: CreateEvalRunRequest_DataSource,
}

/// Details about the run's data source.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateEvalRunRequest_DataSource {
    CreateEvalJsonlRunDataSource(CreateEvalJsonlRunDataSource),

    CreateEvalCompletionsRunDataSource(CreateEvalCompletionsRunDataSource),

    CreateEvalResponsesRunDataSource(CreateEvalResponsesRunDataSource),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFileRequest {
    /// The File object (not file name) to be uploaded.
    pub r#file: String,

    /// The intended purpose of the uploaded file.
    pub r#purpose: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningCheckpointPermissionRequest {
    /// The project identifiers to grant access to.
    pub r#project_ids: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningJobRequest {
    pub r#model: CreateFineTuningJobRequest_Model,

    /// The ID of an uploaded file that contains training data.
    pub r#training_file: String,

    pub r#hyperparameters: Option<CreateFineTuningJobRequest_Hyperparameters>,

    /// A string of up to 64 characters that will be added to your fine-tuned
    /// model name.
    pub r#suffix: Option<String>,

    /// The ID of an uploaded file that contains validation data.
    pub r#validation_file: Option<String>,

    /// A list of integrations to enable for your fine-tuning job.
    pub r#integrations: Option<Vec<CreateFineTuningJobRequest_Integrations>>,

    /// The seed controls the reproducibility of the job.
    pub r#seed: Option<i64>,

    pub r#method: Option<FineTuneMethod>,

    pub r#metadata: Option<Metadata>,
}

/// The hyperparameters used for the fine-tuning job.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningJobRequest_Hyperparameters {
    pub r#batch_size: Option<CreateFineTuningJobRequest_Hyperparameters_BatchSize>,

    pub r#learning_rate_multiplier: Option<CreateFineTuningJobRequest_Hyperparameters_LearningRateMultiplier>,

    pub r#n_epochs: Option<CreateFineTuningJobRequest_Hyperparameters_NEpochs>,
}

/// Number of examples in each batch.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateFineTuningJobRequest_Hyperparameters_BatchSize {
    Text(String),

    Integer(i64),
}

/// Scaling factor for the learning rate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateFineTuningJobRequest_Hyperparameters_LearningRateMultiplier {
    Text(String),

    Number(f64),
}

/// The number of epochs to train the model for.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateFineTuningJobRequest_Hyperparameters_NEpochs {
    Text(String),

    Integer(i64),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningJobRequest_Integrations {
    pub r#type: String,

    pub r#wandb: CreateFineTuningJobRequest_Integrations_Wandb,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CreateFineTuningJobRequest_Integrations_Type {
    #[cfg_attr(feature = "serde", serde(rename = "wandb"))]
    Wandb,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// The settings for your integration with Weights and Biases.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningJobRequest_Integrations_Wandb {
    /// The name of the project that the new run will be created under.
    pub r#project: String,

    /// A display name to set for the run.
    pub r#name: Option<String>,

    /// The entity to use for the run.
    pub r#entity: Option<String>,

    /// A list of tags to be attached to the newly created run.
    pub r#tags: Option<Vec<String>>,
}

/// The name of the model to fine-tune.
pub type CreateFineTuningJobRequest_Model = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageEditRequest {
    pub r#image: CreateImageEditRequest_Image,

    /// A text description of the desired image(s).
    pub r#prompt: String,

    /// An additional image whose fully transparent areas (e.g.
    pub r#mask: Option<String>,

    pub r#model: Option<CreateImageEditRequest_Model>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// The format in which the generated images are returned.
    pub r#response_format: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,

    /// The quality of the image that will be generated.
    pub r#quality: Option<String>,
}

/// The image(s) to edit.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateImageEditRequest_Image {
    Text(String),

    TextArray(Vec<String>),
}

/// The model to use for image generation.
pub type CreateImageEditRequest_Model = String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageRequest {
    /// A text description of the desired image(s).
    pub r#prompt: String,

    pub r#model: Option<CreateImageRequest_Model>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The quality of the image that will be generated.
    pub r#quality: Option<String>,

    /// The format in which generated images with `dall-e-2` and `dall-e-3` are
    /// returned.
    pub r#response_format: Option<String>,

    /// The format in which the generated images are returned.
    pub r#output_format: Option<String>,

    /// The compression level (0-100%) for the generated images.
    pub r#output_compression: Option<i64>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// Control the content-moderation level for images generated by
    /// `gpt-image-1`.
    pub r#moderation: Option<String>,

    /// Allows to set transparency for the background of the generated image(s).
    pub r#background: Option<String>,

    /// The style of the generated images.
    pub r#style: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,
}

/// The model to use for image generation.
pub type CreateImageRequest_Model = String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s).
    pub r#image: String,

    pub r#model: Option<CreateImageVariationRequest_Model>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The format in which the generated images are returned.
    pub r#response_format: Option<String>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,
}

/// The model to use for image generation.
pub type CreateImageVariationRequest_Model = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMessageRequest {
    /// The role of the entity that is creating the message.
    pub r#role: String,

    pub r#content: CreateMessageRequest_Content,

    /// A list of files attached to the message, and the tools they should be
    /// added to.
    pub r#attachments: Option<Vec<CreateMessageRequest_Attachments>>,

    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMessageRequest_Attachments {
    /// The ID of the file to attach to the message.
    pub r#file_id: Option<String>,

    /// The tools to add this file to.
    pub r#tools: Option<Vec<CreateMessageRequest_Attachments_Tools>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateMessageRequest_Attachments_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateMessageRequest_Content {
    /// The text contents of the message.
    Text(String),

    /// An array of content parts with a defined type, each can be of type
    /// `text` or images can be passed with `image_url` or `image_file`.
    Array(Vec<CreateMessageRequest_Content_Variant2>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateMessageRequest_Content_Variant2 {
    MessageContentImageFileObject(MessageContentImageFileObject),

    MessageContentImageUrlObject(MessageContentImageUrlObject),

    MessageRequestContentTextObject(MessageRequestContentTextObject),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationRequest {
    pub r#input: CreateModerationRequest_Input,

    pub r#model: Option<CreateModerationRequest_Model>,
}

/// Input (or inputs) to classify.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateModerationRequest_Input {
    /// A string of text to classify for moderation.
    Text(String),

    /// An array of strings to classify for moderation.
    TextArray(Vec<String>),

    /// An array of multi-modal inputs to the moderation model.
    Array(Vec<CreateModerationRequest_Input_Variant3>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateModerationRequest_Input_Variant3 {
    CreateModerationRequest_Input_Variant3_Variant1(CreateModerationRequest_Input_Variant3_Variant1),

    CreateModerationRequest_Input_Variant3_Variant2(CreateModerationRequest_Input_Variant3_Variant2),
}

/// An object describing an image to classify.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationRequest_Input_Variant3_Variant1 {
    /// Always `image_url`.
    pub r#type: String,

    pub r#image_url: CreateModerationRequest_Input_Variant3_Variant1_ImageUrl,
}

/// Contains either an image URL or a data URL for a base64 encoded image.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationRequest_Input_Variant3_Variant1_ImageUrl {
    /// Either a URL of the image or the base64 encoded image data.
    pub r#url: String,
}

/// An object describing text to classify.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationRequest_Input_Variant3_Variant2 {
    /// Always `text`.
    pub r#type: String,

    /// A string of text to classify.
    pub r#text: String,
}

/// The content moderation model you would like to use.
pub type CreateModerationRequest_Model = String;

/// Represents if a given text input is potentially harmful.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse {
    /// The unique identifier for the moderation request.
    pub r#id: String,

    /// The model used to generate the moderation results.
    pub r#model: String,

    /// A list of moderation objects.
    pub r#results: Vec<CreateModerationResponse_Results>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse_Results {
    /// Whether any of the below categories are flagged.
    pub r#flagged: bool,

    pub r#categories: CreateModerationResponse_Results_Categories,

    pub r#category_scores: CreateModerationResponse_Results_CategoryScores,

    pub r#category_applied_input_types: CreateModerationResponse_Results_CategoryAppliedInputTypes,
}

/// A list of the categories, and whether they are flagged or not.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse_Results_Categories {
    /// Content that expresses, incites, or promotes hate based on race, gender,
    /// ethnicity, religion, nationality, sexual orientation, disability status,
    /// or caste.
    pub r#hate: bool,

    /// Hateful content that also includes violence or serious harm towards the
    /// targeted group based on race, gender, ethnicity, religion, nationality,
    /// sexual orientation, disability status, or caste.
    pub r#hate_threatening: bool,

    /// Content that expresses, incites, or promotes harassing language towards
    /// any target.
    pub r#harassment: bool,

    /// Harassment content that also includes violence or serious harm towards
    /// any target.
    pub r#harassment_threatening: bool,

    /// Content that includes instructions or advice that facilitate the
    /// planning or execution of wrongdoing, or that gives advice or instruction
    /// on how to commit illicit acts.
    pub r#illicit: Option<bool>,

    /// Content that includes instructions or advice that facilitate the
    /// planning or execution of wrongdoing that also includes violence, or that
    /// gives advice or instruction on the procurement of any weapon.
    pub r#illicit_violent: Option<bool>,

    /// Content that promotes, encourages, or depicts acts of self-harm, such as
    /// suicide, cutting, and eating disorders.
    pub r#self_harm: bool,

    /// Content where the speaker expresses that they are engaging or intend to
    /// engage in acts of self-harm, such as suicide, cutting, and eating
    /// disorders.
    pub r#self_harm_intent: bool,

    /// Content that encourages performing acts of self-harm, such as suicide,
    /// cutting, and eating disorders, or that gives instructions or advice on
    /// how to commit such acts.
    pub r#self_harm_instructions: bool,

    /// Content meant to arouse sexual excitement, such as the description of
    /// sexual activity, or that promotes sexual services (excluding sex
    /// education and wellness).
    pub r#sexual: bool,

    /// Sexual content that includes an individual who is under 18 years old.
    pub r#sexual_minors: bool,

    /// Content that depicts death, violence, or physical injury.
    pub r#violence: bool,

    /// Content that depicts death, violence, or physical injury in graphic
    /// detail.
    pub r#violence_graphic: bool,
}

/// A list of the categories along with the input type(s) that the score applies
/// to.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse_Results_CategoryAppliedInputTypes {
    /// The applied input type(s) for the category 'hate'.
    pub r#hate: Vec<String>,

    /// The applied input type(s) for the category 'hate/threatening'.
    pub r#hate_threatening: Vec<String>,

    /// The applied input type(s) for the category 'harassment'.
    pub r#harassment: Vec<String>,

    /// The applied input type(s) for the category 'harassment/threatening'.
    pub r#harassment_threatening: Vec<String>,

    /// The applied input type(s) for the category 'illicit'.
    pub r#illicit: Vec<String>,

    /// The applied input type(s) for the category 'illicit/violent'.
    pub r#illicit_violent: Vec<String>,

    /// The applied input type(s) for the category 'self-harm'.
    pub r#self_harm: Vec<String>,

    /// The applied input type(s) for the category 'self-harm/intent'.
    pub r#self_harm_intent: Vec<String>,

    /// The applied input type(s) for the category 'self-harm/instructions'.
    pub r#self_harm_instructions: Vec<String>,

    /// The applied input type(s) for the category 'sexual'.
    pub r#sexual: Vec<String>,

    /// The applied input type(s) for the category 'sexual/minors'.
    pub r#sexual_minors: Vec<String>,

    /// The applied input type(s) for the category 'violence'.
    pub r#violence: Vec<String>,

    /// The applied input type(s) for the category 'violence/graphic'.
    pub r#violence_graphic: Vec<String>,
}

/// A list of the categories along with their scores as predicted by model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse_Results_CategoryScores {
    /// The score for the category 'hate'.
    pub r#hate: f64,

    /// The score for the category 'hate/threatening'.
    pub r#hate_threatening: f64,

    /// The score for the category 'harassment'.
    pub r#harassment: f64,

    /// The score for the category 'harassment/threatening'.
    pub r#harassment_threatening: f64,

    /// The score for the category 'illicit'.
    pub r#illicit: f64,

    /// The score for the category 'illicit/violent'.
    pub r#illicit_violent: f64,

    /// The score for the category 'self-harm'.
    pub r#self_harm: f64,

    /// The score for the category 'self-harm/intent'.
    pub r#self_harm_intent: f64,

    /// The score for the category 'self-harm/instructions'.
    pub r#self_harm_instructions: f64,

    /// The score for the category 'sexual'.
    pub r#sexual: f64,

    /// The score for the category 'sexual/minors'.
    pub r#sexual_minors: f64,

    /// The score for the category 'violence'.
    pub r#violence: f64,

    /// The score for the category 'violence/graphic'.
    pub r#violence_graphic: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateResponse {
    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,

    pub r#service_tier: Option<ServiceTier>,

    /// The unique ID of the previous response to the model.
    pub r#previous_response_id: Option<String>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: ModelIdsResponses,

    pub r#reasoning: Option<Reasoning>,

    /// An upper bound for the number of tokens that can be generated for a
    /// response, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_output_tokens: Option<i64>,

    /// Inserts a system (or developer) message as the first item in the model's
    /// context.
    pub r#instructions: Option<String>,

    pub r#text: Option<ResponseProperties_Text>,

    /// An array of tools the model may call while generating a response.
    pub r#tools: Option<Vec<Tool>>,

    pub r#tool_choice: Option<ResponseProperties_ToolChoice>,

    /// The truncation strategy to use for the model response.
    pub r#truncation: Option<String>,

    pub r#input: CreateResponse_Variant3_Input,

    /// Specify additional output data to include in the model response.
    pub r#include: Option<Vec<Includable>>,

    /// Whether to allow the model to run tool calls in parallel.
    pub r#parallel_tool_calls: Option<bool>,

    /// Whether to store the generated model response for later retrieval via
    /// API.
    pub r#store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateResponse_Variant3 {
    pub r#input: CreateResponse_Variant3_Input,

    /// Specify additional output data to include in the model response.
    pub r#include: Option<Vec<Includable>>,

    /// Whether to allow the model to run tool calls in parallel.
    pub r#parallel_tool_calls: Option<bool>,

    /// Whether to store the generated model response for later retrieval via
    /// API.
    pub r#store: Option<bool>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,
}

/// Text, image, or file inputs to the model, used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateResponse_Variant3_Input {
    /// A text input to the model, equivalent to a text input with the `user`
    /// role.
    Text(String),

    /// A list of one or many input items to the model, containing different
    /// content types.
    Array(Vec<InputItem>),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRunRequest {
    /// The ID of the [assistant](/docs/api-reference/assistants) to use to
    /// execute this run.
    pub r#assistant_id: String,

    pub r#model: Option<CreateRunRequest_Model>,

    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// Overrides the
    /// [instructions](/docs/api-reference/assistants/createAssistant) of the
    /// assistant.
    pub r#instructions: Option<String>,

    /// Appends additional instructions at the end of the instructions for the
    /// run.
    pub r#additional_instructions: Option<String>,

    /// Adds additional messages to the thread before creating the run.
    pub r#additional_messages: Option<Vec<CreateMessageRequest>>,

    /// Override the tools the assistant can use for this run.
    pub r#tools: Option<Vec<CreateRunRequest_Tools>>,

    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,

    /// The maximum number of prompt tokens that may be used over the course of
    /// the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens that may be used over the course
    /// of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: Option<TruncationObject>,

    pub r#tool_choice: Option<AssistantsApiToolChoiceOption>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// The ID of the [Model](/docs/api-reference/models) to be used to execute this
/// run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateRunRequest_Model {
    Text(String),

    AssistantSupportedModels(AssistantSupportedModels),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateRunRequest_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateSpeechRequest {
    pub r#model: CreateSpeechRequest_Model,

    /// The text to generate audio for.
    pub r#input: String,

    /// Control the voice of your generated audio with additional instructions.
    pub r#instructions: Option<String>,

    /// The voice to use when generating the audio.
    pub r#voice: VoiceIdsShared,

    /// The format to audio in.
    pub r#response_format: Option<String>,

    /// The speed of the generated audio.
    pub r#speed: Option<f64>,
}

/// One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or
/// `gpt-4o-mini-tts`.
pub type CreateSpeechRequest_Model = String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadAndRunRequest {
    /// The ID of the [assistant](/docs/api-reference/assistants) to use to
    /// execute this run.
    pub r#assistant_id: String,

    pub r#thread: Option<CreateThreadRequest>,

    pub r#model: Option<CreateThreadAndRunRequest_Model>,

    /// Override the default system message of the assistant.
    pub r#instructions: Option<String>,

    /// Override the tools the assistant can use for this run.
    pub r#tools: Option<Vec<CreateThreadAndRunRequest_Tools>>,

    pub r#tool_resources: Option<CreateThreadAndRunRequest_ToolResources>,

    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,

    /// The maximum number of prompt tokens that may be used over the course of
    /// the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens that may be used over the course
    /// of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: Option<TruncationObject>,

    pub r#tool_choice: Option<AssistantsApiToolChoiceOption>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// The ID of the [Model](/docs/api-reference/models) to be used to execute this
/// run.
pub type CreateThreadAndRunRequest_Model = String;

/// A set of resources that are used by the assistant's tools.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadAndRunRequest_ToolResources {
    pub r#code_interpreter: Option<CreateThreadAndRunRequest_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<CreateThreadAndRunRequest_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadAndRunRequest_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadAndRunRequest_ToolResources_FileSearch {
    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// attached to this assistant.
    pub r#vector_store_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateThreadAndRunRequest_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

/// Options to create a new thread.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest {
    /// A list of [messages](/docs/api-reference/messages) to start the thread
    /// with.
    pub r#messages: Option<Vec<CreateMessageRequest>>,

    pub r#tool_resources: Option<CreateThreadRequest_ToolResources>,

    pub r#metadata: Option<Metadata>,
}

/// A set of resources that are made available to the assistant's tools in this
/// thread.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest_ToolResources {
    pub r#code_interpreter: Option<CreateThreadRequest_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<CreateThreadRequest_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest_ToolResources_FileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to
    /// this thread.
    pub r#vector_store_ids: Option<Vec<String>>,

    /// A helper to create a [vector
    /// store](/docs/api-reference/vector-stores/object) with file_ids and
    /// attach it to this thread.
    pub r#vector_stores: Option<Vec<CreateThreadRequest_ToolResources_FileSearch_VectorStores>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest_ToolResources_FileSearch_VectorStores {
    /// A list of [file](/docs/api-reference/files) IDs to add to the vector
    /// store.
    pub r#file_ids: Option<Vec<String>>,

    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionRequest {
    /// The audio file object (not file name) to transcribe, in one of these
    /// formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub r#file: String,

    pub r#model: CreateTranscriptionRequest_Model,

    /// The language of the input audio.
    pub r#language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,

    pub r#response_format: Option<AudioResponseFormat>,

    /// The sampling temperature, between 0 and 1.
    pub r#temperature: Option<f64>,

    /// Additional information to include in the transcription response.
    #[cfg_attr(feature = "serde", serde(rename = "include[]"))]
    pub r#include: Option<Vec<TranscriptionInclude>>,

    /// The timestamp granularities to populate for this transcription.
    #[cfg_attr(feature = "serde", serde(rename = "timestamp_granularities[]"))]
    pub r#timestamp_granularities: Option<Vec<String>>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,
}

/// ID of the model to use.
pub type CreateTranscriptionRequest_Model = String;

/// Represents a transcription response returned by model, based on the provided
/// input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseJson {
    /// The transcribed text.
    pub r#text: String,

    /// The log probabilities of the tokens in the transcription.
    pub r#logprobs: Option<Vec<CreateTranscriptionResponseJson_Logprobs>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseJson_Logprobs {
    /// The token in the transcription.
    pub r#token: Option<String>,

    /// The log probability of the token.
    pub r#logprob: Option<f64>,

    /// The bytes of the token.
    pub r#bytes: Option<Vec<f64>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateTranscriptionResponseStreamEvent {
    TranscriptTextDeltaEvent(TranscriptTextDeltaEvent),

    TranscriptTextDoneEvent(TranscriptTextDoneEvent),
}

/// Represents a verbose json transcription response returned by model, based on
/// the provided input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseVerboseJson {
    /// The language of the input audio.
    pub r#language: String,

    /// The duration of the input audio.
    pub r#duration: f64,

    /// The transcribed text.
    pub r#text: String,

    /// Extracted words and their corresponding timestamps.
    pub r#words: Option<Vec<TranscriptionWord>>,

    /// Segments of the transcribed text and their corresponding details.
    pub r#segments: Option<Vec<TranscriptionSegment>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationRequest {
    /// The audio file object (not file name) translate, in one of these
    /// formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub r#file: String,

    pub r#model: CreateTranslationRequest_Model,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,

    /// The format of the output, in one of these options: `json`, `text`,
    /// `srt`, `verbose_json`, or `vtt`.
    pub r#response_format: Option<String>,

    /// The sampling temperature, between 0 and 1.
    pub r#temperature: Option<f64>,
}

/// ID of the model to use.
pub type CreateTranslationRequest_Model = String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationResponseJson {
    pub r#text: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationResponseVerboseJson {
    /// The language of the output translation (always `english`).
    pub r#language: String,

    /// The duration of the input audio.
    pub r#duration: f64,

    /// The translated text.
    pub r#text: String,

    /// Segments of the translated text and their corresponding details.
    pub r#segments: Option<Vec<TranscriptionSegment>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateUploadRequest {
    /// The name of the file to upload.
    pub r#filename: String,

    /// The intended purpose of the uploaded file.
    pub r#purpose: String,

    /// The number of bytes in the file you are uploading.
    pub r#bytes: i64,

    /// The MIME type of the file.
    pub r#mime_type: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreFileBatchRequest {
    /// A list of [File](/docs/api-reference/files) IDs that the vector store
    /// should use.
    pub r#file_ids: Vec<String>,

    pub r#chunking_strategy: Option<ChunkingStrategyRequestParam>,

    pub r#attributes: Option<VectorStoreFileAttributes>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreFileRequest {
    /// A [File](/docs/api-reference/files) ID that the vector store should use.
    pub r#file_id: String,

    pub r#chunking_strategy: Option<ChunkingStrategyRequestParam>,

    pub r#attributes: Option<VectorStoreFileAttributes>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreRequest {
    /// A list of [File](/docs/api-reference/files) IDs that the vector store
    /// should use.
    pub r#file_ids: Option<Vec<String>>,

    /// The name of the vector store.
    pub r#name: Option<String>,

    pub r#expires_after: Option<VectorStoreExpirationAfter>,

    pub r#chunking_strategy: Option<CreateVectorStoreRequest_ChunkingStrategy>,

    pub r#metadata: Option<Metadata>,
}

/// The chunking strategy used to chunk the file(s).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CreateVectorStoreRequest_ChunkingStrategy {
    AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),

    StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteAssistantResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteCertificateResponse {
    /// The object type, must be `certificate.deleted`.
    pub r#object: String,

    /// The ID of the certificate that was deleted.
    pub r#id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteFileResponse {
    pub r#id: String,

    pub r#object: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteFineTuningCheckpointPermissionResponse {
    /// The ID of the fine-tuned model checkpoint permission that was deleted.
    pub r#id: String,

    /// The object type, which is always "checkpoint.permission".
    pub r#object: String,

    /// Whether the fine-tuned model checkpoint permission was successfully
    /// deleted.
    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteMessageResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteModelResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteThreadResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteVectorStoreFileResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteVectorStoreResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

/// Occurs when a stream ends.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoneEvent {
    pub r#event: String,

    pub r#data: String,
}

/// A double click action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoubleClick {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate where the double click occurred.
    pub r#x: i64,

    /// The y-coordinate where the double click occurred.
    pub r#y: i64,
}

/// A drag action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Drag {
    /// Specifies the event type.
    pub r#type: String,

    /// An array of coordinates representing the path of the drag action.
    pub r#path: Vec<Coordinate>,
}

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EasyInputMessage {
    /// The role of the message input.
    pub r#role: String,

    pub r#content: EasyInputMessage_Content,

    /// The type of the message input.
    pub r#type: Option<String>,
}

/// Text, image, or audio input to the model, used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EasyInputMessage_Content {
    /// A text input to the model.
    Text(String),

    InputMessageContentList(InputMessageContentList),
}

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub r#index: i64,

    /// The embedding vector, which is a list of floats.
    pub r#embedding: Vec<f64>,

    /// The object type, which is always "embedding".
    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    pub r#code: Option<String>,

    pub r#message: String,

    pub r#param: Option<String>,

    pub r#type: String,
}

/// Occurs when an [error](/docs/guides/error-codes#api-errors) occurs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorEvent {
    pub r#event: String,

    pub r#data: Error,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorResponse {
    pub r#error: Error,
}

/// An Eval object with a data source config and testing criteria.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eval {
    /// The object type.
    pub r#object: String,

    /// Unique identifier for the evaluation.
    pub r#id: String,

    /// The name of the evaluation.
    pub r#name: String,

    pub r#data_source_config: Eval_DataSourceConfig,

    /// A list of testing criteria.
    pub r#testing_criteria: Vec<Eval_TestingCriteria>,

    /// The Unix timestamp (in seconds) for when the eval was created.
    pub r#created_at: i64,

    pub r#metadata: Option<Metadata>,
}

/// An object representing an error response from the Eval API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalApiError {
    /// The error code.
    pub r#code: String,

    /// The error message.
    pub r#message: String,
}

/// A CustomDataSourceConfig which specifies the schema of your `item` and
/// optionally `sample` namespaces.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalCustomDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// The json schema for the run data source items.
    pub r#schema: EvalCustomDataSourceConfig_Schema,
}

/// The json schema for the run data source items.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalCustomDataSourceConfig_Schema;

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalItem {
    /// The role of the message input.
    pub r#role: String,

    pub r#content: EvalItem_Content,

    /// The type of the message input.
    pub r#type: Option<String>,
}

/// Text inputs to the model - can contain template strings.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EvalItem_Content {
    /// A text input to the model.
    Text(String),

    InputTextContent(InputTextContent),

    EvalItem_Content_Variant3(EvalItem_Content_Variant3),
}

/// A text output from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalItem_Content_Variant3 {
    /// The type of the output text.
    pub r#type: String,

    /// The text output from the model.
    pub r#text: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileContentSource {
    /// The type of jsonl source.
    pub r#type: String,

    /// The content of the jsonl file.
    pub r#content: Vec<EvalJsonlFileContentSource_Content>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileContentSource_Content {
    pub r#item: EvalJsonlFileContentSource_Content_Item,

    pub r#sample: Option<EvalJsonlFileContentSource_Content_Sample>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileContentSource_Content_Item;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileContentSource_Content_Sample;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileIdSource {
    /// The type of jsonl source.
    pub r#type: String,

    /// The identifier of the file.
    pub r#id: String,
}

/// A LabelModelGrader object which uses a model to assign labels to each item
/// in the evaluation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalLabelModelGrader {
    /// The object type, which is always `label_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    pub r#input: Vec<EvalItem>,

    /// The labels to assign to each item in the evaluation.
    pub r#labels: Vec<String>,

    /// The labels that indicate a passing result.
    pub r#passing_labels: Vec<String>,
}

/// An object representing a list of evals.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval objects.
    pub r#data: Vec<Eval>,

    /// The identifier of the first eval in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more evals available.
    pub r#has_more: bool,
}

/// A PythonGrader object that runs a python script on the input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalPythonGrader {
    /// The object type, which is always `python`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The source code of the python script.
    pub r#source: String,

    /// The threshold for the score.
    pub r#pass_threshold: Option<f64>,

    /// The image tag to use for the python script.
    pub r#image_tag: Option<String>,
}

/// A EvalResponsesSource object describing a run data source configuration.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalResponsesSource {
    /// The type of run data source.
    pub r#type: String,

    /// Metadata filter for the responses.
    pub r#metadata: Option<EvalResponsesSource_Metadata>,

    /// The name of the model to find responses for.
    pub r#model: Option<String>,

    /// Optional search string for instructions.
    pub r#instructions_search: Option<String>,

    /// Only include items created after this timestamp (inclusive).
    pub r#created_after: Option<i64>,

    /// Only include items created before this timestamp (inclusive).
    pub r#created_before: Option<i64>,

    /// Whether the response has tool calls.
    pub r#has_tool_calls: Option<bool>,

    /// Optional reasoning effort parameter.
    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// Sampling temperature.
    pub r#temperature: Option<f64>,

    /// Nucleus sampling parameter.
    pub r#top_p: Option<f64>,

    /// List of user identifiers.
    pub r#users: Option<Vec<String>>,

    /// Whether to allow parallel tool calls.
    pub r#allow_parallel_tool_calls: Option<bool>,
}

/// Metadata filter for the responses.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalResponsesSource_Metadata;

/// A schema representing an evaluation run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRun {
    /// The type of the object.
    pub r#object: String,

    /// Unique identifier for the evaluation run.
    pub r#id: String,

    /// The identifier of the associated evaluation.
    pub r#eval_id: String,

    /// The status of the evaluation run.
    pub r#status: String,

    /// The model that is evaluated, if applicable.
    pub r#model: String,

    /// The name of the evaluation run.
    pub r#name: String,

    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub r#created_at: i64,

    /// The URL to the rendered evaluation run report on the UI dashboard.
    pub r#report_url: String,

    pub r#result_counts: EvalRun_ResultCounts,

    /// Usage statistics for each model during the evaluation run.
    pub r#per_model_usage: Vec<EvalRun_PerModelUsage>,

    /// Results per testing criteria applied during the evaluation run.
    pub r#per_testing_criteria_results: Vec<EvalRun_PerTestingCriteriaResults>,

    pub r#data_source: EvalRun_DataSource,

    pub r#metadata: Option<Metadata>,

    pub r#error: EvalApiError,
}

/// An object representing a list of runs for an evaluation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval run objects.
    pub r#data: Vec<EvalRun>,

    /// The identifier of the first eval run in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval run in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more evals available.
    pub r#has_more: bool,
}

/// A schema representing an evaluation run output item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem {
    /// The type of the object.
    pub r#object: String,

    /// Unique identifier for the evaluation run output item.
    pub r#id: String,

    /// The identifier of the evaluation run associated with this output item.
    pub r#run_id: String,

    /// The identifier of the evaluation group.
    pub r#eval_id: String,

    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub r#created_at: i64,

    /// The status of the evaluation run.
    pub r#status: String,

    /// The identifier for the data source item.
    pub r#datasource_item_id: i64,

    /// Details of the input data source item.
    pub r#datasource_item: EvalRunOutputItem_DatasourceItem,

    /// A list of results from the evaluation run.
    pub r#results: Vec<EvalRunOutputItem_Results>,

    pub r#sample: EvalRunOutputItem_Sample,
}

/// Details of the input data source item.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_DatasourceItem;

/// A result object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_Results;

/// An object representing a list of output items for an evaluation run.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItemList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval run output item objects.
    pub r#data: Vec<EvalRunOutputItem>,

    /// The identifier of the first eval run output item in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval run output item in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more eval run output items available.
    pub r#has_more: bool,
}

/// A sample containing the input and output of the evaluation run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_Sample {
    /// An array of input messages.
    pub r#input: Vec<EvalRunOutputItem_Sample_Input>,

    /// An array of output messages.
    pub r#output: Vec<EvalRunOutputItem_Sample_Output>,

    /// The reason why the sample generation was finished.
    pub r#finish_reason: String,

    /// The model used for generating the sample.
    pub r#model: String,

    pub r#usage: EvalRunOutputItem_Sample_Usage,

    pub r#error: EvalApiError,

    /// The sampling temperature used.
    pub r#temperature: f64,

    /// The maximum number of tokens allowed for completion.
    pub r#max_completion_tokens: i64,

    /// The top_p value used for sampling.
    pub r#top_p: f64,

    /// The seed used for generating the sample.
    pub r#seed: i64,
}

/// An input message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_Sample_Input {
    /// The role of the message sender (e.g., system, user, developer).
    pub r#role: String,

    /// The content of the message.
    pub r#content: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_Sample_Output {
    /// The role of the message (e.g.
    pub r#role: Option<String>,

    /// The content of the message.
    pub r#content: Option<String>,
}

/// Token usage details for the sample.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem_Sample_Usage {
    /// The total number of tokens used.
    pub r#total_tokens: i64,

    /// The number of completion tokens generated.
    pub r#completion_tokens: i64,

    /// The number of prompt tokens used.
    pub r#prompt_tokens: i64,

    /// The number of tokens retrieved from cache.
    pub r#cached_tokens: i64,
}

/// Information about the run's data source.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EvalRun_DataSource {
    CreateEvalJsonlRunDataSource(CreateEvalJsonlRunDataSource),

    CreateEvalCompletionsRunDataSource(CreateEvalCompletionsRunDataSource),

    CreateEvalResponsesRunDataSource(CreateEvalResponsesRunDataSource),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRun_PerModelUsage {
    /// The name of the model.
    pub r#model_name: String,

    /// The number of invocations.
    pub r#invocation_count: i64,

    /// The number of prompt tokens used.
    pub r#prompt_tokens: i64,

    /// The number of completion tokens generated.
    pub r#completion_tokens: i64,

    /// The total number of tokens used.
    pub r#total_tokens: i64,

    /// The number of tokens retrieved from cache.
    pub r#cached_tokens: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRun_PerTestingCriteriaResults {
    /// A description of the testing criteria.
    pub r#testing_criteria: String,

    /// Number of tests passed for this criteria.
    pub r#passed: i64,

    /// Number of tests failed for this criteria.
    pub r#failed: i64,
}

/// Counters summarizing the outcomes of the evaluation run.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRun_ResultCounts {
    /// Total number of executed output items.
    pub r#total: i64,

    /// Number of output items that resulted in an error.
    pub r#errored: i64,

    /// Number of output items that failed to pass the evaluation.
    pub r#failed: i64,

    /// Number of output items that passed the evaluation.
    pub r#passed: i64,
}

/// A ScoreModelGrader object that uses a model to assign a score to the input.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalScoreModelGrader {
    /// The object type, which is always `score_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    /// The sampling parameters for the model.
    pub r#sampling_params: Option<EvalScoreModelGrader_SamplingParams>,

    /// The input text.
    pub r#input: Vec<EvalItem>,

    /// The threshold for the score.
    pub r#pass_threshold: Option<f64>,

    /// The range of the score.
    pub r#range: Option<Vec<f64>>,
}

/// The sampling parameters for the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalScoreModelGrader_SamplingParams;

/// A StoredCompletionsDataSourceConfig which specifies the metadata property of
/// your stored completions query.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStoredCompletionsDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    pub r#metadata: Option<Metadata>,

    /// The json schema for the run data source items.
    pub r#schema: EvalStoredCompletionsDataSourceConfig_Schema,
}

/// The json schema for the run data source items.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStoredCompletionsDataSourceConfig_Schema;

/// A StoredCompletionsRunDataSource configuration describing a set of filters
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStoredCompletionsSource {
    /// The type of source.
    pub r#type: String,

    pub r#metadata: Option<Metadata>,

    /// An optional model to filter by (e.g., 'gpt-4o').
    pub r#model: Option<String>,

    /// An optional Unix timestamp to filter items created after this time.
    pub r#created_after: Option<i64>,

    /// An optional Unix timestamp to filter items created before this time.
    pub r#created_before: Option<i64>,

    /// An optional maximum number of items to return.
    pub r#limit: Option<i64>,
}

/// A StringCheckGrader object that performs a string comparison between input
/// and reference using a specified operation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStringCheckGrader {
    /// The object type, which is always `string_check`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The input text.
    pub r#input: String,

    /// The reference text.
    pub r#reference: String,

    /// The string check operation to perform.
    pub r#operation: String,
}

/// A TextSimilarityGrader object which grades text based on similarity metrics.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalTextSimilarityGrader {
    /// The type of grader.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: Option<String>,

    /// The text being graded.
    pub r#input: String,

    /// The text being graded against.
    pub r#reference: String,

    /// A float score where a value greater than or equal indicates a passing
    /// grade.
    pub r#pass_threshold: f64,

    /// The evaluation metric to use.
    pub r#evaluation_metric: String,
}

/// Configuration of data sources used in runs of the evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Eval_DataSourceConfig {
    EvalCustomDataSourceConfig(EvalCustomDataSourceConfig),

    EvalStoredCompletionsDataSourceConfig(EvalStoredCompletionsDataSourceConfig),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Eval_TestingCriteria {
    EvalLabelModelGrader(EvalLabelModelGrader),

    EvalStringCheckGrader(EvalStringCheckGrader),

    EvalTextSimilarityGrader(EvalTextSimilarityGrader),

    EvalPythonGrader(EvalPythonGrader),

    EvalScoreModelGrader(EvalScoreModelGrader),
}

/// A citation to a file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileCitationBody {
    /// The type of the file citation.
    pub r#type: String,

    /// The ID of the file.
    pub r#file_id: String,

    /// The index of the file in the list of files.
    pub r#index: i64,
}

/// A path to a file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilePath {
    /// The type of the file path.
    pub r#type: String,

    /// The ID of the file.
    pub r#file_id: String,

    /// The index of the file in the list of files.
    pub r#index: i64,
}

/// The ranker to use for the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileSearchRanker {
    #[cfg_attr(feature = "serde", serde(rename = "auto"))]
    Auto,

    #[cfg_attr(feature = "serde", serde(rename = "default_2024_08_21"))]
    Default20240821,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// The ranking options for the file search.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchRankingOptions {
    pub r#ranker: Option<FileSearchRanker>,

    /// The score threshold for the file search.
    pub r#score_threshold: f64,
}

/// A tool that searches for relevant content from uploaded files.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchTool {
    /// The type of the file search tool.
    pub r#type: String,

    /// The IDs of the vector stores to search.
    pub r#vector_store_ids: Vec<String>,

    /// The maximum number of results to return.
    pub r#max_num_results: Option<i64>,

    /// Ranking options for search.
    pub r#ranking_options: Option<RankingOptions>,

    pub r#filters: Option<FileSearchTool_Filters>,
}

/// The results of a file search tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    pub r#id: String,

    /// The type of the file search tool call.
    pub r#type: String,

    /// The status of the file search tool call.
    pub r#status: String,

    /// The queries used to search for files.
    pub r#queries: Vec<String>,

    /// The results of the file search tool call.
    pub r#results: Option<Vec<FileSearchToolCall_Results>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchToolCall_Results {
    /// The unique ID of the file.
    pub r#file_id: Option<String>,

    /// The text that was retrieved from the file.
    pub r#text: Option<String>,

    /// The name of the file.
    pub r#filename: Option<String>,

    pub r#attributes: Option<VectorStoreFileAttributes>,

    /// The relevance score of the file - a value between 0 and 1.
    pub r#score: Option<f64>,
}

pub type FileSearchTool_Filters = Option<FileSearchTool_Filters_1>;

/// A filter to apply.
pub type FileSearchTool_Filters_1 = Filters;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Filters {
    ComparisonFilter(ComparisonFilter),

    CompoundFilter(CompoundFilter),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneChatCompletionRequestAssistantMessage {
    /// Controls whether the assistant message is trained against (0 or 1)
    pub r#weight: Option<i64>,

    pub r#content: Option<ChatCompletionRequestAssistantMessage_Content>,

    /// The refusal message by the assistant.
    pub r#refusal: Option<String>,

    /// The role of the messages author, in this case `assistant`.
    pub r#role: String,

    /// An optional name for the participant.
    pub r#name: Option<String>,

    pub r#audio: Option<ChatCompletionRequestAssistantMessage_Audio>,

    pub r#tool_calls: Option<ChatCompletionMessageToolCalls>,

    pub r#function_call: Option<ChatCompletionRequestAssistantMessage_FunctionCall>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneChatCompletionRequestAssistantMessage_Variant1 {
    /// Controls whether the assistant message is trained against (0 or 1)
    pub r#weight: Option<i64>,
}

/// The per-line training example of a fine-tuning input file for chat models
/// using the supervised method.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneChatRequestInput {
    pub r#messages: Option<Vec<FineTuneChatRequestInput_Messages>>,

    /// A list of tools the model may generate JSON inputs for.
    pub r#tools: Option<Vec<ChatCompletionTool>>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,

    /// A list of functions the model may generate JSON inputs for.
    pub r#functions: Option<Vec<ChatCompletionFunctions>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneChatRequestInput_Messages {
    ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),

    ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),

    FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),

    ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),

    ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

/// The per-line training example of a fine-tuning input file for completions
/// models
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneCompletionRequestInput {
    /// The input prompt for this training example.
    pub r#prompt: Option<String>,

    /// The desired completion for this training example.
    pub r#completion: Option<String>,
}

/// Configuration for the DPO fine-tuning method.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneDPOMethod {
    pub r#hyperparameters: Option<FineTuneDPOMethod_Hyperparameters>,
}

/// The hyperparameters used for the fine-tuning job.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneDPOMethod_Hyperparameters {
    pub r#beta: Option<FineTuneDPOMethod_Hyperparameters_Beta>,

    pub r#batch_size: Option<FineTuneDPOMethod_Hyperparameters_BatchSize>,

    pub r#learning_rate_multiplier: Option<FineTuneDPOMethod_Hyperparameters_LearningRateMultiplier>,

    pub r#n_epochs: Option<FineTuneDPOMethod_Hyperparameters_NEpochs>,
}

/// Number of examples in each batch.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneDPOMethod_Hyperparameters_BatchSize {
    Text(String),

    Integer(i64),
}

/// The beta value for the DPO method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneDPOMethod_Hyperparameters_Beta {
    Text(String),

    Number(f64),
}

/// Scaling factor for the learning rate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneDPOMethod_Hyperparameters_LearningRateMultiplier {
    Text(String),

    Number(f64),
}

/// The number of epochs to train the model for.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneDPOMethod_Hyperparameters_NEpochs {
    Text(String),

    Integer(i64),
}

/// The method used for fine-tuning.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneMethod {
    /// The type of method.
    pub r#type: Option<String>,

    pub r#supervised: Option<FineTuneSupervisedMethod>,

    pub r#dpo: Option<FineTuneDPOMethod>,
}

/// The per-line training example of a fine-tuning input file for chat models
/// using the dpo method.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTunePreferenceRequestInput {
    pub r#input: Option<FineTunePreferenceRequestInput_Input>,

    /// The preferred completion message for the output.
    pub r#preferred_completion: Option<Vec<ChatCompletionRequestAssistantMessage>>,

    /// The non-preferred completion message for the output.
    pub r#non_preferred_completion: Option<Vec<ChatCompletionRequestAssistantMessage>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTunePreferenceRequestInput_Input {
    pub r#messages: Option<Vec<FineTunePreferenceRequestInput_Input_Messages>>,

    /// A list of tools the model may generate JSON inputs for.
    pub r#tools: Option<Vec<ChatCompletionTool>>,

    pub r#parallel_tool_calls: Option<ParallelToolCalls>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTunePreferenceRequestInput_Input_Messages {
    ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),

    ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),

    FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),

    ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),

    ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

/// Configuration for the supervised fine-tuning method.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneSupervisedMethod {
    pub r#hyperparameters: Option<FineTuneSupervisedMethod_Hyperparameters>,
}

/// The hyperparameters used for the fine-tuning job.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneSupervisedMethod_Hyperparameters {
    pub r#batch_size: Option<FineTuneSupervisedMethod_Hyperparameters_BatchSize>,

    pub r#learning_rate_multiplier: Option<FineTuneSupervisedMethod_Hyperparameters_LearningRateMultiplier>,

    pub r#n_epochs: Option<FineTuneSupervisedMethod_Hyperparameters_NEpochs>,
}

/// Number of examples in each batch.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneSupervisedMethod_Hyperparameters_BatchSize {
    Text(String),

    Integer(i64),
}

/// Scaling factor for the learning rate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneSupervisedMethod_Hyperparameters_LearningRateMultiplier {
    Text(String),

    Number(f64),
}

/// The number of epochs to train the model for.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuneSupervisedMethod_Hyperparameters_NEpochs {
    Text(String),

    Integer(i64),
}

/// The `checkpoint.permission` object represents a permission for a fine-tuned
/// model checkpoint.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningCheckpointPermission {
    /// The permission identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the permission was created.
    pub r#created_at: i64,

    /// The project identifier that the permission is for.
    pub r#project_id: String,

    /// The object type, which is always "checkpoint.permission".
    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningIntegration {
    /// The type of the integration being enabled for the fine-tuning job
    pub r#type: String,

    pub r#wandb: FineTuningIntegration_Wandb,
}

/// The settings for your integration with Weights and Biases.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningIntegration_Wandb {
    /// The name of the project that the new run will be created under.
    pub r#project: String,

    /// A display name to set for the run.
    pub r#name: Option<String>,

    /// The entity to use for the run.
    pub r#entity: Option<String>,

    /// A list of tags to be attached to the newly created run.
    pub r#tags: Option<Vec<String>>,
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been
/// created through the API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// created.
    pub r#created_at: i64,

    pub r#error: Option<FineTuningJob_Error>,

    /// The name of the fine-tuned model that is being created.
    pub r#fine_tuned_model: Option<String>,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// finished.
    pub r#finished_at: Option<i64>,

    pub r#hyperparameters: FineTuningJob_Hyperparameters,

    /// The base model that is being fine-tuned.
    pub r#model: String,

    /// The object type, which is always "fine_tuning.job".
    pub r#object: String,

    /// The organization that owns the fine-tuning job.
    pub r#organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    pub r#result_files: Vec<String>,

    /// The current status of the fine-tuning job, which can be either
    /// `validating_files`, `queued`, `running`, `succeeded`, `failed`, or
    /// `cancelled`.
    pub r#status: String,

    /// The total number of billable tokens processed by this fine-tuning job.
    pub r#trained_tokens: Option<i64>,

    /// The file ID used for training.
    pub r#training_file: String,

    /// The file ID used for validation.
    pub r#validation_file: Option<String>,

    /// A list of integrations to enable for this fine-tuning job.
    pub r#integrations: Option<Vec<FineTuningIntegration>>,

    /// The seed used for the fine-tuning job.
    pub r#seed: i64,

    /// The Unix timestamp (in seconds) for when the fine-tuning job is
    /// estimated to finish.
    pub r#estimated_finish: Option<i64>,

    pub r#method: Option<FineTuneMethod>,

    pub r#metadata: Option<Metadata>,
}

/// The `fine_tuning.job.checkpoint` object represents a model checkpoint for a
/// fine-tuning job that is ready to use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobCheckpoint {
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub r#created_at: i64,

    /// The name of the fine-tuned checkpoint model that is created.
    pub r#fine_tuned_model_checkpoint: String,

    /// The step number that the checkpoint was created at.
    pub r#step_number: i64,

    pub r#metrics: FineTuningJobCheckpoint_Metrics,

    /// The name of the fine-tuning job that this checkpoint was created from.
    pub r#fine_tuning_job_id: String,

    /// The object type, which is always "fine_tuning.job.checkpoint".
    pub r#object: String,
}

/// Metrics at the step number during the fine-tuning job.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobCheckpoint_Metrics {
    pub r#step: Option<f64>,

    pub r#train_loss: Option<f64>,

    pub r#train_mean_token_accuracy: Option<f64>,

    pub r#valid_loss: Option<f64>,

    pub r#valid_mean_token_accuracy: Option<f64>,

    pub r#full_valid_loss: Option<f64>,

    pub r#full_valid_mean_token_accuracy: Option<f64>,
}

/// Fine-tuning job event object
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobEvent {
    /// The object type, which is always "fine_tuning.job.event".
    pub r#object: String,

    /// The object identifier.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// created.
    pub r#created_at: i64,

    /// The log level of the event.
    pub r#level: String,

    /// The message of the event.
    pub r#message: String,

    /// The type of event.
    pub r#type: Option<String>,

    /// The data associated with the event.
    pub r#data: Option<FineTuningJobEvent_Data>,
}

/// The data associated with the event.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobEvent_Data;

/// For fine-tuning jobs that have `failed`, this will contain more information
/// on the cause of the failure.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJob_Error {
    /// A machine-readable error code.
    pub r#code: String,

    /// A human-readable error message.
    pub r#message: String,

    /// The parameter that was invalid, usually `training_file` or
    /// `validation_file`.
    pub r#param: Option<String>,
}

/// The hyperparameters used for the fine-tuning job.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJob_Hyperparameters {
    pub r#batch_size: Option<FineTuningJob_Hyperparameters_BatchSize>,

    pub r#learning_rate_multiplier: Option<FineTuningJob_Hyperparameters_LearningRateMultiplier>,

    pub r#n_epochs: Option<FineTuningJob_Hyperparameters_NEpochs>,
}

/// Number of examples in each batch.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuningJob_Hyperparameters_BatchSize {
    Text(String),

    Integer(i64),
}

/// Scaling factor for the learning rate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuningJob_Hyperparameters_LearningRateMultiplier {
    Text(String),

    Number(f64),
}

/// The number of epochs to train the model for.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FineTuningJob_Hyperparameters_NEpochs {
    Text(String),

    Integer(i64),
}

/// The output of a function tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallOutputItemParam {
    pub r#id: Option<FunctionCallOutputItemParam_Id>,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// The type of the function tool call output.
    pub r#type: String,

    /// A JSON string of the output of the function tool call.
    pub r#output: String,

    pub r#status: Option<FunctionCallOutputItemParam_Status>,
}

pub type FunctionCallOutputItemParam_Id = Option<String>;

pub type FunctionCallOutputItemParam_Status = Option<String>;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionObject {
    /// A description of what the function does, used by the model to choose
    /// when and how to call the function.
    pub r#description: Option<String>,

    /// The name of the function to be called.
    pub r#name: String,

    pub r#parameters: Option<FunctionParameters>,

    /// Whether to enable strict schema adherence when generating the function
    /// call.
    pub r#strict: Option<bool>,
}

/// The parameters the functions accepts, described as a JSON Schema object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionParameters;

/// Defines a function in your own code the model can choose to call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionTool {
    /// The type of the function tool.
    pub r#type: String,

    /// The name of the function to call.
    pub r#name: String,

    pub r#description: Option<FunctionTool_Description>,

    pub r#parameters: FunctionTool_Parameters,

    pub r#strict: FunctionTool_Strict,
}

/// A tool call to run a function.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCall {
    /// The unique ID of the function tool call.
    pub r#id: Option<String>,

    /// The type of the function tool call.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// The name of the function to run.
    pub r#name: String,

    /// A JSON string of the arguments to pass to the function.
    pub r#arguments: String,

    /// The status of the item.
    pub r#status: Option<String>,
}

/// The output of a function tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallOutput {
    /// The unique ID of the function tool call output.
    pub r#id: Option<String>,

    /// The type of the function tool call output.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// A JSON string of the output of the function tool call.
    pub r#output: String,

    /// The status of the item.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallOutputResource {
    /// The unique ID of the function call tool output.
    pub r#id: String,

    /// The type of the function tool call output.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// A JSON string of the output of the function tool call.
    pub r#output: String,

    /// The status of the item.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallOutputResource_Variant2 {
    /// The unique ID of the function call tool output.
    pub r#id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallResource {
    /// The unique ID of the function tool call.
    pub r#id: String,

    /// The type of the function tool call.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// The name of the function to run.
    pub r#name: String,

    /// A JSON string of the arguments to pass to the function.
    pub r#arguments: String,

    /// The status of the item.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallResource_Variant2 {
    /// The unique ID of the function tool call.
    pub r#id: String,
}

pub type FunctionTool_Description = Option<String>;

pub type FunctionTool_Parameters = Option<FunctionTool_Parameters_1>;

/// A JSON schema object describing the parameters of the function.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionTool_Parameters_1;

pub type FunctionTool_Strict = Option<FunctionTool_Strict_1>;

/// Whether to enforce strict parameter validation.
pub type FunctionTool_Strict_1 = bool;

/// Represents the content or the URL of an image generated by the OpenAI API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// The base64-encoded JSON of the generated image.
    pub r#b64_json: Option<String>,

    /// When using `dall-e-2` or `dall-e-3`, the URL of the generated image if
    /// `response_format` is set to `url` (default value).
    pub r#url: Option<String>,

    /// For `dall-e-3` only, the revised prompt that was used to generate the
    /// image.
    pub r#revised_prompt: Option<String>,
}

/// The response from the image generation endpoint.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImagesResponse {
    /// The Unix timestamp (in seconds) of when the image was created.
    pub r#created: i64,

    /// The list of generated images.
    pub r#data: Option<Vec<Image>>,

    pub r#usage: Option<ImagesResponse_Usage>,
}

/// For `gpt-image-1` only, the token usage information for the image
/// generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImagesResponse_Usage {
    /// The total number of tokens (images and text) used for the image
    /// generation.
    pub r#total_tokens: i64,

    /// The number of tokens (images and text) in the input prompt.
    pub r#input_tokens: i64,

    /// The number of image tokens in the output image.
    pub r#output_tokens: i64,

    pub r#input_tokens_details: ImagesResponse_Usage_InputTokensDetails,
}

/// The input tokens detailed information for the image generation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImagesResponse_Usage_InputTokensDetails {
    /// The number of text tokens in the input prompt.
    pub r#text_tokens: i64,

    /// The number of image tokens in the input prompt.
    pub r#image_tokens: i64,
}

/// Specify additional output data to include in the model response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Includable {
    #[cfg_attr(feature = "serde", serde(rename = "file_search_call.results"))]
    FileSearchCallResults,

    #[cfg_attr(feature = "serde", serde(rename = "message.input_image.image_url"))]
    MessageInputImageImageUrl,

    #[cfg_attr(feature = "serde", serde(rename = "computer_call_output.output.image_url"))]
    ComputerCallOutputOutputImageUrl,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// An audio input to the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputAudio {
    /// The type of the input item.
    pub r#type: String,

    /// Base64-encoded audio data.
    pub r#data: String,

    /// The format of the audio data.
    pub r#format: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InputContent {
    InputTextContent(InputTextContent),

    InputImageContent(InputImageContent),

    InputFileContent(InputFileContent),
}

/// A file input to the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputFileContent {
    /// The type of the input item.
    pub r#type: String,

    pub r#file_id: Option<InputFileContent_FileId>,

    /// The name of the file to be sent to the model.
    pub r#filename: Option<String>,

    /// The content of the file to be sent to the model.
    pub r#file_data: Option<String>,
}

pub type InputFileContent_FileId = Option<String>;

/// An image input to the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputImageContent {
    /// The type of the input item.
    pub r#type: String,

    pub r#image_url: Option<InputImageContent_ImageUrl>,

    pub r#file_id: Option<InputImageContent_FileId>,

    /// The detail level of the image to be sent to the model.
    pub r#detail: String,
}

pub type InputImageContent_FileId = Option<String>;

pub type InputImageContent_ImageUrl = Option<String>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InputItem {
    EasyInputMessage(EasyInputMessage),

    /// An item representing part of the context for the response to be
    /// generated by the model.
    Item(Item),

    ItemReferenceParam(ItemReferenceParam),
}

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessage {
    /// The type of the message input.
    pub r#type: Option<String>,

    /// The role of the message input.
    pub r#role: String,

    /// The status of item.
    pub r#status: Option<String>,

    pub r#content: InputMessageContentList,
}

/// A list of one or many input items to the model, containing different content
/// types.
pub type InputMessageContentList = Vec<InputContent>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessageResource {
    /// The type of the message input.
    pub r#type: Option<String>,

    /// The role of the message input.
    pub r#role: String,

    /// The status of item.
    pub r#status: Option<String>,

    pub r#content: InputMessageContentList,

    /// The unique ID of the message input.
    pub r#id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessageResource_Variant2 {
    /// The unique ID of the message input.
    pub r#id: String,
}

/// A text input to the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputTextContent {
    /// The type of the input item.
    pub r#type: String,

    /// The text input to the model.
    pub r#text: String,
}

/// Represents an individual `invite` to the organization.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Invite {
    /// The object type, which is always `organization.invite`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The email address of the individual to whom the invite was sent
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// `accepted`,`expired`, or `pending`
    pub r#status: String,

    /// The Unix timestamp (in seconds) of when the invite was sent.
    pub r#invited_at: i64,

    /// The Unix timestamp (in seconds) of when the invite expires.
    pub r#expires_at: i64,

    /// The Unix timestamp (in seconds) of when the invite was accepted.
    pub r#accepted_at: Option<i64>,

    /// The projects that were granted membership upon acceptance of the invite.
    pub r#projects: Option<Vec<Invite_Projects>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteDeleteResponse {
    /// The object type, which is always `organization.invite.deleted`
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteListResponse {
    /// The object type, which is always `list`
    pub r#object: String,

    pub r#data: Vec<Invite>,

    /// The first `invite_id` in the retrieved `list`
    pub r#first_id: Option<String>,

    /// The last `invite_id` in the retrieved `list`
    pub r#last_id: Option<String>,

    /// The `has_more` property is used for pagination to indicate there are
    /// additional results.
    pub r#has_more: Option<bool>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteRequest {
    /// Send an email to this address
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// An array of projects to which membership is granted at the same time the
    /// org invite is accepted.
    pub r#projects: Option<Vec<InviteRequest_Projects>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteRequest_Projects {
    /// Project's public ID
    pub r#id: String,

    /// Project membership role
    pub r#role: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Invite_Projects {
    /// Project's public ID
    pub r#id: Option<String>,

    /// Project membership role
    pub r#role: Option<String>,
}

/// Content item used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Item {
    InputMessage(InputMessage),

    OutputMessage(OutputMessage),

    FileSearchToolCall(FileSearchToolCall),

    ComputerToolCall(ComputerToolCall),

    ComputerCallOutputItemParam(ComputerCallOutputItemParam),

    WebSearchToolCall(WebSearchToolCall),

    FunctionToolCall(FunctionToolCall),

    FunctionCallOutputItemParam(FunctionCallOutputItemParam),

    ReasoningItem(ReasoningItem),
}

/// An internal identifier for an item to reference.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemReferenceParam {
    pub r#type: Option<ItemReferenceParam_Type>,

    /// The ID of the item to reference.
    pub r#id: String,
}

pub type ItemReferenceParam_Type = Option<String>;

/// Content item used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemResource {
    InputMessageResource(InputMessageResource),

    OutputMessage(OutputMessage),

    FileSearchToolCall(FileSearchToolCall),

    ComputerToolCall(ComputerToolCall),

    ComputerToolCallOutputResource(ComputerToolCallOutputResource),

    WebSearchToolCall(WebSearchToolCall),

    FunctionToolCallResource(FunctionToolCallResource),

    FunctionToolCallOutputResource(FunctionToolCallOutputResource),
}

/// A collection of keypresses the model would like to perform.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyPress {
    /// Specifies the event type.
    pub r#type: String,

    /// The combination of keys the model is requesting to be pressed.
    pub r#keys: Vec<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListAssistantsResponse {
    pub r#object: String,

    pub r#data: Vec<AssistantObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListAuditLogsResponse {
    pub r#object: String,

    pub r#data: Vec<AuditLog>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListBatchesResponse {
    pub r#data: Vec<Batch>,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListCertificatesResponse {
    pub r#data: Vec<Certificate>,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFilesResponse {
    pub r#object: String,

    pub r#data: Vec<OpenAIFile>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningCheckpointPermissionResponse {
    pub r#data: Vec<FineTuningCheckpointPermission>,

    pub r#object: String,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningJobCheckpointsResponse {
    pub r#data: Vec<FineTuningJobCheckpoint>,

    pub r#object: String,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningJobEventsResponse {
    pub r#data: Vec<FineTuningJobEvent>,

    pub r#object: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListMessagesResponse {
    pub r#object: String,

    pub r#data: Vec<MessageObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListModelsResponse {
    pub r#object: String,

    pub r#data: Vec<Model>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListPaginatedFineTuningJobsResponse {
    pub r#data: Vec<FineTuningJob>,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListRunStepsResponse {
    pub r#object: String,

    pub r#data: Vec<RunStepObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListRunsResponse {
    pub r#object: String,

    pub r#data: Vec<RunObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListVectorStoreFilesResponse {
    pub r#object: String,

    pub r#data: Vec<VectorStoreFileObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListVectorStoresResponse {
    pub r#object: String,

    pub r#data: Vec<VectorStoreObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

/// A log probability object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogProbProperties {
    /// The token that was used to generate the log probability.
    pub r#token: String,

    /// The log probability of the token.
    pub r#logprob: f64,

    /// The bytes that were used to generate the log probability.
    pub r#bytes: Vec<i64>,
}

/// References an image [File](/docs/api-reference/files) in the content of a
/// message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageFileObject {
    /// Always `image_file`.
    pub r#type: String,

    pub r#image_file: MessageContentImageFileObject_ImageFile,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageFileObject_ImageFile {
    /// The [File](/docs/api-reference/files) ID of the image in the message
    /// content.
    pub r#file_id: String,

    /// Specifies the detail level of the image if specified by the user.
    pub r#detail: Option<String>,
}

/// References an image URL in the content of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageUrlObject {
    /// The type of the content part.
    pub r#type: String,

    pub r#image_url: MessageContentImageUrlObject_ImageUrl,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageUrlObject_ImageUrl {
    /// The external URL of the image, must be a supported image types: jpeg,
    /// jpg, png, gif, webp.
    pub r#url: String,

    /// Specifies the detail level of the image.
    pub r#detail: Option<String>,
}

/// The refusal content generated by the assistant.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentRefusalObject {
    /// Always `refusal`.
    pub r#type: String,

    pub r#refusal: String,
}

/// A citation within the message that points to a specific quote from a
/// specific File associated with the assistant or the message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFileCitationObject {
    /// Always `file_citation`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_citation: MessageContentTextAnnotationsFileCitationObject_FileCitation,

    pub r#start_index: i64,

    pub r#end_index: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFileCitationObject_FileCitation {
    /// The ID of the specific File the citation is from.
    pub r#file_id: String,
}

/// A URL for the file that's generated when the assistant used the
/// `code_interpreter` tool to generate a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFilePathObject {
    /// Always `file_path`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_path: MessageContentTextAnnotationsFilePathObject_FilePath,

    pub r#start_index: i64,

    pub r#end_index: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFilePathObject_FilePath {
    /// The ID of the file that was generated.
    pub r#file_id: String,
}

/// The text content that is part of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextObject {
    /// Always `text`.
    pub r#type: String,

    pub r#text: MessageContentTextObject_Text,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextObject_Text {
    /// The data that makes up the text.
    pub r#value: String,

    pub r#annotations: Vec<MessageContentTextObject_Text_Annotations>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MessageContentTextObject_Text_Annotations {
    MessageContentTextAnnotationsFileCitationObject(MessageContentTextAnnotationsFileCitationObject),

    MessageContentTextAnnotationsFilePathObject(MessageContentTextAnnotationsFilePathObject),
}

/// References an image [File](/docs/api-reference/files) in the content of a
/// message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageFileObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `image_file`.
    pub r#type: String,

    pub r#image_file: Option<MessageDeltaContentImageFileObject_ImageFile>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageFileObject_ImageFile {
    /// The [File](/docs/api-reference/files) ID of the image in the message
    /// content.
    pub r#file_id: Option<String>,

    /// Specifies the detail level of the image if specified by the user.
    pub r#detail: Option<String>,
}

/// References an image URL in the content of a message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageUrlObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `image_url`.
    pub r#type: String,

    pub r#image_url: Option<MessageDeltaContentImageUrlObject_ImageUrl>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageUrlObject_ImageUrl {
    /// The URL of the image, must be a supported image types: jpeg, jpg, png,
    /// gif, webp.
    pub r#url: Option<String>,

    /// Specifies the detail level of the image.
    pub r#detail: Option<String>,
}

/// The refusal content that is part of a message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentRefusalObject {
    /// The index of the refusal part in the message.
    pub r#index: i64,

    /// Always `refusal`.
    pub r#type: String,

    pub r#refusal: Option<String>,
}

/// A citation within the message that points to a specific quote from a
/// specific File associated with the assistant or the message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    /// The index of the annotation in the text content part.
    pub r#index: i64,

    /// Always `file_citation`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: Option<String>,

    pub r#file_citation: Option<MessageDeltaContentTextAnnotationsFileCitationObject_FileCitation>,

    pub r#start_index: Option<i64>,

    pub r#end_index: Option<i64>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject_FileCitation {
    /// The ID of the specific File the citation is from.
    pub r#file_id: Option<String>,

    /// The specific quote in the file.
    pub r#quote: Option<String>,
}

/// A URL for the file that's generated when the assistant used the
/// `code_interpreter` tool to generate a file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    /// The index of the annotation in the text content part.
    pub r#index: i64,

    /// Always `file_path`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: Option<String>,

    pub r#file_path: Option<MessageDeltaContentTextAnnotationsFilePathObject_FilePath>,

    pub r#start_index: Option<i64>,

    pub r#end_index: Option<i64>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFilePathObject_FilePath {
    /// The ID of the file that was generated.
    pub r#file_id: Option<String>,
}

/// The text content that is part of a message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `text`.
    pub r#type: String,

    pub r#text: Option<MessageDeltaContentTextObject_Text>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextObject_Text {
    /// The data that makes up the text.
    pub r#value: Option<String>,

    pub r#annotations: Option<Vec<MessageDeltaContentTextObject_Text_Annotations>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MessageDeltaContentTextObject_Text_Annotations {
    MessageDeltaContentTextAnnotationsFileCitationObject(MessageDeltaContentTextAnnotationsFileCitationObject),

    MessageDeltaContentTextAnnotationsFilePathObject(MessageDeltaContentTextAnnotationsFilePathObject),
}

/// Represents a message delta i.e.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaObject {
    /// The identifier of the message, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.message.delta`.
    pub r#object: String,

    pub r#delta: MessageDeltaObject_Delta,
}

/// The delta containing the fields that have changed on the Message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaObject_Delta {
    /// The entity that produced the message.
    pub r#role: Option<String>,

    /// The content of the message in array of text and/or images.
    pub r#content: Option<Vec<MessageDeltaObject_Delta_Content>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MessageDeltaObject_Delta_Content {
    MessageDeltaContentImageFileObject(MessageDeltaContentImageFileObject),

    MessageDeltaContentTextObject(MessageDeltaContentTextObject),

    MessageDeltaContentRefusalObject(MessageDeltaContentRefusalObject),

    MessageDeltaContentImageUrlObject(MessageDeltaContentImageUrlObject),
}

/// Represents a message within a [thread](/docs/api-reference/threads).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.message`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the message was created.
    pub r#created_at: i64,

    /// The [thread](/docs/api-reference/threads) ID that this message belongs
    /// to.
    pub r#thread_id: String,

    /// The status of the message, which can be either `in_progress`,
    /// `incomplete`, or `completed`.
    pub r#status: String,

    pub r#incomplete_details: Option<MessageObject_IncompleteDetails>,

    /// The Unix timestamp (in seconds) for when the message was completed.
    pub r#completed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the message was marked as
    /// incomplete.
    pub r#incomplete_at: Option<i64>,

    /// The entity that produced the message.
    pub r#role: String,

    /// The content of the message in array of text and/or images.
    pub r#content: Vec<MessageObject_Content>,

    /// If applicable, the ID of the [assistant](/docs/api-reference/assistants)
    /// that authored this message.
    pub r#assistant_id: Option<String>,

    /// The ID of the [run](/docs/api-reference/runs) associated with the
    /// creation of this message.
    pub r#run_id: Option<String>,

    /// A list of files attached to the message, and the tools they were added
    /// to.
    pub r#attachments: Option<Vec<MessageObject_Attachments>>,

    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageObject_Attachments {
    /// The ID of the file to attach to the message.
    pub r#file_id: Option<String>,

    /// The tools to add this file to.
    pub r#tools: Option<Vec<MessageObject_Attachments_Tools>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MessageObject_Attachments_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MessageObject_Content {
    MessageContentImageFileObject(MessageContentImageFileObject),

    MessageContentImageUrlObject(MessageContentImageUrlObject),

    MessageContentTextObject(MessageContentTextObject),

    MessageContentRefusalObject(MessageContentRefusalObject),
}

/// On an incomplete message, details about why the message is incomplete.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageObject_IncompleteDetails {
    /// The reason the message is incomplete.
    pub r#reason: String,
}

/// The text content that is part of a message.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageRequestContentTextObject {
    /// Always `text`.
    pub r#type: String,

    /// Text content to be sent to the model
    pub r#text: String,
}

include!("schemas/message_stream_event.rs");

/// Occurs when a [message](/docs/api-reference/messages/object) is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageStreamEvent_Variant1 {
    pub r#event: String,

    pub r#data: MessageObject,
}

/// Occurs when a [message](/docs/api-reference/messages/object) moves to an
/// `in_progress` state.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageStreamEvent_Variant2 {
    pub r#event: String,

    pub r#data: MessageObject,
}

/// Occurs when parts of a [Message](/docs/api-reference/messages/object) are
/// being streamed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageStreamEvent_Variant3 {
    pub r#event: String,

    pub r#data: MessageDeltaObject,
}

/// Occurs when a [message](/docs/api-reference/messages/object) is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageStreamEvent_Variant4 {
    pub r#event: String,

    pub r#data: MessageObject,
}

/// Occurs when a [message](/docs/api-reference/messages/object) ends before it
/// is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageStreamEvent_Variant5 {
    pub r#event: String,

    pub r#data: MessageObject,
}

/// Set of 16 key-value pairs that can be attached to an object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata;

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) when the model was created.
    pub r#created: i64,

    /// The object type, which is always "model".
    pub r#object: String,

    /// The organization that owns the model.
    pub r#owned_by: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModelIds {
    ModelIdsShared(ModelIdsShared),

    ModelIdsResponses(ModelIdsResponses),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModelIdsResponses {
    ModelIdsShared(ModelIdsShared),

    Text(String),
}

pub type ModelIdsShared = String;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModelResponseProperties {
    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,

    pub r#service_tier: Option<ServiceTier>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyAssistantRequest {
    pub r#model: Option<ModifyAssistantRequest_Model>,

    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    pub r#tools: Option<Vec<ModifyAssistantRequest_Tools>>,

    pub r#tool_resources: Option<ModifyAssistantRequest_ToolResources>,

    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// ID of the model to use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModifyAssistantRequest_Model {
    Text(String),

    AssistantSupportedModels(AssistantSupportedModels),
}

/// A set of resources that are used by the assistant's tools.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyAssistantRequest_ToolResources {
    pub r#code_interpreter: Option<ModifyAssistantRequest_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<ModifyAssistantRequest_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyAssistantRequest_ToolResources_CodeInterpreter {
    /// Overrides the list of [file](/docs/api-reference/files) IDs made
    /// available to the `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyAssistantRequest_ToolResources_FileSearch {
    /// Overrides the [vector store](/docs/api-reference/vector-stores/object)
    /// attached to this assistant.
    pub r#vector_store_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModifyAssistantRequest_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyCertificateRequest {
    /// The updated name for the certificate
    pub r#name: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyMessageRequest {
    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyRunRequest {
    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyThreadRequest {
    pub r#tool_resources: Option<ModifyThreadRequest_ToolResources>,

    pub r#metadata: Option<Metadata>,
}

/// A set of resources that are made available to the assistant's tools in this
/// thread.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyThreadRequest_ToolResources {
    pub r#code_interpreter: Option<ModifyThreadRequest_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<ModifyThreadRequest_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyThreadRequest_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyThreadRequest_ToolResources_FileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to
    /// this thread.
    pub r#vector_store_ids: Option<Vec<String>>,
}

/// A mouse move action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Move {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate to move to.
    pub r#x: i64,

    /// The y-coordinate to move to.
    pub r#y: i64,
}

/// The `File` object represents a document that has been uploaded to OpenAI.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenAIFile {
    /// The file identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The size of the file, in bytes.
    pub r#bytes: i64,

    /// The Unix timestamp (in seconds) for when the file was created.
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) for when the file will expire.
    pub r#expires_at: Option<i64>,

    /// The name of the file.
    pub r#filename: String,

    /// The object type, which is always `file`.
    pub r#object: String,

    /// The intended purpose of the file.
    pub r#purpose: String,

    /// Deprecated.
    pub r#status: String,

    /// Deprecated.
    pub r#status_details: Option<String>,
}

/// This is returned when the chunking strategy is unknown.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OtherChunkingStrategyResponseParam {
    /// Always `other`.
    pub r#type: String,
}

/// An audio output from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputAudio {
    /// The type of the output audio.
    pub r#type: String,

    /// Base64-encoded audio data from the model.
    pub r#data: String,

    /// The transcript of the audio data from the model.
    pub r#transcript: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OutputContent {
    OutputTextContent(OutputTextContent),

    RefusalContent(RefusalContent),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OutputItem {
    OutputMessage(OutputMessage),

    FileSearchToolCall(FileSearchToolCall),

    FunctionToolCall(FunctionToolCall),

    WebSearchToolCall(WebSearchToolCall),

    ComputerToolCall(ComputerToolCall),

    ReasoningItem(ReasoningItem),
}

/// An output message from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputMessage {
    /// The unique ID of the output message.
    pub r#id: String,

    /// The type of the output message.
    pub r#type: String,

    /// The role of the output message.
    pub r#role: String,

    /// The content of the output message.
    pub r#content: Vec<OutputContent>,

    /// The status of the message input.
    pub r#status: String,
}

/// A text output from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputTextContent {
    /// The type of the output text.
    pub r#type: String,

    /// The text output from the model.
    pub r#text: String,

    /// The annotations of the text output.
    pub r#annotations: Vec<Annotation>,
}

/// Whether to enable [parallel function
/// calling](/docs/guides/function-calling#configuring-parallel-function-calling)
/// during tool use.
pub type ParallelToolCalls = bool;

/// Static predicted output content, such as the content of a text file that is
/// being regenerated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PredictionContent {
    /// The type of the predicted content you want to provide.
    pub r#type: String,

    pub r#content: PredictionContent_Content,
}

/// The content that should be matched when generating a model response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PredictionContent_Content {
    /// The content used for a Predicted Output.
    Text(String),

    /// An array of content parts with a defined type.
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

/// Represents an individual project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The object type, which is always `organization.project`
    pub r#object: String,

    /// The name of the project.
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the project was created.
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the project was archived or
    /// `null`.
    pub r#archived_at: Option<i64>,

    /// `active` or `archived`
    pub r#status: String,
}

/// Represents an individual API key in a project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKey {
    /// The object type, which is always `organization.project.api_key`
    pub r#object: String,

    /// The redacted value of the API key
    pub r#redacted_value: String,

    /// The name of the API key
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the API key was created
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the API key was last used.
    pub r#last_used_at: i64,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    pub r#owner: ProjectApiKey_Owner,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKeyDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKeyListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectApiKey>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKey_Owner {
    /// `user` or `service_account`
    pub r#type: Option<String>,

    pub r#user: Option<ProjectUser>,

    pub r#service_account: Option<ProjectServiceAccount>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectCreateRequest {
    /// The friendly name of the project, this name appears in reports.
    pub r#name: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectListResponse {
    pub r#object: String,

    pub r#data: Vec<Project>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

/// Represents a project rate limit config.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimit {
    /// The object type, which is always `project.rate_limit`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The model this rate limit applies to.
    pub r#model: String,

    /// The maximum requests per minute.
    pub r#max_requests_per_1_minute: i64,

    /// The maximum tokens per minute.
    pub r#max_tokens_per_1_minute: i64,

    /// The maximum images per minute.
    pub r#max_images_per_1_minute: Option<i64>,

    /// The maximum audio megabytes per minute.
    pub r#max_audio_megabytes_per_1_minute: Option<i64>,

    /// The maximum requests per day.
    pub r#max_requests_per_1_day: Option<i64>,

    /// The maximum batch input tokens per day.
    pub r#batch_1_day_max_input_tokens: Option<i64>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimitListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectRateLimit>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimitUpdateRequest {
    /// The maximum requests per minute.
    pub r#max_requests_per_1_minute: Option<i64>,

    /// The maximum tokens per minute.
    pub r#max_tokens_per_1_minute: Option<i64>,

    /// The maximum images per minute.
    pub r#max_images_per_1_minute: Option<i64>,

    /// The maximum audio megabytes per minute.
    pub r#max_audio_megabytes_per_1_minute: Option<i64>,

    /// The maximum requests per day.
    pub r#max_requests_per_1_day: Option<i64>,

    /// The maximum batch input tokens per day.
    pub r#batch_1_day_max_input_tokens: Option<i64>,
}

/// Represents an individual service account in a project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccount {
    /// The object type, which is always `organization.project.service_account`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the service account
    pub r#name: String,

    /// `owner` or `member`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the service account was created
    pub r#created_at: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountApiKey {
    /// The object type, which is always
    /// `organization.project.service_account.api_key`
    pub r#object: String,

    pub r#value: String,

    pub r#name: String,

    pub r#created_at: i64,

    pub r#id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountCreateRequest {
    /// The name of the service account being created.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountCreateResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#name: String,

    /// Service accounts can only have one role of type `member`
    pub r#role: String,

    pub r#created_at: i64,

    pub r#api_key: ProjectServiceAccountApiKey,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectServiceAccount>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUpdateRequest {
    /// The updated name of the project, this name appears in reports.
    pub r#name: String,
}

/// Represents an individual user in a project.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUser {
    /// The object type, which is always `organization.project.user`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the user
    pub r#name: String,

    /// The email address of the user
    pub r#email: String,

    /// `owner` or `member`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the project was added.
    pub r#added_at: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserCreateRequest {
    /// The ID of the user.
    pub r#user_id: String,

    /// `owner` or `member`
    pub r#role: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectUser>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserUpdateRequest {
    /// `owner` or `member`
    pub r#role: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RankingOptions {
    /// The ranker to use for the file search.
    pub r#ranker: Option<String>,

    /// The score threshold for the file search, a number between 0 and 1.
    pub r#score_threshold: Option<f64>,
}

/// A realtime client event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeClientEvent {
    RealtimeClientEventConversationItemCreate(RealtimeClientEventConversationItemCreate),

    RealtimeClientEventConversationItemDelete(RealtimeClientEventConversationItemDelete),

    RealtimeClientEventConversationItemRetrieve(RealtimeClientEventConversationItemRetrieve),

    RealtimeClientEventConversationItemTruncate(RealtimeClientEventConversationItemTruncate),

    RealtimeClientEventInputAudioBufferAppend(RealtimeClientEventInputAudioBufferAppend),

    RealtimeClientEventInputAudioBufferClear(RealtimeClientEventInputAudioBufferClear),

    RealtimeClientEventOutputAudioBufferClear(RealtimeClientEventOutputAudioBufferClear),

    RealtimeClientEventInputAudioBufferCommit(RealtimeClientEventInputAudioBufferCommit),

    RealtimeClientEventResponseCancel(RealtimeClientEventResponseCancel),

    RealtimeClientEventResponseCreate(RealtimeClientEventResponseCreate),

    RealtimeClientEventSessionUpdate(RealtimeClientEventSessionUpdate),

    RealtimeClientEventTranscriptionSessionUpdate(RealtimeClientEventTranscriptionSessionUpdate),
}

/// Add a new Item to the Conversation's context, including messages, function
/// calls, and function call responses.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `conversation.item.create`.
    pub r#type: String,

    /// The ID of the preceding item after which the new item will be inserted.
    pub r#previous_item_id: Option<String>,

    pub r#item: RealtimeConversationItem,
}

/// Send this event when you want to remove any item from the conversation
/// history.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemDelete {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `conversation.item.delete`.
    pub r#type: String,

    /// The ID of the item to delete.
    pub r#item_id: String,
}

/// Send this event when you want to retrieve the server's representation of a
/// specific item in the conversation history.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `conversation.item.retrieve`.
    pub r#type: String,

    /// The ID of the item to retrieve.
    pub r#item_id: String,
}

/// Send this event to truncate a previous assistant message’s audio.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemTruncate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `conversation.item.truncate`.
    pub r#type: String,

    /// The ID of the assistant message item to truncate.
    pub r#item_id: String,

    /// The index of the content part to truncate.
    pub r#content_index: i64,

    /// Inclusive duration up to which audio is truncated, in milliseconds.
    pub r#audio_end_ms: i64,
}

/// Send this event to append audio bytes to the input audio buffer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `input_audio_buffer.append`.
    pub r#type: String,

    /// Base64-encoded audio bytes.
    pub r#audio: String,
}

/// Send this event to clear the audio bytes in the buffer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `input_audio_buffer.clear`.
    pub r#type: String,
}

/// Send this event to commit the user input audio buffer, which will create a
/// new user message item in the conversation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `input_audio_buffer.commit`.
    pub r#type: String,
}

/// **WebRTC Only:** Emit to cut off the current audio response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// The unique ID of the client event used for error handling.
    pub r#event_id: Option<String>,

    /// The event type, must be `output_audio_buffer.clear`.
    pub r#type: String,
}

/// Send this event to cancel an in-progress response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventResponseCancel {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `response.cancel`.
    pub r#type: String,

    /// A specific response ID to cancel - if not provided, will cancel an
    /// in-progress response in the default conversation.
    pub r#response_id: Option<String>,
}

/// This event instructs the server to create a Response, which means triggering
/// model inference.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventResponseCreate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `response.create`.
    pub r#type: String,

    pub r#response: Option<RealtimeResponseCreateParams>,
}

/// Send this event to update the session’s default configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `session.update`.
    pub r#type: String,

    pub r#session: RealtimeSessionCreateRequest,
}

/// Send this event to update a transcription session.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: Option<String>,

    /// The event type, must be `transcription_session.update`.
    pub r#type: String,

    pub r#session: RealtimeTranscriptionSessionCreateRequest,
}

/// The item to add to the conversation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItem {
    /// The unique ID of the item, this can be generated by the client to help
    /// manage server-side context, but is not required because the server will
    /// generate one if not provided.
    pub r#id: Option<String>,

    /// The type of the item (`message`, `function_call`,
    /// `function_call_output`).
    pub r#type: Option<String>,

    /// Identifier for the API object being returned - always `realtime.item`.
    pub r#object: Option<String>,

    /// The status of the item (`completed`, `incomplete`).
    pub r#status: Option<String>,

    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    pub r#role: Option<String>,

    /// The content of the message, applicable for `message` items.
    pub r#content: Option<Vec<RealtimeConversationItem_Content>>,

    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items).
    pub r#call_id: Option<String>,

    /// The name of the function being called (for `function_call` items).
    pub r#name: Option<String>,

    /// The arguments of the function call (for `function_call` items).
    pub r#arguments: Option<String>,

    /// The output of the function call (for `function_call_output` items).
    pub r#output: Option<String>,
}

/// The item to add to the conversation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItemWithReference {
    /// For an item of type (`message` | `function_call` |
    /// `function_call_output`) this field allows the client to assign the
    /// unique ID of the item.
    pub r#id: Option<String>,

    /// The type of the item (`message`, `function_call`,
    /// `function_call_output`, `item_reference`).
    pub r#type: Option<String>,

    /// Identifier for the API object being returned - always `realtime.item`.
    pub r#object: Option<String>,

    /// The status of the item (`completed`, `incomplete`).
    pub r#status: Option<String>,

    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    pub r#role: Option<String>,

    /// The content of the message, applicable for `message` items.
    pub r#content: Option<Vec<RealtimeConversationItemWithReference_Content>>,

    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items).
    pub r#call_id: Option<String>,

    /// The name of the function being called (for `function_call` items).
    pub r#name: Option<String>,

    /// The arguments of the function call (for `function_call` items).
    pub r#arguments: Option<String>,

    /// The output of the function call (for `function_call_output` items).
    pub r#output: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItemWithReference_Content {
    /// The content type (`input_text`, `input_audio`, `item_reference`,
    /// `text`).
    pub r#type: Option<String>,

    /// The text content, used for `input_text` and `text` content types.
    pub r#text: Option<String>,

    /// ID of a previous conversation item to reference (for `item_reference`
    /// content types in `response.create` events).
    pub r#id: Option<String>,

    /// Base64-encoded audio bytes, used for `input_audio` content type.
    pub r#audio: Option<String>,

    /// The transcript of the audio, used for `input_audio` content type.
    pub r#transcript: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItem_Content {
    /// The content type (`input_text`, `input_audio`, `item_reference`,
    /// `text`).
    pub r#type: Option<String>,

    /// The text content, used for `input_text` and `text` content types.
    pub r#text: Option<String>,

    /// ID of a previous conversation item to reference (for `item_reference`
    /// content types in `response.create` events).
    pub r#id: Option<String>,

    /// Base64-encoded audio bytes, used for `input_audio` content type.
    pub r#audio: Option<String>,

    /// The transcript of the audio, used for `input_audio` content type.
    pub r#transcript: Option<String>,
}

/// The response resource.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse {
    /// The unique ID of the response.
    pub r#id: Option<String>,

    /// The object type, must be `realtime.response`.
    pub r#object: Option<String>,

    /// The final status of the response (`completed`, `cancelled`, `failed`, or
    /// `incomplete`).
    pub r#status: Option<String>,

    pub r#status_details: Option<RealtimeResponse_StatusDetails>,

    /// The list of output items generated by the response.
    pub r#output: Option<Vec<RealtimeConversationItem>>,

    pub r#metadata: Option<Metadata>,

    pub r#usage: Option<RealtimeResponse_Usage>,

    /// Which conversation the response is added to, determined by the
    /// `conversation` field in the `response.create` event.
    pub r#conversation_id: Option<String>,

    /// The voice the model used to respond.
    pub r#voice: Option<VoiceIdsShared>,

    /// The set of modalities the model used to respond.
    pub r#modalities: Option<Vec<String>>,

    /// The format of output audio.
    pub r#output_audio_format: Option<String>,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: Option<f64>,

    pub r#max_output_tokens: Option<RealtimeResponse_MaxOutputTokens>,
}

/// Create a new Realtime response with these parameters
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponseCreateParams {
    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The default system instructions (i.e.
    pub r#instructions: Option<String>,

    /// The voice the model uses to respond.
    pub r#voice: Option<VoiceIdsShared>,

    /// The format of output audio.
    pub r#output_audio_format: Option<String>,

    /// Tools (functions) available to the model.
    pub r#tools: Option<Vec<RealtimeResponseCreateParams_Tools>>,

    /// How the model chooses tools.
    pub r#tool_choice: Option<String>,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: Option<f64>,

    pub r#max_response_output_tokens: Option<RealtimeResponseCreateParams_MaxResponseOutputTokens>,

    pub r#conversation: Option<RealtimeResponseCreateParams_Conversation>,

    pub r#metadata: Option<Metadata>,

    /// Input items to include in the prompt for the model.
    pub r#input: Option<Vec<RealtimeConversationItemWithReference>>,
}

/// Controls which conversation the response is added to.
pub type RealtimeResponseCreateParams_Conversation = String;

/// Maximum number of output tokens for a single assistant response, inclusive
/// of tool calls.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeResponseCreateParams_MaxResponseOutputTokens {
    Integer(i64),

    Text(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponseCreateParams_Tools {
    /// The type of the tool, i.e.
    pub r#type: Option<String>,

    /// The name of the function.
    pub r#name: Option<String>,

    /// The description of the function, including guidance on when and how to
    /// call it, and guidance about what to tell the user when calling (if
    /// anything).
    pub r#description: Option<String>,

    /// Parameters of the function in JSON Schema.
    pub r#parameters: Option<RealtimeResponseCreateParams_Tools_Parameters>,
}

/// Parameters of the function in JSON Schema.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponseCreateParams_Tools_Parameters;

/// Maximum number of output tokens for a single assistant response, inclusive
/// of tool calls, that was used in this response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeResponse_MaxOutputTokens {
    Integer(i64),

    Text(String),
}

/// Additional details about the status.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse_StatusDetails {
    /// The type of error that caused the response to fail, corresponding with
    /// the `status` field (`completed`, `cancelled`, `incomplete`, `failed`).
    pub r#type: Option<String>,

    /// The reason the Response did not complete.
    pub r#reason: Option<String>,

    pub r#error: Option<RealtimeResponse_StatusDetails_Error>,
}

/// A description of the error that caused the response to fail, populated when
/// the `status` is `failed`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse_StatusDetails_Error {
    /// The type of error.
    pub r#type: Option<String>,

    /// Error code, if any.
    pub r#code: Option<String>,
}

/// Usage statistics for the Response, this will correspond to billing.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse_Usage {
    /// The total number of tokens in the Response including input and output
    /// text and audio tokens.
    pub r#total_tokens: Option<i64>,

    /// The number of input tokens used in the Response, including text and
    /// audio tokens.
    pub r#input_tokens: Option<i64>,

    /// The number of output tokens sent in the Response, including text and
    /// audio tokens.
    pub r#output_tokens: Option<i64>,

    pub r#input_token_details: Option<RealtimeResponse_Usage_InputTokenDetails>,

    pub r#output_token_details: Option<RealtimeResponse_Usage_OutputTokenDetails>,
}

/// Details about the input tokens used in the Response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse_Usage_InputTokenDetails {
    /// The number of cached tokens used in the Response.
    pub r#cached_tokens: Option<i64>,

    /// The number of text tokens used in the Response.
    pub r#text_tokens: Option<i64>,

    /// The number of audio tokens used in the Response.
    pub r#audio_tokens: Option<i64>,
}

/// Details about the output tokens used in the Response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse_Usage_OutputTokenDetails {
    /// The number of text tokens used in the Response.
    pub r#text_tokens: Option<i64>,

    /// The number of audio tokens used in the Response.
    pub r#audio_tokens: Option<i64>,
}

/// A realtime server event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeServerEvent {
    RealtimeServerEventConversationCreated(RealtimeServerEventConversationCreated),

    RealtimeServerEventConversationItemCreated(RealtimeServerEventConversationItemCreated),

    RealtimeServerEventConversationItemDeleted(RealtimeServerEventConversationItemDeleted),

    RealtimeServerEventConversationItemInputAudioTranscriptionCompleted(RealtimeServerEventConversationItemInputAudioTranscriptionCompleted),

    RealtimeServerEventConversationItemInputAudioTranscriptionDelta(RealtimeServerEventConversationItemInputAudioTranscriptionDelta),

    RealtimeServerEventConversationItemInputAudioTranscriptionFailed(RealtimeServerEventConversationItemInputAudioTranscriptionFailed),

    RealtimeServerEventConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),

    RealtimeServerEventConversationItemTruncated(RealtimeServerEventConversationItemTruncated),

    RealtimeServerEventError(RealtimeServerEventError),

    RealtimeServerEventInputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),

    RealtimeServerEventInputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),

    RealtimeServerEventInputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),

    RealtimeServerEventInputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),

    RealtimeServerEventRateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),

    RealtimeServerEventResponseAudioDelta(RealtimeServerEventResponseAudioDelta),

    RealtimeServerEventResponseAudioDone(RealtimeServerEventResponseAudioDone),

    RealtimeServerEventResponseAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),

    RealtimeServerEventResponseAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),

    RealtimeServerEventResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),

    RealtimeServerEventResponseContentPartDone(RealtimeServerEventResponseContentPartDone),

    RealtimeServerEventResponseCreated(RealtimeServerEventResponseCreated),

    RealtimeServerEventResponseDone(RealtimeServerEventResponseDone),

    RealtimeServerEventResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),

    RealtimeServerEventResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),

    RealtimeServerEventResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),

    RealtimeServerEventResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),

    RealtimeServerEventResponseTextDelta(RealtimeServerEventResponseTextDelta),

    RealtimeServerEventResponseTextDone(RealtimeServerEventResponseTextDone),

    RealtimeServerEventSessionCreated(RealtimeServerEventSessionCreated),

    RealtimeServerEventSessionUpdated(RealtimeServerEventSessionUpdated),

    RealtimeServerEventTranscriptionSessionUpdated(RealtimeServerEventTranscriptionSessionUpdated),

    RealtimeServerEventOutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),

    RealtimeServerEventOutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),

    RealtimeServerEventOutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),
}

/// Returned when a conversation is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.created`.
    pub r#type: String,

    pub r#conversation: RealtimeServerEventConversationCreated_Conversation,
}

/// The conversation resource.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationCreated_Conversation {
    /// The unique ID of the conversation.
    pub r#id: Option<String>,

    /// The object type, must be `realtime.conversation`.
    pub r#object: Option<String>,
}

/// Returned when a conversation item is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.created`.
    pub r#type: String,

    /// The ID of the preceding item in the Conversation context, allows the
    /// client to understand the order of the conversation.
    pub r#previous_item_id: String,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an item in the conversation is deleted by the client with a
/// `conversation.item.delete` event.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.deleted`.
    pub r#type: String,

    /// The ID of the item that was deleted.
    pub r#item_id: String,
}

/// This event is the output of audio transcription for user audio written to
/// the user audio buffer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.completed`.
    pub r#type: String,

    /// The ID of the user message item containing the audio.
    pub r#item_id: String,

    /// The index of the content part containing the audio.
    pub r#content_index: i64,

    /// The transcribed text.
    pub r#transcript: String,

    /// The log probabilities of the transcription.
    pub r#logprobs: Option<Vec<LogProbProperties>>,
}

/// Returned when the text value of an input audio transcription content part is
/// updated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.delta`.
    pub r#type: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the content part in the item's content array.
    pub r#content_index: Option<i64>,

    /// The text delta.
    pub r#delta: Option<String>,

    /// The log probabilities of the transcription.
    pub r#logprobs: Option<Vec<LogProbProperties>>,
}

/// Returned when input audio transcription is configured, and a transcription
/// request for a user message failed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.failed`.
    pub r#type: String,

    /// The ID of the user message item.
    pub r#item_id: String,

    /// The index of the content part containing the audio.
    pub r#content_index: i64,

    pub r#error: RealtimeServerEventConversationItemInputAudioTranscriptionFailed_Error,
}

/// Details of the transcription error.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed_Error {
    /// The type of error.
    pub r#type: Option<String>,

    /// Error code, if any.
    pub r#code: Option<String>,

    /// A human-readable error message.
    pub r#message: Option<String>,

    /// Parameter related to the error, if any.
    pub r#param: Option<String>,
}

/// Returned when a conversation item is retrieved with
/// `conversation.item.retrieve`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemRetrieved {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.retrieved`.
    pub r#type: String,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an earlier assistant audio message item is truncated by the
/// client with a `conversation.item.truncate` event.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.truncated`.
    pub r#type: String,

    /// The ID of the assistant message item that was truncated.
    pub r#item_id: String,

    /// The index of the content part that was truncated.
    pub r#content_index: i64,

    /// The duration up to which the audio was truncated, in milliseconds.
    pub r#audio_end_ms: i64,
}

/// Returned when an error occurs, which could be a client problem or a server
/// problem.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventError {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `error`.
    pub r#type: String,

    pub r#error: RealtimeServerEventError_Error,
}

/// Details of the error.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventError_Error {
    /// The type of error (e.g., "invalid_request_error", "server_error").
    pub r#type: String,

    /// Error code, if any.
    pub r#code: Option<String>,

    /// A human-readable error message.
    pub r#message: String,

    /// Parameter related to the error, if any.
    pub r#param: Option<String>,

    /// The event_id of the client event that caused the error, if applicable.
    pub r#event_id: Option<String>,
}

/// Returned when the input audio buffer is cleared by the client with a
/// `input_audio_buffer.clear` event.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferCleared {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.cleared`.
    pub r#type: String,
}

/// Returned when an input audio buffer is committed, either by the client or
/// automatically in server VAD mode.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.committed`.
    pub r#type: String,

    /// The ID of the preceding item after which the new item will be inserted.
    pub r#previous_item_id: String,

    /// The ID of the user message item that will be created.
    pub r#item_id: String,
}

/// Sent by the server when in `server_vad` mode to indicate that speech has
/// been detected in the audio buffer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.speech_started`.
    pub r#type: String,

    /// Milliseconds from the start of all audio written to the buffer during
    /// the session when speech was first detected.
    pub r#audio_start_ms: i64,

    /// The ID of the user message item that will be created when speech stops.
    pub r#item_id: String,
}

/// Returned in `server_vad` mode when the server detects the end of speech in
/// the audio buffer.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.speech_stopped`.
    pub r#type: String,

    /// Milliseconds since the session started when speech stopped.
    pub r#audio_end_ms: i64,

    /// The ID of the user message item that will be created.
    pub r#item_id: String,
}

/// **WebRTC Only:** Emitted when the output audio buffer is cleared.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.cleared`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// **WebRTC Only:** Emitted when the server begins streaming audio to the
/// client.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.started`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// **WebRTC Only:** Emitted when the output audio buffer has been completely
/// drained on the server, and no more audio is forthcoming.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.stopped`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// Emitted at the beginning of a Response to indicate the updated rate limits.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `rate_limits.updated`.
    pub r#type: String,

    /// List of rate limit information.
    pub r#rate_limits: Vec<RealtimeServerEventRateLimitsUpdated_RateLimits>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventRateLimitsUpdated_RateLimits {
    /// The name of the rate limit (`requests`, `tokens`).
    pub r#name: Option<String>,

    /// The maximum allowed value for the rate limit.
    pub r#limit: Option<i64>,

    /// The remaining value before the limit is reached.
    pub r#remaining: Option<i64>,

    /// Seconds until the rate limit resets.
    pub r#reset_seconds: Option<f64>,
}

/// Returned when the model-generated audio is updated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// Base64-encoded audio data delta.
    pub r#delta: String,
}

/// Returned when the model-generated audio is done.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,
}

/// Returned when the model-generated transcription of audio output is updated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio_transcript.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The transcript delta.
    pub r#delta: String,
}

/// Returned when the model-generated transcription of audio output is done
/// streaming.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio_transcript.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The final transcript of the audio.
    pub r#transcript: String,
}

/// Returned when a new content part is added to an assistant message item
/// during response generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.content_part.added`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item to which the content part was added.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    pub r#part: RealtimeServerEventResponseContentPartAdded_Part,
}

/// The content part that was added.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartAdded_Part {
    /// The content type ("text", "audio").
    pub r#type: Option<String>,

    /// The text content (if type is "text").
    pub r#text: Option<String>,

    /// Base64-encoded audio data (if type is "audio").
    pub r#audio: Option<String>,

    /// The transcript of the audio (if type is "audio").
    pub r#transcript: Option<String>,
}

/// Returned when a content part is done streaming in an assistant message item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.content_part.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    pub r#part: RealtimeServerEventResponseContentPartDone_Part,
}

/// The content part that is done.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartDone_Part {
    /// The content type ("text", "audio").
    pub r#type: Option<String>,

    /// The text content (if type is "text").
    pub r#text: Option<String>,

    /// Base64-encoded audio data (if type is "audio").
    pub r#audio: Option<String>,

    /// The transcript of the audio (if type is "audio").
    pub r#transcript: Option<String>,
}

/// Returned when a new Response is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.created`.
    pub r#type: String,

    pub r#response: RealtimeResponse,
}

/// Returned when a Response is done streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.done`.
    pub r#type: String,

    pub r#response: RealtimeResponse,
}

/// Returned when the model-generated function call arguments are updated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.function_call_arguments.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the function call item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The ID of the function call.
    pub r#call_id: String,

    /// The arguments delta as a JSON string.
    pub r#delta: String,
}

/// Returned when the model-generated function call arguments are done
/// streaming.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.function_call_arguments.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the function call item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The ID of the function call.
    pub r#call_id: String,

    /// The final arguments as a JSON string.
    pub r#arguments: String,
}

/// Returned when a new Item is created during Response generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.output_item.added`.
    pub r#type: String,

    /// The ID of the Response to which the item belongs.
    pub r#response_id: String,

    /// The index of the output item in the Response.
    pub r#output_index: i64,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an Item is done streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseOutputItemDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.output_item.done`.
    pub r#type: String,

    /// The ID of the Response to which the item belongs.
    pub r#response_id: String,

    /// The index of the output item in the Response.
    pub r#output_index: i64,

    pub r#item: RealtimeConversationItem,
}

/// Returned when the text value of a "text" content part is updated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseTextDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.text.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The text delta.
    pub r#delta: String,
}

/// Returned when the text value of a "text" content part is done streaming.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseTextDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.text.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The final text content.
    pub r#text: String,
}

/// Returned when a Session is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `session.created`.
    pub r#type: String,

    pub r#session: RealtimeSession,
}

/// Returned when a session is updated with a `session.update` event, unless
/// there is an error.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `session.updated`.
    pub r#type: String,

    pub r#session: RealtimeSession,
}

/// Returned when a transcription session is updated with a
/// `transcription_session.update` event, unless there is an error.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `transcription_session.updated`.
    pub r#type: String,

    pub r#session: RealtimeTranscriptionSessionCreateResponse,
}

/// Realtime session object configuration.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession {
    /// Unique identifier for the session that looks like
    /// `sess_1234567890abcdef`.
    pub r#id: Option<String>,

    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The Realtime model used for this session.
    pub r#model: Option<String>,

    /// The default system instructions (i.e.
    pub r#instructions: Option<String>,

    /// The voice the model uses to respond.
    pub r#voice: Option<VoiceIdsShared>,

    /// The format of input audio.
    pub r#input_audio_format: Option<String>,

    /// The format of output audio.
    pub r#output_audio_format: Option<String>,

    pub r#input_audio_transcription: Option<RealtimeSession_InputAudioTranscription>,

    pub r#turn_detection: Option<RealtimeSession_TurnDetection>,

    pub r#input_audio_noise_reduction: Option<RealtimeSession_InputAudioNoiseReduction>,

    /// Tools (functions) available to the model.
    pub r#tools: Option<Vec<RealtimeSession_Tools>>,

    /// How the model chooses tools.
    pub r#tool_choice: Option<String>,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: Option<f64>,

    pub r#max_response_output_tokens: Option<RealtimeSession_MaxResponseOutputTokens>,
}

/// Realtime session object configuration.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest {
    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The Realtime model used for this session.
    pub r#model: Option<String>,

    /// The default system instructions (i.e.
    pub r#instructions: Option<String>,

    /// The voice the model uses to respond.
    pub r#voice: Option<VoiceIdsShared>,

    /// The format of input audio.
    pub r#input_audio_format: Option<String>,

    /// The format of output audio.
    pub r#output_audio_format: Option<String>,

    pub r#input_audio_transcription: Option<RealtimeSessionCreateRequest_InputAudioTranscription>,

    pub r#turn_detection: Option<RealtimeSessionCreateRequest_TurnDetection>,

    pub r#input_audio_noise_reduction: Option<RealtimeSessionCreateRequest_InputAudioNoiseReduction>,

    /// Tools (functions) available to the model.
    pub r#tools: Option<Vec<RealtimeSessionCreateRequest_Tools>>,

    /// How the model chooses tools.
    pub r#tool_choice: Option<String>,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: Option<f64>,

    pub r#max_response_output_tokens: Option<RealtimeSessionCreateRequest_MaxResponseOutputTokens>,
}

/// Configuration for input audio noise reduction.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest_InputAudioNoiseReduction {
    /// Type of noise reduction.
    pub r#type: Option<String>,
}

/// Configuration for input audio transcription, defaults to off and can be  set
/// to `null` to turn off once on.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest_InputAudioTranscription {
    /// The model to use for transcription, current options are
    /// `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
    pub r#model: Option<String>,

    /// The language of the input audio.
    pub r#language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,
}

/// Maximum number of output tokens for a single assistant response, inclusive
/// of tool calls.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeSessionCreateRequest_MaxResponseOutputTokens {
    Integer(i64),

    Text(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest_Tools {
    /// The type of the tool, i.e.
    pub r#type: Option<String>,

    /// The name of the function.
    pub r#name: Option<String>,

    /// The description of the function, including guidance on when and how to
    /// call it, and guidance about what to tell the user when calling (if
    /// anything).
    pub r#description: Option<String>,

    /// Parameters of the function in JSON Schema.
    pub r#parameters: Option<RealtimeSessionCreateRequest_Tools_Parameters>,
}

/// Parameters of the function in JSON Schema.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest_Tools_Parameters;

/// Configuration for turn detection, ether Server VAD or Semantic VAD.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest_TurnDetection {
    /// Type of turn detection.
    pub r#type: Option<String>,

    /// Used only for `semantic_vad` mode.
    pub r#eagerness: Option<String>,

    /// Used only for `server_vad` mode.
    pub r#threshold: Option<f64>,

    /// Used only for `server_vad` mode.
    pub r#prefix_padding_ms: Option<i64>,

    /// Used only for `server_vad` mode.
    pub r#silence_duration_ms: Option<i64>,

    /// Whether or not to automatically generate a response when a VAD stop
    /// event occurs.
    pub r#create_response: Option<bool>,

    /// Whether or not to automatically interrupt any ongoing response with
    /// output to the default conversation (i.e.
    pub r#interrupt_response: Option<bool>,
}

/// A new Realtime session configuration, with an ephermeral key.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse {
    pub r#client_secret: RealtimeSessionCreateResponse_ClientSecret,

    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The default system instructions (i.e.
    pub r#instructions: Option<String>,

    /// The voice the model uses to respond.
    pub r#voice: Option<VoiceIdsShared>,

    /// The format of input audio.
    pub r#input_audio_format: Option<String>,

    /// The format of output audio.
    pub r#output_audio_format: Option<String>,

    pub r#input_audio_transcription: Option<RealtimeSessionCreateResponse_InputAudioTranscription>,

    pub r#turn_detection: Option<RealtimeSessionCreateResponse_TurnDetection>,

    /// Tools (functions) available to the model.
    pub r#tools: Option<Vec<RealtimeSessionCreateResponse_Tools>>,

    /// How the model chooses tools.
    pub r#tool_choice: Option<String>,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: Option<f64>,

    pub r#max_response_output_tokens: Option<RealtimeSessionCreateResponse_MaxResponseOutputTokens>,
}

/// Ephemeral key returned by the API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse_ClientSecret {
    /// Ephemeral key usable in client environments to authenticate connections
    /// to the Realtime API.
    pub r#value: String,

    /// Timestamp for when the token expires.
    pub r#expires_at: i64,
}

/// Configuration for input audio transcription, defaults to off and can be set
/// to `null` to turn off once on.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse_InputAudioTranscription {
    /// The model to use for transcription, `whisper-1` is the only currently
    /// supported model.
    pub r#model: Option<String>,
}

/// Maximum number of output tokens for a single assistant response, inclusive
/// of tool calls.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeSessionCreateResponse_MaxResponseOutputTokens {
    Integer(i64),

    Text(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse_Tools {
    /// The type of the tool, i.e.
    pub r#type: Option<String>,

    /// The name of the function.
    pub r#name: Option<String>,

    /// The description of the function, including guidance on when and how to
    /// call it, and guidance about what to tell the user when calling (if
    /// anything).
    pub r#description: Option<String>,

    /// Parameters of the function in JSON Schema.
    pub r#parameters: Option<RealtimeSessionCreateResponse_Tools_Parameters>,
}

/// Parameters of the function in JSON Schema.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse_Tools_Parameters;

/// Configuration for turn detection.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse_TurnDetection {
    /// Type of turn detection, only `server_vad` is currently supported.
    pub r#type: Option<String>,

    /// Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5.
    pub r#threshold: Option<f64>,

    /// Amount of audio to include before the VAD detected speech (in
    /// milliseconds).
    pub r#prefix_padding_ms: Option<i64>,

    /// Duration of silence to detect speech stop (in milliseconds).
    pub r#silence_duration_ms: Option<i64>,
}

/// Configuration for input audio noise reduction.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession_InputAudioNoiseReduction {
    /// Type of noise reduction.
    pub r#type: Option<String>,
}

/// Configuration for input audio transcription, defaults to off and can be  set
/// to `null` to turn off once on.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession_InputAudioTranscription {
    /// The model to use for transcription, current options are
    /// `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
    pub r#model: Option<String>,

    /// The language of the input audio.
    pub r#language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,
}

/// Maximum number of output tokens for a single assistant response, inclusive
/// of tool calls.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RealtimeSession_MaxResponseOutputTokens {
    Integer(i64),

    Text(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession_Tools {
    /// The type of the tool, i.e.
    pub r#type: Option<String>,

    /// The name of the function.
    pub r#name: Option<String>,

    /// The description of the function, including guidance on when and how to
    /// call it, and guidance about what to tell the user when calling (if
    /// anything).
    pub r#description: Option<String>,

    /// Parameters of the function in JSON Schema.
    pub r#parameters: Option<RealtimeSession_Tools_Parameters>,
}

/// Parameters of the function in JSON Schema.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession_Tools_Parameters;

/// Configuration for turn detection, ether Server VAD or Semantic VAD.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession_TurnDetection {
    /// Type of turn detection.
    pub r#type: Option<String>,

    /// Used only for `semantic_vad` mode.
    pub r#eagerness: Option<String>,

    /// Used only for `server_vad` mode.
    pub r#threshold: Option<f64>,

    /// Used only for `server_vad` mode.
    pub r#prefix_padding_ms: Option<i64>,

    /// Used only for `server_vad` mode.
    pub r#silence_duration_ms: Option<i64>,

    /// Whether or not to automatically generate a response when a VAD stop
    /// event occurs.
    pub r#create_response: Option<bool>,

    /// Whether or not to automatically interrupt any ongoing response with
    /// output to the default conversation (i.e.
    pub r#interrupt_response: Option<bool>,
}

/// Realtime transcription session object configuration.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateRequest {
    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The format of input audio.
    pub r#input_audio_format: Option<String>,

    pub r#input_audio_transcription: Option<RealtimeTranscriptionSessionCreateRequest_InputAudioTranscription>,

    pub r#turn_detection: Option<RealtimeTranscriptionSessionCreateRequest_TurnDetection>,

    pub r#input_audio_noise_reduction: Option<RealtimeTranscriptionSessionCreateRequest_InputAudioNoiseReduction>,

    /// The set of items to include in the transcription.
    pub r#include: Option<Vec<String>>,
}

/// Configuration for input audio noise reduction.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateRequest_InputAudioNoiseReduction {
    /// Type of noise reduction.
    pub r#type: Option<String>,
}

/// Configuration for input audio transcription.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateRequest_InputAudioTranscription {
    /// The model to use for transcription, current options are
    /// `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
    pub r#model: Option<String>,

    /// The language of the input audio.
    pub r#language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,
}

/// Configuration for turn detection, ether Server VAD or Semantic VAD.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateRequest_TurnDetection {
    /// Type of turn detection.
    pub r#type: Option<String>,

    /// Used only for `semantic_vad` mode.
    pub r#eagerness: Option<String>,

    /// Used only for `server_vad` mode.
    pub r#threshold: Option<f64>,

    /// Used only for `server_vad` mode.
    pub r#prefix_padding_ms: Option<i64>,

    /// Used only for `server_vad` mode.
    pub r#silence_duration_ms: Option<i64>,

    /// Whether or not to automatically generate a response when a VAD stop
    /// event occurs.
    pub r#create_response: Option<bool>,

    /// Whether or not to automatically interrupt any ongoing response with
    /// output to the default conversation (i.e.
    pub r#interrupt_response: Option<bool>,
}

/// A new Realtime transcription session configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateResponse {
    pub r#client_secret: RealtimeTranscriptionSessionCreateResponse_ClientSecret,

    /// The set of modalities the model can respond with.
    pub r#modalities: Option<Vec<String>>,

    /// The format of input audio.
    pub r#input_audio_format: Option<String>,

    pub r#input_audio_transcription: Option<RealtimeTranscriptionSessionCreateResponse_InputAudioTranscription>,

    pub r#turn_detection: Option<RealtimeTranscriptionSessionCreateResponse_TurnDetection>,
}

/// Ephemeral key returned by the API.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateResponse_ClientSecret {
    /// Ephemeral key usable in client environments to authenticate connections
    /// to the Realtime API.
    pub r#value: String,

    /// Timestamp for when the token expires.
    pub r#expires_at: i64,
}

/// Configuration of the transcription model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateResponse_InputAudioTranscription {
    /// The model to use for transcription.
    pub r#model: Option<String>,

    /// The language of the input audio.
    pub r#language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: Option<String>,
}

/// Configuration for turn detection.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateResponse_TurnDetection {
    /// Type of turn detection, only `server_vad` is currently supported.
    pub r#type: Option<String>,

    /// Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5.
    pub r#threshold: Option<f64>,

    /// Amount of audio to include before the VAD detected speech (in
    /// milliseconds).
    pub r#prefix_padding_ms: Option<i64>,

    /// Duration of silence to detect speech stop (in milliseconds).
    pub r#silence_duration_ms: Option<i64>,
}

/// **o-series models only** Configuration options for [reasoning
/// models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reasoning {
    pub r#effort: Option<ReasoningEffort>,

    /// A summary of the reasoning performed by the model.
    pub r#summary: Option<String>,

    /// **Deprecated:** use `summary` instead.
    pub r#generate_summary: Option<String>,
}

/// **o-series models only** Constrains effort on reasoning for [reasoning
/// models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReasoningEffort {
    #[cfg_attr(feature = "serde", serde(rename = "low"))]
    Low,

    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    Medium,

    #[cfg_attr(feature = "serde", serde(rename = "high"))]
    High,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// A description of the chain of thought used by a reasoning model while
/// generating a response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReasoningItem {
    /// The type of the object.
    pub r#type: String,

    /// The unique identifier of the reasoning content.
    pub r#id: String,

    /// Reasoning text contents.
    pub r#summary: Vec<ReasoningItem_Summary>,

    /// The status of the item.
    pub r#status: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReasoningItem_Summary {
    /// The type of the object.
    pub r#type: String,

    /// A short summary of the reasoning used by the model when generating the
    /// response.
    pub r#text: String,
}

/// A refusal from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefusalContent {
    /// The type of the refusal.
    pub r#type: String,

    /// The refusal explanationfrom the model.
    pub r#refusal: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub r#metadata: Option<Metadata>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: Option<String>,

    pub r#service_tier: Option<ServiceTier>,

    /// The unique ID of the previous response to the model.
    pub r#previous_response_id: Option<String>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: ModelIdsResponses,

    pub r#reasoning: Option<Reasoning>,

    /// An upper bound for the number of tokens that can be generated for a
    /// response, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_output_tokens: Option<i64>,

    /// Inserts a system (or developer) message as the first item in the model's
    /// context.
    pub r#instructions: Option<String>,

    pub r#text: Option<ResponseProperties_Text>,

    /// An array of tools the model may call while generating a response.
    pub r#tools: Vec<Tool>,

    pub r#tool_choice: ResponseProperties_ToolChoice,

    /// The truncation strategy to use for the model response.
    pub r#truncation: Option<String>,

    /// Unique identifier for this Response.
    pub r#id: String,

    /// The object type of this resource - always set to `response`.
    pub r#object: String,

    /// The status of the response generation.
    pub r#status: Option<String>,

    /// Unix timestamp (in seconds) of when this Response was created.
    pub r#created_at: f64,

    pub r#error: Option<ResponseError>,

    pub r#incomplete_details: Option<Response_Variant3_IncompleteDetails>,

    /// An array of content items generated by the model.
    pub r#output: Vec<OutputItem>,

    /// SDK-only convenience property that contains the aggregated text output
    /// from all `output_text` items in the `output` array, if any are present.
    pub r#output_text: Option<String>,

    pub r#usage: Option<ResponseUsage>,

    /// Whether to allow the model to run tool calls in parallel.
    pub r#parallel_tool_calls: bool,
}

/// Emitted when there is a partial audio response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// A chunk of Base64 encoded response audio bytes.
    pub r#delta: String,
}

/// Emitted when the audio response is complete.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioDoneEvent {
    /// The type of the event.
    pub r#type: String,
}

/// Emitted when there is a partial transcript of audio.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioTranscriptDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The partial transcript of the audio response.
    pub r#delta: String,
}

/// Emitted when the full audio transcript is completed.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioTranscriptDoneEvent {
    /// The type of the event.
    pub r#type: String,
}

/// Emitted when a partial code snippet is added by the code interpreter.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    /// The partial code snippet added by the code interpreter.
    pub r#delta: String,
}

/// Emitted when code snippet output is finalized by the code interpreter.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    /// The final code snippet output by the code interpreter.
    pub r#code: String,
}

/// Emitted when the code interpreter call is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when a code interpreter call is in progress.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when the code interpreter is actively interpreting the code snippet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when the model response is complete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// Properties of the completed response.
    pub r#response: Response,
}

/// Emitted when a new content part is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseContentPartAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the content part was added to.
    pub r#item_id: String,

    /// The index of the output item that the content part was added to.
    pub r#output_index: i64,

    /// The index of the content part that was added.
    pub r#content_index: i64,

    /// The content part that was added.
    pub r#part: OutputContent,
}

/// Emitted when a content part is done.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseContentPartDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the content part was added to.
    pub r#item_id: String,

    /// The index of the output item that the content part was added to.
    pub r#output_index: i64,

    /// The index of the content part that is done.
    pub r#content_index: i64,

    /// The content part that is done.
    pub r#part: OutputContent,
}

/// An event that is emitted when a response is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCreatedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that was created.
    pub r#response: Response,
}

/// An error object returned when the model fails to generate a Response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseError {
    pub r#code: ResponseErrorCode,

    /// A human-readable description of the error.
    pub r#message: String,
}

/// The error code for the response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResponseErrorCode {
    #[cfg_attr(feature = "serde", serde(rename = "server_error"))]
    ServerError,

    #[cfg_attr(feature = "serde", serde(rename = "rate_limit_exceeded"))]
    RateLimitExceeded,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_prompt"))]
    InvalidPrompt,

    #[cfg_attr(feature = "serde", serde(rename = "vector_store_timeout"))]
    VectorStoreTimeout,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_image"))]
    InvalidImage,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_image_format"))]
    InvalidImageFormat,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_base64_image"))]
    InvalidBase64Image,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_image_url"))]
    InvalidImageUrl,

    #[cfg_attr(feature = "serde", serde(rename = "image_too_large"))]
    ImageTooLarge,

    #[cfg_attr(feature = "serde", serde(rename = "image_too_small"))]
    ImageTooSmall,

    #[cfg_attr(feature = "serde", serde(rename = "image_parse_error"))]
    ImageParseError,

    #[cfg_attr(feature = "serde", serde(rename = "image_content_policy_violation"))]
    ImageContentPolicyViolation,

    #[cfg_attr(feature = "serde", serde(rename = "invalid_image_mode"))]
    InvalidImageMode,

    #[cfg_attr(feature = "serde", serde(rename = "image_file_too_large"))]
    ImageFileTooLarge,

    #[cfg_attr(feature = "serde", serde(rename = "unsupported_image_media_type"))]
    UnsupportedImageMediaType,

    #[cfg_attr(feature = "serde", serde(rename = "empty_image_file"))]
    EmptyImageFile,

    #[cfg_attr(feature = "serde", serde(rename = "failed_to_download_image"))]
    FailedToDownloadImage,

    #[cfg_attr(feature = "serde", serde(rename = "image_file_not_found"))]
    ImageFileNotFound,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// Emitted when an error occurs.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseErrorEvent {
    /// The type of the event.
    pub r#type: String,

    /// The error code.
    pub r#code: Option<String>,

    /// The error message.
    pub r#message: String,

    /// The error parameter.
    pub r#param: Option<String>,
}

/// An event that is emitted when a response fails.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFailedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that failed.
    pub r#response: Response,
}

/// Emitted when a file search call is completed (results found).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is initiated.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// Emitted when a file search call is initiated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is initiated.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// Emitted when a file search is currently searching.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallSearchingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is searching.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// JSON object response format.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonObject {
    /// The type of response format being defined.
    pub r#type: String,
}

/// JSON Schema response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonSchema {
    /// The type of response format being defined.
    pub r#type: String,

    pub r#json_schema: ResponseFormatJsonSchema_JsonSchema,
}

/// The schema for the response format, described as a JSON Schema object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonSchemaSchema;

/// Structured Outputs configuration options, including a JSON Schema.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonSchema_JsonSchema {
    /// A description of what the response format is for, used by the model to
    /// determine how to respond in the format.
    pub r#description: Option<String>,

    /// The name of the response format.
    pub r#name: String,

    pub r#schema: Option<ResponseFormatJsonSchemaSchema>,

    /// Whether to enable strict schema adherence when generating the output.
    pub r#strict: Option<bool>,
}

/// Default response format.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatText {
    /// The type of response format being defined.
    pub r#type: String,
}

/// Emitted when there is a partial function-call arguments delta.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the function-call arguments delta is
    /// added to.
    pub r#item_id: String,

    /// The index of the output item that the function-call arguments delta is
    /// added to.
    pub r#output_index: i64,

    /// The function-call arguments delta that is added.
    pub r#delta: String,
}

/// Emitted when function-call arguments are finalized.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    pub r#type: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item.
    pub r#output_index: i64,

    /// The function-call arguments.
    pub r#arguments: String,
}

/// Emitted when the response is in progress.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that is in progress.
    pub r#response: Response,
}

/// An event that is emitted when a response finishes as incomplete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseIncompleteEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that was incomplete.
    pub r#response: Response,
}

/// A list of Response items.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseItemList {
    /// The type of object returned, must be `list`.
    pub r#object: String,

    /// A list of items used to generate this response.
    pub r#data: Vec<ItemResource>,

    /// Whether there are more items available.
    pub r#has_more: bool,

    /// The ID of the first item in the list.
    pub r#first_id: String,

    /// The ID of the last item in the list.
    pub r#last_id: String,
}

/// Output types that you would like the model to generate.
pub type ResponseModalities = Vec<String>;

/// Emitted when a new output item is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseOutputItemAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that was added.
    pub r#output_index: i64,

    /// The output item that was added.
    pub r#item: OutputItem,
}

/// Emitted when an output item is marked done.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseOutputItemDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that was marked done.
    pub r#output_index: i64,

    /// The output item that was marked done.
    pub r#item: OutputItem,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseProperties {
    /// The unique ID of the previous response to the model.
    pub r#previous_response_id: Option<String>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: Option<ModelIdsResponses>,

    pub r#reasoning: Option<Reasoning>,

    /// An upper bound for the number of tokens that can be generated for a
    /// response, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_output_tokens: Option<i64>,

    /// Inserts a system (or developer) message as the first item in the model's
    /// context.
    pub r#instructions: Option<String>,

    pub r#text: Option<ResponseProperties_Text>,

    /// An array of tools the model may call while generating a response.
    pub r#tools: Option<Vec<Tool>>,

    pub r#tool_choice: Option<ResponseProperties_ToolChoice>,

    /// The truncation strategy to use for the model response.
    pub r#truncation: Option<String>,
}

/// Configuration options for a text response from the model.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseProperties_Text {
    pub r#format: Option<TextResponseFormatConfiguration>,
}

/// How the model should select which tool (or tools) to use when generating a
/// response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ResponseProperties_ToolChoice {
    ToolChoiceOptions(ToolChoiceOptions),

    ToolChoiceTypes(ToolChoiceTypes),

    ToolChoiceFunction(ToolChoiceFunction),
}

/// Emitted when a new reasoning summary part is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary part is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary part is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    pub r#part: ResponseReasoningSummaryPartAddedEvent_Part,
}

/// The summary part that was added.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartAddedEvent_Part {
    /// The type of the summary part.
    pub r#type: String,

    /// The text of the summary part.
    pub r#text: String,
}

/// Emitted when a reasoning summary part is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary part is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary part is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    pub r#part: ResponseReasoningSummaryPartDoneEvent_Part,
}

/// The completed summary part.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartDoneEvent_Part {
    /// The type of the summary part.
    pub r#type: String,

    /// The text of the summary part.
    pub r#text: String,
}

/// Emitted when a delta is added to a reasoning summary text.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary text delta is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary text delta is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The text delta that was added to the summary.
    pub r#delta: String,
}

/// Emitted when a reasoning summary text is completed.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary text is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary text is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The full text of the completed reasoning summary.
    pub r#text: String,
}

/// Emitted when there is a partial refusal text.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseRefusalDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the refusal text is added to.
    pub r#item_id: String,

    /// The index of the output item that the refusal text is added to.
    pub r#output_index: i64,

    /// The index of the content part that the refusal text is added to.
    pub r#content_index: i64,

    /// The refusal text that is added.
    pub r#delta: String,
}

/// Emitted when refusal text is finalized.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseRefusalDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the refusal text is finalized.
    pub r#item_id: String,

    /// The index of the output item that the refusal text is finalized.
    pub r#output_index: i64,

    /// The index of the content part that the refusal text is finalized.
    pub r#content_index: i64,

    /// The refusal text that is finalized.
    pub r#refusal: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ResponseStreamEvent {
    ResponseAudioDeltaEvent(ResponseAudioDeltaEvent),

    ResponseAudioDoneEvent(ResponseAudioDoneEvent),

    ResponseAudioTranscriptDeltaEvent(ResponseAudioTranscriptDeltaEvent),

    ResponseAudioTranscriptDoneEvent(ResponseAudioTranscriptDoneEvent),

    ResponseCodeInterpreterCallCodeDeltaEvent(ResponseCodeInterpreterCallCodeDeltaEvent),

    ResponseCodeInterpreterCallCodeDoneEvent(ResponseCodeInterpreterCallCodeDoneEvent),

    ResponseCodeInterpreterCallCompletedEvent(ResponseCodeInterpreterCallCompletedEvent),

    ResponseCodeInterpreterCallInProgressEvent(ResponseCodeInterpreterCallInProgressEvent),

    ResponseCodeInterpreterCallInterpretingEvent(ResponseCodeInterpreterCallInterpretingEvent),

    ResponseCompletedEvent(ResponseCompletedEvent),

    ResponseContentPartAddedEvent(ResponseContentPartAddedEvent),

    ResponseContentPartDoneEvent(ResponseContentPartDoneEvent),

    ResponseCreatedEvent(ResponseCreatedEvent),

    ResponseErrorEvent(ResponseErrorEvent),

    ResponseFileSearchCallCompletedEvent(ResponseFileSearchCallCompletedEvent),

    ResponseFileSearchCallInProgressEvent(ResponseFileSearchCallInProgressEvent),

    ResponseFileSearchCallSearchingEvent(ResponseFileSearchCallSearchingEvent),

    ResponseFunctionCallArgumentsDeltaEvent(ResponseFunctionCallArgumentsDeltaEvent),

    ResponseFunctionCallArgumentsDoneEvent(ResponseFunctionCallArgumentsDoneEvent),

    ResponseInProgressEvent(ResponseInProgressEvent),

    ResponseFailedEvent(ResponseFailedEvent),

    ResponseIncompleteEvent(ResponseIncompleteEvent),

    ResponseOutputItemAddedEvent(ResponseOutputItemAddedEvent),

    ResponseOutputItemDoneEvent(ResponseOutputItemDoneEvent),

    ResponseReasoningSummaryPartAddedEvent(ResponseReasoningSummaryPartAddedEvent),

    ResponseReasoningSummaryPartDoneEvent(ResponseReasoningSummaryPartDoneEvent),

    ResponseReasoningSummaryTextDeltaEvent(ResponseReasoningSummaryTextDeltaEvent),

    ResponseReasoningSummaryTextDoneEvent(ResponseReasoningSummaryTextDoneEvent),

    ResponseRefusalDeltaEvent(ResponseRefusalDeltaEvent),

    ResponseRefusalDoneEvent(ResponseRefusalDoneEvent),

    ResponseTextAnnotationDeltaEvent(ResponseTextAnnotationDeltaEvent),

    ResponseTextDeltaEvent(ResponseTextDeltaEvent),

    ResponseTextDoneEvent(ResponseTextDoneEvent),

    ResponseWebSearchCallCompletedEvent(ResponseWebSearchCallCompletedEvent),

    ResponseWebSearchCallInProgressEvent(ResponseWebSearchCallInProgressEvent),

    ResponseWebSearchCallSearchingEvent(ResponseWebSearchCallSearchingEvent),
}

/// Emitted when a text annotation is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextAnnotationDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text annotation was added to.
    pub r#item_id: String,

    /// The index of the output item that the text annotation was added to.
    pub r#output_index: i64,

    /// The index of the content part that the text annotation was added to.
    pub r#content_index: i64,

    /// The index of the annotation that was added.
    pub r#annotation_index: i64,

    pub r#annotation: Annotation,
}

/// Emitted when there is an additional text delta.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text delta was added to.
    pub r#item_id: String,

    /// The index of the output item that the text delta was added to.
    pub r#output_index: i64,

    /// The index of the content part that the text delta was added to.
    pub r#content_index: i64,

    /// The text delta that was added.
    pub r#delta: String,
}

/// Emitted when text content is finalized.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text content is finalized.
    pub r#item_id: String,

    /// The index of the output item that the text content is finalized.
    pub r#output_index: i64,

    /// The index of the content part that the text content is finalized.
    pub r#content_index: i64,

    /// The text content that is finalized.
    pub r#text: String,
}

/// Represents token usage details including input tokens, output tokens, a
/// breakdown of output tokens, and the total tokens used.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub r#input_tokens: i64,

    pub r#input_tokens_details: ResponseUsage_InputTokensDetails,

    /// The number of output tokens.
    pub r#output_tokens: i64,

    pub r#output_tokens_details: ResponseUsage_OutputTokensDetails,

    /// The total number of tokens used.
    pub r#total_tokens: i64,
}

/// A detailed breakdown of the input tokens.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseUsage_InputTokensDetails {
    /// The number of tokens that were retrieved from the cache.
    pub r#cached_tokens: i64,
}

/// A detailed breakdown of the output tokens.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseUsage_OutputTokensDetails {
    /// The number of reasoning tokens.
    pub r#reasoning_tokens: i64,
}

/// Emitted when a web search call is completed.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

/// Emitted when a web search call is initiated.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

/// Emitted when a web search call is executing.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallSearchingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response_Variant3 {
    /// Unique identifier for this Response.
    pub r#id: String,

    /// The object type of this resource - always set to `response`.
    pub r#object: String,

    /// The status of the response generation.
    pub r#status: Option<String>,

    /// Unix timestamp (in seconds) of when this Response was created.
    pub r#created_at: f64,

    pub r#error: Option<ResponseError>,

    pub r#incomplete_details: Option<Response_Variant3_IncompleteDetails>,

    /// An array of content items generated by the model.
    pub r#output: Vec<OutputItem>,

    /// SDK-only convenience property that contains the aggregated text output
    /// from all `output_text` items in the `output` array, if any are present.
    pub r#output_text: Option<String>,

    pub r#usage: Option<ResponseUsage>,

    /// Whether to allow the model to run tool calls in parallel.
    pub r#parallel_tool_calls: bool,
}

/// Details about why the response is incomplete.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response_Variant3_IncompleteDetails {
    /// The reason why the response is incomplete.
    pub r#reason: Option<String>,
}

/// Usage statistics related to the run.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunCompletionUsage {
    /// Number of completion tokens used over the course of the run.
    pub r#completion_tokens: i64,

    /// Number of prompt tokens used over the course of the run.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used (prompt + completion).
    pub r#total_tokens: i64,
}

/// Represents an execution run on a [thread](/docs/api-reference/threads).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the run was created.
    pub r#created_at: i64,

    /// The ID of the [thread](/docs/api-reference/threads) that was executed on
    /// as a part of this run.
    pub r#thread_id: String,

    /// The ID of the [assistant](/docs/api-reference/assistants) used for
    /// execution of this run.
    pub r#assistant_id: String,

    /// The status of the run, which can be either `queued`, `in_progress`,
    /// `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`,
    /// `incomplete`, or `expired`.
    pub r#status: String,

    pub r#required_action: Option<RunObject_RequiredAction>,

    pub r#last_error: Option<RunObject_LastError>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub r#expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was started.
    pub r#started_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub r#cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run failed.
    pub r#failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was completed.
    pub r#completed_at: Option<i64>,

    pub r#incomplete_details: Option<RunObject_IncompleteDetails>,

    /// The model that the [assistant](/docs/api-reference/assistants) used for
    /// this run.
    pub r#model: String,

    /// The instructions that the [assistant](/docs/api-reference/assistants)
    /// used for this run.
    pub r#instructions: String,

    /// The list of tools that the [assistant](/docs/api-reference/assistants)
    /// used for this run.
    pub r#tools: Vec<RunObject_Tools>,

    pub r#metadata: Option<Metadata>,

    pub r#usage: Option<RunCompletionUsage>,

    /// The sampling temperature used for this run.
    pub r#temperature: Option<f64>,

    /// The nucleus sampling value used for this run.
    pub r#top_p: Option<f64>,

    /// The maximum number of prompt tokens specified to have been used over the
    /// course of the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens specified to have been used over
    /// the course of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: Option<TruncationObject>,

    pub r#tool_choice: Option<AssistantsApiToolChoiceOption>,

    pub r#parallel_tool_calls: ParallelToolCalls,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// Details on why the run is incomplete.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject_IncompleteDetails {
    /// The reason why the run is incomplete.
    pub r#reason: Option<String>,
}

/// The last error associated with this run.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject_LastError {
    /// One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`.
    pub r#code: String,

    /// A human-readable description of the error.
    pub r#message: String,
}

/// Details on the action required to continue the run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject_RequiredAction {
    /// For now, this is always `submit_tool_outputs`.
    pub r#type: String,

    pub r#submit_tool_outputs: RunObject_RequiredAction_SubmitToolOutputs,
}

/// Details on the tool outputs needed for this run to continue.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject_RequiredAction_SubmitToolOutputs {
    /// A list of the relevant tool calls.
    pub r#tool_calls: Vec<RunToolCallObject>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunObject_Tools {
    AssistantToolsCode(AssistantToolsCode),

    AssistantToolsFileSearch(AssistantToolsFileSearch),

    AssistantToolsFunction(AssistantToolsFunction),
}

/// Usage statistics related to the run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepCompletionUsage {
    /// Number of completion tokens used over the course of the run step.
    pub r#completion_tokens: i64,

    /// Number of prompt tokens used over the course of the run step.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used (prompt + completion).
    pub r#total_tokens: i64,
}

/// Represents a run step delta i.e.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaObject {
    /// The identifier of the run step, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run.step.delta`.
    pub r#object: String,

    pub r#delta: RunStepDeltaObject_Delta,
}

/// The delta containing the fields that have changed on the run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaObject_Delta {
    pub r#step_details: Option<RunStepDeltaObject_Delta_StepDetails>,
}

/// The details of the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepDeltaObject_Delta_StepDetails {
    RunStepDeltaStepDetailsMessageCreationObject(RunStepDeltaStepDetailsMessageCreationObject),

    RunStepDeltaStepDetailsToolCallsObject(RunStepDeltaStepDetailsToolCallsObject),
}

/// Details of the message creation by the run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    /// Always `message_creation`.
    pub r#type: String,

    pub r#message_creation: Option<RunStepDeltaStepDetailsMessageCreationObject_MessageCreation>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsMessageCreationObject_MessageCreation {
    /// The ID of the message that was created by this run step.
    pub r#message_id: Option<String>,
}

/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call.
    pub r#id: Option<String>,

    /// The type of tool call.
    pub r#type: String,

    pub r#code_interpreter: Option<RunStepDeltaStepDetailsToolCallsCodeObject_CodeInterpreter>,
}

/// The Code Interpreter tool call definition.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject_CodeInterpreter {
    /// The input to the Code Interpreter tool call.
    pub r#input: Option<String>,

    /// The outputs from the Code Interpreter tool call.
    pub r#outputs: Option<Vec<RunStepDeltaStepDetailsToolCallsCodeObject_CodeInterpreter_Outputs>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepDeltaStepDetailsToolCallsCodeObject_CodeInterpreter_Outputs {
    RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject(RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject),

    RunStepDeltaStepDetailsToolCallsCodeOutputImageObject(RunStepDeltaStepDetailsToolCallsCodeOutputImageObject),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    /// The index of the output in the outputs array.
    pub r#index: i64,

    /// Always `image`.
    pub r#type: String,

    pub r#image: Option<RunStepDeltaStepDetailsToolCallsCodeOutputImageObject_Image>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject_Image {
    /// The [file](/docs/api-reference/files) ID of the image.
    pub r#file_id: Option<String>,
}

/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    /// The index of the output in the outputs array.
    pub r#index: i64,

    /// Always `logs`.
    pub r#type: String,

    /// The text output from the Code Interpreter tool call.
    pub r#logs: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call object.
    pub r#id: Option<String>,

    /// The type of tool call.
    pub r#type: String,

    /// For now, this is always going to be an empty object.
    pub r#file_search: RunStepDeltaStepDetailsToolCallsFileSearchObject_FileSearch,
}

/// For now, this is always going to be an empty object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject_FileSearch;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call object.
    pub r#id: Option<String>,

    /// The type of tool call.
    pub r#type: String,

    pub r#function: Option<RunStepDeltaStepDetailsToolCallsFunctionObject_Function>,
}

/// The definition of the function that was called.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject_Function {
    /// The name of the function.
    pub r#name: Option<String>,

    /// The arguments passed to the function.
    pub r#arguments: Option<String>,

    /// The output of the function.
    pub r#output: Option<String>,
}

/// Details of the tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    /// Always `tool_calls`.
    pub r#type: String,

    /// An array of tool calls the run step was involved in.
    pub r#tool_calls: Option<Vec<RunStepDeltaStepDetailsToolCallsObject_ToolCalls>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepDeltaStepDetailsToolCallsObject_ToolCalls {
    RunStepDeltaStepDetailsToolCallsCodeObject(RunStepDeltaStepDetailsToolCallsCodeObject),

    RunStepDeltaStepDetailsToolCallsFileSearchObject(RunStepDeltaStepDetailsToolCallsFileSearchObject),

    RunStepDeltaStepDetailsToolCallsFunctionObject(RunStepDeltaStepDetailsToolCallsFunctionObject),
}

/// Details of the message creation by the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsMessageCreationObject {
    /// Always `message_creation`.
    pub r#type: String,

    pub r#message_creation: RunStepDetailsMessageCreationObject_MessageCreation,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsMessageCreationObject_MessageCreation {
    /// The ID of the message that was created by this run step.
    pub r#message_id: String,
}

/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeObject {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    pub r#code_interpreter: RunStepDetailsToolCallsCodeObject_CodeInterpreter,
}

/// The Code Interpreter tool call definition.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeObject_CodeInterpreter {
    /// The input to the Code Interpreter tool call.
    pub r#input: String,

    /// The outputs from the Code Interpreter tool call.
    pub r#outputs: Vec<RunStepDetailsToolCallsCodeObject_CodeInterpreter_Outputs>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepDetailsToolCallsCodeObject_CodeInterpreter_Outputs {
    RunStepDetailsToolCallsCodeOutputLogsObject(RunStepDetailsToolCallsCodeOutputLogsObject),

    RunStepDetailsToolCallsCodeOutputImageObject(RunStepDetailsToolCallsCodeOutputImageObject),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    /// Always `image`.
    pub r#type: String,

    pub r#image: RunStepDetailsToolCallsCodeOutputImageObject_Image,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeOutputImageObject_Image {
    /// The [file](/docs/api-reference/files) ID of the image.
    pub r#file_id: String,
}

/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    /// Always `logs`.
    pub r#type: String,

    /// The text output from the Code Interpreter tool call.
    pub r#logs: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchObject {
    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    pub r#file_search: RunStepDetailsToolCallsFileSearchObject_FileSearch,
}

/// For now, this is always going to be an empty object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchObject_FileSearch {
    pub r#ranking_options: Option<RunStepDetailsToolCallsFileSearchRankingOptionsObject>,

    /// The results of the file search.
    pub r#results: Option<Vec<RunStepDetailsToolCallsFileSearchResultObject>>,
}

/// The ranking options for the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    pub r#ranker: FileSearchRanker,

    /// The score threshold for the file search.
    pub r#score_threshold: f64,
}

/// A result instance of the file search.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    /// The ID of the file that result was found in.
    pub r#file_id: String,

    /// The name of the file that result was found in.
    pub r#file_name: String,

    /// The score of the result.
    pub r#score: f64,

    /// The content of the result that was found.
    pub r#content: Option<Vec<RunStepDetailsToolCallsFileSearchResultObject_Content>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchResultObject_Content {
    /// The type of the content.
    pub r#type: Option<String>,

    /// The text content of the file.
    pub r#text: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFunctionObject {
    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    pub r#function: RunStepDetailsToolCallsFunctionObject_Function,
}

/// The definition of the function that was called.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFunctionObject_Function {
    /// The name of the function.
    pub r#name: String,

    /// The arguments passed to the function.
    pub r#arguments: String,

    /// The output of the function.
    pub r#output: Option<String>,
}

/// Details of the tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsObject {
    /// Always `tool_calls`.
    pub r#type: String,

    /// An array of tool calls the run step was involved in.
    pub r#tool_calls: Vec<RunStepDetailsToolCallsObject_ToolCalls>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepDetailsToolCallsObject_ToolCalls {
    RunStepDetailsToolCallsCodeObject(RunStepDetailsToolCallsCodeObject),

    RunStepDetailsToolCallsFileSearchObject(RunStepDetailsToolCallsFileSearchObject),

    RunStepDetailsToolCallsFunctionObject(RunStepDetailsToolCallsFunctionObject),
}

/// Represents a step in execution of a run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepObject {
    /// The identifier of the run step, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run.step`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the run step was created.
    pub r#created_at: i64,

    /// The ID of the [assistant](/docs/api-reference/assistants) associated
    /// with the run step.
    pub r#assistant_id: String,

    /// The ID of the [thread](/docs/api-reference/threads) that was run.
    pub r#thread_id: String,

    /// The ID of the [run](/docs/api-reference/runs) that this run step is a
    /// part of.
    pub r#run_id: String,

    /// The type of run step, which can be either `message_creation` or
    /// `tool_calls`.
    pub r#type: String,

    /// The status of the run step, which can be either `in_progress`,
    /// `cancelled`, `failed`, `completed`, or `expired`.
    pub r#status: String,

    pub r#step_details: RunStepObject_StepDetails,

    pub r#last_error: Option<RunStepObject_LastError>,

    /// The Unix timestamp (in seconds) for when the run step expired.
    pub r#expired_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    pub r#cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step failed.
    pub r#failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step completed.
    pub r#completed_at: Option<i64>,

    pub r#metadata: Option<Metadata>,

    pub r#usage: Option<RunStepCompletionUsage>,
}

/// The last error associated with this run step.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepObject_LastError {
    /// One of `server_error` or `rate_limit_exceeded`.
    pub r#code: String,

    /// A human-readable description of the error.
    pub r#message: String,
}

/// The details of the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RunStepObject_StepDetails {
    RunStepDetailsMessageCreationObject(RunStepDetailsMessageCreationObject),

    RunStepDetailsToolCallsObject(RunStepDetailsToolCallsObject),
}

include!("schemas/run_step_stream_event.rs");

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
/// created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant1 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) moves to
/// an `in_progress` state.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant2 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

/// Occurs when parts of a [run step](/docs/api-reference/run-steps/step-object)
/// are being streamed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant3 {
    pub r#event: String,

    pub r#data: RunStepDeltaObject,
}

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
/// completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant4 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) fails.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant5 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) is
/// cancelled.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant6 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

/// Occurs when a [run step](/docs/api-reference/run-steps/step-object) expires.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepStreamEvent_Variant7 {
    pub r#event: String,

    pub r#data: RunStepObject,
}

include!("schemas/run_stream_event.rs");

/// Occurs when a new [run](/docs/api-reference/runs/object) is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant1 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) expires.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant10 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) moves to a `queued`
/// status.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant2 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) moves to an
/// `in_progress` status.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant3 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) moves to a
/// `requires_action` status.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant4 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant5 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) ends with status
/// `incomplete`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant6 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) fails.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant7 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) moves to a `cancelling`
/// status.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant8 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Occurs when a [run](/docs/api-reference/runs/object) is cancelled.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStreamEvent_Variant9 {
    pub r#event: String,

    pub r#data: RunObject,
}

/// Tool call objects
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunToolCallObject {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of tool call the output is required for.
    pub r#type: String,

    pub r#function: RunToolCallObject_Function,
}

/// The function definition.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunToolCallObject_Function {
    /// The name of the function.
    pub r#name: String,

    /// The arguments that the model expects you to pass to the function.
    pub r#arguments: String,
}

/// A screenshot action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Screenshot {
    /// Specifies the event type.
    pub r#type: String,
}

/// A scroll action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scroll {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate where the scroll occurred.
    pub r#x: i64,

    /// The y-coordinate where the scroll occurred.
    pub r#y: i64,

    /// The horizontal scroll distance.
    pub r#scroll_x: i64,

    /// The vertical scroll distance.
    pub r#scroll_y: i64,
}

/// Specifies the latency tier to use for processing the request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ServiceTier {
    #[cfg_attr(feature = "serde", serde(rename = "auto"))]
    Auto,

    #[cfg_attr(feature = "serde", serde(rename = "default"))]
    Default,

    #[cfg_attr(feature = "serde", serde(rename = "flex"))]
    Flex,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategy {
    /// The maximum number of tokens in each chunk.
    pub r#max_chunk_size_tokens: i64,

    /// The number of tokens that overlap between chunks.
    pub r#chunk_overlap_tokens: i64,
}

/// Customize your own chunking strategy by setting chunk size and chunk
/// overlap.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategyRequestParam {
    /// Always `static`.
    pub r#type: String,

    pub r#static: StaticChunkingStrategy,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategyResponseParam {
    /// Always `static`.
    pub r#type: String,

    pub r#static: StaticChunkingStrategy,
}

/// Not supported with latest reasoning models `o3` and `o4-mini`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StopConfiguration {
    #[default]
    Null,

    Text(String),

    TextArray(Vec<String>),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmitToolOutputsRunRequest {
    /// A list of tools for which the outputs are being submitted.
    pub r#tool_outputs: Vec<SubmitToolOutputsRunRequest_ToolOutputs>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmitToolOutputsRunRequest_ToolOutputs {
    /// The ID of the tool call in the `required_action` object within the run
    /// object the output is being submitted for.
    pub r#tool_call_id: Option<String>,

    /// The output of the tool call to be submitted to continue the run.
    pub r#output: Option<String>,
}

/// An object specifying the format that the model must output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TextResponseFormatConfiguration {
    ResponseFormatText(ResponseFormatText),

    TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),

    ResponseFormatJsonObject(ResponseFormatJsonObject),
}

/// JSON Schema response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextResponseFormatJsonSchema {
    /// The type of response format being defined.
    pub r#type: String,

    /// A description of what the response format is for, used by the model to
    /// determine how to respond in the format.
    pub r#description: Option<String>,

    /// The name of the response format.
    pub r#name: String,

    pub r#schema: ResponseFormatJsonSchemaSchema,

    /// Whether to enable strict schema adherence when generating the output.
    pub r#strict: Option<bool>,
}

/// Represents a thread that contains [messages](/docs/api-reference/messages).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the thread was created.
    pub r#created_at: i64,

    pub r#tool_resources: Option<ThreadObject_ToolResources>,

    pub r#metadata: Option<Metadata>,
}

/// A set of resources that are made available to the assistant's tools in this
/// thread.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadObject_ToolResources {
    pub r#code_interpreter: Option<ThreadObject_ToolResources_CodeInterpreter>,

    pub r#file_search: Option<ThreadObject_ToolResources_FileSearch>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadObject_ToolResources_CodeInterpreter {
    /// A list of [file](/docs/api-reference/files) IDs made available to the
    /// `code_interpreter` tool.
    pub r#file_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadObject_ToolResources_FileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to
    /// this thread.
    pub r#vector_store_ids: Option<Vec<String>>,
}

include!("schemas/thread_stream_event.rs");

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToggleCertificatesRequest {
    pub r#certificate_ids: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Tool {
    FileSearchTool(FileSearchTool),

    FunctionTool(FunctionTool),

    WebSearchPreviewTool(WebSearchPreviewTool),

    ComputerUsePreviewTool(ComputerUsePreviewTool),
}

/// Use this option to force the model to call a specific function.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolChoiceFunction {
    /// For function calling, the type is always `function`.
    pub r#type: String,

    /// The name of the function to call.
    pub r#name: String,
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ToolChoiceOptions {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,

    #[cfg_attr(feature = "serde", serde(rename = "auto"))]
    Auto,

    #[cfg_attr(feature = "serde", serde(rename = "required"))]
    Required,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// Indicates that the model should use a built-in tool to generate a response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolChoiceTypes {
    /// The type of hosted tool the model should to use.
    pub r#type: String,
}

/// Emitted when there is an additional text delta.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The text delta that was additionally transcribed.
    pub r#delta: String,

    /// The log probabilities of the delta.
    pub r#logprobs: Option<Vec<TranscriptTextDeltaEvent_Logprobs>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDeltaEvent_Logprobs {
    /// The token that was used to generate the log probability.
    pub r#token: Option<String>,

    /// The log probability of the token.
    pub r#logprob: Option<f64>,

    /// The bytes that were used to generate the log probability.
    pub r#bytes: Option<Vec<i64>>,
}

/// Emitted when the transcription is complete.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The text that was transcribed.
    pub r#text: String,

    /// The log probabilities of the individual tokens in the transcription.
    pub r#logprobs: Option<Vec<TranscriptTextDoneEvent_Logprobs>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDoneEvent_Logprobs {
    /// The token that was used to generate the log probability.
    pub r#token: Option<String>,

    /// The log probability of the token.
    pub r#logprob: Option<f64>,

    /// The bytes that were used to generate the log probability.
    pub r#bytes: Option<Vec<i64>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TranscriptionInclude {
    #[cfg_attr(feature = "serde", serde(rename = "logprobs"))]
    Logprobs,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionSegment {
    /// Unique identifier of the segment.
    pub r#id: i64,

    /// Seek offset of the segment.
    pub r#seek: i64,

    /// Start time of the segment in seconds.
    pub r#start: f64,

    /// End time of the segment in seconds.
    pub r#end: f64,

    /// Text content of the segment.
    pub r#text: String,

    /// Array of token IDs for the text content.
    pub r#tokens: Vec<i64>,

    /// Temperature parameter used for generating the segment.
    pub r#temperature: f64,

    /// Average logprob of the segment.
    pub r#avg_logprob: f64,

    /// Compression ratio of the segment.
    pub r#compression_ratio: f64,

    /// Probability of no speech in the segment.
    pub r#no_speech_prob: f64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionWord {
    /// The text content of the word.
    pub r#word: String,

    /// Start time of the word in seconds.
    pub r#start: f64,

    /// End time of the word in seconds.
    pub r#end: f64,
}

/// Controls for how a thread will be truncated prior to the run.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TruncationObject {
    /// The truncation strategy to use for the thread.
    pub r#type: String,

    /// The number of most recent messages from the thread when constructing the
    /// context for the run.
    pub r#last_messages: Option<i64>,
}

/// An action to type in text.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type {
    /// Specifies the event type.
    pub r#type: String,

    /// The text to type.
    pub r#text: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateVectorStoreFileAttributesRequest {
    pub r#attributes: Option<VectorStoreFileAttributes>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateVectorStoreRequest {
    /// The name of the vector store.
    pub r#name: Option<String>,

    pub r#expires_after: Option<VectorStoreExpirationAfter>,

    pub r#metadata: Option<Metadata>,
}

/// The Upload object can accept byte chunks in the form of Parts.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Upload {
    /// The Upload unique identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the Upload was created.
    pub r#created_at: i64,

    /// The name of the file to be uploaded.
    pub r#filename: String,

    /// The intended number of bytes to be uploaded.
    pub r#bytes: i64,

    /// The intended purpose of the file.
    pub r#purpose: String,

    /// The status of the Upload.
    pub r#status: String,

    /// The Unix timestamp (in seconds) for when the Upload will expire.
    pub r#expires_at: i64,

    /// The object type, which is always "upload".
    pub r#object: Option<String>,

    /// The ready File object after the Upload is completed.
    pub r#file: Option<OpenAIFile>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UploadCertificateRequest {
    /// An optional name for the certificate
    pub r#name: Option<String>,

    /// The certificate content in PEM format
    pub r#content: String,
}

/// The upload Part represents a chunk of bytes we can add to an Upload object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UploadPart {
    /// The upload Part unique identifier, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the Part was created.
    pub r#created_at: i64,

    /// The ID of the Upload object that this Part was added to.
    pub r#upload_id: String,

    /// The object type, which is always `upload.part`.
    pub r#object: String,
}

/// A citation for a web resource used to generate a model response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UrlCitationBody {
    /// The type of the URL citation.
    pub r#type: String,

    /// The URL of the web resource.
    pub r#url: String,

    /// The index of the first character of the URL citation in the message.
    pub r#start_index: i64,

    /// The index of the last character of the URL citation in the message.
    pub r#end_index: i64,

    /// The title of the web resource.
    pub r#title: String,
}

/// The aggregated audio speeches usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageAudioSpeechesResult {
    pub r#object: String,

    /// The number of characters processed.
    pub r#characters: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated audio transcriptions usage details of the specific time
/// bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageAudioTranscriptionsResult {
    pub r#object: String,

    /// The number of seconds processed.
    pub r#seconds: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated code interpreter sessions usage details of the specific time
/// bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageCodeInterpreterSessionsResult {
    pub r#object: String,

    /// The number of code interpreter sessions.
    pub r#num_sessions: Option<i64>,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,
}

/// The aggregated completions usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageCompletionsResult {
    pub r#object: String,

    /// The aggregated number of text input tokens used, including cached
    /// tokens.
    pub r#input_tokens: i64,

    /// The aggregated number of text input tokens that has been cached from
    /// previous requests.
    pub r#input_cached_tokens: Option<i64>,

    /// The aggregated number of text output tokens used.
    pub r#output_tokens: i64,

    /// The aggregated number of audio input tokens used, including cached
    /// tokens.
    pub r#input_audio_tokens: Option<i64>,

    /// The aggregated number of audio output tokens used.
    pub r#output_audio_tokens: Option<i64>,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,

    /// When `group_by=batch`, this field tells whether the grouped usage result
    /// is batch or not.
    pub r#batch: Option<bool>,
}

/// The aggregated embeddings usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageEmbeddingsResult {
    pub r#object: String,

    /// The aggregated number of input tokens used.
    pub r#input_tokens: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated images usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageImagesResult {
    pub r#object: String,

    /// The number of images processed.
    pub r#images: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=source`, this field provides the source of the grouped
    /// usage result, possible values are `image.generation`, `image.edit`,
    /// `image.variation`.
    pub r#source: Option<String>,

    /// When `group_by=size`, this field provides the image size of the grouped
    /// usage result.
    pub r#size: Option<String>,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated moderations usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageModerationsResult {
    pub r#object: String,

    /// The aggregated number of input tokens used.
    pub r#input_tokens: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageResponse {
    pub r#object: String,

    pub r#data: Vec<UsageTimeBucket>,

    pub r#has_more: bool,

    pub r#next_page: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageTimeBucket {
    pub r#object: String,

    pub r#start_time: i64,

    pub r#end_time: i64,

    pub r#result: Vec<UsageTimeBucket_Result>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum UsageTimeBucket_Result {
    UsageCompletionsResult(UsageCompletionsResult),

    UsageEmbeddingsResult(UsageEmbeddingsResult),

    UsageModerationsResult(UsageModerationsResult),

    UsageImagesResult(UsageImagesResult),

    UsageAudioSpeechesResult(UsageAudioSpeechesResult),

    UsageAudioTranscriptionsResult(UsageAudioTranscriptionsResult),

    UsageVectorStoresResult(UsageVectorStoresResult),

    UsageCodeInterpreterSessionsResult(UsageCodeInterpreterSessionsResult),

    CostsResult(CostsResult),
}

/// The aggregated vector stores usage details of the specific time bucket.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageVectorStoresResult {
    pub r#object: String,

    /// The vector stores usage in bytes.
    pub r#usage_bytes: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,
}

/// Represents an individual `user` within an organization.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    /// The object type, which is always `organization.user`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the user
    pub r#name: String,

    /// The email address of the user
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the user was added.
    pub r#added_at: i64,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserListResponse {
    pub r#object: String,

    pub r#data: Vec<User>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserRoleUpdateRequest {
    /// `owner` or `reader`
    pub r#role: String,
}

/// The expiration policy for a vector store.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies.
    pub r#anchor: String,

    /// The number of days after the anchor time that the vector store will
    /// expire.
    pub r#days: i64,
}

/// Set of 16 key-value pairs that can be attached to an object.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileAttributes;

/// A batch of files attached to a vector store.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileBatchObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store.file_batch`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the vector store files batch
    /// was created.
    pub r#created_at: i64,

    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// that the [File](/docs/api-reference/files) is attached to.
    pub r#vector_store_id: String,

    /// The status of the vector store files batch, which can be either
    /// `in_progress`, `completed`, `cancelled` or `failed`.
    pub r#status: String,

    pub r#file_counts: VectorStoreFileBatchObject_FileCounts,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileBatchObject_FileCounts {
    /// The number of files that are currently being processed.
    pub r#in_progress: i64,

    /// The number of files that have been processed.
    pub r#completed: i64,

    /// The number of files that have failed to process.
    pub r#failed: i64,

    /// The number of files that where cancelled.
    pub r#cancelled: i64,

    /// The total number of files.
    pub r#total: i64,
}

/// Represents the parsed content of a vector store file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileContentResponse {
    /// The object type, which is always `vector_store.file_content.page`
    pub r#object: String,

    /// Parsed content of the file.
    pub r#data: Vec<VectorStoreFileContentResponse_Data>,

    /// Indicates if there are more content pages to fetch.
    pub r#has_more: bool,

    /// The token for the next page, if any.
    pub r#next_page: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileContentResponse_Data {
    /// The content type (currently only `"text"`)
    pub r#type: Option<String>,

    /// The text content
    pub r#text: Option<String>,
}

/// A list of files attached to a vector store.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store.file`.
    pub r#object: String,

    /// The total vector store usage in bytes.
    pub r#usage_bytes: i64,

    /// The Unix timestamp (in seconds) for when the vector store file was
    /// created.
    pub r#created_at: i64,

    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// that the [File](/docs/api-reference/files) is attached to.
    pub r#vector_store_id: String,

    /// The status of the vector store file, which can be either `in_progress`,
    /// `completed`, `cancelled`, or `failed`.
    pub r#status: String,

    pub r#last_error: Option<VectorStoreFileObject_LastError>,

    pub r#chunking_strategy: Option<VectorStoreFileObject_ChunkingStrategy>,

    pub r#attributes: Option<VectorStoreFileAttributes>,
}

/// The strategy used to chunk the file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VectorStoreFileObject_ChunkingStrategy {
    StaticChunkingStrategyResponseParam(StaticChunkingStrategyResponseParam),

    OtherChunkingStrategyResponseParam(OtherChunkingStrategyResponseParam),
}

/// The last error associated with this vector store file.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileObject_LastError {
    /// One of `server_error` or `rate_limit_exceeded`.
    pub r#code: String,

    /// A human-readable description of the error.
    pub r#message: String,
}

/// A vector store is a collection of processed files can be used by the
/// `file_search` tool.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the vector store was created.
    pub r#created_at: i64,

    /// The name of the vector store.
    pub r#name: String,

    /// The total number of bytes used by the files in the vector store.
    pub r#usage_bytes: i64,

    pub r#file_counts: VectorStoreObject_FileCounts,

    /// The status of the vector store, which can be either `expired`,
    /// `in_progress`, or `completed`.
    pub r#status: String,

    pub r#expires_after: Option<VectorStoreExpirationAfter>,

    /// The Unix timestamp (in seconds) for when the vector store will expire.
    pub r#expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the vector store was last
    /// active.
    pub r#last_active_at: Option<i64>,

    pub r#metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreObject_FileCounts {
    /// The number of files that are currently being processed.
    pub r#in_progress: i64,

    /// The number of files that have been successfully processed.
    pub r#completed: i64,

    /// The number of files that have failed to process.
    pub r#failed: i64,

    /// The number of files that were cancelled.
    pub r#cancelled: i64,

    /// The total number of files.
    pub r#total: i64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchRequest {
    pub r#query: VectorStoreSearchRequest_Query,

    /// Whether to rewrite the natural language query for vector search.
    pub r#rewrite_query: Option<bool>,

    /// The maximum number of results to return.
    pub r#max_num_results: Option<i64>,

    pub r#filters: Option<VectorStoreSearchRequest_Filters>,

    pub r#ranking_options: Option<VectorStoreSearchRequest_RankingOptions>,
}

/// A filter to apply based on file attributes.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VectorStoreSearchRequest_Filters {
    ComparisonFilter(ComparisonFilter),

    CompoundFilter(CompoundFilter),
}

/// A query string for a search
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VectorStoreSearchRequest_Query {
    Text(String),

    TextArray(Vec<String>),
}

/// Ranking options for search.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchRequest_RankingOptions {
    pub r#ranker: Option<String>,

    pub r#score_threshold: Option<f64>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultContentObject {
    /// The type of content.
    pub r#type: String,

    /// The text content returned from search.
    pub r#text: String,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultItem {
    /// The ID of the vector store file.
    pub r#file_id: String,

    /// The name of the vector store file.
    pub r#filename: String,

    /// The similarity score for the result.
    pub r#score: f64,

    pub r#attributes: Option<VectorStoreFileAttributes>,

    /// Content chunks from the file.
    pub r#content: Vec<VectorStoreSearchResultContentObject>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultsPage {
    /// The object type, which is always `vector_store.search_results.page`
    pub r#object: String,

    pub r#search_query: Vec<String>,

    /// The list of search result items.
    pub r#data: Vec<VectorStoreSearchResultItem>,

    /// Indicates if there are more results to fetch.
    pub r#has_more: bool,

    /// The token for the next page, if any.
    pub r#next_page: Option<String>,
}

pub type VoiceIdsShared = String;

/// A wait action.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wait {
    /// Specifies the event type.
    pub r#type: String,
}

/// High level guidance for the amount of context window space to use for the
/// search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WebSearchContextSize {
    #[cfg_attr(feature = "serde", serde(rename = "low"))]
    Low,

    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    Medium,

    #[cfg_attr(feature = "serde", serde(rename = "high"))]
    High,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

/// Approximate location parameters for the search.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchLocation {
    /// The two-letter [ISO country
    /// code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g.
    pub r#country: Option<String>,

    /// Free text input for the region of the user, e.g.
    pub r#region: Option<String>,

    /// Free text input for the city of the user, e.g.
    pub r#city: Option<String>,

    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of
    /// the user, e.g.
    pub r#timezone: Option<String>,
}

/// This tool searches the web for relevant results to use in a response.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchPreviewTool {
    /// The type of the web search tool.
    pub r#type: String,

    pub r#user_location: Option<WebSearchPreviewTool_UserLocation>,

    /// High level guidance for the amount of context window space to use for
    /// the search.
    pub r#search_context_size: Option<String>,
}

pub type WebSearchPreviewTool_UserLocation = Option<WebSearchPreviewTool_UserLocation_1>;

/// The user's location.
pub type WebSearchPreviewTool_UserLocation_1 = ApproximateLocation;

/// The results of a web search tool call.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchToolCall {
    /// The unique ID of the web search tool call.
    pub r#id: String,

    /// The type of the web search tool call.
    pub r#type: String,

    /// The status of the web search tool call.
    pub r#status: String,
}

include!("schemas/footer.rs");
