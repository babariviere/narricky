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
sync = 60

## This is an example, this may change in the future
[rule.rule_name_here]
description = "move all gmail account to folder gmail"
any = true
conditions = [ "from contains gmail.com",
	"subject is Hello world" ]
actions = [ "move to gmail" ]
exceptions = []
```

If you want rules to be done in priority, just put a number before rule name.
Example `[rule.1_priority_one]`.

`any`, when it's true, stop check other conditions and directly do actions. By default, `any` is equal to false.

## Account configuration
First you have to set your `username` (email address) and `password`.

After that, you need (will be automatic after) to give the domain name of your imap server and is port.
`secure` field is here to enable a secure connection or not.

`sync` is the sync interval, in seconds, between each data poll. If you don't set it, it will be equal to 60 by default.

## List of conditions (and exceptions)
Conditions 3 fields:
- First one is the field, ex: recipient, sender...
- Second one is the checker, ex: is, contains...
- Third one is the text to check

Here are all the fields for now:

`sender` - Who send the mail

`recipient` - Who received the mail

`cc` - Who are in the copy field

`subject` - Mail subject
    
<br />

Here all the checkers for now:

`is` - Text should match exactly

`contains` - Text contains specified text

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
