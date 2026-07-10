use crate::research::classification::{ResearchKeywordsRepository, ResearchTopicsRepository};
use crate::research::works::dtos::GetWorksQuery;
use crate::research::works::entity::{AuthorshipPosition, WorkId, WorkType};
use crate::research::works::errors::WorksError;
use crate::research::works::openalex::OpenAlexClient;
use crate::research::works::repository::{NewAuthorship, NewWork, WorksRepository};
use crate::research::works::views::{SourceView, SyncResultView, WorkDetailView, WorkView};
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct WorksService {
    works: Arc<WorksRepository>,
    openalex: Arc<OpenAlexClient>,
    topics: Arc<ResearchTopicsRepository>,
    keywords: Arc<ResearchKeywordsRepository>,
}

impl WorksService {
    pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<WorkView>> {
        let works = self.works.list(query).await?;
        Ok(works
            .into_iter()
            .map(|w| WorkView {
                id: w.id,
                openalex_id: w.openalex_id,
                title: w.title,
                r#abstract: w.r#abstract,
                doi: w.doi,
                publication_date: w.publication_date,
                publication_year: w.publication_year,
                r#type: w.ty,
                lang: w.lang,
                is_accepted: w.is_accepted,
                is_published: w.is_published,
                primary_source_id: w.primary_source_id,
            })
            .collect())
    }

    pub async fn find_by_id(&self, id: WorkId) -> AppResult<Option<WorkDetailView>> {
        let work = match self.works.find_by_id(&id).await? {
            Some(w) => w,
            None => return Ok(None),
        };

        let source = match work.primary_source_id {
            Some(sid) => self
                .works
                .find_source_by_id(&sid)
                .await?
                .map(|s| SourceView {
                    id: s.id,
                    openalex_id: s.openalex_id,
                    display_name: s.display_name,
                    ty: s.ty,
                }),
            None => None,
        };

        let authorships = self.works.list_authorships(&work.id).await?;
        let topics = self.works.list_topics_with_ancestry(&work.id).await?;
        let keywords = self.works.list_keywords_with_names(&work.id).await?;

        Ok(Some(WorkDetailView {
            work: WorkView {
                id: work.id,
                openalex_id: work.openalex_id,
                title: work.title,
                r#abstract: work.r#abstract,
                doi: work.doi,
                publication_date: work.publication_date,
                publication_year: work.publication_year,
                r#type: work.ty,
                lang: work.lang,
                is_accepted: work.is_accepted,
                is_published: work.is_published,
                primary_source_id: work.primary_source_id,
            },
            source,
            authorships,
            topics,
            keywords,
        }))
    }

    pub async fn sync_from_openalex(&self, academic_id: Uuid) -> AppResult<SyncResultView> {
        let orcid = self
            .works
            .academic_has_orcid(academic_id)
            .await?
            .ok_or(WorksError::AcademicNotFound)?;

        let oa_works = self.openalex.list_works_by_orcid(&orcid).await?;
        let works_fetched = oa_works.len();

        let mut created = 0usize;
        let mut skipped = 0usize;
        let mut authorships_count = 0usize;
        let mut topics_count = 0usize;
        let mut keywords_count = 0usize;
        let mut errors = Vec::new();

        let unknown_topic_id = self.topics.unknown_topic_id().await.ok();
        let unknown_keyword_id = self.keywords.unknown_keyword_id().await.ok();

        for oa_work in &oa_works {
            let work_result = process_single_work(
                &self.works,
                &self.topics,
                &self.keywords,
                oa_work,
                unknown_topic_id,
                unknown_keyword_id,
            )
            .await;

            match work_result {
                Ok(stats) => {
                    if stats.was_inserted {
                        created += 1;
                        authorships_count += stats.authorships;
                        topics_count += stats.topics;
                        keywords_count += stats.keywords;
                    } else {
                        skipped += 1;
                    }
                }
                Err(e) => {
                    errors.push(format!("{}: {}", oa_work.id, e));
                }
            }
        }

        Ok(SyncResultView {
            academic_id,
            academic_orcid: orcid,
            works_fetched,
            works_created: created,
            works_skipped: skipped,
            authorships_inserted: authorships_count,
            topics_linked: topics_count,
            keywords_linked: keywords_count,
            errors,
        })
    }
}

struct ProcessStats {
    was_inserted: bool,
    authorships: usize,
    topics: usize,
    keywords: usize,
}

async fn process_single_work(
    repo: &WorksRepository,
    topics_repo: &ResearchTopicsRepository,
    keywords_repo: &ResearchKeywordsRepository,
    oa_work: &papers_openalex::Work,
    unknown_topic_id: Option<crate::research::classification::ResearchTopicId>,
    unknown_keyword_id: Option<crate::research::classification::ResearchKeywordId>,
) -> AppResult<ProcessStats> {
    let title = oa_work
        .title
        .clone()
        .or_else(|| oa_work.display_name.clone())
        .unwrap_or_default();

    let pub_date = oa_work
        .publication_date
        .as_deref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let work_type = parse_oa_type(oa_work.r#type.as_deref());
    let lang = oa_work.language.clone().unwrap_or_else(|| "en".to_string());

    let (is_accepted, is_published) = oa_work
        .primary_location
        .as_ref()
        .map(|loc| {
            (
                loc.is_accepted.unwrap_or(false),
                loc.is_published.unwrap_or(false),
            )
        })
        .unwrap_or((false, false));

    let source_id = if let Some(s) = oa_work
        .primary_location
        .as_ref()
        .and_then(|l| l.source.as_ref())
    {
        let source_ty = s.r#type.clone().unwrap_or_else(|| "unknown".to_string());
        Some(
            repo.upsert_source(
                &s.id.clone().unwrap_or_default(),
                &s.display_name.clone().unwrap_or_default(),
                &source_ty,
            )
            .await?,
        )
    } else {
        None
    };

    let new_work = NewWork {
        openalex_id: oa_work.id.clone(),
        title,
        abstract_text: oa_work.abstract_text.clone(),
        doi: oa_work.doi.clone(),
        publication_date: pub_date,
        publication_year: oa_work.publication_year.map(|y| y as i16),
        ty: work_type,
        lang,
        is_accepted,
        is_published,
        primary_source_id: source_id,
    };
    let inserted = repo.insert_work(&new_work).await?;

    let work_id = match inserted {
        Some(id) => id,
        None => {
            return Ok(ProcessStats {
                was_inserted: false,
                authorships: 0,
                topics: 0,
                keywords: 0,
            });
        }
    };

    let mut authorships = 0usize;
    if let Some(auths) = &oa_work.authorships {
        for auth in auths {
            let orcid = match auth.author.as_ref().and_then(|a| a.orcid.as_deref()) {
                Some(o) => o,
                None => continue,
            };

            let academic_name = repo.find_academic_name_by_orcid(orcid).await?;
            let is_external = academic_name.is_none();
            let name = if is_external {
                auth.author
                    .as_ref()
                    .and_then(|a| a.display_name.as_deref())
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                academic_name.unwrap_or_default()
            };

            let position = match auth.author_position.as_deref() {
                Some("first") => AuthorshipPosition::First,
                Some("last") => AuthorshipPosition::Last,
                _ => AuthorshipPosition::Middle,
            };

            let new_authorship = NewAuthorship {
                work_id,
                orcid: orcid.to_string(),
                name,
                is_external,
                is_corresponding: auth.is_corresponding.unwrap_or(false),
                affiliations: auth.raw_affiliation_strings.clone().unwrap_or_default(),
                position,
            };
            repo.insert_authorship(&new_authorship).await?;
            authorships += 1;
        }
    }

    let mut topics = 0usize;
    if let Some(work_topics) = &oa_work.topics {
        for t in work_topics {
            let topic_id = match &t.id {
                Some(id) => topics_repo
                    .find_by_openalex_id(id)
                    .await?
                    .map(|rt| *rt.id)
                    .or(unknown_topic_id.map(|id| *id)),
                None => unknown_topic_id.map(|id| *id),
            };

            if let Some(tid) = topic_id {
                repo.link_topic(&work_id, tid, t.score.unwrap_or(0.0))
                    .await?;
                topics += 1;
            }
        }
    }

    let mut kw_count = 0usize;
    if let Some(kws) = &oa_work.keywords {
        for k in kws {
            let keyword_id = match &k.id {
                Some(id) => {
                    let name = k.display_name.as_deref().unwrap_or("Unknown");
                    Some(keywords_repo.upsert(id, name).await?)
                }
                None => unknown_keyword_id,
            };

            if let Some(kid) = keyword_id {
                let score = k.score.unwrap_or(0.0);
                repo.link_keyword(&work_id, *kid, score).await?;
                kw_count += 1;
            }
        }
    }

    Ok(ProcessStats {
        was_inserted: true,
        authorships,
        topics,
        keywords: kw_count,
    })
}

fn parse_oa_type(ty: Option<&str>) -> WorkType {
    match ty {
        Some("article") => WorkType::Article,
        Some("book") => WorkType::Book,
        Some("book-chapter") => WorkType::BookChapter,
        Some("book-review") => WorkType::BookReview,
        Some("conference-abstract") => WorkType::ConferenceAbstract,
        Some("conference-paper") => WorkType::ConferencePaper,
        Some("data-paper") => WorkType::DataPaper,
        Some("dissertation") => WorkType::Dissertation,
        Some("editorial") => WorkType::Editorial,
        Some("erratum") => WorkType::Erratum,
        Some("letter") => WorkType::Letter,
        Some("libguide") => WorkType::Libguide,
        Some("other") => WorkType::Other,
        Some("paratext") => WorkType::Paratext,
        Some("peer-review") => WorkType::PeerReview,
        Some("preprint") => WorkType::Preprint,
        Some("reference-entry") => WorkType::ReferenceEntry,
        Some("report") => WorkType::Report,
        Some("retraction") => WorkType::Retraction,
        Some("review") => WorkType::Review,
        Some("software") => WorkType::Software,
        Some("software-paper") => WorkType::SoftwarePaper,
        Some("standard") => WorkType::Standard,
        Some("supplementary-materials") => WorkType::SupplementaryMaterials,
        _ => WorkType::Other,
    }
}
