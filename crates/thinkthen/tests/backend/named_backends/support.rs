//! Markers, counting listeners, the counting proxy, and the scratch home.

use crate::child::ChildEnvironment as _;
use std::io::{BufRead as _, BufReader};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use conformance_backend::{Canned, Listener};

/// Each key variable and the marker it holds in every run.
pub(crate) const TYPESAFE: (&str, &str) = ("TYPESAFE_API_KEY", "sk-typesafe-marker-0334a");
pub(crate) const LIQUIDAI: (&str, &str) = ("LIQUIDAI_API_KEY", "sk-liquidai-marker-0334b");
pub(crate) const LIQUID: (&str, &str) = ("LIQUID_API_KEY", "sk-liquid-marker-0334c");
pub(crate) const PRIMARY: (&str, &str) = ("THINKTHEN_API_KEY", "sk-primary-marker-0334d");
pub(crate) const CONFIGURED: (&str, &str) = ("LOCAL_D1_KEY", "sk-configured-marker-0334e");
pub(crate) const OLLAMA: (&str, &str) = ("OLLAMA_API_KEY", "sk-ollama-marker-0339a");
pub(crate) const EXPLICIT: &str = "sk-explicit-marker-0334f";

pub(crate) const MARKERS: [(&str, &str); 6] =
    [TYPESAFE, LIQUIDAI, LIQUID, PRIMARY, CONFIGURED, OLLAMA];

/// The count places after the markers: the explicit key, then no key at all.
pub(crate) const EXPLICIT_PLACE: usize = MARKERS.len();
pub(crate) const KEYLESS: usize = MARKERS.len() + 1;

/// One count per marker, the explicit key, and no key.
pub(crate) type Counts = [usize; MARKERS.len() + 2];

/// The two provider bases, spelled here because tests may name them.
pub(crate) const LIQUID_BASE: &str = "https://api.liquid.ai/decisions/v1";
pub(crate) const TYPESAFE_BASE: &str = "https://api.typesafe.ai/v1";

/// A loopback backend that answers every decide question and counts by key.
pub(crate) fn listener() -> Listener {
    Listener::answering(|body| {
        let request: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();
        let model = request["model"].as_str().unwrap_or("m").to_owned();
        let answers: Vec<String> = request["questions"]
            .as_object()
            .map(|questions| {
                questions
                    .keys()
                    .map(|name| format!(r#""{name}":{{"type":"noul","noul":0.92}}"#))
                    .collect()
            })
            .unwrap_or_default();
        Canned::ok(&format!(
            r#"{{"model":"{model}","answers":{{{}}},"usage":{{"input_tokens":9,"output_tokens":3}}}}"#,
            answers.join(",")
        ))
    })
    .expect("a loopback listener")
}

/// How many requests this listener received under each marker since the last
/// call, in `MARKERS` order, then the explicit marker, then requests with no
/// key at all. The listener hands each request over once.
pub(crate) fn by_marker(listener: &Listener) -> Counts {
    let mut counts = Counts::default();
    for request in listener.requests() {
        let header = request
            .header("authorization")
            .unwrap_or_default()
            .to_owned();
        let place = MARKERS
            .iter()
            .position(|(_, marker)| header == format!("Bearer {marker}"))
            .or_else(|| (header == format!("Bearer {EXPLICIT}")).then_some(EXPLICIT_PLACE))
            .unwrap_or(KEYLESS);
        counts[place] += 1;
    }
    counts
}

/// The counts that mean `n` requests under the marker at `place` and none other.
pub(crate) fn only(place: usize, n: usize) -> Counts {
    let mut counts = Counts::default();
    counts[place] = n;
    counts
}

/// A loopback proxy that counts each connection and closes it.
pub(crate) struct Proxy {
    pub(crate) url: String,
    count: Arc<AtomicUsize>,
}

impl Proxy {
    pub(crate) fn start() -> Self {
        let socket = TcpListener::bind("127.0.0.1:0").expect("a proxy port");
        let url = format!("http://{}", socket.local_addr().expect("an address"));
        let count = Arc::new(AtomicUsize::new(0));
        let seen = Arc::clone(&count);
        std::thread::spawn(move || {
            for stream in socket.incoming().flatten() {
                seen.fetch_add(1, Ordering::SeqCst);
                let mut line = String::new();
                let _read = BufReader::new(&stream).read_line(&mut line);
            }
        });
        Self { url, count }
    }

    pub(crate) fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

/// A scratch home: its own configuration, cache, and usage folders.
pub(crate) struct Home {
    pub(crate) root: PathBuf,
}

impl Home {
    pub(crate) fn new(name: &str) -> Self {
        static HOMES: AtomicUsize = AtomicUsize::new(0);
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "named-{name}-{}-{}",
            std::process::id(),
            HOMES.fetch_add(1, Ordering::Relaxed)
        ));
        let _absent = std::fs::remove_dir_all(&root);
        let home = Self { root };
        std::fs::create_dir_all(home.config_folder()).expect("a scratch home");
        home
    }

    /// The folder that holds the configuration file.
    fn config_folder(&self) -> PathBuf {
        crate::child::Folder::Config.under(&self.root)
    }

    /// Write the configuration file, or leave none when `text` is empty.
    pub(crate) fn config(&self, text: &str) -> &Self {
        if !text.is_empty() {
            let path = self.config_folder().join("config.json");
            std::fs::write(&path, text).expect("a configuration file");
            crate::child::private_file(&path).expect("a private configuration fixture");
        }
        self
    }

    /// The variables every run takes: the scratch folders and each marker.
    pub(crate) fn environment(&self) -> Vec<(String, String)> {
        let mut environment = vec![
            ("HOME".to_owned(), self.path("home")),
            ("THINKTHEN_TEST_RETRY_WAIT_MS".to_owned(), "1".to_owned()),
            (
                "THINKTHEN_TEST_INPUT_PAUSE_MS".to_owned(),
                "10000".to_owned(),
            ),
        ];
        // The configuration and cache folders go under the root. On macOS
        // that moves `HOME` from `home` to the root.
        for folder in [crate::child::Folder::Config, crate::child::Folder::Cache] {
            let (name, value) = folder.variable(&self.root);
            environment.push((name.to_owned(), value));
        }
        environment.extend(
            MARKERS
                .iter()
                .map(|(name, marker)| ((*name).to_owned(), (*marker).to_owned())),
        );
        environment
    }

    /// Write evidence lines to a file and return its path.
    pub(crate) fn evidence(&self, lines: &[&str]) -> String {
        let path = self.root.join("evidence.txt");
        std::fs::write(&path, lines.join("\n") + "\n").expect("an evidence file");
        path.to_string_lossy().into_owned()
    }

    pub(crate) fn path(&self, name: &str) -> String {
        self.root.join(name).to_string_lossy().into_owned()
    }

    /// Run the command with this home, these changes, and this input.
    ///
    /// A change with an empty value removes the variable.
    pub(crate) fn run(&self, arguments: &[&str], changes: &[(&str, &str)]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment().args(arguments);
        for (name, value) in self.environment() {
            command.env(name, value);
        }
        for (name, value) in changes {
            if value.is_empty() {
                command.env_remove(name);
            } else {
                command.env(name, value);
            }
        }
        crate::run::output(&mut command).expect("the command runs")
    }

    /// Assert no marker reached any file under this home.
    pub(crate) fn assert_no_marker_in_files(&self) {
        assert_no_marker_under(&self.root);
    }
}

/// Assert no marker, the explicit one included, sits in any file under `root`.
pub(crate) fn assert_no_marker_under(root: &Path) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            assert_no_marker_under(&path);
        } else if let Ok(bytes) = std::fs::read(&path) {
            assert_no_marker(
                &String::from_utf8_lossy(&bytes),
                &path.display().to_string(),
            );
        }
    }
}

/// Assert no marker sits in this text.
pub(crate) fn assert_no_marker(text: &str, what: &str) {
    for (_, marker) in MARKERS {
        assert!(!text.contains(marker), "{what} holds a marker");
    }
    assert!(!text.contains(EXPLICIT), "{what} holds the explicit marker");
}

/// Both outputs as text, after asserting neither holds a marker.
pub(crate) fn said(output: &Output) -> (String, String) {
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert_no_marker(&stdout, "standard output");
    assert_no_marker(&stderr, "standard error");
    (stdout, stderr)
}
