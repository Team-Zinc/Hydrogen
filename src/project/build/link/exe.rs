use crate::project::Project;

pub fn get_output_flags(o: &str) -> Vec<&str> {
    vec!["-o", o]
}

pub fn get_link_flags(p: &Project) -> Vec<String> {
    let mut flags: Vec<String> = Vec::new();
    
    if let Some(ref real) = p.real_actual {
        if let Some(ref deps) = real.dependencies {
            flags.push("-L./target/".to_owned());
            println!("{}", std::env::current_dir().unwrap().display());
            for dep in deps {
                if dep.external.is_some() {
                    flags.push(["-l", dep.name.as_str()].join(""));

                    continue;
                }
                // We have us a local dependency
                flags.push([
                    "-l",
                    dep.name.replace("lib", "").as_str()
                ].join(""));
            }
        }
    }

    if std::env::consts::OS == "macos" {
        flags.append(&mut vec!["-macosx_version_min".into(), "10.13".into()]);
    }

    let mut base = vec![
        "-execute".into(), "-lm".into(),
        "-arch".into(), std::env::consts::ARCH.into()
    ];
    base.append(&mut flags);

    base
}

pub fn get_output_file(p: &Project) -> String {
    ["target", p.get_name().unwrap().as_str()].join("/")
}
