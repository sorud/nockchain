use std::num::NonZero;
use std::time::Duration;

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

// Kademlia constants
/** How often we should run a kademlia bootstrap to keep our peer table fresh */
const KADEMLIA_BOOTSTRAP_INTERVAL: Duration = Duration::from_secs(300);

// If the --force-peer cli arg is passed, we will force dial it every FORCE_PEER_BOOT_INTERVAL
const FORCE_PEER_DIAL_INTERVAL: Duration = Duration::from_secs(600);

/** How long we should keep a peer connection alive with no traffic */
const SWARM_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

// Core protocol (QUIC/ping/etc) constants
/** How many times we should retry dialing our initial peers if we can't get Kademlia initialized */
// TODO: Make command-line configurable
const INITIAL_PEER_RETRIES: u32 = 5;
/** How often we should send a keep-alive message to a peer */
const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(15);
/** How long should we wait before timing out the connection */
// const CONNECTION_TIMEOUT: Duration = SWARM_IDLE_TIMEOUT;
/** How long should we wait before timing out the handshake */
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);
/** How long QUIC should wait before timing out an idle connection */
// const MAX_IDLE_TIMEOUT_MILLISECS: u32 = CONNECTION_TIMEOUT.as_millis() as u32;
/** How often we should send an identify message to a peer */
const IDENTIFY_INTERVAL: Duration = KADEMLIA_BOOTSTRAP_INTERVAL;

/** Maximum number of established *incoming* connections */
const MAX_ESTABLISHED_INCOMING_CONNECTIONS: u32 = 32;

/** Maximum number of established *incoming* connections */
const MAX_ESTABLISHED_OUTGOING_CONNECTIONS: u32 = 16;

/** Maximum number of established connections */
const MAX_ESTABLISHED_CONNECTIONS: u32 = 48;

/** Maximum number of established connections with a single peer ID */
const MAX_ESTABLISHED_CONNECTIONS_PER_PEER: u32 = 2;

/** Maximum pending incoming connections */
const MAX_PENDING_INCOMING_CONNECTIONS: u32 = 16;

/** Maximum pending outcoing connections */
const MAX_PENDING_OUTGOING_CONNECTIONS: u32 = 16;

// Request/response constants
// const REQUEST_RESPONSE_MAX_CONCURRENT_STREAMS: usize = MAX_ESTABLISHED_CONNECTIONS as usize * 2;
const REQUEST_RESPONSE_TIMEOUT: Duration = Duration::from_secs(20);
const REQUEST_HIGH_THRESHOLD: u64 = 60;
const REQUEST_HIGH_RESET: Duration = Duration::from_secs(60);

// ALL PROTOCOLS MUST HAVE UNIQUE VERSIONS
const REQ_RES_PROTOCOL_VERSION: &str = "/nockchain-1-req-res";
const KAD_PROTOCOL_VERSION: &str = "/nockchain-1-kad";
const IDENTIFY_PROTOCOL_VERSION: &str = "/nockchain-1-identify";

const PEER_STORE_RECORD_CAPACITY: usize = 10 * 1024;

/// Configuration struct that allows overriding default constants from environment variables
#[derive(Debug, Deserialize, Clone)]
pub struct LibP2PConfig {
    /// How often we should run a kademlia bootstrap to keep our peer table fresh (seconds)
    #[serde(default = "default_kademlia_bootstrap_interval_secs")]
    pub kademlia_bootstrap_interval_secs: u64,

    /// How often we should force dial force peers (seconds)
    #[serde(default = "default_force_peer_dial_interval_secs")]
    pub force_peer_dial_interval_secs: u64,

    /// How long we should keep a peer connection alive with no traffic (seconds)
    #[serde(default = "default_swarm_idle_timeout_secs")]
    pub swarm_idle_timeout_secs: u64,

    /// How many times we should retry dialing our initial peers if we can't get Kademlia initialized
    #[serde(default = "default_initial_peer_retries")]
    pub initial_peer_retries: u32,

    /// How often we should send a keep-alive message to a peer (seconds)
    #[serde(default = "default_keep_alive_interval_secs")]
    pub keep_alive_interval_secs: u64,

    /// How long should we wait before timing out the handshake (seconds)
    #[serde(default = "default_handshake_timeout_secs")]
    pub handshake_timeout_secs: u64,

    /// How often we should send an identify message to a peer (seconds)
    #[serde(default = "default_identify_interval_secs")]
    pub identify_interval_secs: u64,

    /// Maximum number of established incoming connections
    #[serde(default = "default_max_established_incoming_connections")]
    pub max_established_incoming_connections: u32,

    /// Maximum number of established outgoing connections
    #[serde(default = "default_max_established_outgoing_connections")]
    pub max_established_outgoing_connections: u32,

    /// Maximum number of established connections
    #[serde(default = "default_max_established_connections")]
    pub max_established_connections: u32,

    /// Maximum number of established connections with a single peer ID
    #[serde(default = "default_max_established_connections_per_peer")]
    pub max_established_connections_per_peer: u32,

    /// Maximum pending incoming connections
    #[serde(default = "default_max_pending_incoming_connections")]
    pub max_pending_incoming_connections: u32,

    /// Maximum pending outgoing connections
    #[serde(default = "default_max_pending_outgoing_connections")]
    pub max_pending_outgoing_connections: u32,

    /// Request/response timeout (seconds)
    #[serde(default = "default_request_response_timeout_secs")]
    pub request_response_timeout_secs: u64,

    /// Request high threshold
    #[serde(default = "default_request_high_threshold")]
    pub request_high_threshold: u64,

    /// Request high reset timeout (seconds)
    #[serde(default = "default_request_high_reset_secs")]
    pub request_high_reset_secs: u64,

    // These have to be static.
    // /// Request/response protocol version
    // #[serde(default = "default_req_res_protocol_version")]
    // pub req_res_protocol_version: String,

    // /// Kademlia protocol version
    // #[serde(default = "default_kad_protocol_version")]
    // pub kad_protocol_version: String,
    /// Identify protocol version
    #[serde(default = "default_identify_protocol_version")]
    pub identify_protocol_version: String,

    /// Peer store record capacity
    /// This is the maximum number of records that can be stored in the peer store.
    #[serde(default = "default_peer_store_record_capacity")]
    pub peer_store_record_capacity: NonZero<usize>,

    /// Interval for logging peer status
    /// This is the interval at which peer status will be logged.
    #[serde(default = "default_peer_status_log_interval_secs")]
    pub peer_status_log_interval_secs: u64,
}

// Default value functions
fn default_kademlia_bootstrap_interval_secs() -> u64 {
    KADEMLIA_BOOTSTRAP_INTERVAL.as_secs()
}
fn default_force_peer_dial_interval_secs() -> u64 {
    FORCE_PEER_DIAL_INTERVAL.as_secs()
}
fn default_swarm_idle_timeout_secs() -> u64 {
    SWARM_IDLE_TIMEOUT.as_secs()
}
fn default_initial_peer_retries() -> u32 {
    INITIAL_PEER_RETRIES
}
fn default_keep_alive_interval_secs() -> u64 {
    KEEP_ALIVE_INTERVAL.as_secs()
}
fn default_handshake_timeout_secs() -> u64 {
    HANDSHAKE_TIMEOUT.as_secs()
}
fn default_identify_interval_secs() -> u64 {
    IDENTIFY_INTERVAL.as_secs()
}
fn default_max_established_incoming_connections() -> u32 {
    MAX_ESTABLISHED_INCOMING_CONNECTIONS
}
fn default_max_established_outgoing_connections() -> u32 {
    MAX_ESTABLISHED_OUTGOING_CONNECTIONS
}
fn default_max_established_connections() -> u32 {
    MAX_ESTABLISHED_CONNECTIONS
}
fn default_max_established_connections_per_peer() -> u32 {
    MAX_ESTABLISHED_CONNECTIONS_PER_PEER
}
fn default_max_pending_incoming_connections() -> u32 {
    MAX_PENDING_INCOMING_CONNECTIONS
}
fn default_max_pending_outgoing_connections() -> u32 {
    MAX_PENDING_OUTGOING_CONNECTIONS
}
fn default_request_response_timeout_secs() -> u64 {
    REQUEST_RESPONSE_TIMEOUT.as_secs()
}
fn default_request_high_threshold() -> u64 {
    REQUEST_HIGH_THRESHOLD
}
fn default_request_high_reset_secs() -> u64 {
    REQUEST_HIGH_RESET.as_secs()
}

fn default_identify_protocol_version() -> String {
    IDENTIFY_PROTOCOL_VERSION.to_string()
}

fn default_peer_store_record_capacity() -> NonZero<usize> {
    PEER_STORE_RECORD_CAPACITY
        .try_into()
        .expect("Peer store record capacity must be non-zero")
}

fn default_peer_status_log_interval_secs() -> u64 {
    60 // Log peer status every 60 seconds
}

// Do _not_ use this default implementation in production code. It's just a fallback.
// Use from_env() to load from environment variables with sensible defaults.
impl Default for LibP2PConfig {
    fn default() -> Self {
        Self {
            kademlia_bootstrap_interval_secs: default_kademlia_bootstrap_interval_secs(),
            force_peer_dial_interval_secs: default_force_peer_dial_interval_secs(),
            swarm_idle_timeout_secs: default_swarm_idle_timeout_secs(),
            initial_peer_retries: default_initial_peer_retries(),
            keep_alive_interval_secs: default_keep_alive_interval_secs(),
            handshake_timeout_secs: default_handshake_timeout_secs(),
            identify_interval_secs: default_identify_interval_secs(),
            max_established_incoming_connections: default_max_established_incoming_connections(),
            max_established_outgoing_connections: default_max_established_outgoing_connections(),
            max_established_connections: default_max_established_connections(),
            max_established_connections_per_peer: default_max_established_connections_per_peer(),
            max_pending_incoming_connections: default_max_pending_incoming_connections(),
            max_pending_outgoing_connections: default_max_pending_outgoing_connections(),
            request_response_timeout_secs: default_request_response_timeout_secs(),
            request_high_threshold: default_request_high_threshold(),
            request_high_reset_secs: default_request_high_reset_secs(),
            identify_protocol_version: default_identify_protocol_version(),
            peer_store_record_capacity: default_peer_store_record_capacity(),
            peer_status_log_interval_secs: default_peer_status_log_interval_secs(),
        }
    }
}

impl LibP2PConfig {
    /// Load configuration from environment variables with NOCKCHAIN_LIBP2P_ prefix
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("NOCKCHAIN_LIBP2P"))
            .build()?;

        config.try_deserialize()
    }

    /// Load configuration from environment variables, falling back to defaults on error
    pub fn from_env_or_default() -> Self {
        Self::from_env().unwrap_or_default()
    }

    pub fn kad_protocol_version() -> &'static str {
        KAD_PROTOCOL_VERSION
    }

    pub fn req_res_protocol_version() -> &'static str {
        REQ_RES_PROTOCOL_VERSION
    }

    /// Get kademlia bootstrap interval as Duration
    pub fn kademlia_bootstrap_interval(&self) -> Duration {
        Duration::from_secs(self.kademlia_bootstrap_interval_secs)
    }

    /// Get force peer dial interval as Duration
    pub fn force_peer_dial_interval(&self) -> Duration {
        Duration::from_secs(self.force_peer_dial_interval_secs)
    }

    /// Get swarm idle timeout as Duration
    pub fn swarm_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.swarm_idle_timeout_secs)
    }

    /// Get keep alive interval as Duration
    pub fn keep_alive_interval(&self) -> Duration {
        Duration::from_secs(self.keep_alive_interval_secs)
    }

    /// Get handshake timeout as Duration
    pub fn handshake_timeout(&self) -> Duration {
        Duration::from_secs(self.handshake_timeout_secs)
    }

    /// Get identify interval as Duration
    pub fn identify_interval(&self) -> Duration {
        Duration::from_secs(self.identify_interval_secs)
    }

    /// Get request response timeout as Duration
    pub fn request_response_timeout(&self) -> Duration {
        Duration::from_secs(self.request_response_timeout_secs)
    }

    /// Get request high reset as Duration
    pub fn request_high_reset(&self) -> Duration {
        Duration::from_secs(self.request_high_reset_secs)
    }

    /// Get connection timeout (same as swarm idle timeout)
    pub fn connection_timeout(&self) -> Duration {
        self.swarm_idle_timeout()
    }

    /// Get max idle timeout in milliseconds for QUIC
    pub fn max_idle_timeout_millisecs(&self) -> u32 {
        self.connection_timeout().as_millis() as u32
    }

    /// Get request response max concurrent streams
    pub fn request_response_max_concurrent_streams(&self) -> usize {
        self.max_established_connections as usize * 2
    }

    pub fn peer_status_log_interval_secs(&self) -> std::time::Duration {
        Duration::from_secs(self.peer_status_log_interval_secs)
    }
}
