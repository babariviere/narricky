# narricky
Apply rules to multiple mail account to keep them organized

## Configuration
For configuration, you need to create one TOML file for each account.

Config files are formated like this:

```toml
[account]
username = "your@mail.here"
password = "your password"
domain = "imap.gmail.com"
port = 993
secure = true

## This is an example, this may change in the future
[rule.rule_name_here]
conditions = [ "from contains gmail.com",
	"subject is Hello world" ]
actions = [ "move to gmail" ]
```

## List of conditions
TODO

# List of actions
TODO
