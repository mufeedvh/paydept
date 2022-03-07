use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::collections::HashMap;

use url::Url;
use regex::Regex;
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use yaml_rust::{YamlLoader, Yaml};

/// List of support websites
static SUPPORT_WEBSITES: [&'static str; 11] = [
    "opencollective.com",
    "www.patreon.com",
    "patreon.com",
    "ko-fi.com",
    "tidelift.com",
    "www.buymeacoffee.com",
    "paypal.me",
    "otechie.com",
    "liberapay.com",
    "funding.communitybridge.org",
    "crowdfunding.lfx.linuxfoundation.org"
];

/// Filter for anomalies
static BLACKLIST: [&'static str; 2] = [
    "opencollective.com/debug",
    "tidelift.com/security"
];

/// HashMap to store the results
pub static DEP_SUPPORT_LIST: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::with_capacity(500))
});

/// Check if the URL's host is one of the support websites.
fn check_host(url: &str) -> bool {
    for value in BLACKLIST {
        if url.contains(value) {
            return false
        }
    }

    match Url::parse(url) {
        Ok(parsed_url) => {
            match parsed_url.host_str() {
                Some(host) => {
                    SUPPORT_WEBSITES.contains(&host)
                },
                None => false
            }
        },
        Err(_) => false
    }
}

/// Get the dependencies root directory name
fn get_dependency_name(path: &mut PathBuf, root: bool) -> Option<String> {
    // if pathbuf is currently reading a file under root
    // pop once, otherwise traverse directory twice.
    if root {
        path.pop();
    } else {
        path.pop();
        path.pop();
    }

    if path.file_name().is_some() {
        // this will definitely bite me later
        return Some(String::from(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
        ))
    } else {
        None
    }
}

/// Insert the dependency and it's respective support links to the HashMap.
fn insert_dependency(path: &mut PathBuf, url: String, root: bool) {
    let dep_name = get_dependency_name(path, root);

    match dep_name {
        Some(dep_name) => {
            DEP_SUPPORT_LIST.lock().insert(dep_name, url);
        },
        None => ()
    }
}

/// Construct support URLs from FUNDING.yml keywords.
/// 
/// Follows [GitHub FUNDING.yml Format](https://docs.github.com/articles/displaying-a-sponsor-button-in-your-repository).
fn construct_support_url(key: &Yaml, value: &Yaml, urls: &mut Vec<String>) {
    match key {
        Yaml::String(platform) => {            
            let platform = platform.as_str();
            match platform {
                "github" => {
                    match value {
                        Yaml::Array(array) => {
                            for username in array {
                                urls.push(format!(
                                    "https://github.com/sponsors/{}",
                                    username.as_str().unwrap()
                                ))
                            }
                        },
                        _ => ()
                    }
                },
                "custom" => {
                    match value {
                        Yaml::Array(array) => {
                            for url in array {
                                urls.push(
                                    String::from(url.as_str().unwrap())
                                )
                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            }

            let value_str = match value.as_str() {
                Some(value) => value,
                None => return
            };

            match platform {
                "patreon" => {
                    urls.push(format!("https://www.patreon.com/{}", value_str))
                },
                "open_collective" => {
                    urls.push(format!("https://opencollective.com/{}", value_str))
                },
                "ko_fi" => {
                    urls.push(format!("https://ko-fi.com/{}", value_str))
                },
                "tidelift" => {
                    urls.push(format!("https://tidelift.com/funding/github/{}", value_str))
                },                
                "community_bridge" => {
                    urls.push(format!("https://funding.communitybridge.org/projects/{}", value_str))
                },
                "liberapay" => {
                    urls.push(format!("https://liberapay.com/{}", value_str))
                },
                "issuehunt" => {
                    urls.push(format!("https://issuehunt.io/r/{}", value_str))
                },
                "otechie" => {
                    urls.push(format!("https://otechie.com/{}", value_str))
                },
                "lfx_crowdfunding" => {
                    urls.push(format!("https://crowdfunding.lfx.linuxfoundation.org/projects/{}", value_str))
                },
                _ => ()
            }
        },
        _ => ()
    }
}

/// Get values from the FUNDING.yml file.
fn parse_funding_yaml(file_c: &str) -> Option<Vec<String>> {
    let mut support_urls: Vec<String> = Vec::with_capacity(10);
    
    match YamlLoader::load_from_str(file_c) {
        Ok(yaml) => {
            for value in yaml.clone() {
                match value {
                    Yaml::Hash(values) => {
                        for (key, value) in values.iter() {
                            construct_support_url(
                                key,
                                value,
                                &mut support_urls
                            )
                        }
                    },
                    _ => ()
                }
            }
        },
        Err(_) => return None
    };

    Some(support_urls)
}

/// Read all FUNDING.yml files to get support links.
pub fn funding_yml_files(files: Vec<PathBuf>) -> Result<()> {
    for mut file in files {
        if file.is_file() {
            let read_file = fs::read(file.clone())?;
            let file_contents = String::from_utf8_lossy(&read_file);

            let support_urls = parse_funding_yaml(&file_contents);

            match support_urls {
                Some(urls) => {
                    if !urls.is_empty() {
                        insert_dependency(&mut file, urls.join("\n"), false)
                    }
                },
                None => ()
            }
        }
    }

    Ok(())
}

/// Parse and sort support URLs in README files.
pub fn parse_urls(files: Vec<PathBuf>) -> Result<()> {
    let url_regex = Regex::new(
        r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
    ).unwrap();

    for mut file in files {
        if file.is_file() {
            let read_file = fs::read(file.clone())?;
            let file_contents = String::from_utf8_lossy(&read_file);

            for url in url_regex.captures_iter(&file_contents) {
                if check_host(&url[0]) {
                    let mut url = String::from(&url[0]);
                    // links from markdown consume ')' and '.'
                    while url.ends_with(')') || url.ends_with('.') {
                        url.pop();
                    }

                    insert_dependency(&mut file, url, true)
                }
            }
        }
    }

    Ok(())
}