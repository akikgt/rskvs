
use clap::Clap;

#[derive(Clap)]
#[clap(version = std::env!("CARGO_PKG_VERSION"), author = "aki")]
struct Opts {
    cmd: String,
    #[clap(name = "KEY")]
    key: Option<String>,
    #[clap(name = "VALUE")]
    value: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut kv: kvs::KvStore = kvs::KvStore::new();

    match opts.cmd.as_str() {
        "set" => {
            panic!("unimplemented");
            // kv.set(opts.key.expect("unimplemented"), opts.value.expect("unimplemented"));
        },
        "get" => {
            panic!("unimplemented");
            // let res = kv.get(opts.key.expect("must specify key"));
            // print!("{}", res.unwrap());
        },
        "rm" => {
            panic!("unimplemented");
            // kv.remove(opts.key.expect("must specify key"));
        }
        _ => panic!("unimplemented"),
    }
}
