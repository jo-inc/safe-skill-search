use crate::db::{Database, Skill};
use anyhow::Result;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

const API_BASE: &str = "https://skills.sh";

#[derive(Debug, Deserialize)]
struct SearchResponse {
    skills: Vec<SkillsShSkill>,
    #[allow(dead_code)]
    count: usize,
}

#[derive(Debug, Deserialize)]
struct SkillsShSkill {
    id: String,
    name: String,
    installs: i64,
    #[serde(rename = "topSource")]
    top_source: Option<String>,
}

pub async fn sync_skillssh(db: &mut Database) -> Result<()> {
    tracing::info!("Syncing skills.sh registry...");

    let client = reqwest::Client::builder()
        .user_agent("skill-search/0.1")
        .build()?;

    let mut total = 0;
    let queries = ["", "a", "e", "i", "o", "u", "s", "t", "n", "r", "code", "docker", "git", "api", "test", "debug", "python", "rust", "javascript", "typescript"];

    for query in queries {
        let url = format!("{}/api/search?q={}&limit=100", API_BASE, query);
        
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(data) = resp.json::<SearchResponse>().await {
                    for skill in data.skills {
                        if let Err(e) = upsert_skillssh_skill(db, &skill) {
                            tracing::debug!("Failed to upsert skill {}: {}", skill.id, e);
                        } else {
                            total += 1;
                        }
                    }
                }
            }
            Ok(resp) => {
                tracing::debug!("skills.sh API error for query '{}': {}", query, resp.status());
            }
            Err(e) => {
                tracing::debug!("skills.sh request failed for query '{}': {}", query, e);
            }
        }
    }

    tracing::info!("Synced {} skills from skills.sh", total);

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    db.set_last_sync("skillssh", now, None)?;

    Ok(())
}

fn upsert_skillssh_skill(db: &mut Database, skill: &SkillsShSkill) -> Result<()> {
    let source = skill.top_source.as_deref().unwrap_or("");
    
    let (github_url, description) = if !source.is_empty() {
        let skill_id = skill.id.split('/').last().unwrap_or(&skill.id);
        (
            format!("https://github.com/{}/tree/main/skills/{}", source, skill_id),
            format!("From {}", source),
        )
    } else {
        (
            format!("https://skills.sh/skills/{}", skill.id),
            String::new(),
        )
    };

    let slug = skill.id.replace('/', "__");

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let db_skill = Skill {
        id: 0,
        slug,
        name: skill.name.clone(),
        registry: "skillssh".to_string(),
        description,
        skill_md: String::new(),
        github_url,
        version: None,
        stars: skill.installs,
        trusted: false,
        updated_at: now,
    };

    db.upsert_skill(&db_skill)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_base_url() {
        assert_eq!(API_BASE, "https://skills.sh");
    }
}
