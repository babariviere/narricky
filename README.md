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
description = "move all gmail account to folder gmail"
conditions = [ "from contains gmail.com",
	"subject is Hello world" ]
actions = [ "move to gmail" ]
exceptions = []
```

## List of conditions
TODO

## List of actions
`no more rules` - Stop applying rules

`copy to <folder>` - Copy mail to folder

`move to <folder>` - Move mail to folder

`delete` - Delete mail (not permanent)

`permanent delete` - Delete mail (permanent)

`forward to <mail@address>` - Forward mail to an another person

`reply with <text>` - Reply to sender with text

`set flag <flag>` - Apply flag to mail

`remove flag <flag>` - Remove flag from mail

`clear flags` - Remove all flags from mail

`mark as important` - Mark mail as important

`mark as read` - Mark mail as read

## List of exceptions
TODO
