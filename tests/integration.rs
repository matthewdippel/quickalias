extern crate quick_alias;
extern crate tempfile;

static TEST_FIN: &'static str = "./tests/test_fin";

#[cfg(test)]

mod integration {
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};

    use tempfile;

    use quick_alias::AliasConfig;

    use TEST_FIN;

    #[test]
    fn test_file_write() {
        let mut ac = AliasConfig::new(PathBuf::new());
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        ac.add_alias("cp".to_string(), "cp -i".to_string());
        ac.add_alias("rebash".to_string(), "source ~/.bashrc".to_string());
        ac.dump_aliases_to_specified_file(&tmpfile).unwrap();
        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read
        let mut buf = String::new();
        tmpfile.read_to_string(&mut buf).unwrap();
        assert_eq!(
            "alias cp=\"cp -i\"\nalias rebash=\"source ~/.bashrc\"\n",
            buf
        );
    }

    #[test]
    fn test_file_read() {
        let mut ac = AliasConfig::new(PathBuf::new());
        let file = File::open(TEST_FIN).unwrap();
        ac.load_from_file(file).unwrap();
        assert_eq!(
            ac.remove_alias("cp".to_string()).unwrap(),
            "cp -i".to_string()
        );
        assert_eq!(
            ac.remove_alias("rebash".to_string()).unwrap(),
            "source ~/.bashrc".to_string()
        );
    }
}
