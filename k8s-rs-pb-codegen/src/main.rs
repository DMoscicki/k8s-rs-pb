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
        Customize::default().before("#[derive(::serde::Deserialize)]\n#[serde(rename_all = \"snake_case\")]")
    }

    fn field(&self, field: &FieldDescriptor) -> Customize {

        if BMAP_FIELDS.contains(&field.proto().name()) && field.is_map() {
            return Customize::default().before("#[serde(with = \"crate::quantity_parse\")]\n#[serde(default)]")
        }

        if field.is_repeated() {
            return Customize::default().before("#[serde(default)]");
        }

        if field.proto().type_() == Type::TYPE_MESSAGE && field.is_singular() {
            if field.proto().name().to_lowercase().contains("time") || field.proto().name() == "startedAt" || field.proto().name() == "finishedAt" {
                return Customize::default().before("#[serde(with = \"crate::custom_date\")]\n#[serde(default)]")
            }

            return Customize::default().before("#[serde(with = \"crate::MessageFieldDef\")]\n#[serde(default)]")
        }
        if field.is_map() {
            return Customize::default().before("#[serde(default)]");
        } 
        
        Customize::default()
    }

    fn special_field(&self, message: &MessageDescriptor, field: &str) -> Customize {
        Customize::default().before("#[serde(skip)]")
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);
    println!("{}", args[2]);

    if args.len() < 3 {
        eprintln!("need flags");
        process::exit(1)
    }

    if args[1] != "overwrite" {
        println!("{}", args[1]);
        eprintln!("need overwrite flag");
        process::exit(1)
    }

    let res = match args[2].as_str() {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => {
            eprintln!("incorrect flag");
            process::exit(1)
        }
    };

    let k8s_pb = Path::new("./k8s-rs-pb/src");

    let mut protos: Vec<PathBuf> = Vec::new();

    match res {
        true => {
            let mut mod_files: Vec<PathBuf> = Vec::new();

            let target_dir = Path::new("./k8s-rs-pb/src");

            overwrite_mods(target_dir, &mut mod_files).unwrap();

            add_mods(target_dir, &mut mod_files).unwrap();
        }
        false => {
            search_protos(k8s_pb, &mut protos)?;

            let custom = Customize::default().btreemaps(true)
                                    .gen_mod_rs(false).generate_accessors(true)
                                    .generate_getter(true).inside_protobuf(false);
        
            for v in &protos {
                let splitx = v.clone().as_path().to_str().unwrap().replace("./k8s-rs-pb/src/", "").replace("/generated.proto", "");
                let mod_dir = format!("k8s-rs-pb/src/{}", splitx);

                fs::create_dir_all(mod_dir.clone())?;

                protobuf_codegen::Codegen::new()
                                .protoc()
                                .includes(&["./k8s-rs-pb/src"])
                                .input(v.as_path())
                                .out_dir(format!("k8s-rs-pb/src/{}", splitx))
                                .customize(custom.clone())
                                .customize_callback(GenStruct)
                                .run_from_script();

                let generated_rs = format!("{}/{}", mod_dir, "generated.rs");
                let mod_rs = format!("{}/{}", mod_dir, "mod.rs");
                
                fs::rename(generated_rs, mod_rs).unwrap();
            }
        }
    }

    Ok(())
}

fn overwrite_mods<'a>(path: &Path, mod_files: &'a mut Vec<PathBuf>) -> io::Result<()> {
    for val in fs::read_dir(path)? {
        let res = val?;
        if res.file_type()?.is_dir() {
            overwrite_mods(res.path().as_path(), mod_files)?
        }
        if res.file_name().to_str().unwrap() == "mod.rs" {
            let res_p = res.path().clone();
            let splitx: Vec<&str> = res_p.as_path().to_str().unwrap().split("/").collect();
            let index = splitx.iter().position(|&n| {
                n == "v1"
                    || n == "v1alpha1"
                    || n == "v1alpha2"
                    || n == "v2"
                    || n == "v1beta1"
                    || n == "v2beta1"
                    || n == "v2beta2"
                    || n == "v1beta2"
                    || n == "v1beta3"
            });
            if let Some(idx) = index {
                let content = fs::read_to_string(res.path())?;

                let to_str = format!("::{}::", splitx.get(idx).unwrap());

                let new_content = content.replace("::generated::", &to_str);

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(res.path())?;

                file.write(new_content.as_bytes())?;
                file.flush()?
            }

            mod_files.push(res.path());
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

fn search_protos<'a>(path: &Path, protos: &'a mut Vec<PathBuf>) -> io::Result<()> {
    let dir = fs::read_dir(path);

    for v in dir? {
        let file = v?;
        if file.path().is_dir() {
            search_protos(file.path().as_path(), protos)?;
        }
        if file.path().is_file() && file.path().extension().unwrap() == "proto" {
            protos.push(file.path());
        }
    }

    Ok(())
}
