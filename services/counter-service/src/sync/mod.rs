//! Counter sync strategies — OfflineFirst synchronization.
//!
//! Phase 0: Stub — sync logic will be implemented when
//! the OfflineFirst story is implemented for the platform.
//!
//! ## Planned strategies
//! - `OfflineFirstCounterSync` — local write first, background sync to Turso cloud
//! - `OnlineOnlyCounterSync` — direct cloud write for shared counters

/// Sync strategy tag for counter operations.
///
/// This will be used to enforce the storage policy rule that
/// every repo method must declare its SyncStrategy at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterSyncStrategy {
    /// Local write first, async background sync (default for tenant-private counters)
    OfflineFirst,
    /// Direct cloud write only (for shared/platform counters)
    OnlineOnly,
}
