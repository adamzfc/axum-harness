//! Counter interfaces layer — HTTP/gRPC adapters for external consumption.
//!
//! ## Phase 0: Empty stub
//!
//! Per ARCHITECTURE.md, HTTP composition lives in `servers/bff/`, not here.
//! This module is reserved for future:
//! - gRPC service definitions (when inter-service communication moves to gRPC)
//! - Internal HTTP routes (if counter-service is extracted to its own deployable)
//!
//! For now, the BFF layer (`servers/bff/web-bff`) imports counter-service
//! traits and implements HTTP handlers.
