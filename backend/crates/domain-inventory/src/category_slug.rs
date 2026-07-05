/// Slugifies a category name for URL-safe identifiers (kebab-case).
pub fn slugify_name(name: &str) -> String {
    let mut out = String::new();
    let mut last_hyphen = false;

    for ch in name.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_hyphen = false;
        } else if !last_hyphen && !out.is_empty() {
            out.push('-');
            last_hyphen = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    out
}

/// Picks a unique slug by appending `-2`, `-3`, … when `base` is taken.
pub fn unique_slug(base: &str, taken: &[String]) -> String {
    if base.is_empty() {
        return String::new();
    }
    if !taken.iter().any(|slug| slug == base) {
        return base.to_owned();
    }
    for index in 2.. {
        let candidate = format!("{base}-{index}");
        if !taken.iter().any(|slug| slug == &candidate) {
            return candidate;
        }
    }
    base.to_owned()
}

#[cfg(test)]
mod tests {
    use super::{slugify_name, unique_slug};

    #[test]
    fn given_name_when_slugify_then_kebab_case() {
        assert_eq!(slugify_name("Bebidas Geladas"), "bebidas-geladas");
    }

    #[test]
    fn given_taken_slug_when_unique_then_suffix() {
        let taken = vec!["bebidas".into(), "bebidas-2".into()];
        assert_eq!(unique_slug("bebidas", &taken), "bebidas-3");
    }
}
