use crate::config_types::ThemeColors;
use anyhow::Context;
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::DocumentMut;

pub const THEMES_DIR_NAME: &str = "themes";

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeFileSpec {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_dark: bool,
    pub colors: ThemeColors,
}

#[derive(Debug, Deserialize)]
struct ThemeFileDoc {
    #[serde(default)]
    id: Option<String>,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    is_dark: Option<bool>,
    #[serde(default)]
    colors: ThemeColors,
}

pub fn themes_dir(code_home: &Path) -> PathBuf {
    code_home.join(THEMES_DIR_NAME)
}

pub fn list_theme_files(code_home: &Path) -> anyhow::Result<Vec<ThemeFileSpec>> {
    let dir = themes_dir(code_home);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("create themes directory at {}", dir.display()))?;

    let mut paths = std::fs::read_dir(&dir)
        .with_context(|| format!("read themes directory at {}", dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
        })
        .collect::<Vec<_>>();

    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut out = Vec::new();
    for path in paths {
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(error) => {
                tracing::warn!("failed to read theme file {}: {error}", path.display());
                continue;
            }
        };

        let parsed = match toml::from_str::<ThemeFileDoc>(&contents) {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::warn!("failed to parse theme file {}: {error}", path.display());
                continue;
            }
        };

        let fallback_id = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(slugify_theme_id)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| "custom-theme".to_string());
        let id = parsed
            .id
            .as_deref()
            .map(slugify_theme_id)
            .filter(|value| !value.is_empty())
            .unwrap_or(fallback_id);

        let name = parsed.name.trim().to_string();
        if name.is_empty() {
            tracing::warn!("skipping theme file {} with empty name", path.display());
            continue;
        }

        out.push(ThemeFileSpec {
            id,
            name,
            description: parsed
                .description
                .as_deref()
                .map(str::trim)
                .filter(|description| !description.is_empty())
                .unwrap_or("Custom theme")
                .to_string(),
            is_dark: parsed.is_dark.unwrap_or(false),
            colors: parsed.colors,
        });
    }

    Ok(out)
}

pub fn write_theme_file(
    code_home: &Path,
    spec: &ThemeFileSpec,
    overwrite_existing: bool,
) -> anyhow::Result<PathBuf> {
    let dir = themes_dir(code_home);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("create themes directory at {}", dir.display()))?;

    let id = slugify_theme_id(&spec.id);
    let file_name = format!("{id}.toml");
    let path = dir.join(file_name);
    if path.exists() && !overwrite_existing {
        return Ok(path);
    }

    let mut doc = DocumentMut::new();
    doc["id"] = toml_edit::value(id);
    doc["name"] = toml_edit::value(spec.name.trim().to_string());
    doc["description"] = toml_edit::value(spec.description.trim().to_string());
    doc["is_dark"] = toml_edit::value(spec.is_dark);

    let mut colors_table = toml_edit::Table::new();
    colors_table.set_implicit(false);
    set_theme_color_value(&mut colors_table, "primary", &spec.colors.primary);
    set_theme_color_value(&mut colors_table, "secondary", &spec.colors.secondary);
    set_theme_color_value(&mut colors_table, "background", &spec.colors.background);
    set_theme_color_value(&mut colors_table, "foreground", &spec.colors.foreground);
    set_theme_color_value(&mut colors_table, "border", &spec.colors.border);
    set_theme_color_value(
        &mut colors_table,
        "border_focused",
        &spec.colors.border_focused,
    );
    set_theme_color_value(&mut colors_table, "selection", &spec.colors.selection);
    set_theme_color_value(&mut colors_table, "cursor", &spec.colors.cursor);
    set_theme_color_value(&mut colors_table, "success", &spec.colors.success);
    set_theme_color_value(&mut colors_table, "warning", &spec.colors.warning);
    set_theme_color_value(&mut colors_table, "error", &spec.colors.error);
    set_theme_color_value(&mut colors_table, "info", &spec.colors.info);
    set_theme_color_value(&mut colors_table, "text", &spec.colors.text);
    set_theme_color_value(&mut colors_table, "text_dim", &spec.colors.text_dim);
    set_theme_color_value(&mut colors_table, "text_bright", &spec.colors.text_bright);
    set_theme_color_value(&mut colors_table, "keyword", &spec.colors.keyword);
    set_theme_color_value(&mut colors_table, "string", &spec.colors.string);
    set_theme_color_value(&mut colors_table, "comment", &spec.colors.comment);
    set_theme_color_value(&mut colors_table, "function", &spec.colors.function);
    set_theme_color_value(&mut colors_table, "spinner", &spec.colors.spinner);
    set_theme_color_value(&mut colors_table, "progress", &spec.colors.progress);
    doc["colors"] = toml_edit::Item::Table(colors_table);

    std::fs::write(&path, doc.to_string())
        .with_context(|| format!("write theme file {}", path.display()))?;
    Ok(path)
}

pub fn slugify_theme_id(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    let mut prev_dash = false;
    for ch in input.chars() {
        let normalized = if ch.is_ascii_alphanumeric() {
            Some(ch.to_ascii_lowercase())
        } else {
            None
        };

        if let Some(value) = normalized {
            slug.push(value);
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }
    while slug.starts_with('-') {
        slug.remove(0);
    }

    if slug.is_empty() {
        "custom-theme".to_string()
    } else {
        slug
    }
}

fn set_theme_color_value(table: &mut toml_edit::Table, key: &str, value: &Option<String>) {
    if let Some(color) = value.as_deref().map(str::trim).filter(|color| !color.is_empty()) {
        table.insert(key, toml_edit::value(color));
    }
}
