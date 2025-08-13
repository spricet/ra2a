use crate::core::agent::{
    AgentCapabilities, AgentCard, AgentCardSignature, AgentInterface, AgentProvider, AgentSkill,
    Security, TransportProtocol,
};
use crate::core::util::SecurityScheme;
use std::collections::HashMap;

pub fn sample_agent_card() -> AgentCard {
    AgentCard {
        protocol_version: "0.2.9".to_string(),
        name: "GeoSpatial Route Planner Agent".to_string(),
        description: "Provides advanced route planning, traffic analysis, and custom map generation services. This agent can calculate optimal routes, estimate travel times considering real-time traffic, and create personalized maps with points of interest.".to_string(),
        url: "https://georoute-agent.example.com/a2a/v1".to_string(),
        preferred_transport: Some(TransportProtocol::JsonRpc),
        additional_interfaces: vec![
            AgentInterface::new(
                "https://georoute-agent.example.com/a2a/v1",
                TransportProtocol::JsonRpc),
            AgentInterface::new(
                "https://georoute-agent.example.com/a2a/grpc",
                TransportProtocol::Grpc),
            AgentInterface::new(
                "https://georoute-agent.example.com/a2a/json",
                TransportProtocol::HttpJson),
        ],
        icon_url: "https://georoute-agent.example.com/icon.png".to_string(),
        provider: Some(AgentProvider::empty()
            .with_organization("Example Geo Services Inc.")
            .with_url("https://www.examplegeoservices.com")
        ),
        version: "1.2.0".to_string(),
        documentation_url: "https://docs.examplegeoservices.com/georoute-agent/api".to_string(),
        capabilities: Some(AgentCapabilities::new_default()
            .with_streaming(true)
            .with_push_notifications(true)
            .with_state_transition_history(false)),
        security_schemes:
        HashMap::from([
            ("google".to_string(), SecurityScheme::open_id_connect(
                "https://accounts.google.com/.well-known/openid-configuration",
                None::<&str>),
            )
        ]
        ),
        security: vec![
            Security::empty()
                .with_scheme("google", vec![
                    "openid",
                    "profile",
                    "email",
                ])
        ],
        default_input_modes: vec!["application/json".to_string(), "text/plain".to_string()],
        default_output_modes: vec!["application/json".to_string(), "image/png".to_string()],
        skills: vec![
            AgentSkill {
                id: "route-optimizer-traffic".to_string(),
                name: "Traffic-Aware Route Optimizer".to_string(),
                description: "Calculates the optimal driving route between two or more locations, taking into account real-time traffic conditions, road closures, and user preferences (e.g., avoid tolls, prefer highways).".to_string(),
                tags: vec![
                    "maps".to_string(),
                    "routing".to_string(),
                    "navigation".to_string(),
                    "directions".to_string(),
                    "traffic".to_string()
                ],
                examples: vec![
                    "Plan a route from '1600 Amphitheatre Parkway, Mountain View, CA' to 'San Francisco International Airport' avoiding tolls.".to_string(),
                    "{\"origin\": {\"lat\": 37.422, \"lng\": -122.084}, \"destination\": {\"lat\": 37.7749, \"lng\": -122.4194}, \"preferences\": [\"avoid_ferries\"]}".to_string()
                ],
                input_modes: vec![
                    "application/json".to_string(),
                    "text/plain".to_string()
                ],
                output_modes: vec![
                    "application/json".to_string(),
                    "application/vnd.geo+json".to_string(),
                    "text/html".to_string()
                ],
                security: vec![],
            },
            AgentSkill {
                id: "custom-map-generator".to_string(),
                name: "Personalized Map Generator".to_string(),
                description: "Creates custom map images or interactive map views based on user-defined points of interest, routes, and style preferences. Can overlay data layers.".to_string(),
                tags: vec![
                    "maps".to_string(),
                    "customization".to_string(),
                    "visualization".to_string(),
                    "cartography".to_string(),
                ],
                examples: vec![
                    "Generate a map of my upcoming road trip with all planned stops highlighted.".to_string(),
                    "Show me a map visualizing all coffee shops within a 1-mile radius of my current location.".to_string(),
                ],
                input_modes: vec![
                    "application/json".to_string(),
                ],
                output_modes: vec![
                    "image/png".to_string(),
                    "image/jpeg".to_string(),
                    "application/json".to_string(),
                    "text/html".to_string()
                ],
                security: vec![],
            }
        ],
        supports_authenticated_extended_card: true,
        signatures: vec![
            AgentCardSignature {
                protected: "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpPU0UiLCJraWQiOiJrZXktMSIsImprdSI6Imh0dHBzOi8vZXhhbXBsZS5jb20vYWdlbnQvandrcy5qc29uIn0".to_string(),
                signature: "QFdkNLNszlGj3z3u0YQGt_T9LixY3qtdQpZmsTdDHDe3fXV9y9-B3m2-XgCpzuhiLt8E0tV6HXoZKHv4GtHgKQ".to_string(),
                header: None,
            }
        ],
    }
}
