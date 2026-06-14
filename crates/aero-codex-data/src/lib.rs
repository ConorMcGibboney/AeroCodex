#![forbid(unsafe_code)]
//! Dataset provenance records for AeroCodex reference data.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetRecord {
    pub id: &'static str,
    pub source_title: &'static str,
    pub source_url: Option<&'static str>,
    pub license_or_status: &'static str,
    pub extraction_method: &'static str,
    pub checksum_sha256: Option<&'static str>,
    pub units: &'static str,
    pub valid_range: &'static str,
    pub reviewer: Option<&'static str>,
}

impl DatasetRecord {
    #[must_use]
    pub const fn redistribution_reviewed(&self) -> bool {
        self.reviewer.is_some()
    }
}
