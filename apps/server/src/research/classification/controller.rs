use crate::research::classification::*;
use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/works-classification")]
pub struct WorksClassificationController {
	work_classifications: Arc<WorkClassificationRepository>,
}

impl WorksClassificationController {
	#[get("/domains")]
	pub async fn get_domains(&self, req: Request) -> WebResult<Vec<ResearchDomain>> {
		let query = req.query_validator::<WorkClassificationQueryDto>()?;
		let filter = ClassificationFilter::from(query.unwrap_or_default());

		Ok(self.work_classifications.list_domains(filter).await?)
	}

	#[get("/fields")]
	pub async fn get_fields(&self, req: Request) -> WebResult<Vec<ResearchField>> {
		let query = req.query_validator::<WorkClassificationQueryDto>()?;
		let filter = ClassificationFilter::from(query.unwrap_or_default());

		Ok(self.work_classifications.list_fields(filter).await?)
	}

	#[get("/subfields")]
	pub async fn get_subfields(&self, req: Request) -> WebResult<Vec<ResearchSubfield>> {
		let query = req.query_validator::<WorkClassificationQueryDto>()?;
		let filter = ClassificationFilter::from(query.unwrap_or_default());

		Ok(self.work_classifications.list_subfields(filter).await?)
	}

	#[get("/topics")]
	pub async fn get_topics(&self, req: Request) -> WebResult<Vec<ResearchTopic>> {
		let query = req.query_validator::<WorkClassificationQueryDto>()?;
		let filter = ClassificationFilter::from(query.unwrap_or_default());

		Ok(self.work_classifications.list_topics(filter).await?)
	}

	#[get("/keywords")]
	pub async fn get_keywords(&self, req: Request) -> WebResult<Vec<ResearchKeyword>> {
		let query = req.query_validator::<WorkClassificationQueryDto>()?;
		let filter = ClassificationFilter::from(query.unwrap_or_default());

		Ok(self.work_classifications.list_keywords(filter).await?)
	}
}
