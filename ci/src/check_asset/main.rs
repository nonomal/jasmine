use anyhow::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

const OWNER: &str = "niuhuan";
const REPO: &str = "jasmine";
const UA: &str = "niuhuan jasmine ci";

#[tokio::main]
async fn main() -> Result<()> {
    let gh_token = std::env::var("GH_TOKEN")?;
    if gh_token.is_empty() {
        panic!("Please set GH_TOKEN");
    }
    let flutter_version = std::env::var("flutter_version")?;
    if flutter_version.is_empty() {
        panic!("Please set flutter_version");
    }

    let target = std::env::var("TARGET")?;

    let vs_code_txt = tokio::fs::read_to_string("version.code.txt").await?;

    let code = vs_code_txt.trim();

    let mut release_file_name = match target.as_str() {
        "macos" => format!("jasmine-{}.dmg", code),
        "ios" => format!("jasmine-{}-nosign.ipa", code),
        "windows" => format!("jasmine-{}-windows-x86_64.zip", code),
        "linux" => format!("jasmine-{}-x86_64.AppImage", code),
        "android-arm32" => format!("jasmine-{}-arm32.apk", code),
        "android-arm64" => format!("jasmine-{}-arm64.apk", code),
        "android-x86_64" => format!("jasmine-{}-x86_64.apk", code),
        un => panic!("unknown target : {}", un),
    };
    if flutter_version.starts_with("2.") {
        release_file_name = format!("z-of-{}", release_file_name);
    }

    let client = reqwest::ClientBuilder::new().user_agent(UA).build()?;

    let check_response = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            OWNER, REPO, code
        ))
        .header("Authorization", format!("token {}", gh_token))
        .send()
        .await?;

    match check_response.status().as_u16() {
        200 => (),
        404 => println!("release not exists"),
        code => {
            let text = check_response.text().await?;
            panic!("error for check release : {} : {}", code, text);
        }
    }
    let release: Release = check_response.json().await?;

    let ass_names: Vec<String> = release.assets.iter().map(|a| a.name.clone()).collect();
    println!(
        "::set-output name=skip_build::{}",
        ass_names.contains(&release_file_name)
    );
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Value,
    pub uploader: Uploader,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}
