// BES Book Formatter — CoverConfigRepository (module-7-cover-design)
//
// CRUD sobre a tabela `cover_configs`.

use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CoverConfig, CoverConfigInput};

pub struct CoverConfigRepository {
    pool: SqlitePool,
}

impl CoverConfigRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Busca cover config por project_id. Retorna None se ainda não configurado.
    pub async fn get_by_project(&self, project_id: &str) -> Result<Option<CoverConfig>, String> {
        sqlx::query_as::<_, CoverConfig>(
            "SELECT * FROM cover_configs WHERE project_id = ?1",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB error in get_by_project: {}", e))
    }

    /// Upsert cover config (INSERT OR REPLACE).
    pub async fn upsert(&self, input: &CoverConfigInput, spine_mm: f64) -> Result<CoverConfig, String> {
        let existing = self.get_by_project(&input.project_id).await?;

        let id = existing
            .as_ref()
            .map(|c| c.id.clone())
            .unwrap_or_else(|| Uuid::new_v4().to_string().replace('-', ""));

        let defaults = CoverConfig {
            id: id.clone(),
            project_id: input.project_id.clone(),
            template_id: "minimal".into(),
            genre: "fiction".into(),
            platform: "amazon-kdp".into(),
            title_override: None,
            subtitle: None,
            author_override: None,
            back_cover_text: String::new(),
            primary_color: "#991B1B".into(),
            secondary_color: "#F8F6F0".into(),
            font_title: "Playfair Display".into(),
            font_author: "Lato".into(),
            cover_image_path: None,
            cover_image_original: None,
            cover_image_dpi: None,
            page_count: 0,
            spine_width_mm: 0.0,
            paper_type: "white".into(),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        let base = existing.unwrap_or(defaults);

        let template_id = input.template_id.as_deref().unwrap_or(&base.template_id).to_string();
        let genre = input.genre.as_deref().unwrap_or(&base.genre).to_string();
        let platform = input.platform.as_deref().unwrap_or(&base.platform).to_string();
        let back_cover_text = input.back_cover_text.as_deref().unwrap_or(&base.back_cover_text).to_string();
        let primary_color = input.primary_color.as_deref().unwrap_or(&base.primary_color).to_string();
        let secondary_color = input.secondary_color.as_deref().unwrap_or(&base.secondary_color).to_string();
        let font_title = input.font_title.as_deref().unwrap_or(&base.font_title).to_string();
        let font_author = input.font_author.as_deref().unwrap_or(&base.font_author).to_string();
        let paper_type = input.paper_type.as_deref().unwrap_or(&base.paper_type).to_string();
        let page_count = input.page_count.unwrap_or(base.page_count);
        let cover_image_path = input.cover_image_path.as_ref().or(base.cover_image_path.as_ref()).cloned();
        let title_override = input.title_override.as_ref().or(base.title_override.as_ref()).cloned();
        let subtitle = input.subtitle.as_ref().or(base.subtitle.as_ref()).cloned();
        let author_override = input.author_override.as_ref().or(base.author_override.as_ref()).cloned();

        sqlx::query(
            r#"INSERT INTO cover_configs (
                id, project_id, template_id, genre, platform,
                title_override, subtitle, author_override, back_cover_text,
                primary_color, secondary_color, font_title, font_author,
                cover_image_path, page_count, spine_width_mm, paper_type, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, datetime('now'))
            ON CONFLICT(project_id) DO UPDATE SET
                template_id = excluded.template_id,
                genre = excluded.genre,
                platform = excluded.platform,
                title_override = excluded.title_override,
                subtitle = excluded.subtitle,
                author_override = excluded.author_override,
                back_cover_text = excluded.back_cover_text,
                primary_color = excluded.primary_color,
                secondary_color = excluded.secondary_color,
                font_title = excluded.font_title,
                font_author = excluded.font_author,
                cover_image_path = excluded.cover_image_path,
                page_count = excluded.page_count,
                spine_width_mm = excluded.spine_width_mm,
                paper_type = excluded.paper_type,
                updated_at = datetime('now')"#,
        )
        .bind(&id)
        .bind(&input.project_id)
        .bind(&template_id)
        .bind(&genre)
        .bind(&platform)
        .bind(&title_override)
        .bind(&subtitle)
        .bind(&author_override)
        .bind(&back_cover_text)
        .bind(&primary_color)
        .bind(&secondary_color)
        .bind(&font_title)
        .bind(&font_author)
        .bind(&cover_image_path)
        .bind(page_count)
        .bind(spine_mm)
        .bind(&paper_type)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB error in upsert: {}", e))?;

        self.get_by_project(&input.project_id)
            .await?
            .ok_or_else(|| "Cover config not found after upsert".to_string())
    }

    /// Atualiza spine_width_mm quando o page_count muda.
    pub async fn update_spine_width(&self, project_id: &str, spine_mm: f64) -> Result<(), String> {
        sqlx::query(
            "UPDATE cover_configs SET spine_width_mm = ?1, updated_at = datetime('now') WHERE project_id = ?2",
        )
        .bind(spine_mm)
        .bind(project_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB error in update_spine_width: {}", e))?;
        Ok(())
    }

    /// Deleta cover config de um projeto.
    pub async fn delete_by_project(&self, project_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM cover_configs WHERE project_id = ?1")
            .bind(project_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB error in delete_by_project: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spine_width_calc_amazon_kdp() {
        // 200 pages × 0.0025" × 25.4 mm/in = 12.7mm
        let result = 200.0_f64 * 0.0025 * 25.4;
        assert!((result - 12.7).abs() < 0.01, "KDP 200p spine = 12.7mm");
    }

    #[test]
    fn test_spine_width_calc_ingram() {
        // 200 pages × 0.002252" × 25.4 mm/in = 11.44mm
        let result = 200.0_f64 * 0.002252 * 25.4;
        assert!((result - 11.44).abs() < 0.01, "IngramSpark 200p spine ~= 11.44mm");
    }
}
