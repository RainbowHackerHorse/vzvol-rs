use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use figment::{Figment, providers::{Env, Serialized}};

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Env::prefixed("VZVOL_"))
        .extract()
        .expect("Configuring from environment should always succeed");

    // Do any special definitions for `config` as needed here

    config
});

#[derive(Serialize, Deserialize, Debug)]
struct Config {
vol_size:String,
vol_name: Option<String>,
fs_type: Option<FsType>,
vol_type: Option<VolType>,
// Zuser: Option<Zuser>, // Make enum later 
// ZUSERHOME=$( grep "${ZUSER}": /etc/passwd | awk -F ":" '{print $6}' )
host_zroot: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum FsType  {
    zfs,
    ufs,
    fat32,
    ext2,
    ext3,
    ext4,
    xfs,
}

#[derive(Serialize, Deserialize, Debug)]
enum VolType {
    VolTypeRaw,
    VolTypeVirtualbox,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vol_size:"10G".to_string(),
            vol_name: None,
            fs_type: None,
            host_zroot: None,
            vol_type: None
        }
    }
}

fn main() {
    println!("{:?}", *CONFIG);
}