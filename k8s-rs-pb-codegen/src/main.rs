use std::{
    env, fs::{self, OpenOptions}, io::{self, Write}, path::{Path, PathBuf}, process
};

const BMAP_FIELDS: [&'static str; 13] = ["max", "min", "default", "defaultRequest", "maxLimitRequestRatio", "capacity", "allocatable",
"podFixed", "limits", "requests", "used", "hard", "allocatedResources"];

use protobuf::{
    descriptor::field_descriptor_proto::Type,
    reflect::{FieldDescriptor, MessageDescriptor},
};
use protobuf_codegen::{Customize, CustomizeCallback};

pub struct GenStruct;

impl CustomizeCallback for GenStruct {
    fn message(&self, message: &MessageDescriptor) -> Customize {
        if message.name() != "Quantity" && message.name() != "Time" && message.name() != "Duration" {
            if message.name() == "ContainerMetrics" {
                return Customize::default().before("#[derive(::serde::Deserialize, ::serde::Serialize)]\n#[serde(rename_all = \"snake_case\")]");
            }
            return Customize::default().before("#[derive(::serde::Deserialize, ::serde::Serialize)]\n#[serde(rename_all = \"snake_case\")]");
        }

        Customize::default()
    }

    fn field(&self, field: &FieldDescriptor) -> Customize {

        let version = env::var("version").unwrap();

        // if BMAP_FIELDS.contains(&field.proto().name()) && field.is_map() {
        //     return Customize::default().before("#[serde(with = \"crate::quantity_parse\")]\n#[serde(default)]")
        // }
        
        if BMAP_FIELDS.contains(&field.proto().name()) && field.is_map() {
            return Customize::default().before("#[serde(default)]")
        }

        if field.is_repeated() {
            return Customize::default().before("#[serde(default)]");
        }

        if field.proto().type_name().contains(".apimachinery.pkg.util.intstr.IntOrString") {
            let msg = format!("#[serde(with = \"crate::{}::intorstr\")]\n#[serde(default)]", version);

            return Customize::default()
                .before(&msg);
        }

        if field.proto().type_() == Type::TYPE_MESSAGE && field.is_singular() {
            let msg = format!("#[serde(with = \"crate::{}::MessageFieldDef\")]\n#[serde(default)]", &version);

            return Customize::default().before(&msg);
        }

        if field.is_map() {
            return Customize::default().before("#[serde(default)]");
        } 
        
        Customize::default()
    }

    fn special_field(&self, message: &MessageDescriptor, _field: &str) -> Customize {
        if message.name() != "Quantity" && message.name() != "Time" && message.name() != "Duration" {
            return Customize::default().before("#[serde(skip)]");
        }
        Customize::default()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} overwrite <true|false> <version>", args[0]);
        process::exit(1);
    }

    let version_dir = &args[3];

    std::env::set_var("version", version_dir);

    let path_str = format!("./k8s-rs-pb/src/{}", version_dir);
    let k8s_pb = Path::new(&path_str);

    let res = match args[2].as_str() {
        "true" | "t" => true,
        "false" | "f" => false,
        _ => {
            eprintln!("Invalid flag");
            process::exit(1);
        }
    };

    let base_path = Path::new("k8s-rs-pb/src").join(version_dir);

    let mut protos: Vec<PathBuf> = Vec::new();

    match res {
        true => {
            let mut mod_files: Vec<PathBuf> = Vec::new();
            overwrite_mods(k8s_pb, &mut mod_files, &version_dir)?;
            add_mods(k8s_pb, &mut mod_files)?;
        }
        false => {
            search_protos(&base_path, &mut protos)?;

            let custom = Customize::default()
                .btreemaps(true)
                .gen_mod_rs(false)
                .generate_accessors(true)
                .generate_getter(true)
                .inside_protobuf(false);

            for proto_file in &protos {
                let relative_path = proto_file.strip_prefix(&base_path)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_path_buf();
                
                let out_dir = base_path.join(&relative_path);
                fs::create_dir_all(&out_dir)?;

                protobuf_codegen::Codegen::new()
                    .protoc()
                    .includes(&[base_path.to_str().unwrap()])
                    .input(proto_file)
                    .out_dir(&out_dir)
                    .customize(custom.clone())
                    .customize_callback(GenStruct)
                    .run_from_script();

                let generated_rs = out_dir.join("generated.rs");
                let mod_rs = out_dir.join("mod.rs");
                fs::rename(generated_rs, mod_rs)?;
            }
        }
    }

    Ok(())
}

// fn overwrite_mods<'a>(path: &Path, mod_files: &'a mut Vec<PathBuf>) -> io::Result<()> {
//     for val in fs::read_dir(path)? {
//         let res = val?;
//         if res.file_type()?.is_dir() {
//             overwrite_mods(res.path().as_path(), mod_files)?
//         }
//         if res.file_name().to_str().unwrap() == "mod.rs" {
//             let res_p = res.path().clone();
//             let splitx: Vec<&str> = res_p.as_path().to_str().unwrap().split("/").collect();
//             let index = splitx.iter().position(|&n| {
//                 n == "v1"
//                     || n == "v1alpha1"
//                     || n == "v1alpha2"
//                     || n == "v2"
//                     || n == "v1beta1"
//                     || n == "v2beta1"
//                     || n == "v2beta2"
//                     || n == "v1beta2"
//                     || n == "v1beta3"
//             });
//             if let Some(idx) = index {
//                 let content = fs::read_to_string(res.path())?;

//                 let to_str = format!("::{}::", splitx.get(idx).unwrap());

//                 let new_content = content.replace("::generated::", &to_str);

//                 let mut file = OpenOptions::new()
//                     .write(true)
//                     .truncate(true)
//                     .open(res.path())?;

//                 file.write(new_content.as_bytes())?;
//                 file.flush()?
//             }

//             mod_files.push(res.path());
//         }
//     }
//     Ok(())
// }

fn overwrite_mods(path: &Path, mod_files: &mut Vec<PathBuf>, version: &str) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            overwrite_mods(&path, mod_files, version)?;
        } else if entry.file_name() == "mod.rs" {
            let res_p = entry.path().clone();
            let splitx: Vec<&str> = res_p.as_path().to_str().unwrap().split("/").collect();
            let index = splitx.iter().position(|&n| {
                n == "v1"
                    || n == "v1alpha1"
                    || n == "v1alpha2"
                    || n == "v1alpha3"
                    || n == "v2"
                    || n == "v1beta1"
                    || n == "v2beta1"
                    || n == "v2beta2"
                    || n == "v1beta2"
                    || n == "v1beta3"
            });
            if let Some(idx) = index {

                let idx = format!("super::{}::file_descriptor().clone()", splitx.get(idx).unwrap());

                let content = fs::read_to_string(&path)?;
                let new_content = content
                    .replace("super::generated::file_descriptor().clone()", &idx)
                    // .replace("::generated::", &format!("::{}::", version))
                    .replace("super::generated::", &format!("crate::{}::", version));

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&path)?;
                
                file.write_all(new_content.as_bytes())?;
            }
            mod_files.push(path);
        }
    }
    Ok(())
}


fn add_mods<'a>(path: &Path, mod_files: &'a mut Vec<PathBuf>) -> io::Result<()> {
    let protos_dir = fs::read_dir(path)?;
    for v in protos_dir {
        let cur = v?;
        if cur.path().is_dir() {
            add_mods(&cur.path(), mod_files)?;
        }
        let mod_file = cur.path().clone().parent().unwrap().join("mod.rs");
        if !mod_files.contains(&mod_file) {
            let mut f = fs::OpenOptions::new()
                .append(true)
                .write(true)
                .create(true)
                .open(mod_file)?;
            let to_file = format!("pub mod {};\n", cur.file_name().to_str().unwrap());
            f.write(to_file.as_bytes())?;
            f.flush()?
        }
    }
    Ok(())
}

fn search_protos(path: &Path, protos: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            search_protos(&path, protos)?;
        } else if path.extension().map(|s| s == "proto").unwrap_or(false) {
            protos.push(path);
        }
    }
    Ok(())
}
