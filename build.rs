use std::fs::{File, read_dir};
use std::io::{Read, Write};
use toml::Value;
use toml::value::Value::*;
use std::string::String;
use toml::map::Map;
use core::fmt::Write as _Write;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("messages.rs");
    let mut target = File::create(dest_path)?;
    target.write(b"#[allow(unreachable_patterns)]
    fn to_string(msg: &Message, locale: &str) -> String {
            let s = "
    )?;

    let mut default_values = String::new();
    for dir in read_dir("i18n")? {
        let path = dir?.path();
        if path.is_file() {
            let p = path.as_path();
            if p.file_name().unwrap().to_str().unwrap().ends_with(".toml") {
                let mut f = File::open(p)?;
                let mut content = String::new();
                f.read_to_string(&mut content)?;

                let value = content.parse::<Value>()?;
                match value {
                    Table(map) => {
                        for (locale, values) in &map {
                            match values {
                                Table(vals) => {
                                    if locale.as_str().eq("default") {
                                        write_values(vals, &mut default_values)?;
                                    } else {
                                        write_locale(locale, vals, &mut target)?;
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            }
        }
    }
    target.write(b"{std::string::String::new()};\n\nif s.is_empty() {\n")?;
    target.write(default_values.as_bytes())?;
    target.write(b"} else {s}\n}")?;

    println!("cargo:rerun-if-changed=build.rs,i18n/*");

    Ok(())
}

fn write_locale(locale: &String, values: &Map<String, Value>, f: &mut File) -> std::io::Result<()> {
    f.write(b"if locale.starts_with(\"")?;
    f.write(locale.as_bytes())?;
    f.write(b"\") {\n")?;
    let mut vals = String::new();
    write_values(values, &mut vals)?;
    f.write(vals.as_bytes())?;
    f.write(b"} else ")?;
    Ok(())
}

fn write_values(values: &Map<String, Value>, s: &mut String) -> std::io::Result<()> {
    let result = _write_values(values, s);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(std::io::Error::from(std::io::ErrorKind::Other))
    }
}

fn _write_values(values: &Map<String, Value>, s: &mut String) -> std::fmt::Result {
    s.write_str("\tmatch msg {\n")?;
    for (k, v) in values {
        match v {
            Value::String(v) => {
                let vs : Vec<&str> = v.matches("{}").collect();
                let count = vs.len();
                s.write_str("\t\tMessage::")?;
                s.write_str(k)?;
                if count == 0 {
                    s.write_str(" => std::string::String::from(\"")?;
                    s.write_str(v)?;
                    s.write_str("\"),\n")?;
                } else {
                    s.write_str("(")?;
                    for i in 0..count {
                        if i > 0 {
                            s.write_str(", ")?;
                        }
                        s.write_str("v")?;
                        s.write_str(i.to_string().as_str())?;
                    }
                    s.write_str(")")?;
                    s.write_str(" => std::format!(\"")?;
                    s.write_str(v)?;
                    s.write_str("\"")?;
                    for i in 0..count {
                        s.write_str(", v")?;
                        s.write_str(i.to_string().as_str())?;
                    }
                    s.write_str("),\n")?;
                }
            },
            _ => ()
        }
    }
    s.write_str("\t\t_ => std::string::String::new()\n\t}\n")?;
    Ok(())
}