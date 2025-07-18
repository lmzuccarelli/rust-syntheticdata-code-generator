use crate::custom::error::GenerateError;
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub base_type: String,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub function: Option<Vec<String>>,
    pub limits_array: Option<String>,
    pub output: bool,
    pub init: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameters {
    pub name: String,
    pub description: String,
    pub params: Vec<Parameter>,
    pub log_level: String,
    pub project: String,
    pub working_dir: String,
}

#[derive(Debug, Clone)]
pub struct ImplGeneratorInterface {}

pub trait GeneratorInterface {
    fn read(&self, dir: String) -> Result<Parameters, GenerateError>;
    fn create(&self, data: Parameters) -> Result<(), GenerateError>;
}

impl GeneratorInterface for ImplGeneratorInterface {
    fn read(&self, name: String) -> Result<Parameters, GenerateError> {
        let json_data = std::fs::File::open(&name).unwrap();
        let params: Parameters = serde_json::from_reader(json_data).unwrap();
        Ok(params)
    }

    fn create(&self, data: Parameters) -> Result<(), GenerateError> {
        let mut body = String::from("use rand::Rng;\n");
        body.push_str("use std::cmp;\n");
        body.push_str("use serde_derive::{Deserialize, Serialize};\n");
        body.push_str("use std::fs;\n\n");
        let mut schema = String::from("#[derive(Serialize, Deserialize, Clone, Debug)]\n");
        schema.push_str(&format!(
            "pub struct {} {}\n\tname: String,\n\tcount: usize,\n{}\n\n",
            data.name, "{", "}"
        ));
        let mut impl_body = String::from(&format!("impl {} {}", data.name, "{\n"));
        impl_body.push_str(&format!(
            "\tpub fn new(name: String, count: usize) -> {} {}\n",
            data.name, "{"
        ));
        impl_body.push_str(&format!(
            "\t\treturn {} {} name,count,{} {}\n\n",
            data.name, "{", "}", "}"
        ));
        let mut generate = String::from("\tpub fn generate(&mut self) {\n");
        let mut rules_engine =
            String::from("\npub fn rules_engine(key: &str,status: i8,value: f32) -> i8 {\n");

        rules_engine.push_str("\tif status == 2 { return 2; }\n");
        rules_engine.push_str("\tmatch key {\n");
        generate.push_str("\t\tlet mut rng = rand::rng();\n");
        generate.push_str(&format!("\t\tlet mut status: i8 = -1;\n"));
        let header = data
            .params
            .iter()
            .map(|x| x.clone().name)
            .collect::<Vec<String>>();

        generate.push_str(&format!(
            "\t\tlet mut data_string = String::from(\"{}\\n\");\n",
            header.join(",").to_string()
        ));

        for item in data.params.iter() {
            if item.limits_array.is_none() && !item.output {
                return Err(GenerateError::new(
                    "[create] building limits array all outputs must have limits_field set",
                ));
            }
            if item.limits_array.is_some() && !item.output {
                generate.push_str(&format!(
                    "\t\tlet {}_range: Vec<Vec<f32>> = {};\n",
                    item.name,
                    item.limits_array.as_ref().unwrap()
                ))
            }
        }

        // this is specific to a status with 3 values
        // TODO: add status range
        //generate.push_str("\t\tfor x in 0..3 {\n");
        generate.push_str("\t\tlet mut x:usize;\n");
        generate.push_str("\t\t\tfor _n in 0..self.count {\n");
        // build schema and body
        for item in data.params.iter() {
            if !item.output {
                generate.push_str("\t\t\t\tx = rng.random_range(0..3);\n");
                generate.push_str(&format!(
                    "\t\t\t\tlet {} = rng.random_range({}_range[x][0]..{}_range[x][1]);\n",
                    item.name, item.name, item.name,
                ));
                generate.push_str(&format!(
                    "\t\t\t\tstatus = rules_engine(\"{}\",status,{});\n",
                    item.name, item.name
                ));
                generate.push_str(&format!(
                    "\t\t\t\tdata_string.push_str(&format!(\"{},\",{}));\n",
                    "{}", item.name
                ));
                rules_engine.push_str(&format!("\t\t\"{}\" => {}\n", item.name, "{"));
                for line in item.function.as_ref().unwrap().iter() {
                    if line.contains("limits_array") {
                        let limits_array = item.limits_array.as_ref().unwrap();
                        let vecs = limits_array
                            .split("vec![")
                            .map(|x| x.replace("]", ""))
                            .filter(|x| x.len() > 0)
                            .collect::<Vec<String>>();
                        let mut vec_limits: Vec<Vec<&str>> = Vec::new();
                        // notice reverse order
                        for vec in vecs.iter().rev() {
                            let (a, b) = vec.split_once(",").unwrap();
                            let v: Vec<&str> = vec![a, b];
                            vec_limits.insert(0, v.clone());
                        }
                        let max_value = match line {
                            x if x.contains("limits_array[0][1]") => line
                                .replace("limits_array[0][1]", &vec_limits[0][1].replace(",", "")),
                            x if x.contains("limits_array[1][1]") => line
                                .replace("limits_array[1][1]", &vec_limits[1][1].replace(",", "")),
                            x if x.contains("limits_array[2][1]") => line
                                .replace("limits_array[2][1]", &vec_limits[2][1].replace(",", "")),
                            x if x.contains("limits_array[3][1]") => line
                                .replace("limits_array[3][1]", &vec_limits[3][1].replace(",", "")),
                            &_ => line.to_string(),
                        };
                        rules_engine.push_str(&format!("\t\t\t{}\n", max_value));
                    } else {
                        rules_engine.push_str(&format!("\t\t\t{}\n", line));
                    }
                }
                rules_engine.push_str("\t\t},\n");
            } else {
                generate
                    .push_str("\t\t\t\tdata_string.push_str(&format!(\"{}{}\",status,\"\\n\"));\n");

                generate.push_str(&format!("\t\t\t\tstatus = {};\n", item.init));
            }
        }
        rules_engine.push_str("\t\t&_ => return status,\n");
        //generate.push_str("\t\t\t}\n\t\t}\n");
        generate.push_str("\n\t\t}\n");
        let file = "\t\tfs::write(format!(\"results/{}-{}.csv\",self.name,self.count),data_string).expect(\"should write to file\");\n";
        generate.push_str(&file);
        generate.push_str("\t}\n");
        body.push_str(&schema);
        body.push_str(&impl_body);
        body.push_str(&generate);
        body.push_str("}\n");
        rules_engine.push_str("\t}\n}\n");
        body.push_str(&rules_engine);

        // create the library project
        let project_dir = format!(
            "{}/{}-{}",
            data.working_dir,
            data.project,
            data.name.to_lowercase(),
        );
        let _res = fs::create_dir_all(project_dir.clone() + &"/src");
        let res = fs::write(format!("{}/src/lib.rs", project_dir), body);
        if res.is_err() {
            return Err(GenerateError::new(&format!(
                "[create] writing file {} {}",
                "src/lib.rs",
                res.err().unwrap().to_string().to_lowercase()
            )));
        }

        // update project toml file
        let mut toml = String::from("[package]\n");
        toml.push_str(&format!("name = \"sd-gen-{}\"\n", data.name.to_lowercase()));
        toml.push_str("version = \"0.0.1\"\n");
        toml.push_str("edition = \"2021\"\n\n");
        toml.push_str("[dependencies]\n");
        toml.push_str("rand = \"0.9.1\"\n");
        toml.push_str("serde = \"1.0.219\"\n");
        toml.push_str("serde_derive = \"1.0.219\"\n\n");
        toml.push_str("[lib]\n");
        toml.push_str(&format!("name = \"sd_gen_{}\"\n", data.name.to_lowercase()));
        toml.push_str("path = \"src/lib.rs\"\n");
        let res = fs::write(format!("{}/Cargo.toml", project_dir), toml);
        if res.is_err() {
            return Err(GenerateError::new(&format!(
                "[create] writing file {} {}",
                "Cargo.toml",
                res.err().unwrap().to_string().to_lowercase()
            )));
        }
        Ok(())
    }
}
