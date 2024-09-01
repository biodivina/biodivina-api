//! Parser for biodivina CSV datasets.
//!
//! Support converting from CSV to JSON and vice versa.

/// Main parser data structure.
#[allow(dead_code)]
pub struct MainData {
    ///
    pub group_id: String,
    pub dataset_id: String,
    pub species_id: String,
    pub id: usize,
    pub taxonomy: Taxonomy,
}

/// Taxonomy data structure.
#[allow(dead_code)]
pub struct Taxonomy {
    pub taxon_rank: String,
    pub kingdom: String,
    pub phylum: String,
    pub taxon_class: String,
    pub taxon_order: String,
    pub taxon_family: String,
    pub genus: String,
    pub species: String,
    pub species_author_last_name: String,
    pub species_year: String,
}
