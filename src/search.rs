use crate::db::Database;
use crate::embeddings;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub slug: String,
    pub name: String,
    pub registry: String,
    pub description: String,
    pub github_url: String,
    pub score: f32,
}

pub async fn search(
    db: &Database,
    query: &str,
    limit: usize,
    registry: Option<&str>,
) -> Result<Vec<SearchResult>> {
    // Try vector search first
    let query_embedding = embeddings::embed_query(query)?;
    let vector_results = db.vector_search(&query_embedding, limit * 2, registry)?;

    if !vector_results.is_empty() {
        let results: Vec<SearchResult> = vector_results
            .into_iter()
            .take(limit)
            .map(|(skill, distance)| SearchResult {
                slug: skill.slug,
                name: skill.name,
                registry: skill.registry,
                description: skill.description,
                github_url: skill.github_url,
                score: 1.0 - distance, // Convert distance to similarity
            })
            .collect();
        return Ok(results);
    }

    // Fallback to fuzzy search
    let fuzzy_results = db.fuzzy_search(query, limit)?;
    let results: Vec<SearchResult> = fuzzy_results
        .into_iter()
        .enumerate()
        .map(|(i, skill)| SearchResult {
            slug: skill.slug,
            name: skill.name,
            registry: skill.registry,
            description: skill.description,
            github_url: skill.github_url,
            score: 1.0 - (i as f32 * 0.1), // Simple rank-based score
        })
        .collect();

    Ok(results)
}
