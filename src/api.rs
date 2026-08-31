use serde::Deserialize;

pub const API_BASE: &str = "https://nevo.is-a.dev/api";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Project {
    pub name: String,
    pub year: u32,
    #[serde(default)]
    pub live_url: Option<String>,
    #[serde(default)]
    pub source_code: Option<String>,
    pub description: String,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub tech_stack: Vec<String>,
    pub thumbnail: String,
    pub slug: Option<String>,
    #[serde(default)]
    pub hide: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StackItem {
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StackGroup {
    #[serde(rename = "type")]
    pub type_name: String,
    pub items: Vec<StackItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub start_date: String,
    pub end_date: Option<String>,
    #[serde(default)]
    pub hide: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BlogSummary {
    #[serde(rename = "_id")]
    pub _id: Option<String>,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub reading_time: Option<String>,
    pub slug: Option<String>,
    #[serde(default)]
    pub views: Option<i64>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BlogPost {
    #[serde(rename = "_id")]
    pub _id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub doc: String,
    #[serde(default)]
    pub reading_time: Option<String>,
    pub slug: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Resume {
    pub url: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct ApiError(pub String);

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Api {
    client: reqwest::Client,
}

impl Api {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("nevotui/0.1")
            .build()
            .expect("failed to build http client");
        Self { client }
    }

    pub async fn projects(&self) -> Result<Vec<Project>, ApiError> {
        let res = self
            .client
            .get(format!("{API_BASE}/projects"))
            .send()
            .await
            .map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("projects: HTTP {}", res.status())));
        }
        let body: Data<Vec<Project>> = res.json().await.map_err(api_err)?;
        Ok(body.data.into_iter().filter(|p| !p.hide).collect())
    }

    pub async fn stack(&self) -> Result<Vec<StackGroup>, ApiError> {
        let res = self
            .client
            .get(format!("{API_BASE}/stack"))
            .send()
            .await
            .map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("stack: HTTP {}", res.status())));
        }
        let body: Data<Vec<StackGroup>> = res.json().await.map_err(api_err)?;
        Ok(body.data)
    }

    pub async fn experience(&self) -> Result<Vec<Experience>, ApiError> {
        let res = self
            .client
            .get(format!("{API_BASE}/experience"))
            .send()
            .await
            .map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("experience: HTTP {}", res.status())));
        }
        let body: Data<Vec<Experience>> = res.json().await.map_err(api_err)?;
        Ok(body.data.into_iter().filter(|e| !e.hide).collect())
    }

    pub async fn blog_list(&self) -> Result<Vec<BlogSummary>, ApiError> {
        let res = self
            .client
            .get(format!("{API_BASE}/blog"))
            .send()
            .await
            .map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("blog: HTTP {}", res.status())));
        }
        let body: Data<Vec<BlogSummary>> = res.json().await.map_err(api_err)?;
        Ok(body.data)
    }

    pub async fn blog_post(&self, id: &str) -> Result<BlogPost, ApiError> {
        let url = format!("{API_BASE}/blog/{id}");
        let res = self.client.get(&url).send().await.map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("blog/{id}: HTTP {}", res.status())));
        }
        let body: Data<BlogPost> = res.json().await.map_err(api_err)?;
        Ok(body.data)
    }

    pub async fn resume(&self) -> Result<Resume, ApiError> {
        let res = self
            .client
            .get(format!("{API_BASE}/resume?info=true"))
            .send()
            .await
            .map_err(api_err)?;
        if !res.status().is_success() {
            return Err(ApiError(format!("resume: HTTP {}", res.status())));
        }
        res.json().await.map_err(api_err)
    }
}

fn api_err(e: reqwest::Error) -> ApiError {
    ApiError(e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn live_deserialization_smoke() {
        let api = Api::new();

        let projects = api.projects().await.expect("projects fetch");
        assert!(!projects.is_empty(), "expected at least one project");
        let p = &projects[0];
        assert!(!p.name.is_empty());
        assert!(!p.description.is_empty());

        let stack = api.stack().await.expect("stack fetch");
        assert!(!stack.is_empty(), "expected at least one stack group");
        assert!(!stack[0].items.is_empty(), "expected stack items");

        let exp = api.experience().await.expect("experience fetch");
        assert!(!exp.is_empty(), "expected experience");

        let blog = api.blog_list().await.expect("blog fetch");
        assert!(!blog.is_empty(), "expected blog posts");
        let first_id = blog[0]._id.clone().expect("blog post id");
        assert!(!blog[0].title.is_empty());

        let post = api.blog_post(&first_id).await.expect("blog post fetch");
        assert!(!post.doc.is_empty(), "expected post body");

        let resume = api.resume().await.expect("resume fetch");
        assert!(resume.url.is_some(), "expected resume url");

        assert!(blog[0]._id.is_some(), "_id must map from JSON '_id'");
    }
}
