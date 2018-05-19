# sbanken-cli

A fast terminal client written in rust to use with sbanken's open APIs.

The client currently supports all the APIs available to date.

Link to Sbanken's swagger documentation:
- [Sbanken Bank API](https://api.sbanken.no/Bank/swagger/)
- [Sbanken Customers API](https://api.sbanken.no/Customers/swagger/)

Feedback and PRs are welcomed.

## Requirements
Requires Rust 1.26 or newer.

To build from source and installing, you can head over to [rustup](https://rustup.rs/) to get started.

## How to build from source and installing

```bash
$ cargo build --release
$ mv ./target/release/sbanken-cli /usr/local/bin
```

## Environment variables
`SBANKEN_CLIENT_ID`, `SBANKEN_SECRET` and `SBANKEN_CUSTOMER_ID` are required environment-variables.

`SBANKEN_CLIENT_ID` and `SBANKEN_SECRET` are provided through the developer-pages on [https://sbanken.no/](https://sbanken.no/). Navigate through `Mine instillinger > Sbanken Beta > Utviklerportalen`. `SBANKEN_CUSTOMER_ID` is your social security number.

Set `SBANKEN_COLOR=1` if you want colored output.

### Auto-complete scripts

Auto-complete scripts can be generated through the cli.

Bash:
```bash
$ sbanken-cli generate-bash-completions >> ~/.bashrc
```
Fish:
```bash
$ sbanken-cli generate-fish-completions > ~/.config/fish/completions/sbanken-cli.fish
```

##### Use envchain to manage your secrets for additional security (optional)
An optional opinionated way to store your credentials could be using [envchain](https://github.com/sorah/envchain)
for added security. How you store your secrets are entirely up to you. Storing your credentials i.e. `.bashrc` is a-okey.

Once installed you have installed envchain, do the following:

```bash
$ envchain --set sbanken SBANKEN_CLIENT_ID SBANKEN_SECRET SBANKEN_CUSTOMER_ID SBANKEN_COLOR
```
Using sbanken-cli with envchain will then become:
```bash
$ envchain sbanken sbanken-cli transactions -a <account> -f 2017-10-15 -t 2017-10-20 -l 30
```
Using envchain, will be prompted for your credentials based on your configuration and preference.

If that's a little tedious for your taste, you can always make it convenient:

_Bash example_
```bash
sbanken-cli() {
    envchain sbanken sbanken-cli "@"
}
```

_Fish example_
```fish
function sbanken-cli
    envchain sbanken sbanken-cli $argv
end
```

##### Moving forward
In time there might be a interactive mode, but for now you can very much tailor one for your use-case:

For example:
```fish
function transfer
	set from_account (sbanken-cli account -l  | fzf --prompt="From account: " | awk '{print $(NF)}' | sed 's/\]//g')
	sbanken-cli account -l | fzf -1 -q "$from_account" | xargs echo "From account: "
	set to_account (sbanken-cli account -l  | fzf --prompt="To account: " | awk '{print $(NF)}' | sed 's/\]//g')
	sbanken-cli account -l | fzf -1 -q "$to_account" | xargs echo "To account: "
	read --prompt "echo 'Amount: '" -l amount
	read --prompt "echo 'Message: '" -l message
	sbanken-cli transfer -f "$from_account" -t "$to_account" -a "$amount" -m "$message"
end
```
