use thiserror::Error;

macro_rules! model_id {
	(struct $name:ident, key: $entity_name:literal) => {
		#[derive(::std::fmt::Debug, ::std::default::Default, ::toasty::Embed)]
		pub struct $name(::uuid::Uuid);

		impl $name {
			pub fn new() -> Self {
				Self(::uuid::Uuid::new_v4())
			}

			pub fn from_uuid(uuid: ::uuid::Uuid) -> Self {
				Self(uuid)
			}

			pub fn parse(input: &str) -> ::std::result::Result<Self, $crate::shared::IdError> {
				<::uuid::Uuid as ::std::str::FromStr>::from_str(input)
					.map(Self::from_uuid)
					.map_err(|_| $crate::shared::IdError::Invalid {
						entity: $entity_name,
						value: input.to_string(),
					})
			}
		}

		impl ::std::str::FromStr for $name {
			type Err = $crate::shared::IdError;

			fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
				Self::parse(s)
			}
		}

		impl ::std::fmt::Display for $name {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				::std::write!(f, "{}", self.0)
			}
		}

		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				::serde::Serialize::serialize(&self.0, serializer)
			}
		}

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				let value = <::uuid::Uuid as ::serde::Deserialize>::deserialize(deserializer)?;
				Ok(Self::from_uuid(value))
			}
		}

		impl ::std::ops::Deref for $name {
			type Target = ::uuid::Uuid;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl ::std::cmp::PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				self.0 == other.0
			}
		}

		impl ::std::cmp::Eq for $name {}

		impl ::std::cmp::PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
				::std::option::Option::Some(::std::cmp::Ord::cmp(self, other))
			}
		}

		impl ::std::cmp::Ord for $name {
			fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
				::std::cmp::Ord::cmp(&self.0, &other.0)
			}
		}

		impl ::std::hash::Hash for $name {
			fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
				::std::hash::Hash::hash(&self.0, state);
			}
		}

		impl ::std::clone::Clone for $name {
			fn clone(&self) -> Self {
				Self(self.0)
			}
		}

		impl ::std::marker::Copy for $name {}
	};
}

#[derive(Debug, Error)]
pub enum IdError {
	#[error("Invalid id for '{entity}': '{value}'")]
	Invalid { entity: &'static str, value: String },
}

pub(crate) use model_id;
