"""Run a lean Rust consumer with its own trace logger against loopback HTTP."""
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import subprocess
from threading import Thread

KEY = 'synthetic-library-canary-0238'


def check(consumer):
    """Use the caller's existing manifest and Cargo source configuration."""
    manifest = consumer / 'Cargo.toml'
    manifest.write_text(manifest.read_text() + 'log="0.4"\n')
    source = consumer / 'src'
    source.mkdir(exist_ok=True)
    (source / 'main.rs').write_text('''use std::sync::Mutex;
use thinkthen::{Decide, Key, Record, Settings};
struct Logger(Mutex<Vec<String>>);
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool { true }
    fn log(&self, record: &log::Record<'_>) {
        self.0.lock().unwrap().push(format!("{} {}", record.target(), record.args()));
    }
    fn flush(&self) {}
}
static LOGGER: Logger = Logger(Mutex::new(Vec::new()));
fn main() -> Result<(), thinkthen::Error> {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    let engine = Settings::default().url(std::env::args().nth(1).unwrap())
        .api_key(Key::new("CANARY_KEY").unwrap()).engine(|_| None)?;
    let row = engine.run(&Decide::new("Ready?")?, Record::text("evidence"))?;
    assert!(row.error().is_none());
    log::debug!(target: "embedding_host", "host debug enabled");
    let records = LOGGER.0.lock().unwrap();
    assert!(records.iter().any(|message| message.contains("host debug enabled")));
    // The HTTP dependency splits wire bytes across trace rows. Refuse the
    // header dump itself, including keys that straddle rows.
    assert!(!records.iter().any(|message| message.contains("CANARY_KEY")
        || message.contains("authorization") || message.starts_with("ureq_proto::util ")),
        "transport logging exposed request headers");
    Ok(())
}
'''.replace('CANARY_KEY', KEY))
    requests = []

    class Backend(BaseHTTPRequestHandler):
        def do_POST(self):
            assert self.headers.get('Authorization') == f'Bearer {KEY}'
            request = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
            requests.append(self.path)
            body = json.dumps({'answers': {name: {'type': 'noul', 'noul': 0.9}
                                           for name in request['questions']}}).encode()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_args):
            pass

    with HTTPServer(('127.0.0.1', 0), Backend) as server:
        thread = Thread(target=server.serve_forever)
        thread.start()
        try:
            for profile in ([], ['--release']):
                subprocess.run(['cargo', 'run', '--offline', '--quiet', *profile, '--',
                                f'http://127.0.0.1:{server.server_port}/v1'], cwd=consumer, check=True)
        finally:
            server.shutdown()
            thread.join()
    assert requests == ['/v1/systemone', '/v1/systemone']
    compatible = manifest.read_text()
    manifest.write_text(compatible.replace(
        'log="0.4"', 'log={version="0.4",features=["release_max_level_trace"]}'))
    incompatible = subprocess.run(['cargo', 'check', '--offline', '--quiet', '--release'],
                                  cwd=consumer, text=True, capture_output=True)
    manifest.write_text(compatible)
    assert incompatible.returncode != 0, 'release trace must not override credential secrecy'
    assert 'multiple release_max_level_* features set' in incompatible.stderr, \
        'release trace must fail because its logging ceiling conflicts'
    print('lean library consumer: debug and release retain no headers or key; release trace fails safely')
