//! OpenFGA authorization model DSL.
//!
//! Defines the authorization model structure compatible with OpenFGA's
//! authorization model language.

use std::collections::HashMap;

/// A type definition in the authorization model (e.g., "user", "tenant", "counter").
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeDefinition {
    pub r#type: String,
    pub relations: HashMap<String, RelationDefinition>,
}

/// A relation definition — specifies which users or types can hold this relation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelationDefinition {
    /// Directly related types (e.g., `["user", "group#member"]`).
    pub directly_related_user_types: Vec<String>,
}

/// Full authorization model — list of type definitions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthorizationModel {
    pub schema_version: String,
    pub type_definitions: Vec<TypeDefinition>,
}

impl AuthorizationModel {
    /// Default authorization model for the counter-service multi-tenant scenario.
    ///
    /// Types:
    /// - `user` — authenticated identity
    /// - `tenant` — multi-tenant boundary
    /// - `counter` — counter resource within a tenant
    ///
    /// Relations:
    /// - `tenant#owner` — user who owns the tenant
    /// - `tenant#member` — user who belongs to the tenant
    /// - `counter#tenant` — tenant that owns the counter
    /// - `counter#can_read` — computed: member of counter's tenant
    /// - `counter#can_write` — computed: owner of counter's tenant
    pub fn default_counter_model() -> Self {
        let mut model = Self {
            schema_version: "1.1".to_string(),
            type_definitions: Vec::new(),
        };

        // Type: user
        model.type_definitions.push(TypeDefinition {
            r#type: "user".to_string(),
            relations: HashMap::new(),
        });

        // Type: tenant
        let mut tenant_relations = HashMap::new();
        tenant_relations.insert(
            "owner".to_string(),
            RelationDefinition {
                directly_related_user_types: vec!["user".to_string()],
            },
        );
        tenant_relations.insert(
            "member".to_string(),
            RelationDefinition {
                directly_related_user_types: vec!["user".to_string()],
            },
        );
        model.type_definitions.push(TypeDefinition {
            r#type: "tenant".to_string(),
            relations: tenant_relations,
        });

        // Type: counter
        let mut counter_relations = HashMap::new();
        counter_relations.insert(
            "tenant".to_string(),
            RelationDefinition {
                directly_related_user_types: vec!["tenant".to_string()],
            },
        );
        counter_relations.insert(
            "can_read".to_string(),
            RelationDefinition {
                directly_related_user_types: vec!["tenant#member".to_string()],
            },
        );
        counter_relations.insert(
            "can_write".to_string(),
            RelationDefinition {
                directly_related_user_types: vec!["tenant#owner".to_string()],
            },
        );
        model.type_definitions.push(TypeDefinition {
            r#type: "counter".to_string(),
            relations: counter_relations,
        });

        model
    }

    /// Serialize to OpenFGA JSON DSL format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
