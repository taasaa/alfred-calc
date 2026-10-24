//! Alfred script-filter JSON output.

use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    pub items: Vec<Item>,
}

#[derive(Serialize)]
pub struct Item {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<String>,
    pub valid: bool,
}

impl Output {
    pub fn single(item: Item) -> Output {
        Output { items: vec![item] }
    }
}

impl Item {
    /// A conversion the user can copy or paste.
    pub fn result(title: String, subtitle: String, arg: String) -> Item {
        Item {
            title,
            subtitle: Some(subtitle),
            arg: Some(arg),
            valid: true,
        }
    }

    /// Informational row: shows guidance, cannot be actioned.
    pub fn info(title: String, subtitle: String) -> Item {
        Item {
            title,
            subtitle: Some(subtitle),
            arg: None,
            valid: false,
        }
    }

    /// A real failure: token unknown, categories mismatched, data unavailable.
    pub fn error(title: String, subtitle: String) -> Item {
        Item {
            title,
            subtitle: Some(subtitle),
            arg: None,
            valid: false,
        }
    }
}
