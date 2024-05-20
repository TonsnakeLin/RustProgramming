use clap::{crate_authors, App, Arg};
use tokio::runtime::{Builder, Runtime};
use std::sync::Arc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use mini_redis::Result;
use my_redis::redis_get_set;


fn new_multi_thread_runtime() -> Arc<Runtime> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("linpin-thread")
        .thread_stack_size(16 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    Arc::new(runtime)
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct TokioExampleConfig {
    pub block_in_async_driver: bool, 
}   

impl Default for TokioExampleConfig {
    fn default() -> TokioExampleConfig {
        TokioExampleConfig { 
            block_in_async_driver: false,
         }
    }
}

pub fn get_bool_from_string(switch : &str) -> bool {
    if switch.eq_ignore_ascii_case("true") {
        println!("run in biad mode");
        return true;
    }
    false
}

#[tokio::main]
async fn main() -> Result<()>{
    let app = App::new("tokio_example")
    .about("A program used to show tokio usage")
    .author(crate_authors!("linpin"))
    .arg(
        Arg::with_name("block-in-async-driver")
        .short("biad")
        .long("block-in-async-driver")
        .value_name("int")
        .help("execute the case blocking in the async driver")
        .takes_value(true),
    )
    .get_matches();

    let mut config = TokioExampleConfig::default();
    if let Some(biad) = app.value_of("block-in-async-driver") {
        config.block_in_async_driver = get_bool_from_string(biad);
    }

    redis_get_set().await?;
    println!("
    ");

    let rt = new_multi_thread_runtime();

    if config.block_in_async_driver {
        rt.block_on(async {
            println!("this is executed by block on")
        });
    }

    Ok(())
}
