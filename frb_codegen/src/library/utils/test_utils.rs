use crate::utils::path_utils::path_to_string;
use log::debug;
use serde_json::Value;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub(crate) fn get_test_fixture_dir(fixture_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures")
        .join(fixture_name)
        .canonicalize()
        .unwrap()
}

/// "golden" means comparison tests
/// see, for example, https://api.flutter.dev/flutter/flutter_test/matchesGoldenFile.html
/// for more information
pub(crate) fn json_golden_test(
    actual: &Value,
    matcher_path: &Path,
    sanitizers: &[(String, String)],
) -> anyhow::Result<()> {
    let mut actual_str = serde_json::to_string_pretty(actual)?;
    for sanitizer in sanitizers.iter() {
        actual_str = actual_str.replace(&sanitizer.0, &sanitizer.1);
    }
    let actual: Value = serde_json::from_str(&actual_str)?;
    debug!("json_golden_test actual:\n{actual_str}");

    raw_golden_test(actual, &actual_str, matcher_path, |x| {
        Ok(serde_json::from_str(&x)?)
    })
}

pub(crate) fn text_golden_test(actual: String, matcher_path: &Path) -> anyhow::Result<()> {
    raw_golden_test(actual.clone(), &actual, matcher_path, Ok)
}

fn raw_golden_test<T, F>(
    actual: T,
    actual_str: &str,
    matcher_path: &Path,
    deserializer: F,
) -> anyhow::Result<()>
where
    T: Eq + Debug,
    F: Fn(String) -> anyhow::Result<T>,
{
    let expect = deserializer(if matcher_path.exists() {
        fs::read_to_string(matcher_path)?
    } else {
        "".to_string()
    })?;

    if enable_update_golden() {
        if actual != expect {
            debug!("write golden data");
            fs::write(matcher_path, actual_str)?;
        }
    } else {
        assert_eq!(actual, expect);
    }

    Ok(())
}

fn enable_update_golden() -> bool {
    let env_var = env::var("UPDATE_GOLDEN")
        .ok()
        .or_else(|| env::var("UPDATE_GOLDENS").ok())
        .unwrap_or_default()
        .to_lowercase();
    env_var == "true" || env_var == "1"
}

pub(crate) fn create_path_sanitizers(test_fixture_dir: &Path) -> Vec<(String, String)> {
    vec![
        ("\\\\".into(), "/".into()),
        (
            path_to_string(&test_fixture_dir)?.replace('\\', "/"),
            "{the-working-directory}".to_owned(),
        ),
    ]
}
