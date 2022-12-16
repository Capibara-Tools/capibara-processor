use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
struct YamlFunction {
    summary: Box<String>,
    returns: Box<String>,
    parameters: Vec<Parameter>,
    description: Box<String>,
    associated: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Function {
    name: Box<String>,
    header: HeaderSummary,
    summary: Box<String>,
    returns: Box<String>,
    parameters: Vec<Parameter>,
    description: Box<String>,
    associated: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct YamlHeader {
    os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Header {
    #[serde(rename = "ref")]
    _ref: Box<String>,
    name: Box<String>,
    os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HeaderSummary {
    #[serde(rename = "ref")]
    _ref: Box<String>,
    name: Box<String>,
    os_affinity: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Parameter {
    name: Box<String>,
    #[serde(rename = "type")]
    _type: Box<String>,
    description: Box<String>,
}

#[derive(Serialize, Deserialize)]
struct Document {
    build_date: String,
    reference_url: String,
    headers: Vec<Header>,
    /*
    enums: Vec<>,
    types: Vec<None>,
    */
    functions: Vec<Function>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).unwrap();
    let reference_url = args.get(2).unwrap();

    let header_paths = find_header_paths(filepath);
    let headers = discover_headers(filepath, header_paths.clone());
    let functions = discover_functions(filepath, header_paths.clone());

    let document = Document {
        build_date: chrono::Utc::now().to_rfc3339(),
        reference_url: reference_url.to_string(),
        headers: headers,
        functions: functions,
    };

    let document_json_result = serde_json::to_string(&document);

    match document_json_result {
        Ok(json) => std::fs::write("./capibara.json", json).unwrap(),
        Err(error) => {
            eprintln!("Document Error: {:?}", error);
        }
    }
}

fn find_header_paths(filepath: &String) -> Vec<String> {
    let mut header_paths = Vec::new();
    let mut visited_paths = Vec::new();
    let mut entries = fs::read_dir(filepath).unwrap();
    let mut last_path;
    let mut last_folder_path = String::new();

    loop {
        if let Some(Ok(path)) = entries.next() {
            let path_as_str = path.path().to_str().unwrap().clone().to_string();

            last_path = path_as_str.clone();

            if visited_paths.contains(&last_path) {
                continue;
            }

            visited_paths.push(last_path.clone());

            if path.path().is_dir() {
                last_folder_path = path_as_str.clone();
                entries = fs::read_dir(&path.path()).unwrap();
            } else if path.path().ends_with("meta.yaml") {
                header_paths.push(path_as_str.clone());
            }
        } else {
            if last_folder_path.as_str() == filepath.as_str() {
                break;
            }

            entries =
                fs::read_dir(PathBuf::from(&last_folder_path).as_path().parent().unwrap()).unwrap();

            last_folder_path = PathBuf::from(last_folder_path)
                .as_path()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
    }

    header_paths
}

fn discover_headers(filepath: &String, header_paths: Vec<String>) -> Vec<Header> {
    let mut headers_to_return = Vec::new();

    for header_path in header_paths {
        let path = header_path;

        let header;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(yaml) => {
                header = Header {
                    _ref: Box::new(
                        PathBuf::from(&path)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .trim_start_matches(&format!("{}/", filepath))
                            .to_string(),
                    ),
                    name: Box::new(
                        PathBuf::from(&path)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .trim_start_matches(&format!("{}/", filepath))
                            .to_string()
                            + ".h",
                    ),
                    os_affinity: yaml.os_affinity,
                };

                headers_to_return.push(header)
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return headers_to_return;
            }
        }
    }

    headers_to_return
}

fn discover_functions(filepath: &String, header_paths: Vec<String>) -> Vec<Function> {
    let mut functions_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(yaml) => {
                header_summary = HeaderSummary {
                    _ref: Box::new(
                        PathBuf::from(&path)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .trim_start_matches(&format!("{}/", filepath))
                            .to_string(),
                    ),
                    name: Box::new(
                        PathBuf::from(&path)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .trim_start_matches(&format!("{}/", filepath))
                            .to_string()
                            + ".h",
                    ),
                    os_affinity: yaml.os_affinity,
                }
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return functions_to_return;
            }
        }

        let header_parent = PathBuf::from(&path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let entries = fs::read_dir(header_parent).unwrap();

        for entry in entries {
            if let Ok(file_path) = entry {
                if file_path.file_name().to_str().unwrap().starts_with("fn-") {
                    let file_contents = std::fs::read_to_string(file_path.path()).unwrap();

                    let function_yaml_result = serde_yaml::from_str::<YamlFunction>(&file_contents);

                    match function_yaml_result {
                        Ok(yaml) => {
                            let function = Function {
                                name: Box::new(
                                    file_path
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                        .replace("fn-", ""),
                                ),
                                header: header_summary.clone(),
                                summary: yaml.summary,
                                returns: yaml.returns,
                                parameters: yaml.parameters,
                                description: yaml.description,
                                associated: yaml.associated,
                            };

                            functions_to_return.push(function);
                        }
                        Err(error) => eprintln!("Function Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    functions_to_return
}
