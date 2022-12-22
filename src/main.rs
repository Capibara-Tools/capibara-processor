use definitions::{
    _macro::{Macro, YamlMacro},
    _struct::{Struct, YamlStruct},
    enumeration::{Enumeration, YamlEnumeration},
    function::{Function, YamlFunction},
    header::{Header, HeaderSummary, YamlHeader},
    typedef::{Typedef, TypedefRef, YamlTypedef},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::PathBuf};
mod definitions;

#[derive(Serialize, Deserialize)]
struct Document {
    build_date: String,
    reference_url: String,
    headers: Vec<Header>,
    macros: Vec<Macro>,
    enums: Vec<Enumeration>,
    structs: Vec<Struct>,
    typedefs: Vec<Typedef>,
    functions: Vec<Function>,
}

fn main() {
    println!("Capibara Processor");
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).unwrap();
    println!("Filepath:\t\t{}", filepath);
    let reference_url = args.get(2).unwrap();
    println!("Reference URL:\t{}", reference_url);

    let mut os_affinities = HashMap::new();

    let header_paths = find_header_paths(filepath);
    println!("Found {} header paths", header_paths.len());
    let macros = discover_macros(filepath, header_paths.clone(), &mut os_affinities);
    println!("Found {} macros", macros.len());
    let enums = discover_enums(filepath, header_paths.clone(), &mut os_affinities);
    println!("Found {} enums", enums.len());
    let structs = discover_structs(filepath, header_paths.clone(), &mut os_affinities);
    println!("Found {} structs", structs.len());

    let typedefs = discover_typedefs(
        filepath,
        header_paths.clone(),
        &mut os_affinities,
        &enums,
        &structs,
    );
    println!("Found {} typedefs", typedefs.len());

    let functions = discover_functions(filepath, header_paths.clone(), &mut os_affinities);
    println!("Found {} functions", functions.len());

    let headers = discover_headers(filepath, header_paths.clone(), &mut os_affinities);
    println!("Found {} headers", headers.len());

    let document = Document {
        build_date: chrono::Utc::now().to_rfc3339(),
        reference_url: reference_url.to_string(),
        headers: headers,
        macros: macros,
        enums: enums,
        structs: structs,
        typedefs: typedefs,
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

fn discover_headers(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
) -> Vec<Header> {
    let mut headers_to_return = Vec::new();

    for header_path in header_paths {
        let path = header_path.clone();

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
                    summary: yaml.summary,
                    os_affinity: get_header_os_affinity(os_affinities, &header_path.clone()),
                };

                headers_to_return.push(header)
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return headers_to_return;
            }
        }
    }

    headers_to_return.sort_by_key(|k| k._ref.clone());

    headers_to_return
}

fn discover_functions(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
) -> Vec<Function> {
    let mut functions_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(_yaml) => {
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
                                os_affinity: yaml.os_affinity.clone(),
                            };

                            if os_affinities.contains_key(&path.clone()) {
                                let header_affinity = os_affinities.get_mut(&path).unwrap();
                                for value in yaml.os_affinity.iter() {
                                    let new_value = value.clone();
                                    if !header_affinity.contains(&new_value) {
                                        header_affinity.push(new_value);
                                    }
                                }
                            } else {
                                os_affinities.insert(path.clone(), yaml.os_affinity.clone());
                            }

                            functions_to_return.push(function);
                        }
                        Err(error) => eprintln!("Function Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    functions_to_return.sort_by_key(|k| k.name.clone());

    functions_to_return
}

fn discover_macros(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
) -> Vec<Macro> {
    let mut macros_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(_yaml) => {
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
                }
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return macros_to_return;
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
                if file_path.file_name().to_str().unwrap().starts_with("mo-") {
                    let file_contents = std::fs::read_to_string(file_path.path()).unwrap();

                    let enum_yaml_result = serde_yaml::from_str::<YamlMacro>(&file_contents);

                    match enum_yaml_result {
                        Ok(yaml) => {
                            let _macro = Macro {
                                name: Box::new(
                                    file_path
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                        .replace("mo-", ""),
                                ),
                                header: header_summary.clone(),
                                summary: yaml.summary,
                                kind: yaml.kind,
                                description: yaml.description,
                                os_affinity: yaml.os_affinity.clone(),
                            };

                            if os_affinities.contains_key(&path.clone()) {
                                let header_affinity = os_affinities.get_mut(&path).unwrap();
                                for value in yaml.os_affinity.iter() {
                                    let new_value = value.clone();
                                    if !header_affinity.contains(&new_value) {
                                        header_affinity.push(value.to_string());
                                    }
                                }
                            } else {
                                os_affinities.insert(path.clone(), yaml.os_affinity);
                            }

                            macros_to_return.push(_macro);
                        }
                        Err(error) => eprintln!("Macro Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    macros_to_return.sort_by_key(|k| k.name.clone());

    macros_to_return
}

fn discover_enums(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
) -> Vec<Enumeration> {
    let mut enums_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(_yaml) => {
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
                }
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return enums_to_return;
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
                if file_path.file_name().to_str().unwrap().starts_with("em-") {
                    let file_contents = std::fs::read_to_string(file_path.path()).unwrap();

                    let enum_yaml_result = serde_yaml::from_str::<YamlEnumeration>(&file_contents);

                    match enum_yaml_result {
                        Ok(yaml) => {
                            let enumeration = Enumeration {
                                name: Box::new(
                                    file_path
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                        .replace("em-", ""),
                                ),
                                header: header_summary.clone(),
                                summary: yaml.summary,
                                variants: yaml.variants,
                                description: yaml.description,
                                os_affinity: yaml.os_affinity.clone(),
                            };

                            if os_affinities.contains_key(&path.clone()) {
                                let header_affinity = os_affinities.get_mut(&path).unwrap();
                                for value in yaml.os_affinity.iter() {
                                    let new_value = value.clone();
                                    if !header_affinity.contains(&new_value) {
                                        header_affinity.push(value.to_string());
                                    }
                                }
                            } else {
                                os_affinities.insert(path.clone(), yaml.os_affinity);
                            }

                            enums_to_return.push(enumeration);
                        }
                        Err(error) => eprintln!("Enum Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    enums_to_return.sort_by_key(|k| k.name.clone());

    enums_to_return
}

fn discover_structs(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
) -> Vec<Struct> {
    let mut structs_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(_yaml) => {
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
                }
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return structs_to_return;
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
                if file_path.file_name().to_str().unwrap().starts_with("st-") {
                    let file_contents = std::fs::read_to_string(file_path.path()).unwrap();

                    let struct_yaml_result = serde_yaml::from_str::<YamlStruct>(&file_contents);

                    match struct_yaml_result {
                        Ok(yaml) => {
                            let _struct = Struct {
                                name: Box::new(
                                    file_path
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                        .replace("st-", ""),
                                ),
                                header: header_summary.clone(),
                                summary: yaml.summary,
                                fields: yaml.fields,
                                description: yaml.description,
                                os_affinity: yaml.os_affinity.clone(),
                            };

                            if os_affinities.contains_key(&path.clone()) {
                                let header_affinity = os_affinities.get_mut(&path).unwrap();
                                for value in yaml.os_affinity.iter() {
                                    let new_value = value.clone();
                                    if !header_affinity.contains(&new_value) {
                                        header_affinity.push(value.to_string());
                                    }
                                }
                            } else {
                                os_affinities.insert(path.clone(), yaml.os_affinity);
                            }

                            structs_to_return.push(_struct);
                        }
                        Err(error) => eprintln!("Struct Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    structs_to_return.sort_by_key(|k| k.name.clone());

    structs_to_return
}

fn discover_typedefs(
    filepath: &String,
    header_paths: Vec<String>,
    os_affinities: &mut HashMap<String, Vec<String>>,
    enums: &Vec<Enumeration>,
    structs: &Vec<Struct>,
) -> Vec<Typedef> {
    let mut typedefs_to_return = Vec::new();

    for header in header_paths {
        let path = header;

        let header_summary;

        let header_file_contents = std::fs::read_to_string(PathBuf::from(&path)).unwrap();
        let header_yaml_result = serde_yaml::from_str::<YamlHeader>(&header_file_contents);

        match header_yaml_result {
            Ok(_yaml) => {
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
                }
            }
            Err(error) => {
                eprintln!("Header Error: {:?}", error);
                return typedefs_to_return;
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
                if file_path.file_name().to_str().unwrap().starts_with("tf-") {
                    let file_contents = std::fs::read_to_string(file_path.path()).unwrap();

                    let typedef_yaml_result = serde_yaml::from_str::<YamlTypedef>(&file_contents);

                    match typedef_yaml_result {
                        Ok(yaml) => {
                            let mut associated_ref = TypedefRef::None(());

                            if &*yaml.associated_ref != "" {
                                let re = Regex::new(r"(.+)/(.+)").unwrap();
                                let caps = re.captures(&yaml.associated_ref).unwrap();

                                let header_ref = caps.get(1).map_or("", |m| m.as_str());
                                let definition_ref = caps.get(2).map_or("", |m| m.as_str());

                                for _enum in enums.iter() {
                                    if &*_enum.name == definition_ref
                                        && &*_enum.header._ref == header_ref
                                    {
                                        let cloned_enum = _enum.clone();
                                        associated_ref = TypedefRef::Enumeration(cloned_enum);
                                        break;
                                    }
                                }

                                if associated_ref == TypedefRef::None(()) {
                                    for _struct in structs.iter() {
                                        if &*_struct.name == definition_ref
                                            && &*_struct.header._ref == header_ref
                                        {
                                            let cloned_struct = _struct.clone();
                                            associated_ref = TypedefRef::Struct(cloned_struct);
                                            break;
                                        }
                                    }
                                }

                                if associated_ref == TypedefRef::None(()) {
                                    eprintln!(
                                        "Typedef associated_ref look up failed for: {}/{}",
                                        header_ref, definition_ref
                                    )
                                }
                            }

                            let typedef = Typedef {
                                name: Box::new(
                                    file_path
                                        .path()
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                        .replace("tf-", ""),
                                ),
                                header: header_summary.clone(),
                                summary: yaml.summary,
                                _type: yaml._type,
                                associated_ref: associated_ref,
                                description: yaml.description,
                                os_affinity: yaml.os_affinity.clone(),
                            };

                            if os_affinities.contains_key(&path.clone()) {
                                let header_affinity = os_affinities.get_mut(&path).unwrap();
                                for value in yaml.os_affinity.iter() {
                                    let new_value = value.clone();
                                    if !header_affinity.contains(&new_value) {
                                        header_affinity.push(value.to_string());
                                    }
                                }
                            } else {
                                os_affinities.insert(path.clone(), yaml.os_affinity);
                            }

                            typedefs_to_return.push(typedef);
                        }
                        Err(error) => eprintln!("Typedef Error ({:?}): {:?}", file_path, error),
                    }
                }
            }
        }
    }

    typedefs_to_return.sort_by_key(|k| k.name.clone());

    typedefs_to_return
}

fn get_header_os_affinity(
    os_affinities: &mut HashMap<String, Vec<String>>,
    header_path: &String,
) -> Vec<String> {
    let value = os_affinities.get_mut(header_path);

    if let Some(value) = value {
        return value.to_owned();
    } else {
        return Vec::new();
    }
}
