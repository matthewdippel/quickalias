extern crate quick_alias;

static TEST_FIN: &'static str = "./tests/test_fin";

#[cfg(test)]

mod integration {
    use std::path::PathBuf;
    use std::fs::File;

    use quick_alias::AliasConfig;

    use TEST_FIN;

    #[test]
    fn test_file_write() {}
    #[test]
    fn test_file_read() {
        let mut ac = AliasConfig::new(PathBuf::new());
        let file = File::open(TEST_FIN).unwrap();
        ac.load_from_file(file);
        assert_eq!(
            ac.alias_to_command("cp".to_string()).unwrap(),
            "cp -i".to_string()
        );
        assert_eq!(
            ac.alias_to_command("rebash".to_string()).unwrap(),
            "source ~/.bashrc".to_string()
        );
    }
}
