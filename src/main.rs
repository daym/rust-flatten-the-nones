
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PspDirectoryEntryBlob {
        #[serde(default)]
        pub flash_location: Option<u32>,
        #[serde(default)]
        pub size: Option<u32>, // FIXME u64
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PspDirectoryEntry {
        #[serde(flatten)]
        //#[serde(default)]
        pub blob: Option<PspDirectoryEntryBlob>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct PspEntry {
        target: PspDirectoryEntry,
}

static A: &str = "{ target: {} }";

fn main() {
    let q = json5::from_str::<PspEntry>(A).unwrap();
    println!("{:?}", q);
}
