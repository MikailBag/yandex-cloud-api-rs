use std::path::PathBuf;

pub struct CloudService {
    name: String,
    version: String,
    client_name: Option<String>,
    endpoint: Option<String>,
    grpc: Vec<Grpc>,
}

struct Grpc {
    file_path: String,
    code_path: String,
}

impl CloudService {
    pub fn new(name: &str, version: &str) -> Self {
        CloudService {
            name: name.to_string(),
            version: version.to_string(),
            client_name: None,
            endpoint: None,
            grpc: Vec::new(),
        }
    }

    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.to_string());
        self
    }

    pub fn with_grpc(mut self, file_path: &str, code_path: &str) -> Self {
        let g = Grpc {
            file_path: file_path.to_string(),
            code_path: code_path.to_string(),
        };
        self.grpc.push(g);
        self
    }

    pub fn with_client_name(mut self, client_name: &str) -> Self {
        self.client_name = Some(client_name.to_string());
        self
    }

    fn src_path(&self) -> PathBuf {
        PathBuf::from(format!("./{}-{}/src", self.name, self.version))
    }

    fn do_emit(&self) {
        println!("------ Compiling {}/{} ------", self.name, self.version);
        if self.grpc.is_empty() {
            panic!("No grpc services configured");
        }

        let client_name = self
            .client_name
            .as_ref()
            .expect("client name not configured");

        let gen_dir = self.src_path().join("gen");

        let builder = tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .out_dir(&gen_dir);

        let mut protos = Vec::new();
        for g in &self.grpc {
            protos.push(format!(
                "./vendor/yandex/cloud/{}/{}/{}",
                self.name, self.version, g.file_path
            ));
        }
        println!("Compiling protos");
        builder
            .compile(&protos, &["./vendor"])
            .expect("failed to compile service");
        println!("Generating pb.rs (file which imports all tonic definitions and stubs)");

        let mut pkg_pathes = Vec::new();

        let items = std::fs::read_dir(&gen_dir).expect("failed to open ./gen");
        for item in items {
            let item = item.expect("failed to get item info");
            let path = item.path();
            let name = path.file_stem().expect("invalid file name");
            let name = name.to_str().expect("invalid file name");
            dbg!(&path);
            dbg!(name);
            if !path.file_name().unwrap().to_str().unwrap().ends_with(".rs") {
                continue;
            }

            println!("Found generated file {name}");

            pkg_pathes.push(
                name.split('.')
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
            )
        }
        pkg_pathes.push(Vec::new());
        pkg_pathes.sort();
        pkg_pathes.push(Vec::new());
        assert!(pkg_pathes[0].is_empty());

        let mut pb_rs = String::new();

        for window in pkg_pathes.windows(2) {
            let mut prev = window[0].clone();
            let cur = window[1].clone();
            let mut common_prefix = 0;
            while common_prefix < std::cmp::min(prev.len(), cur.len()) {
                if prev[common_prefix] == cur[common_prefix] {
                    common_prefix += 1;
                } else {
                    break;
                }
            }
            while prev.len() > common_prefix {
                pb_rs += "}\n";
                prev.pop();
            }
            if cur.is_empty() {
                break;
            }
            for i in common_prefix..cur.len() {
                pb_rs += &format!("pub mod {} {{", cur[i]);
            }
            pb_rs += &format!("include!(\"gen/{}.rs\");", cur.join("."));
        }
        let pb_rs_path = self.src_path().join("pb.rs");
        std::fs::write(pb_rs_path, pb_rs).unwrap();
    }

    pub fn emit(self) {
        self.do_emit()
    }
}
