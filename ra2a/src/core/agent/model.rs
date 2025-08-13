use crate::core::util::{Object, SecurityScheme, StringList};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The AgentCard is a self-describing manifest for an agent. It provides essential
/// metadata including the agent's identity, capabilities, skills, supported
/// communication methods, and security requirements.
#[derive(Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentCard {
    /// The version of the A2A protocol this agent supports.
    #[cfg_attr(feature = "grpc", prost(string, tag = "16"))]
    pub protocol_version: String,

    /// A human-readable name for the agent.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub name: String,

    /// A human-readable description of the agent, assisting users and other agents in understanding its purpose.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub description: String,

    /// The preferred endpoint URL for interacting with the agent.
    /// This URL MUST support the transport specified by 'preferredTransport'.
    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub url: String,

    /// The transport protocol for the preferred endpoint (the main 'url' field).
    /// If not specified, defaults to 'JSONRPC'.
    ///
    /// IMPORTANT: The transport specified here MUST be available at the main 'url'.
    /// This creates a binding between the main URL and its supported transport protocol.
    /// Clients should prefer this transport and URL combination when both are supported.
    #[cfg_attr(feature = "grpc", prost(message, tag = "14"))]
    pub preferred_transport: Option<TransportProtocol>,

    /// A list of additional supported interfaces (transport and URL combinations).
    /// This allows agents to expose multiple transports, potentially at different URLs.
    ///
    /// Best practices:
    /// - SHOULD include all supported transports for completeness
    /// - SHOULD include an entry matching the main 'url' and 'preferredTransport'
    /// - MAY reuse URLs if multiple transports are available at the same endpoint
    /// - MUST accurately declare the transport available at each URL
    ///
    /// Clients can select any interface from this list based on their transport capabilities
    /// and preferences. This enables transport negotiation and fallback scenarios.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "15"))]
    pub additional_interfaces: Vec<AgentInterface>,

    /// Information about the agent's service provider.
    #[cfg_attr(feature = "grpc", prost(message, tag = "4"))]
    pub provider: Option<AgentProvider>,

    /// The agent's own version number. The format is defined by the provider.
    #[cfg_attr(feature = "grpc", prost(string, tag = "5"))]
    pub version: String,

    /// An optional URL to the agent's documentation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[cfg_attr(feature = "grpc", prost(string, tag = "6"))]
    pub documentation_url: String,

    /// A declaration of optional capabilities supported by the agent.
    #[cfg_attr(feature = "grpc", prost(message, tag = "7"))]
    pub capabilities: Option<AgentCapabilities>,

    /// A declaration of the security schemes available to authorize requests. The key is the
    /// scheme name. Follows the OpenAPI 3.0 Security Scheme Object.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "grpc", prost(map = "string, message", tag = "8"))]
    pub security_schemes: HashMap<String, SecurityScheme>,

    /// A list of security requirement objects that apply to all agent interactions. Each object
    /// lists security schemes that can be used. Follows the OpenAPI 3.0 Security Requirement Object.
    /// This list can be seen as an OR of ANDs. Each object in the list describes one possible
    /// set of security requirements that must be present on a request. This allows specifying,
    /// for example, "callers must either use OAuth OR an API Key AND mTLS."
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "9"))]
    pub security: Vec<Security>,

    /// Default set of supported input MIME types for all skills, which can be
    /// overridden on a per-skill basis.
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "10"))]
    pub default_input_modes: Vec<String>,

    /// Default set of supported output MIME types for all skills, which can be
    /// overridden on a per-skill basis.
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "11"))]
    pub default_output_modes: Vec<String>,

    /// The set of skills, or distinct capabilities, that the agent can perform.
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "12"))]
    pub skills: Vec<AgentSkill>,

    /// If true, the agent can provide an extended agent card with additional details
    /// to authenticated users. Defaults to false.
    #[serde(default)]
    #[cfg_attr(feature = "grpc", prost(bool, tag = "13"))]
    pub supports_authenticated_extended_card: bool,

    /// JSON Web Signatures computed for this AgentCard.
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "17"))]
    pub signatures: Vec<AgentCardSignature>,

    /// An optional URL to an icon for the agent.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[cfg_attr(feature = "grpc", prost(string, tag = "18"))]
    pub icon_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TransportProtocol {
    #[serde(rename = "JSONRPC")]
    JsonRpc,
    #[serde(rename = "GRPC")]
    Grpc,
    #[serde(rename = "HTTP+JSON")]
    HttpJson,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentInterface {
    /// The URL where this interface is available. Must be a valid absolute HTTPS URL in production.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub url: String,

    /// The transport protocol supported at this URL.
    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub transport: Option<TransportProtocol>,
}

/// Represents the service provider of an agent.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentProvider {
    /// The name of the agent provider's organization.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub organization: String,

    /// A URL for the agent provider's website or relevant documentation.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub url: String,
}

/// Defines optional capabilities supported by an agent.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentCapabilities {
    /// Indicates if the agent supports Server-Sent Events (SSE) for streaming responses.
    #[serde(default)]
    #[cfg_attr(feature = "grpc", prost(bool, tag = "1"))]
    pub streaming: bool,

    /// Indicates if the agent supports sending push notifications for asynchronous task updates.
    #[serde(default)]
    #[cfg_attr(feature = "grpc", prost(bool, tag = "2"))]
    pub push_notifications: bool,

    /// Indicates if the agent provides a history of state transitions for a task.
    #[serde(default)]
    #[cfg_attr(feature = "grpc", prost(bool, tag = "3"))]
    pub state_transition_history: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "4"))]
    pub extensions: Vec<AgentExtension>,
}

/// A declaration of a protocol extension supported by an Agent.
#[derive(Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentExtension {
    /// The unique URI identifying the extension
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub url: String,

    /// A human-readable description of how this agent uses the extension.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub description: String,

    /// If true, the client must understand and comply with the extension's requirements
    /// to interact with the agent.
    #[cfg_attr(feature = "grpc", prost(bool, tag = "3"))]
    pub required: bool,

    /// Optional, extension-specific configuration parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "grpc", prost(message, tag = "4"))]
    pub params: Option<Object>,
}

/// Represents a distinct capability or function that an agent can perform.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentSkill {
    /// A unique identifier for the agent's skill.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub id: String,

    /// A human-readable name for the skill.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub name: String,

    /// A detailed description of the skill, intended to help clients or users
    /// understand its purpose and functionality.
    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub description: String,

    /// A set of keywords describing the skill's capabilities.
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "4"))]
    pub tags: Vec<String>,

    /// Example prompts or scenarios that this skill can handle. Provides a hint to
    /// the client on how to use the skill.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "5"))]
    pub examples: Vec<String>,

    /// The set of supported input MIME types for this skill, overriding the agent's defaults.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "6"))]
    pub input_modes: Vec<String>,

    /// The set of supported output MIME types for this skill, overriding the agent's defaults.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "7"))]
    pub output_modes: Vec<String>,

    /// Security schemes necessary for the agent to leverage this skill.
    /// As in the overall AgentCard.Security, this list represents a logical OR of security
    /// requirement objects. Each object is a set of security schemes that must be used together
    /// (a logical AND).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "8"))]
    pub security: Vec<Security>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug, Default))]
pub struct Security {
    #[serde(flatten)]
    #[cfg_attr(feature = "grpc", prost(map = "string, message", tag = "1"))]
    pub schemes: HashMap<String, StringList>,
}

/// AgentCardSignature represents a JWS signature of an AgentCard.
/// This follows the JSON format of an RFC 7515 JSON Web Signature (JWS).
#[derive(Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct AgentCardSignature {
    /// The protected JWS header for the signature. This is a Base64url-encoded
    /// JSON object, as per RFC 7515.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub protected: String,

    /// The computed signature, Base64url-encoded.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub signature: String,

    /// The unprotected JWS header values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    pub header: Option<Object>,
}

impl Security {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_scheme(
        mut self,
        scheme: impl Into<String>,
        scopes: Vec<impl Into<String>>,
    ) -> Self {
        self.schemes.insert(
            scheme.into(),
            StringList {
                list: scopes.into_iter().map(|s| s.into()).collect(),
            },
        );
        self
    }
}

impl AgentProvider {
    pub fn empty() -> Self {
        Self {
            organization: String::new(),
            url: String::new(),
        }
    }

    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = organization.into();
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }
}

impl AgentCapabilities {
    pub fn new_default() -> Self {
        Self {
            streaming: false,
            push_notifications: false,
            state_transition_history: false,
            extensions: vec![],
        }
    }

    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    pub fn with_push_notifications(mut self, push_notifications: bool) -> Self {
        self.push_notifications = push_notifications;
        self
    }

    pub fn with_state_transition_history(mut self, state_transition_history: bool) -> Self {
        self.state_transition_history = state_transition_history;
        self
    }

    pub fn with_extension(mut self, extension: AgentExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn with_extensions(mut self, extensions: Vec<AgentExtension>) -> Self {
        self.extensions.extend(extensions);
        self
    }

    pub fn set_extensions(mut self, extensions: Vec<AgentExtension>) -> Self {
        self.extensions = extensions;
        self
    }
}

impl TransportProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            TransportProtocol::JsonRpc => "JSONRPC",
            TransportProtocol::Grpc => "GRPC",
            TransportProtocol::HttpJson => "HTTP+JSON",
        }
    }
}

impl TryFrom<&str> for TransportProtocol {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "JSONRPC" => TransportProtocol::JsonRpc,
            "GRPC" => TransportProtocol::Grpc,
            "HTTP+JSON" => TransportProtocol::HttpJson,
            _ => return Err(()),
        };
        Ok(val)
    }
}

impl Default for TransportProtocol {
    fn default() -> Self {
        Self::JsonRpc
    }
}

impl AgentInterface {
    pub fn new(url: impl Into<String>, transport: TransportProtocol) -> Self {
        Self {
            url: url.into(),
            transport: Some(transport),
        }
    }
}

#[cfg(feature = "grpc")]
impl prost::Message for TransportProtocol {
    fn encode_raw(&self, buf: &mut impl bytes::BufMut)
    where
        Self: Sized,
    {
        let val = self.as_str().to_string();
        prost::encoding::string::encode(1, &val, buf)
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        wire_type: prost::encoding::WireType,
        buf: &mut impl bytes::Buf,
        ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        let mut s = String::new();
        prost::encoding::string::merge(wire_type, &mut s, buf, ctx)?;
        *self = match TransportProtocol::try_from(s.as_str()) {
            Ok(p) => p,
            Err(_) => {
                return Err(prost::DecodeError::new(format!(
                    "unknown transport protocol: {}",
                    &s
                )));
            }
        };
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        let val = self.as_str().to_string();
        prost::encoding::string::encoded_len(1, &val)
    }

    fn clear(&mut self) {
        *self = TransportProtocol::default();
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::core::agent::{AgentCard, sample_agent_card};

    #[test]
    fn spec_example_serde() {
        let expected = sample_agent_card();
        println!("{}", serde_json::to_string_pretty(&expected).unwrap());

        // https://a2a-protocol.org/latest/specification/#57-sample-agent-card
        let example = include_str!("../../../tests/resources/sample-agent-card.json");
        let deserialized = serde_json::from_str::<AgentCard>(example).unwrap();
        assert_eq!(deserialized, expected);
    }
}
