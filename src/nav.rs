//! The site's navigation: the sidebar groups, and the previous and next page
//! for each page. Both come from one ordered list, so the sidebar and the pager
//! always agree.

use serde::Serialize;

use crate::SITE_URL;

/// A link is true when it exists, so `@if ($pager.prev)` reads an `Option<Link>`.
#[derive(Debug, Clone, Serialize, propel_macros::Truthy)]
pub struct Link {
    pub href: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct Group {
    pub heading: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize)]
pub struct Pager {
    pub prev: Option<Link>,
    pub next: Option<Link>,
}

/// Every page in the sidebar, in order. Previous and next follow this order.
/// The groups are the sections of the site: getting started, the models and
/// queries, the schema, the advanced API, and the comparison with Laravel.
fn groups() -> Vec<Group> {
    let group = |heading: &str, links: &[(&str, &str)]| Group {
        heading: heading.into(),
        links: links
            .iter()
            .map(|(href, label)| Link {
                href: (*href).into(),
                label: (*label).into(),
            })
            .collect(),
    };

    vec![
        group(
            "Getting started",
            &[
                ("/docs/introduction", "Introduction"),
                ("/docs/install", "Installation"),
                ("/docs/connecting", "Connecting"),
            ],
        ),
        group(
            "Models and queries",
            &[
                ("/docs/models", "Models"),
                ("/docs/queries", "Queries"),
                ("/docs/transactions", "Transactions"),
                ("/docs/errors", "Errors"),
            ],
        ),
        group("Schema", &[("/docs/migrations", "Migrations")]),
        group("Advanced", &[("/docs/blocking", "Blocking API")]),
        group(
            "Reference",
            &[("/docs/differences", "Differences from Laravel")],
        ),
    ]
}

/// The address of every page in the sidebar, in order. The sitemap lists them.
pub fn paths() -> Vec<String> {
    groups()
        .into_iter()
        .flat_map(|group| group.links)
        .map(|link| link.href)
        .collect()
}

/// The sidebar title of the page at `path`, if the page is in the sidebar.
pub fn title(path: &str) -> Option<String> {
    groups()
        .into_iter()
        .flat_map(|group| group.links)
        .find(|link| link.href == path)
        .map(|link| link.label)
}

/// The context of every documentation page. The layout reads `title`, `nav` and
/// `pager` as top-level fields, because a compiled view reads each field as
/// `ctx.field`.
#[derive(Debug, Serialize)]
pub struct DocPage {
    pub title: String,
    pub nav: Vec<Group>,
    pub pager: Pager,
    /// The page's absolute address, for the canonical and Open Graph URLs.
    pub canonical: String,
}

impl DocPage {
    /// `path` is the page's address. Only pages in the sidebar get a previous
    /// and a next link.
    pub fn new(path: &str, title: &str) -> Self {
        let nav = groups();
        let links: Vec<Link> = nav.iter().flat_map(|group| group.links.clone()).collect();
        let index = links.iter().position(|link| link.href == path);

        let pager = Pager {
            prev: index
                .and_then(|i| i.checked_sub(1))
                .map(|i| links[i].clone()),
            next: index.and_then(|i| links.get(i + 1).cloned()),
        };

        // The error page has no address of its own, so its canonical is the home.
        let canonical = if path.is_empty() {
            format!("{SITE_URL}/")
        } else {
            format!("{SITE_URL}{path}")
        };

        DocPage {
            title: title.into(),
            nav,
            pager,
            canonical,
        }
    }
}
