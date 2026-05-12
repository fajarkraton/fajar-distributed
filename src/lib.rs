//! fajar-distributed — Distributed runtime for Fajar Lang.
//!
//! Extracted from fajar-lang per Compass §5.1 ("Hapus dari core").
//! Skeleton stage — source files will land at Phase F.2.

#![doc(html_root_url = "https://docs.rs/fajar-distributed/0.1.0")]

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_version_matches_cargo() {
        assert_eq!(version(), "0.1.0");
    }
}
