use std::{
    io::{Bytes, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use console::style;
use directories::ProjectDirs;
use mdget::{
    Config, Dependencies, File, MinecraftVersions, ModrinthProject, ProjectVersion, ProjectVersions,
};
use serde_json::Value;
use sha2::{Digest, Sha256, Sha512};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path: std::path::PathBuf;
    let mut data_dir: std::path::PathBuf;

    if let Some(proj_dirs) = ProjectDirs::from("dev", "kalkafox", "mdget") {
        config_path = proj_dirs.config_dir().to_path_buf();
        data_dir = proj_dirs.data_dir().to_path_buf();
    } else {
        return Err("Failed to get project directories!".into());
    }

    // Create config directory if it doesn't exist
    if !config_path.exists() {
        std::fs::create_dir_all(&config_path)?;
    }

    // Create data directory if it doesn't exist
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }
    // First thing to do: Check if we're connected to the internet!
    mdget::cinfoln(" Getting Minecraft versions from API...");

    // timeout after 5 seconds
    let timeout = std::time::Duration::from_secs(5);

    let mut headers = reqwest::header::HeaderMap::new();

    // User-Agent: kalkafox/mdget/0.1.0
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(mdget::USER_AGENT),
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(timeout)
        .default_headers(headers.clone())
        .build()?;
    let resp = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send();

    match resp {
        Ok(_) => {
            mdget::cinfoln(" Done!");
        }
        Err(_) => {
            mdget::cerrorln("Failed to connect to the internet! Make sure you're connected :(");
            return Ok(());
        }
    }

    let resp = resp?.json::<MinecraftVersions>()?;

    // Read the config file
    config_path.push("config.toml");

    if !config_path.exists() {
        // Create the config file if it doesn't exist
        let config: Config = Config {
            version: "1.16.5".to_string(),
            loader: "fabric".to_string(),
        };

        // serialize the config to toml
        let toml = toml::to_string(&config)?;

        // write the config to the config file
        std::fs::write(&config_path, toml)?;
    }

    let config = std::fs::read_to_string(&config_path)?;

    let toml = toml::from_str::<Config>(&config);

    match toml {
        Ok(_) => {}
        Err(_) => {
            mdget::cerrorln("Failed to parse config file!");
            // Move the config file to config.toml.old
            let mut old_config_path = config_path.clone();
            old_config_path.set_extension("old");
            std::fs::rename(&config_path, &old_config_path)?;
            let mut config: Config = Config {
                version: "1.16.5".to_string(),
                loader: "fabric".to_string(),
            };

            // attempt to read the old config file
            let old_config = std::fs::read_to_string(&old_config_path)?;

            let old_toml = toml::from_str::<toml::Value>(&old_config)?;

            if let Some(version) = old_toml.get("version") {
                config.version = version.as_str().unwrap().to_string();
            }

            if let Some(loader) = old_toml.get("loader") {
                config.loader = loader.as_str().unwrap().to_string();
            }

            // serialize the config to toml
            let toml = toml::to_string(&config)?;

            // write the config to the config file
            std::fs::write(&config_path, toml)?;
        }
    }

    let mut config = toml?;

    // Parse the config file using toml

    mdget::cinfoln("Hello, world!");

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        mdget::cinfoln("Usage: mdget <command> [args]");
        return Ok(());
    }

    match args[1].as_str() {
        "version" => {
            if args.len() == 2 {
                mdget::cinfoln(format!("Current version: {}", config.version).as_str());
                return Ok(());
            }
            match args[2].as_str() {
                "set" => {
                    if args.len() < 3 {
                        mdget::cerrorln("Usage: mdget version set <version>");
                        return Ok(());
                    }

                    // Check if the version is valid
                    let version = resp.versions.iter().find(|v| v.id == args[3]);

                    if version.is_none() {
                        mdget::cerrorln(format!("Invalid version {}!", args[3]).as_str());
                        return Ok(());
                    }

                    let version = version.unwrap();

                    mdget::cinfoln(format!("Setting version to {}", version.id).as_str());

                    config.version = version.id.to_string();
                }
                "get" => {
                    mdget::cinfo(format!("Current version: {}", config.version).as_str());
                    return Ok(());
                }
                _ => {}
            }
        }
        "mod" => {
            let mods = std::env::args()
                .skip(2)
                .filter(|arg| !arg.starts_with("-"))
                .collect::<Vec<String>>();

            // Set terminal raw mode

            mdget::cinfo("Collected mods: ");

            mods.iter()
                .for_each(|modid| print!("{} ", style(modid).fg(console::Color::Cyan).bold()));

            println!();

            if mods.len() == 0 {
                mdget::cerror("Usage: mdget mod <modid> [modid...]");
                return Ok(());
            }

            mdget::cinfoln("Preparing to query Modrinth API...");

            let mut mods_data = Vec::new();

            mods.iter().for_each(|modid| {
                let mod_url = format!("{}/project/{}", mdget::API_URL, modid);
                let dependencies_url = format!("{}/project/{}/dependencies", mdget::API_URL, modid);
                let resp = client.get(mod_url.as_str()).headers(headers.clone()).send();

                match resp {
                    Ok(_) => {}
                    Err(_) => {
                        mdget::cerrorln(format!("Failed to query mod {}!", modid).as_str());
                        return;
                    }
                }

                let resp = resp.unwrap();

                if resp.status().is_success() {
                    let data = resp.json::<ModrinthProject>().unwrap();

                    let exists = data.game_versions.iter().any(|v| v == &config.version);

                    if !exists {
                        mdget::cerrorln(
                            format!("Mod {} doesn't support version {}!", modid, config.version)
                                .as_str(),
                        );
                        return;
                    }

                    mods_data.push(data);

                    let res = client
                        .get(dependencies_url.as_str())
                        .headers(headers.clone())
                        .send();

                    match res {
                        Ok(_) => {}
                        Err(_) => {
                            mdget::cerrorln(format!("Failed to query mod {}!", modid).as_str());
                            return;
                        }
                    }

                    let res = res.unwrap();

                    if res.status().is_success() {
                        let data = res.json::<Dependencies>().unwrap();

                        if data.projects.is_empty() {
                            mdget::cinfoln("No dependencies!");
                            return;
                        }

                        data.projects.iter().for_each(|dep| {
                            let dep_url = format!("{}/project/{}", mdget::API_URL, dep.id);
                            let resp = client.get(dep_url.as_str()).headers(headers.clone()).send();

                            match resp {
                                Ok(_) => {}
                                Err(_) => {
                                    mdget::cerrorln(
                                        format!("Failed to query mod {}!", modid).as_str(),
                                    );
                                    return;
                                }
                            }

                            let resp = resp.unwrap();

                            if resp.status().is_success() {
                                let data = resp.json::<ModrinthProject>().unwrap();

                                let exists =
                                    data.game_versions.iter().any(|v| v == &config.version);

                                if !exists {
                                    mdget::cerrorln(
                                        format!(
                                            "Mod {} doesn't support version {}!",
                                            modid, config.version
                                        )
                                        .as_str(),
                                    );
                                    return;
                                }

                                mods_data.push(data);
                            } else {
                                mdget::cerrorln(format!("Failed to query mod {}!", modid).as_str());
                            }
                        });
                    }
                } else {
                    mdget::cerrorln(format!("Failed to query mod {}!", modid).as_str());
                }
            });

            mods_data.iter().for_each(|data| {
                mdget::cinfoln(format!("Downloading mod {}...", data.title).as_str());

                let version_url = format!("{}/project/{}/version", mdget::API_URL, data.id);

                let resp = client
                    .get(version_url.as_str())
                    .headers(headers.clone())
                    .send();

                match resp {
                    Ok(_) => {}
                    Err(_) => {
                        mdget::cerrorln(format!("Failed to query mod {}!", data.title).as_str());
                        return;
                    }
                }

                let resp = resp.unwrap();

                let version_data = resp.json::<ProjectVersions>();

                match version_data {
                    Ok(_) => {}
                    Err(err) => {
                        mdget::cerrorln(format!("Failed to query mod {}!", data.title).as_str());
                        mdget::cerrorln(format!("{}", err).as_str());
                        return;
                    }
                }

                let version_data = version_data.unwrap();

                let versions: Vec<&ProjectVersion> = version_data
                    .iter()
                    .filter(|v| {
                        v.game_versions.iter().any(|v| v == &config.version)
                            && v.loaders.iter().any(|l| l == &config.loader)
                    })
                    .collect();

                if versions.is_empty() {
                    mdget::cerrorln(
                        format!(
                            "Mod {} doesn't support version {} or loader {}!",
                            style(&data.title).cyan(),
                            style(&config.version).cyan(),
                            style(&config.loader).cyan()
                        )
                        .as_str(),
                    );
                    return;
                }

                // Get latest file
                let file = versions.iter().max_by_key(|v| &v.date_published);

                if file.is_none() {
                    mdget::cerrorln(
                        format!(
                            "Mod {} doesn't support version {}!",
                            data.title, config.version
                        )
                        .as_str(),
                    );
                    return;
                }

                let file = file.unwrap();

                let version_number = file.version_number.clone().unwrap();

                let file_url = &file.files[0].url;

                mdget::cinfoln(&format!(
                    "Downloading file: {}",
                    style(&file.files[0].filename).cyan()
                ));

                let mut hasher = Sha512::new();

                let resp = client.get(file_url).headers(headers.clone()).send();

                match resp {
                    Ok(_) => {}
                    Err(_) => {
                        mdget::cerrorln(format!("Failed to download file {}!", file_url).as_str());
                        return;
                    }
                }

                let mut resp = resp.unwrap();

                let file_data = resp.bytes();

                match file_data {
                    Ok(_) => {}
                    Err(_) => {
                        mdget::cerrorln(format!("Failed to download file {}!", file_url).as_str());
                        return;
                    }
                }

                let file_data = file_data.unwrap();

                hasher.update(&file_data);

                let hash = hasher.finalize();

                let hash = format!("{:x}", hash);

                mdget::cinfoln(
                    format!(
                        "Verifying file {} with hash {}...",
                        style(file_url).cyan(),
                        style(&hash[0..15]).green()
                    )
                    .as_str(),
                );

                if hash != file.files[0].hashes.sha512 {
                    mdget::cerrorln(format!("Failed to verify file {}!", file_url).as_str());
                    return;
                }

                // save to current working directory
                let mut path = std::env::current_dir().unwrap();
                path.push(&file.files[0].filename);

                let mut file = std::fs::File::create(&path).unwrap();

                file.write_all(&file_data).unwrap();
            });
        }
        _ => {
            mdget::cerrorln(format!("Invalid command {}!", style(&args[1]).red()).as_str());
        }
    }

    // Save the config file
    let toml = toml::to_string(&config)?;

    // write the config to the config file
    std::fs::write(&config_path, toml)?;

    Ok(())
}
