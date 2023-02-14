use std::env;
use log::{Level, LevelFilter, Record};
use env_logger::Builder;

pub fn init() {
    let mut builder = Builder::new();

    if env::var("RUST_LOG").is_ok() {
        builder.parse_env("RUST_LOG");
    } else {
        let format = |record: &Record| {
            if record.level() == Level::Info {
                format!("{}", record.args())
            } else {
                format!(
                    "[{}:{}] {}",
                    record.location().module_path(),
                    record.level(),
                    record.args()
                )
            }
        };

        builder.format(format).filter(None, LevelFilter::Info);

        builder.parse_filters("wifi-connect=info,iron::iron=off");
    }

    builder.init().unwrap();
}
