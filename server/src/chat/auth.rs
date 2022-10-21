use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::Arc;

use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery};
use async_graphql::parser::types::{ExecutableDocument, Selection};
use async_graphql::{ServerError, ServerResult, Variables};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use warp::{cookie, Filter};

use crate::chat::User;

const JWT_SECRET: &[u8] = b"secret!";
pub const AUTH_COOKIE_NAME: &str = "chat_user";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    id: String,
}

pub fn create_auth_token(user: &User) -> String {
    let auth_user = AuthUser {
        id: user.id.clone(),
    };

    encode(
        &Header::new(Algorithm::HS512),
        &auth_user,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap()
}

pub fn with_auth() -> impl Filter<Extract = (Option<AuthUser>,), Error = Infallible> + Clone {
    cookie::optional(AUTH_COOKIE_NAME).map(|cookie: Option<String>| {
        info!("auth cookie: {cookie:?}");

        let mut validation = Validation::new(Algorithm::HS512);
        validation.required_spec_claims = HashSet::new();
        let result = decode(&cookie?, &DecodingKey::from_secret(JWT_SECRET), &validation);

        info!("{result:?}");

        result.ok()?.claims
    })
}

pub struct AuthExtensionFactory;

impl ExtensionFactory for AuthExtensionFactory {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension)
    }
}

pub struct AuthExtension;

#[async_trait::async_trait]
impl Extension for AuthExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let document = next.run(ctx, query, variables).await?;

        let auth_header = ctx.data_unchecked::<Option<AuthUser>>();

        if auth_header.is_some() {
            info!("found auth header {auth_header:?}");
            return Ok(document);
        };

        let field_names = find_top_level_field_names(&document);

        info!("No auth header, but trying to get access to fields {field_names:?}");

        if operations_need_authentication(field_names) {
            warn!("Denying access due to missing authentication");
            return Err(ServerError::new(
                "Missing authentication for operation",
                None,
            ));
        }
        Ok(document)
    }
}

fn find_top_level_field_names(document: &ExecutableDocument) -> Vec<&str> {
    document
        .operations
        .iter()
        .flat_map(|(_, operation)| &operation.node.selection_set.node.items)
        .filter_map(|selection| match &selection.node {
            Selection::Field(field) => Some(&field.node),
            _ => None,
        })
        .map(|field| field.name.node.as_str())
        .collect::<Vec<_>>()
}

fn operations_need_authentication(field_names: Vec<&str>) -> bool {
    const WHITELISTED_OPERATIONS: [&str; 2] = ["__schema", "register"];

    field_names
        .into_iter()
        .any(|field_name| !WHITELISTED_OPERATIONS.contains(&field_name))
}
