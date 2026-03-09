// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use crate::prelude::{String, Vec};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rasn", derive(rasn::AsnType), rasn(automatic_tags))]
#[cfg_attr(feature = "musli", derive(musli::Encode, musli::Decode))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct PackageMetadata {
    pub info: PackageInfo,
    #[cfg_attr(feature = "serde", serde(default))]
    pub urls: Vec<PackageUrl>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rasn", derive(rasn::AsnType), rasn(automatic_tags))]
#[cfg_attr(feature = "musli", derive(musli::Encode, musli::Decode))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub requires_dist: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rasn", derive(rasn::AsnType), rasn(automatic_tags))]
#[cfg_attr(feature = "musli", derive(musli::Encode, musli::Decode))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct PackageUrl {
    pub filename: String,
    pub packagetype: String,
    pub url: String,
}
