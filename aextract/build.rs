use std::{env, fs, path::Path, process::Command};

use quote::quote;
use reqwest::Url;

const OODLE_ROOT: &'static str = "https://github.com/WorkingRobot/OodleUE/raw/refs/heads/main/Engine/Source/Programs/Shared/EpicGames.Oodle/Sdk";
const OODLE_VERSION: &'static str = "2.9.10";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

#[derive(Clone, Debug)]
struct Platform {
    pub id: String,
    pub dylib: String,
    pub runtime: String,
    pub executable: String,
}

impl Platform {
    pub fn new(
        id: impl AsRef<str>,
        dylib: impl AsRef<str>,
        runtime: impl AsRef<str>,
        executable: impl AsRef<str>,
    ) -> Self {
        Self {
            id: id.as_ref().to_string(),
            dylib: dylib.as_ref().to_string(),
            runtime: runtime.as_ref().to_string(),
            executable: executable.as_ref().to_string(),
        }
    }

    pub fn lib_url(&self) -> String {
        format!(
            "{OODLE_ROOT}/{OODLE_VERSION}/{}",
            self.dylib.trim_start_matches('/')
        )
    }

    pub fn lib_filename(&self) -> String {
        Url::parse(&self.lib_url())
            .unwrap()
            .path()
            .rsplit_once("/")
            .unwrap()
            .1
            .to_string()
    }

    pub fn publish(&self) -> () {
        fs::create_dir_all(
            Path::new(&env::var_os("OUT_DIR").unwrap())
                .join("binaries")
                .join(self.id.clone()),
        )
        .expect("Should be able to create binary output directory.");
        let mut cmd = Command::new("dotnet");
        cmd.arg("publish")
            .arg("-r")
            .arg(self.runtime.clone())
            .arg("-o")
            .arg(
                Path::new(&env::var_os("OUT_DIR").unwrap())
                    .join("binaries")
                    .join(self.id.clone()),
            )
            .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("AExSidecar"));

        match cmd.spawn() {
            Ok(child) => match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        p!(
                            "Successfully published sidecar for {} ({})",
                            self.id,
                            self.runtime
                        );
                    } else {
                        p!(
                            "ERR: Failed to publish sidecar for {} ({}) due to an internal error: {}\n\n{}\n{}",
                            self.id,
                            self.runtime,
                            output.status.code().unwrap_or(-1),
                            String::from_utf8(output.stdout).unwrap_or(String::from("UNKNOWN.")),
                            String::from_utf8(output.stderr).unwrap_or(String::from("UNKNOWN."))
                        );
                    }
                }
                Err(e) => {
                    p!(
                        "ERR: Failed to verify publish of sidecar for {} ({}): {:?}",
                        self.id,
                        self.runtime,
                        e
                    );
                }
            },
            Err(e) => {
                p!(
                    "ERR: Failed to publish sidecar for {} ({}): {:?}",
                    self.id,
                    self.runtime,
                    e
                );
            }
        }
    }

    pub fn download_lib(&self) -> () {
        if !Path::new(&env::var_os("OUT_DIR").unwrap())
            .join("shared_libs")
            .exists()
        {
            fs::create_dir_all(Path::new(&env::var_os("OUT_DIR").unwrap()).join("shared_libs"))
                .expect("Should be able to create shared library output directory.");
        }
        let mut req = reqwest::blocking::get(self.lib_url()).expect("Failed to request dylib.");
        let mut dl_file = fs::File::create(
            Path::new(&env::var_os("OUT_DIR").unwrap())
                .join("shared_libs")
                .join(self.lib_filename()),
        )
        .unwrap();
        req.copy_to(&mut dl_file).unwrap();
    }
    pub fn create_getter(&self) -> () {
        let id = self.id.clone();
        let exe_name = self.executable.clone();
        let dylib_name = self.lib_filename();

        let tokens = quote! {
            #[cfg(target_family = "unix")]
            const EXECUTABLE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/binaries/", #id, "/", #exe_name));
            #[cfg(target_family = "unix")]
            const DYLIB: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shared_libs/", #dylib_name));

            #[cfg(target_family = "windows")]
            const EXECUTABLE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "\\binaries\\", #id, "\\", #exe_name));
            #[cfg(target_family = "windows")]
            const DYLIB: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "\\shared_libs\\", #dylib_name));

            pub fn binaries() -> ((String, &'static [u8]), (String, &'static [u8])) {
                ((String::from(#exe_name), EXECUTABLE), (String::from(#dylib_name), DYLIB))
            }
        };
        let tree: syn::File = syn::parse2(tokens).unwrap();
        let formatted = prettyplease::unparse(&tree);
        fs::write(
            Path::new(&env::var_os("OUT_DIR").unwrap()).join("binaries.rs"),
            formatted,
        )
        .unwrap();
    }
}

fn get_platforms() -> Vec<Platform> {
    let mut platforms: Vec<Platform> = Vec::new();

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    platforms.push(Platform::new(
        "linux_x86_64",
        "linux/lib/liboo2corelinux64.so.9",
        "linux-x64",
        "AExSidecar",
    ));

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    platforms.push(Platform::new(
        "linux_aarch64",
        "linuxarm/lib/liboo2corelinuxarm64.so.9",
        "linux-arm64",
        "AExSidecar",
    ));

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    platforms.push(Platform::new(
        "windows_x86_64",
        "win/redist/oo2core_9_win64.dll",
        "win-x64",
        "AExSidecar.exe",
    ));

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    platforms.push(Platform::new(
        "macos_x86_64",
        "mac/lib/liboo2coremac64.2.9.10.dylib",
        "osx-x64",
        "AExSidecar",
    ));

    if platforms.len() == 0 {
        panic!("Target platform(s) not supported.");
    }

    platforms
}

fn main() {
    let platform = get_platforms().first().unwrap().clone();
    p!("Building dependencies for {}", platform.id);
    platform.publish();
    platform.download_lib();
    platform.create_getter();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=AExSidecar/AExSidecar.csproj");
    println!("cargo::rerun-if-changed=AExSidecar/Program.cs");
}
