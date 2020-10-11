use bitspec::*;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use text_io::read;

fn as_field_list_c_syntax(line: &FixedLine) -> String {
    let mut output = String::new();
    let name = line.name.replace(" ", "_");
    output += &format!("const union Field {}[] = {{\n", name);
    for meta in &line.fields {
        match &meta.field {
            Field::Bool(b) => {
                output += &format!("\t{{.Bool = {{ // {}\n", meta.name);
                output += &format!("\t\toffset: {}}},\n", b.offset);
                output += &format!("\t}},\n");
            }
            Field::F32(f) => {
                output += &format!("\t{{.F32 = {{ // {}\n", meta.name);
                output += &format!("\t\tdecode_add: {},\n", f.decode_add);
                output += &format!("\t\tdecode_scale: {},\n", f.decode_scale);
                output += &format!("\t\tlength: {},\n", f.length);
                output += &format!("\t\toffset: {}}},\n", f.offset);
                output += &format!("\t}},\n");
            }
            Field::F64(f) => {
                output += &format!("\t{{.F64 = {{ // {}\n", meta.name);
                output += &format!("\t\tdecode_add: {},\n", f.decode_add);
                output += &format!("\t\tdecode_scale: {},\n", f.decode_scale);
                output += &format!("\t\tlength: {},\n", f.length);
                output += &format!("\t\toffset: {}}},\n", f.offset);
                output += &format!("\t}},\n");
            }
        }
    }
    output += &format!("}};");
    output
}

fn as_field_list_rust_syntax(line: &FixedLine) -> String {
    let mut output = String::new();
    let name = line.name.replace(" ", "_");
    output += &format!("fields: &[ // {}\n", name);
    for meta in &line.fields {
        match &meta.field {
            Field::Bool(b) => { 
                output += &format!("\tField::Bool(BoolField {{ // {}\n", meta.name);
                output += &format!("\t\toffset: {}", b.offset);
            }
            Field::F32(f) => { 
                output += &format!("\tField::F32(FloatField {{ // {}\n", meta.name);
                output += &format!("\t\tdecode_add: {:.10},\n\t\tdecode_scale: {:.10},\n\t\tlength: {},\n\t\toffset: {}",
                    f.decode_add, f.decode_scale, f.length, f.offset);
            }
            Field::F64(f) => { 
                output += &format!("\tField::F64(FloatField {{ // {}\n", meta.name);
                output += &format!("\t\tdecode_add: {:.10},\n\t\tdecode_scale: {:.10},\n\t\tlength: {},\n\t\toffset: {}",
                    f.decode_add,
                    f.decode_scale, 
                    f.length,
                    f.offset);
            }
        }
        output += &format!("\t}}),\n");
    }
    output += &format!("];");
    output
}

fn main() {
    write_template().unwrap();
    println!("template crated in the specs dir");

    //user copies template and modifies copy
    println!("enter name of created spec");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut spec_path = PathBuf::from("specs");
    spec_path.push(input.trim());
    spec_path.set_extension("yaml");
    dbg!(&spec_path);
    let f = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(spec_path)
        .unwrap();
    
    let meta_spec = serde_yaml::from_reader::<File, MetaDataSpec>(f).unwrap();
    let metadata: FixedLine = meta_spec.into();
    
    println!("print fields list [rust syntax]? Y/n");
    let print_fields: String = read!("{}\n");
    if print_fields != "n" {
        println!("******\n{}\n******", as_field_list_rust_syntax(&metadata));
    }

    println!("print fields list [c syntax]? Y/n");
    let print_fields: String = read!("{}\n");
    if print_fields != "n" {
        println!("******\n{}\n******", as_field_list_c_syntax(&metadata));
    }

    println!("print needed C source? Y/n");
    let print_fields: String = read!("{}\n");
    if print_fields != "n" {
        println!("******\n{}\n******", include_str!("../c_src/encoding.h"));
        println!("******\n{}\n******", include_str!("../c_src/encoding.c"));
    }    
}
