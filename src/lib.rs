//! fajar-distributed — Distributed runtime for Fajar Lang.
//!
//! RPC framework, distributed tensors, cluster scheduling, and fault tolerance.
//! Provides built-in primitives for multi-node ML training and microservices
//! without external frameworks.
//!
//! Extracted from fajar-lang per Compass §5.1 ("Hapus dari core").

#![doc(html_root_url = "https://docs.rs/fajar-distributed/0.1.0")]
// Nightly clippy allow-list — mirrors fajar-lang's src/lib.rs allow-list at extraction time.
#![allow(clippy::collapsible_if)]

pub mod cluster;
pub mod data_plane;
pub mod deploy;
pub mod discovery;
pub mod dist_bench;
pub mod fault_tolerance;
pub mod fault_tolerance_v2;
pub mod ml_training;
pub mod raft;
pub mod rpc;
pub mod rpc_v2;
pub mod scheduler;
pub mod security;
pub mod tensors;
pub mod transport;
