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

	#[get("/research-lines")]
	pub async fn get_research_lines(&self) -> WebResult<Vec<ResearchLine>> {
		Ok(self.work_classifications.list_research_lines().await?)
	}

	#[put("/research-line-mappings")]
	pub async fn update_research_line_mapping(&self, req: Request) -> WebResult<()> {
		let dto = req.body::<UpdateMappingBody>()?;

		if self
			.work_classifications
			.find_research_line_by_id(dto.research_line_id)
			.await?
			.is_none()
		{
			return Err(JsonResponse::NotFound())?;
		};

		let Some(mut subfield) = self
			.work_classifications
			.find_subfield_by_openalex_id(&dto.subfield_openalex_id)
			.await?
		else {
			return Err(JsonResponse::NotFound())?;
		};

		subfield.research_line_id = Some(dto.research_line_id);
		self.work_classifications.save_subfield(&subfield).await?;

		Ok(())
	}
}
