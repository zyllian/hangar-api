use constcat::concat;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::object::*;

/// base url for normal api calls
const BASE_API_URL: &str = "https://hangar.papermc.io/api/v1";

/// Trait implemented on all request structs.
pub trait HangarRequest {
	/// Gets the URL this request should be sent to.
	fn url(&self) -> String;
}

/// Searches all the projects on Hangar, or for a single user. Requires the `view_public_info` permission.
#[derive(Debug, Default, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(into)))]
pub struct ProjectsRequest {
	/// Whether to prioritize the project with an exact name match if present
	prioritize_exact_match: Option<bool>,
	/// Pagination information
	#[builder(!default)]
	#[serde(flatten)]
	pagination: Pagination,
	/// Used to sort the result
	sort: Option<ProjectsSort>,
	/// A category to filter for
	category: Option<Category>,
	/// A platform to filter for
	platform: Option<Platform>,
	/// The author of the project
	owner: Option<String>,
	/// The query to use when searching
	query: Option<String>,
	/// A license to filter for
	license: Option<String>,
	/// A platform version to filter for
	version: Option<String>,
	/// A tag to filter for
	tag: Option<String>,
	/// The member of the project
	member: Option<String>,
}

impl HangarRequest for ProjectsRequest {
	fn url(&self) -> String {
		concat!(BASE_API_URL, "/projects").to_string()
	}
}

#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
	pub pagination: PaginationResponse,
	pub result: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationResponse {
	/// The maximum amount of items to return
	pub limit: i64,
	/// Where to start searching
	pub offset: i64,
	/// The total number of records
	pub count: i64,
}

/// Returns info on a specific project. Requires the `view_public_info` permission.
#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProjectRequest {
	/// The slug of the project to return
	#[serde(skip)]
	pub slug: String,
}

impl HangarRequest for ProjectRequest {
	fn url(&self) -> String {
		format!("{}/projects/{}", BASE_API_URL, self.slug)
	}
}

/// Returns a page of a project. Requires visibility of the page.
#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PageRequest {
	/// The slug of the project to return the page for
	#[serde(skip)]
	pub slug: String,
	/// The path of the page
	pub path: String,
}

impl HangarRequest for PageRequest {
	fn url(&self) -> String {
		format!("{}/pages/page/{}", BASE_API_URL, self.slug)
	}
}

/// Returns all versions of a project. Requires the `view_public_info` permission in the project or owning organization.
#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct VersionsRequest {
	/// The slug of the project to return versions for
	#[builder(!default)]
	#[serde(skip)]
	pub slug: String,
	#[builder(!default)]
	#[serde(flatten)]
	pub pagination: Pagination,
	/// Whether to include hidden-by-default channels in the result, defaults to try
	pub include_hidden_channels: Option<bool>,
	/// A name of a version channel to filter for
	pub channel: Option<String>,
	/// A platform name to filter for
	pub platform: Option<Platform>,
	/// A platform version to filter for
	pub platform_version: Option<String>,
}

impl HangarRequest for VersionsRequest {
	fn url(&self) -> String {
		format!("{}/projects/{}/versions", BASE_API_URL, self.slug)
	}
}

#[derive(Debug, Deserialize)]
pub struct VersionsResponse {
	pub pagination: PaginationResponse,
	pub result: Vec<Version>,
}

/// Returns a specific version of a project. Requires the `view_public_info` permission in the project or owning organization.
#[derive(Debug, Serialize, TypedBuilder)]
pub struct VersionRequest {
	/// The slug of the project to return the version for
	#[serde(skip)]
	pub slug: String,
	/// The name of the version to return
	#[serde(skip)]
	pub name: String,
}

impl HangarRequest for VersionRequest {
	fn url(&self) -> String {
		format!(
			"{}/projects/{}/versions/{}",
			BASE_API_URL, self.slug, self.name
		)
	}
}
