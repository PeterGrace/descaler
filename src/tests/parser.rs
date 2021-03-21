#[cfg(test)]
use crate::assert_err;
#[cfg(test)]
use crate::lib::config::AppConfig;

#[test]
fn test_app_config_invalid_yaml() {
    let test_string = String::from("@#%@#^@#^%^$%#^@#$@#$%^");
    let obj = serde_yaml::from_str::<AppConfig>(test_string.as_str());
    assert_err!(obj, Err(serde_yaml::Error { .. }));
}
#[test]
fn test_app_config_valid_yaml() {
    const input_data: &str = "
---
source_url: 'http://localhost'
check_interval: 10
enumerate_nodes_secs: 30
enumerate_scalers_secs: 30
";
    let value = serde_yaml::from_str::<AppConfig>(input_data).unwrap();
    let control: AppConfig = AppConfig {
        source_url: String::from("http://localhost"),
        check_interval: 10,
        enumerate_nodes_secs: 30,
        enumerate_scalers_secs: 30,
    };
    assert_eq!(value, control)
}
