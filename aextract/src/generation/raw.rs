use duct::cmd;
use log::{error, info};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, read_dir},
    io::{BufRead, BufReader, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

use crate::types::satisfactory_raw::{AssetReference, Generated, Generator, parse_docs_json};

#[cfg(target_family = "unix")]
include!(concat!(env!("OUT_DIR"), "/binaries.rs"));

#[cfg(target_family = "windows")]
include!(concat!(env!("OUT_DIR"), "\\binaries.rs"));

fn generate_asset_request(
    generated: Generated,
    workdir: PathBuf,
) -> std::result::Result<(), Box<dyn Error>> {
    let mut requests: HashMap<String, String> = HashMap::new();
    for (_, desc) in generated.descriptions {
        for item in [desc.big_icon, desc.icon] {
            if let Some(AssetReference {
                asset_type,
                asset_path,
                asset_id: Some(id),
            }) = item
            {
                if !requests.contains_key(&id) && asset_type == String::from("Texture2D") {
                    requests.insert(
                        id,
                        format!(
                            "TEXTURE::/FactoryGame/Content/{}",
                            asset_path
                                .trim_start_matches('/')
                                .split_once('/')
                                .unwrap()
                                .1
                        ),
                    );
                }
            }
        }
    }

    requests.insert(
        String::from("MapSlice0_0"),
        String::from(
            "TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_0-0.Map_0-0"
        )
    );
    requests.insert(
        String::from("MapSlice1_0"),
        String::from(
            "TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_1-0.Map_1-0"
        )
    );
    requests.insert(
        String::from("MapSlice0_1"),
        String::from(
            "TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_0-1.Map_0-1"
        )
    );
    requests.insert(
        String::from("MapSlice1_1"),
        String::from(
            "TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_1-1.Map_1-1"
        )
    );

    info!(
        "Requesting {} items from Satisfactory assets",
        requests.len()
    );
    let mut file = fs::File::create(workdir.join("asset_req.txt"))?;
    for (id, tail) in requests {
        file.write(format!("{id}::{tail}\n").as_bytes())?;
    }

    Ok(())
}

pub fn generate_raw_data(
    options: crate::Cli,
    workdir: PathBuf,
) -> std::result::Result<Generated, Box<dyn Error>> {
    info!(
        "Parsing Docs.json at {}",
        options.docs_path().to_string_lossy().to_string()
    );
    let parsed = parse_docs_json(options.docs_path())?;
    let generated = Generator::new(parsed).generate();
    generate_asset_request(generated.clone(), workdir.clone())?;

    info!(
        "Unpacking binaries to {}",
        workdir.to_string_lossy().to_string()
    );
    let Binaries { executable, oodle, skiasharp, libblake } = binaries();
    fs::write(workdir.join(executable.name()), executable.bytes())?;
    fs::write(workdir.join(oodle.name()), oodle.bytes())?;
    fs::write(workdir.join(skiasharp.name()), skiasharp.bytes())?;
    fs::write(workdir.join(libblake.name()), libblake.bytes())?;


    #[cfg(unix)]
    {
        info!("Setting binary perms...");
        let mut exe_perms = fs::metadata(workdir.join(executable.name()))?.permissions();
        exe_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(executable.name()), exe_perms)?;

        let mut lib_perms = fs::metadata(workdir.join(oodle.name()))?.permissions();
        lib_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(oodle.name()), lib_perms)?;

        let mut lib_perms = fs::metadata(workdir.join(skiasharp.name()))?.permissions();
        lib_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(skiasharp.name()), lib_perms)?;

        let mut lib_perms = fs::metadata(workdir.join(libblake.name()))?.permissions();
        lib_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(libblake.name()), lib_perms)?;
    }

    let _paks = options.paks_path();
    let _comr = options.community_resources_path();

    let sidecar = if options.fhs {
        info!(
            "Calling sidecar in FHS: steam-run -a AExSidecar {} {} {} {} {}",
            PathBuf::from(".")
                .join(executable.name())
                .to_str()
                .unwrap(),
            _paks.to_str().unwrap(),
            _comr.to_str().unwrap(),
            "asset_req.txt",
            oodle.name()
        );
        cmd!(
            "steam-run",
            "-a",
            "AExSidecar",
            PathBuf::from(".").join(executable.name()),
            _paks.to_str().unwrap(),
            _comr.to_str().unwrap(),
            "asset_req.txt",
            oodle.name()
        )
    } else {
        info!(
            "Calling sidecar: {} {} {} {} {}",
            executable.name(),
            _paks.to_str().unwrap(),
            _comr.to_str().unwrap(),
            "asset_req.txt",
            oodle.name()
        );
        cmd!(
            workdir.join(executable.name()).to_str().unwrap(),
            _paks.to_str().unwrap(),
            _comr.to_str().unwrap(),
            "asset_req.txt",
            oodle.name()
        )
    };
    let sidecar = sidecar.dir(workdir.clone());
    info!("Sidecar: {sidecar:?}");

    info!("Sidecar running in {:?}", workdir.clone());
    info!(
        "Contents: {:?}",
        read_dir(workdir.clone()).unwrap().collect::<Vec<_>>()
    );
    let reader = sidecar.stderr_to_stdout().reader().or_else(|e| {
        error!("Failed to run: {e:?}");
        Err(e)
    })?;
    let lines = BufReader::new(reader).lines();

    info!("Sidecar status:");

    for line in lines {
        match line {
            Ok(val) => {
                info!(" .. {}", val);
            }
            Err(e) => {
                info!(" .. ERROR: {e:?}");
                break;
            }
        }
    }

    Ok(generated)
}
