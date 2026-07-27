mod repository;
pub use repository::*;

use serde::Serialize;
use toasty::Model;

#[derive(Debug, Clone, Serialize, Model)]
pub struct Country {
	#[key]
	code: String,
	name: String,
}
