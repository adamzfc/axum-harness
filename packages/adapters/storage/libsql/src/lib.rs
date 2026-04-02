//! LibSQL adapter — implementations of LibSqlPort for embedded and remote backends.

pub mod embedded;
pub mod remote;

pub use embedded::EmbeddedLibSql;
pub use remote::TursoDb;
