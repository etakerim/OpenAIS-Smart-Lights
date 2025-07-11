use std::io::Read;
use std::fs::File;
use yaml_rust::YamlLoader;


pub struct Discovery {
    source: String,
    controllers: Vec<String>,
    lights: Vec<String>,
    switches: Vec<String>
}


impl Discovery {

    pub fn new(source: String) -> Self {
        Self { 
            source: source,
            controllers: Vec::new(),
            lights: Vec::new(),
            switches: Vec::new()
        }
    }

    pub fn load_topology(&mut self) {
        // in reality it will download file from remote resource

        let mut file = File::open(&self.source).expect("Unable to open topology");
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("Unable to read topology");
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let conf = &docs[0];

        let lights = conf["lights"].as_vec().unwrap_or_else(|| {
            panic!("Variable 'lights' is missing in topology");
        });
        for ip in lights {
            self.lights.push(String::from(ip.as_str().unwrap()));
        }

        let switches = conf["switches"].as_vec().unwrap_or_else(|| {
            panic!("Variable 'switches' is missing in topology");
        });
        for ip in switches {
            self.switches.push(String::from(ip.as_str().unwrap()));
        }

        let controllers = conf["controllers"].as_vec().unwrap_or_else(|| {
            panic!("Variable 'controllers' is missing in topology");
        });
        for ip in controllers {
            self.controllers.push(String::from(ip.as_str().unwrap()));
        }
    }

    pub fn switches(&self) -> Vec<String> {
        self.switches.clone()
    }

    pub fn lights(&self) -> Vec<String> {
        self.lights.clone()
    }
}