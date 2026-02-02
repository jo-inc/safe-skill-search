use crate::db::Database;
use anyhow::Result;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::{IndexRecordOption, Schema, STORED, STRING, TEXT, Field, Value};
use tantivy::{Index, IndexWriter, Term, TantivyDocument};

pub struct SearchIndex {
    index: Index,
    #[allow(dead_code)]
    schema: Schema,
    slug_field: Field,
    name_field: Field,
    description_field: Field,
    content_field: Field,
    registry_field: Field,
}

impl SearchIndex {
    pub fn open_or_create(index_path: &Path) -> Result<Self> {
        std::fs::create_dir_all(index_path)?;

        let mut schema_builder = Schema::builder();
        let slug_field = schema_builder.add_text_field("slug", TEXT | STORED);
        let name_field = schema_builder.add_text_field("name", TEXT | STORED);
        let description_field = schema_builder.add_text_field("description", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT);
        let registry_field = schema_builder.add_text_field("registry", STRING | STORED);
        let schema = schema_builder.build();

        let index = if index_path.join("meta.json").exists() {
            Index::open_in_dir(index_path)?
        } else {
            Index::create_in_dir(index_path, schema.clone())?
        };

        Ok(Self {
            index,
            schema,
            slug_field,
            name_field,
            description_field,
            content_field,
            registry_field,
        })
    }

    pub fn rebuild(&self, db: &Database) -> Result<()> {
        let mut index_writer: IndexWriter = self.index.writer(50_000_000)?;
        index_writer.delete_all_documents()?;

        let skills = db.get_all_skills()?;
        tracing::info!("Indexing {} skills", skills.len());

        for skill in skills {
            let mut doc = TantivyDocument::new();
            doc.add_text(self.slug_field, &skill.slug);
            doc.add_text(self.name_field, &skill.name);
            doc.add_text(self.description_field, &skill.description);
            doc.add_text(self.registry_field, &skill.registry);
            // Combine name, description, and skill_md for full-text search
            let content = format!("{} {} {}", skill.name, skill.description, skill.skill_md);
            doc.add_text(self.content_field, &content);
            index_writer.add_document(doc)?;
        }

        index_writer.commit()?;
        tracing::info!("Index rebuilt");
        Ok(())
    }

    pub fn search(&self, query_str: &str, limit: usize, registry: Option<&str>) -> Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.name_field, self.description_field, self.content_field],
        );
        let text_query = query_parser.parse_query(query_str)?;

        // Build final query with optional registry filter
        let final_query: Box<dyn tantivy::query::Query> = if let Some(reg) = registry {
            let registry_term = Term::from_field_text(self.registry_field, reg);
            let registry_query = TermQuery::new(registry_term, IndexRecordOption::Basic);
            Box::new(BooleanQuery::new(vec![
                (Occur::Must, text_query),
                (Occur::Must, Box::new(registry_query)),
            ]))
        } else {
            text_query
        };

        let top_docs = searcher.search(&*final_query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            
            let slug = doc.get_first(self.slug_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let name = doc.get_first(self.name_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let description = doc.get_first(self.description_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let registry = doc.get_first(self.registry_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            results.push(SearchResult {
                slug,
                name,
                description,
                registry,
                score,
            });
        }

        Ok(results)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub registry: String,
    pub score: f32,
}

impl SearchResult {
    pub fn unique_key(&self) -> String {
        format!("{}:{}", self.registry, self.slug)
    }
}
