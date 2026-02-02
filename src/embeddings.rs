use crate::db::Database;
use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub async fn index_all(db: &mut Database) -> Result<()> {
    let skills = db.get_skills_needing_embedding()?;

    if skills.is_empty() {
        tracing::info!("All skills already indexed");
        return Ok(());
    }

    tracing::info!("Indexing {} skills...", skills.len());

    // Initialize embedding model (downloads on first use, ~30MB)
    let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;

    // Batch process for efficiency
    let batch_size = 32;
    for chunk in skills.chunks(batch_size) {
        let texts: Vec<String> = chunk
            .iter()
            .map(|s| {
                // Combine name, description, and first part of SKILL.md for embedding
                let md_preview = s.skill_md.chars().take(1000).collect::<String>();
                format!("{}\n{}\n{}", s.name, s.description, md_preview)
            })
            .collect();

        let embeddings = model.embed(texts, None)?;

        for (skill, embedding) in chunk.iter().zip(embeddings.iter()) {
            db.store_embedding(skill.id, embedding)?;
        }

        tracing::debug!("Indexed batch of {} skills", chunk.len());
    }

    tracing::info!("Indexing complete");
    Ok(())
}

pub fn embed_query(query: &str) -> Result<Vec<f32>> {
    let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;
    let embeddings = model.embed(vec![query.to_string()], None)?;
    Ok(embeddings.into_iter().next().unwrap_or_default())
}
