use std::io::Result;
use semver::Version;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{read_dir, self};
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaStructure {
    pub versions : Box<[Version]>,
    pub migrations : HashMap<(Version, Version), Url>,
    pub schema_versions : Box<[Url]>
}


// pub fn schema_structure_from_local_directories(local_directories: [Url]) -> SchemaStructure {
    // so we want to walk the file tree, find the yaml files, look at the names
    // and figure out which files are migration files (rust files)
    // and which files are schema definition files (yaml files[someday these may be proto files,
    // json files, thrift files, but for now just looking for my own format)

    // for dir in local_directories {
    //    let path = Path::new(dir.path());
    // }



// }
// ok, so loop should be relatively simple, two vecs, one of files one of dirs, then you loop while
// there are dirs, each iteration you take one off the dirs vec and enumerate one level, putting
// the dirs with the dirs and the files with the files. then at the end you return the files.
// if you want to return dirs too you just have a third vec of dirs that have already been used and
// you put the dir that you popped off for a given iteration into that vec at the end of the
// iteration.
fn local_fs_enumerate_one_level(local_directories: Vec<Url>) -> Vec<Result<std::path::PathBuf>> {
    return local_directories
        .iter()
        .flat_map(|url| {
                  let dir_result = fs::read_dir(Path::new(url.path()));

                  match dir_result {
                      Ok(dir) =>
                          dir
                            .flat_map(|entry_result|
                              match entry_result {
                                Ok(entry) => vec![Ok(entry.path())],
                                Err(e) => { warn!("failed with error {}", e); vec![Err(e)] }
                              }
                            )
                            .collect(),
                      Err(e) => { warn!("failed with error {}", e); vec![Err(e)] }
                  }
        })
        .collect();
}

#[test]
fn test_local_fs_enumerate_one_level() {
    for file_path in local_fs_enumerate_one_level(vec![Url::parse("file:///").unwrap()]) {
        println!("{:?}", file_path)
    }
}
