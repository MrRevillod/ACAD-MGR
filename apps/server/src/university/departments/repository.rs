use crate::shared::{AppResult, Tx};
use crate::university::departments::Department;
use sword::prelude::*;

#[injectable]
pub struct DepartmentsRepository;

impl DepartmentsRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, department: &Department) -> AppResult<()> {
        sqlx::query("INSERT INTO departments (id, name, faculty_id) VALUES ($1, $2, $3)")
            .bind(&department.id)
            .bind(&department.name)
            .bind(&department.faculty_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
