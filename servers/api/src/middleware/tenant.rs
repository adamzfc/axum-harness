//! Tenant extraction middleware — extracts tenant_id from JWT Bearer token.
//!
//! Environment-gated JWT verification:
//! - Dev mode (jwt_secret == "dev-secret-change-in-production"): insecure decode
//!   without signature verification (backward-compatible with Phase 6).
//! - Prod mode: full HS256 signature verification + exp claim validation.
//!
//! SEC-01: Replaces `dangerous::insecure_decode` with proper verification.
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use domain::ports::TenantId;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, dangerous::insecure_decode, decode};
use serde::Deserialize;

/// JWT claims we need — only `sub` matters for tenant identification.
#[derive(Debug, Deserialize)]
struct IdTokenClaims {
    sub: String,
}

/// Default dev secret used by the boilerplate.
/// When jwt_secret matches this value, fall back to insecure decode (dev only).
const DEV_SECRET: &str = "dev-secret-change-in-production";

/// Extract tenant_id from Authorization: Bearer <id_token> header.
///
/// Reads `jwt_secret` from `AppState` in request extensions.
/// - Dev mode (secret == DEV_SECRET): insecure payload-only decode with warning.
/// - Prod mode: HS256 signature verification + exp claim validation.
///
/// On failure: returns 401 UNAUTHORIZED.
pub async fn tenant_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // 1. Extract Bearer token from Authorization header
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Read jwt_secret from request extensions (set by Extension layer)
    let jwt_secret: String = req
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_default();

    // 3. Decode JWT — env-gated: dev mode vs prod mode
    let token_data = if jwt_secret == DEV_SECRET {
        tracing::warn!(
            "JWT: using insecure decode (dev-secret) — set APP_AUTH__JWT_SECRET for production"
        );
        insecure_decode::<IdTokenClaims>(token).map_err(|e| {
            tracing::debug!(error = %e, "insecure_decode failed");
            StatusCode::UNAUTHORIZED
        })?
    } else {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        decode::<IdTokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::warn!(error = %e, "JWT signature/exp validation failed");
            StatusCode::UNAUTHORIZED
        })?
    };

    // 4. Inject tenant_id into request extensions
    req.extensions_mut().insert(TenantId(token_data.claims.sub));

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

    fn make_test_token(sub: &str) -> String {
        #[derive(serde::Serialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }
        encode(
            &Header::new(Algorithm::HS256),
            &Claims {
                sub: sub.to_string(),
                exp: 9999999999,
            },
            &EncodingKey::from_secret(b"test-secret"),
        )
        .unwrap()
    }

    #[test]
    fn extract_sub_from_valid_jwt() {
        let token = make_test_token("google-sub-123");
        let claims: IdTokenClaims = insecure_decode::<IdTokenClaims>(&token).unwrap().claims;
        assert_eq!(claims.sub, "google-sub-123");
    }

    #[test]
    fn reject_invalid_jwt_format() {
        let result = insecure_decode::<IdTokenClaims>("not-a-jwt");
        assert!(result.is_err());
    }

    #[test]
    fn reject_empty_token() {
        let result = insecure_decode::<IdTokenClaims>("");
        assert!(result.is_err());
    }

    // --- HS256 signature verification tests (SEC-01) ---

    fn make_hs256_token(sub: &str, exp: usize, secret: &[u8]) -> String {
        #[derive(serde::Serialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }
        encode(
            &Header::new(Algorithm::HS256),
            &Claims {
                sub: sub.to_string(),
                exp,
            },
            &EncodingKey::from_secret(secret),
        )
        .unwrap()
    }

    #[test]
    fn hs256_roundtrip_valid_signature() {
        let secret = b"prod-secret-key";
        let token = make_hs256_token("tenant-abc", 9999999999, secret);

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let claims: IdTokenClaims =
            decode::<IdTokenClaims>(&token, &DecodingKey::from_secret(secret), &validation)
                .unwrap()
                .claims;

        assert_eq!(claims.sub, "tenant-abc");
    }

    #[test]
    fn hs256_rejects_expired_token() {
        let secret = b"prod-secret-key";
        // exp = 0 → always expired
        let token = make_hs256_token("tenant-abc", 0, secret);

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let result =
            decode::<IdTokenClaims>(&token, &DecodingKey::from_secret(secret), &validation);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.kind(),
            &jsonwebtoken::errors::ErrorKind::ExpiredSignature
        );
    }

    #[test]
    fn hs256_rejects_invalid_signature() {
        let token_secret = b"signing-secret";
        let wrong_secret = b"wrong-secret";
        let token = make_hs256_token("tenant-abc", 9999999999, token_secret);

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let result =
            decode::<IdTokenClaims>(&token, &DecodingKey::from_secret(wrong_secret), &validation);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.kind(),
            &jsonwebtoken::errors::ErrorKind::InvalidSignature
        );
    }

    #[test]
    fn dev_secret_falls_back_to_insecure_decode() {
        // When jwt_secret == DEV_SECRET, insecure_decode should work (no signature check)
        let token = make_hs256_token("dev-tenant", 9999999999, b"any-secret");
        let claims: IdTokenClaims = insecure_decode::<IdTokenClaims>(&token).unwrap().claims;
        assert_eq!(claims.sub, "dev-tenant");
    }
}
