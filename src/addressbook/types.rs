//! Addressbook shared across all protocols.

use alloc::string::String;

/// An addressbook collection.
///
/// Strict least-common-denominator shape: only fields that are
/// first-class in every protocol the crate targets (vdir, CardDAV).
/// Partial-coverage fields (description, color, ctag) remain
/// `Option<String>` and are populated by the backends that know them.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct Addressbook {
    /// Backend-specific identifier.
    ///
    /// For vdir this is the final path segment (collection
    /// directory name); for CardDAV it is the last non-empty path
    /// segment of the collection URL.
    pub id: String,

    /// Human-readable display name.
    pub name: String,

    /// Free-form description, when the backend exposes it.
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,

    /// ASCII `#RRGGBB` color marker, when the backend exposes it.
    #[cfg_attr(feature = "serde", serde(default))]
    pub color: Option<String>,

    /// Collection state token (CTag, RFC 6578 §6.2), when the backend
    /// exposes it. Used to detect whether the addressbook content
    /// changed without listing every card.
    #[cfg_attr(feature = "serde", serde(default))]
    pub ctag: Option<String>,
}

/// Partial update applied to an [`Addressbook`].
///
/// Every field is optional: `None` means "leave untouched", `Some`
/// means "replace with this value" (including `Some(None)` to clear an
/// optional field).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct AddressbookDiff {
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<Option<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub color: Option<Option<String>>,
}
