use serde_json::Value;
use std::fmt::Display;

use console::{style, Color};
use serde_derive::{Deserialize, Serialize};

pub const USER_AGENT: &'static str = "kalkafox/mdget/0.1.0";

pub const API_URL: &'static str = "https://api.modrinth.com/v2";

// Implement display and ToString here
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub loader: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersions {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: i64,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthProject {
    pub id: String,
    pub slug: String,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub team: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "body_url")]
    pub body_url: Value,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub queued: Value,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: Value,
    #[serde(rename = "moderator_message")]
    pub moderator_message: Value,
    pub license: License,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub downloads: i64,
    pub followers: i64,
    pub categories: Vec<String>,
    #[serde(rename = "additional_categories")]
    pub additional_categories: Vec<Value>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub versions: Vec<String>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "wiki_url")]
    pub wiki_url: Value,
    #[serde(rename = "discord_url")]
    pub discord_url: String,
    #[serde(rename = "donation_urls")]
    pub donation_urls: Vec<DonationUrl>,
    pub gallery: Vec<Value>,
    #[serde(rename = "flame_anvil_project")]
    pub flame_anvil_project: Value,
    #[serde(rename = "flame_anvil_user")]
    pub flame_anvil_user: Value,
    pub color: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    pub projects: Vec<ModrinthProject>,
    pub versions: Vec<ProjectVersion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub body: String,
    #[serde(rename = "additional_categories")]
    pub additional_categories: Vec<Value>,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "wiki_url")]
    pub wiki_url: String,
    #[serde(rename = "discord_url")]
    pub discord_url: String,
    #[serde(rename = "donation_urls")]
    pub donation_urls: Vec<DonationUrl>,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub downloads: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    pub color: i64,
    pub id: String,
    pub team: String,
    #[serde(rename = "body_url")]
    pub body_url: Value,
    #[serde(rename = "moderator_message")]
    pub moderator_message: Value,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub followers: i64,
    pub status: String,
    pub license: License,
    pub versions: Vec<String>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub gallery: Vec<Gallery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
    pub ordering: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DVersion {
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: String,
    pub dependencies: Vec<Dependency>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: String,
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: i64,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Value,
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: Option<String>,
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    #[serde(rename = "dependency_type")]
    pub dependency_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    #[serde(rename = "file_type")]
    pub file_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

pub type ProjectVersions = Vec<ProjectVersion>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub name: Option<String>,
    #[serde(rename = "version_number")]
    pub version_number: Option<String>,
    pub changelog: Option<String>,
    pub dependencies: Vec<Dependency>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "version_type")]
    pub version_type: Option<String>,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: Option<String>,
    #[serde(rename = "requested_status")]
    pub requested_status: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(rename = "author_id")]
    pub author_id: Option<String>,
    #[serde(rename = "date_published")]
    pub date_published: Option<String>,
    pub downloads: i64,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Option<String>,
    pub files: Vec<File>,
}

// Helper function for printing colored messages
fn print_colored_message(msg_type: &str, msg: &str, color: Color) {
    // Get the current time
    let now = chrono::Local::now();
    print!(
        "{}{}{} {} {}",
        style("[").bold(),
        style(now.format("%H:%M:%S")).fg(Color::Cyan).bold(),
        style("]").bold(),
        style(msg_type).fg(color).bold(),
        msg
    );
}

// colored info
pub fn cinfo(msg: &str) {
    print_colored_message("  ", msg, Color::Green);
}

// colored info with newline
pub fn cinfoln(msg: &str) {
    cinfo(msg);
    println!();
}

// colored warning
pub fn cwarn(msg: &str) {
    print_colored_message("  ", msg, Color::Yellow);
}

// colored warning with newline
pub fn cwarnln(msg: &str) {
    cwarn(msg);
    println!();
}

// colored error
pub fn cerror(msg: &str) {
    print_colored_message("  ", msg, Color::Red);
}

// colored error with newline
pub fn cerrorln(msg: &str) {
    cerror(msg);
    println!();
}
