//! Markup building blocks shared by the framework crates, so every framework
//! renders the same board. Classes are Tailwind + DaisyUI; `safelist.html` in
//! each framework crate lists them for Tailwind.

pub mod class {
    pub const LIST: &str = "flex flex-col gap-3";
    pub const CARD: &str = "card card-border bg-base-100";
    pub const CARD_BODY: &str = "card-body flex-row items-start gap-4 p-4";
    pub const VOTE: &str =
        "btn btn-outline btn-sm h-auto min-h-0 w-14 flex-col gap-0 py-1 aria-pressed:btn-primary";
    pub const TITLE: &str = "font-semibold leading-snug";
    pub const DESCRIPTION: &str = "mt-1 text-sm text-base-content/70 line-clamp-3";
    pub const META: &str = "mt-2 flex flex-wrap items-center gap-1";
    pub const BADGE: &str = "badge badge-sm badge-ghost";
    pub const COLUMNS: &str = "grid gap-6 md:grid-cols-3";
    pub const COLUMN_TITLE: &str = "mb-3 flex items-center gap-2 text-sm font-semibold uppercase tracking-wide text-base-content/60";
    pub const EMPTY: &str = "rounded-box border border-dashed border-base-300 p-6 text-center text-sm text-base-content/50";
    pub const FEED: &str = "flex flex-col gap-12";
    pub const ENTRY_META: &str = "flex flex-wrap items-center gap-2 text-sm text-base-content/60";
    pub const ENTRY_TITLE: &str = "mt-2 text-2xl font-bold";
    pub const PROSE: &str = "prose mt-4 max-w-none";
    pub const ARROW_PATH: &str = "m18 15-6-6-6 6";
    pub const CREDIT: &str = "mt-6 text-right text-xs text-base-content/50";
    pub const CREDIT_LINK: &str = "link link-hover";
}

/// Target of the "Powered by SeggWat" credit under each component.
pub const CREDIT_URL: &str = "https://seggwat.com/?utm_source=seggwat-sdk&utm_medium=board";

/// The credit ships by default on every site using the SDK, which is what
/// Google's link-spam policy calls a widget link, so it is not followed. Sites
/// that want a followed link hide the credit and render their own.
pub const CREDIT_REL: &str = "nofollow noopener";

/// Display label for an idea status.
pub fn status_label(status: &str) -> &str {
    match status {
        "Started" => "In progress",
        "Completed" => "Shipped",
        other => other,
    }
}

/// Label and DaisyUI badge class for a changelog category slug.
pub fn category_badge(category: &str) -> (&'static str, &'static str) {
    match category {
        "added" => ("Added", "badge-success"),
        "improved" => ("Improved", "badge-info"),
        "fixed" => ("Fixed", "badge-warning"),
        "changed" => ("Changed", "badge-neutral"),
        "removed" => ("Removed", "badge-error"),
        "security" => ("Security", "badge-error"),
        _ => ("Update", "badge-ghost"),
    }
}
