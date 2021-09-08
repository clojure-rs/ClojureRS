#[macro_use]
extern crate rust_clojure;

use std::fs::File;
use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};
use std::io::BufRead;
use std::io::BufReader;

use rust_clojure::environment;
use rust_clojure::environment::Environment;
use rust_clojure::reader;
use rust_clojure::value::{Evaluable, ToValue, Value};
use std::rc::Rc;
use std::process::{Command, Stdio};

use rust_clojure::symbol::*;
use rust_clojure::persistent_list::ToPersistentListIter;
// use rust_clojure::persistent_list_map::*;
use rust_clojure::persistent_vector::{PersistentVector, ToPersistentVectorIter};


fn codegen_value (value: Rc<Value>) -> String {
    match value.as_ref() {
        Value::Symbol(s) => format!("sym!(\"{}\").to_rc_value()", s.name),
        Value::PersistentList(l) => {
            let vals: String = Rc::new(l.clone()).iter()
                                .map(codegen_value)
                                .collect::<Vec<_>>()
                                .join(" \n");
            
            format!("list_val!({})", vals)
        },
        Value::PersistentVector(pv) => {
            let vals: String = Rc::new(pv.clone()).iter()
                            .map(codegen_value)
                            .collect::<Vec<_>>()
                            .join(",");
            format!("PersistentVector{{ vals: vec![{}] }}", vals)
        },
        Value::String(s) => format!("String::from(\"{}\").to_rc_value()", s),
        Value::Boolean(b) => format!("{}.to_rc_value()", b),
        Value::I32(i) => format!("{}i32.to_rc_value()", i),
        Value::F64(f)=> format!("{}f64.to_rc_value()", f),
        Value::Nil => {
            "Value::Nil".into()
        }
        _ => "".into()
    }    
}

fn apply_rustfmt(source: String) -> io::Result<String>
{
    let rustfmt = rustfmt_path();
    let mut cmd = Command::new(&rustfmt);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    let mut child = cmd.spawn().expect("Error running rustfmt");
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output)?;

    let status = child.wait()?;
    let source = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
            anything that could panic",
    );
    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(bindings),
            Some(2) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors.".to_string(),
            )),
            Some(3) => {
                println!("Rustfmt could not format some lines.");
                Ok(bindings)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            )),
        },
        _ => Ok(source)
    }
}
fn read_source<B: BufRead>(mut reader: B) -> Vec<Value>
{
    let mut last_val = reader::read(&mut reader);

    let mut values = Vec::new();
    values.push(last_val.clone());
    let value = loop {
        // @TODO this is hardcoded until we refactor Conditions to have keys, so that
        //       we can properly identify them
        // @FIXME
        if let Value::Condition(cond) = &last_val {
            if cond != "Tried to read empty stream; unexpected EOF" {
                println!("Error reading file: {}", cond);
            }

            break last_val;
        }

        last_val = reader::read(&mut reader);
        if let Value::Condition(cond) = last_val.clone() {
        }else{
            values.push(last_val.clone());
        }
    };
    values
}

/// Gets the rustfmt path to rustfmt the generated bindings.
fn rustfmt_path() -> std::path::PathBuf {
    if let Ok(rustfmt) = std::env::var("RUSTFMT") {
        rustfmt.into()
    }else{
        "rustfmt".into()
    }
}

fn main() -> io::Result<()>
{
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("A Clojure-to-Rust transpiler\n\n\
                   Usage: {} <clojure-file> <output-path> [output-filename]\n\n\
                   [output-filename] defaults to main.rs\n", args[0]);
        std::process::exit(-1);
    }
    let output_folder: PathBuf = (&args[2]).into();
    let source_file: String = (&args[1]).into();
    
    let output_filename = args.get(3).cloned().unwrap_or("main.rs".into());
    
    let output_file: PathBuf = output_folder.join(output_filename);

    let codegen_template = include_str!("codegen.rs.tmpl");

    let core = File::open(source_file).unwrap();
    let reader = BufReader::new(core);
    let values = read_source(reader);
    let v_str = values.clone().into_iter()
                .map(|v| codegen_value(Rc::new(v)))
                .collect::<Vec<_>>()
                .join(",\n");


    let value_vec_code= format!("vec![{}];", v_str);
    let codegen_output = codegen_template.replace("%%", &value_vec_code);
    let codegen_output = apply_rustfmt(codegen_output)?;

    fs::create_dir_all(&output_folder)?;
    let mut fp = File::create(&output_file)?;
    write!(fp, "{}", codegen_output)?;
    println!("Code written to {:?}", output_file);
    Ok(())
}
