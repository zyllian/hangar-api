use std::fmt::Display;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
pub struct Pagination {
	pub limit: i64,
	pub offset: i64,
}

impl Default for Pagination {
	fn default() -> Self {
		Self {
			limit: 25,
			offset: 0,
		}
	}
}

impl From<(i64, i64)> for Pagination {
	fn from(value: (i64, i64)) -> Self {
		Self {
			limit: value.0,
			offset: value.1,
		}
	}
}

/// for some reason sorting is.. backwards by default? and there's no mention of this in the api documentation
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum ProjectsSort {
	#[serde(rename = "-views")]
	Views,
	#[serde(rename = "-downloads")]
	Downloads,
	#[serde(rename = "-newest")]
	Newest,
	#[serde(rename = "-stars")]
	Stars,
	#[serde(rename = "-updated")]
	Updated,
	#[serde(rename = "-recent-downloads")]
	RecentDownloads,
	#[serde(rename = "-recent-views")]
	RecentViews,
	Slug, // this one *isn't* inverted though..
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "snake_case")]
pub enum Category {
	AdminTools,
	Chat,
	DevTools,
	Economy,
	Gameplay,
	Games,
	Protection,
	RolePlaying,
	WorldManagement,
	Misc,
	Undefined,
}

impl Display for Category {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::AdminTools => "Admin Tools",
			Self::Chat => "Chat",
			Self::DevTools => "Dev Tools",
			Self::Economy => "Economy",
			Self::Gameplay => "Gameplay",
			Self::Games => "Games",
			Self::Protection => "Protection",
			Self::RolePlaying => "Role Playing",
			Self::WorldManagement => "World Management",
			Self::Misc => "Misc",
			Self::Undefined => "Undefined",
		};
		write!(f, "{s}")
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Platform {
	Paper,
	Waterfall,
	Velocity,
}

impl Display for Platform {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Paper => "Paper",
			Self::Waterfall => "Waterfall",
			Self::Velocity => "Velocity",
		};
		write!(f, "{s}")
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
	#[serde(deserialize_with = "time::serde::rfc3339::deserialize")]
	pub created_at: OffsetDateTime,
	/// The unique name of the project
	pub name: String,
	/// The namespace of the project
	pub namespace: Namespace,
	/// Stats of the project
	pub stats: ProjectStats,
	/// The category of the project
	pub category: Category,
	/// The last time the project was updated
	#[serde(deserialize_with = "time::serde::rfc3339::deserialize")]
	pub last_updated: OffsetDateTime,
	/// The visibility of a project or version
	pub visibility: Visibility,
	/// The url to the project's icon
	pub avatar_url: String,
	/// The short description of the project
	pub description: String,
	/// Information about your interactions with the project
	pub user_actions: UserActions,
	/// The settings of the project
	pub settings: ProjectSettings,
}

#[derive(Debug, Deserialize)]
pub struct Namespace {
	pub owner: String,
	pub slug: String,
}

impl Namespace {
	pub fn url(&self) -> String {
		format!("https://hangar.papermc.io/{}/{}", self.owner, self.slug)
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStats {
	pub views: i64,
	pub downloads: i64,
	pub recent_views: i64,
	pub recent_downloads: i64,
	pub stars: i64,
	pub watchers: i64,
}

/// The visibility of a project or version
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
	Public,
	New,
	NeedsChanges,
	NeedsApproval,
	SoftDelete,
}

#[derive(Debug, Deserialize)]
pub struct UserActions {
	pub starred: bool,
	pub watching: bool,
	pub flagged: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectSettings {
	pub links: Vec<Link>,
	pub tags: Vec<ProjectTags>,
	pub license: License,
	pub keywords: Vec<String>,
	pub sponsors: String,
	pub donation: Donation,
}

#[derive(Debug, Deserialize)]
pub struct Link {
	pub id: i64,
	/// Type of the link. Either SIDEBAR or TOP
	#[serde(rename = "type")]
	pub link_type: String,
	pub title: Option<String>,
	pub links: Vec<ActualLink>,
}

#[derive(Debug, Deserialize)]
pub struct ActualLink {
	pub id: i64,
	pub name: String,
	/// they don't follow their own schema.. this is supposed to be required
	pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectTags {
	Addon,
	Library,
	SupportsFolia,
}

#[derive(Debug, Deserialize)]
pub struct License {
	pub name: Option<String>,
	pub url: Option<String>,
	#[serde(rename = "type")]
	pub license_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Donation {
	pub enable: bool,
	pub subject: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	#[serde(deserialize_with = "time::serde::rfc3339::deserialize")]
	pub created_at: OffsetDateTime,
	pub name: String,
	pub visibility: Visibility,
	pub description: String,
	pub stats: VersionStats,
	pub author: String,
	pub review_state: ReviewState,
	pub channel: Channel,
	pub pinned_status: PinnedStatus,
	pub downloads: ByPlatform<VersionDownloads>,
	pub plugin_dependencies: ByPlatform<Vec<VersionPluginDependencies>>,
	pub platform_dependencies: ByPlatform<Vec<String>>,
	pub platform_dependencies_formatted: ByPlatform<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionStats {
	pub total_downloads: i64,
	pub platform_downloads: ByPlatform<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ByPlatform<T> {
	#[serde(rename = "PAPER")]
	pub paper: Option<T>,
	#[serde(rename = "WATERFALL")]
	pub waterfall: Option<T>,
	#[serde(rename = "VELOCITY")]
	pub velocity: Option<T>,
}

impl<T> ByPlatform<T> {
	pub fn get(&self, platform: Platform) -> Option<&T> {
		match platform {
			Platform::Paper => self.paper.as_ref(),
			Platform::Waterfall => self.waterfall.as_ref(),
			Platform::Velocity => self.velocity.as_ref(),
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (Platform, &T)> {
		self.paper
			.iter()
			.map(|v| (Platform::Paper, v))
			.chain(self.waterfall.iter().map(|v| (Platform::Waterfall, v)))
			.chain(self.velocity.iter().map(|v| (Platform::Velocity, v)))
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewState {
	Unreviewed,
	Reviewed,
	UnderReview,
	PartiallyReviewed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
	#[serde(deserialize_with = "time::serde::rfc3339::deserialize")]
	pub created_at: OffsetDateTime,
	pub name: String,
	pub description: Option<String>,
	pub color: String,
	pub flags: Vec<ChannelFlags>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelFlags {
	Frozen,
	Unstable,
	Pinned,
	SendsNotifications,
	HideByDefault,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PinnedStatus {
	None,
	Version,
	Channel,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VersionDownloads {
	#[serde(rename_all = "camelCase")]
	Internal {
		file_info: VersionDownloadsFileInfo,
		/// Hangar download url if not an external download
		download_url: String,
	},
	#[serde(rename_all = "camelCase")]
	External {
		/// External download url if not directly uploaded to Hangar
		external_url: String,
	},
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionDownloadsFileInfo {
	pub name: String,
	pub size_bytes: i64,
	pub sha256_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionPluginDependencies {
	/// Name of the plugin dependency. For non-external dependencies, this should be the Hangar project name
	pub name: String,
	/// Whether the dependency is required for the plugin to function
	pub required: bool,
	/// External url to download the dependency from if not a Hangar project, else null
	pub external_url: Option<String>,
	/// Server platform
	pub platform: Platform,
}
