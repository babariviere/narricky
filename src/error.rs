
const MISSING_ACCOUNT_ERR: &'static str = "Account field is missing from config file, \
                to fix this error please add these lines at the start \
                into your config file:\n\
                [account]\n\
                username = \"your@mail.here\"\n\
                password = \"your password\"\n\
                domain = \"imap.gmail.com\"\n\
                port = 993\n\
                secure = true\n";

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Imap(::imap::error::Error);
        OpensslStack(::openssl::error::ErrorStack);
        TomlDe(::toml::de::Error);
    }

    errors {
        InvalidAction(action: String) {
            description("given action is invalid")
            display("action `{}` is invalid", action)
        }
        MissingAccount {
            description("no account field in configuration file")
            display("{}", MISSING_ACCOUNT_ERR)
        }
    }
}
