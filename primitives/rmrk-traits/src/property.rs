// Copyright (C) 2021-2022 RMRK
// This file is part of rmrk-substrate.
// License: Apache 2.0 modified by RMRK, see https://github.com/rmrk-team/rmrk-substrate/blob/main/LICENSE

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::Serialize;

#[cfg(feature = "std")]
use crate::serialize;

#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(
	feature = "std",
	serde(bound = r#"
			BoundedKey: AsRef<[u8]>,
			BoundedValue: AsRef<[u8]>
		"#)
)]
pub struct PropertyInfo<BoundedKey, BoundedValue> {
	/// Key of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub key: BoundedKey,

	/// Value of the property
	#[cfg_attr(feature = "std", serde(with = "serialize::vec"))]
	pub value: BoundedValue,
}
