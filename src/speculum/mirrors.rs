use crate::Mirror;
use serde::Deserialize;

/// Contains information about Mirrors.
///
/// Filter actions and take is avalible throug this interface
#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors {
    cutoff: u64,
    last_check: String,
    num_checks: u64,
    check_frequency: u64,
    pub urls: Vec<Mirror>,
    version: u64,
}
