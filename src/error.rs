

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
    }

    errors {
        MissingAccount {
            description("no account field in configuration file")
            display("no account field in configuration file")
        }
    }
}
