mod repository;
pub use repository::*;

use toasty::Model;

#[derive(Model)]
pub struct Country {
	#[key]
	code: String,
	name: String,
}
