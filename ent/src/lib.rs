//! To conveniently keep the AerynOS recipes repository up to date, it is
//! important to have a way to check whether recipes are outdated or whether
//! they have CVEs that need addressing.
//!
//! The over-arching design goal of the present solution is to make it
//! convenient to create different "views" of a cached set of data representing
//! our recipes repository.
//!
//! Therefore, the idea with this code is that we create RecipeMonitor structs
//! for each recipe in the recipes repository and cache those.
//!
//! For each of these RecipeMonitors, we then fetch the relevant upstream data
//! and save that as an <Upstream>Monitor. This could and should be extended to
//! also cover CVE information.
//!
//! <Upstream> can be either Release, Rss or Cve, such that we can use multiple
//! sources for fallback purposes. This could be useful for private/internal
//! repos that may not be publicly available and thus not have a public
//! release-monitoring id.
//!
//! Each class of collected Monitor data is written atomically to one database
//! file per run. This means that we can build "views" that are only depend on
//! database files being *present*. This makes for an efficient WORM scenario.
//!
//! In addition, assuming that the database format is well defined, it also
//! means that consumers can use either ent-generated JSON output XOR
//! deserialise the database schema on their own.
//!
//! Implementation wise, we have decided to use D.J. Bernsteins CDB format.
//! This is a very efficient file-backed key value store, that is written
//! atomically to disk in a single pass.
//!
//! Hence, to achieve the goals stated above, the idea is to make the CDB cache
//! directly iterable and searchable via async, such that we can search for
//! (and stream) both all entries and all entries matching the relevant keys
//!
//! To ensure that we can update the on-disk format, the format of each set of
//! CDB files will thus need to be versioned, as the serialisation routine is
//! responsible for serialising the correct number of (ordered) fields into a
//! single struct per format version.
//!
//! It is possible that we will want to generate quick reverse lookup tables
//! as well, though we may also be able to exploit the fact that CDB allows
//! for one key to be present multiple times with different values.

/// Q: Would it make sense to update each DB independently?
/// A: If it did, we would need to have the refresh keyed to the upstream
///    git ref? I.e. perhaps there is value in being able to refresh *to*
///    a git ref, recipe tree cache wise?
///    Then the respective release-, rss-, and cve-monitoring db caches
///    can also be updated independently?
///    This would effectively enable different update cadences for the
///    tree and for the respective upstream db caches?

/// Q: As a corollary to the above, could this proposed design actually
///    be used to save "monitoring state" for the associated channel's
///    history identifiers in terms of "this is the known state of
///    the moss-format repo index for this identifier?"
/// A: Yes. If we split out the ability to cache the recipes tree vs.
///    the individual upstream monitors, we could conveniently generate
///    reports for future LTS scenarios in terms of CVEs, by simply
///    reusing the newest upstream monitor cache dbs and creating views
///    against each tagged (= permanent) history identifier.
///    This would serve as an efficient means to track multiple "releases"
///    against fresh upstream/rss/cve caches?

/// Will be read from a checked out clone on each refresh
#[derive(Debug)]
pub struct RecipeMonitor {
    pub recipe_name: String,
    pub current_version: String,
    /// to be used for lookups in ReleaseMonitor DBs
    pub upstream_id: Some(u32),
    /// to be used for lookups in RssMonitor DBs
    pub rss_url: Some(String),
}

/// Only valid if an upstream id is found.
#[derive(Debug)]
pub struct ReleaseMonitor {
    pub upstream_id: u32,
    pub upstream_name: String,
    pub latest_version: String,
}

/// Only valid if an upstream rss feed is found.
///
/// The idea is that we can parse the latest_version from this feed.
#[derive(Debug)]
pub struct RssMonitor {
    pub rss_url: String,
    pub latest_version: String,
}

/// Need to do some more research on this
// pub struct CveMonitor {
// }

/// query the upstream db, create structs, stream structs to cache
fn build_cache() {}
/// stream structs from cache (run build_cache if it does not exist)
fn read_cache() {}
/// prepare a collection of updatable recipes
fn compute_updates() {}
/// show the collection of updatable recipes
fn show_updates() {}
