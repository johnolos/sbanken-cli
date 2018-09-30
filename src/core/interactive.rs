use core::entities::AccountObj;
use core::error::Error;
use io;
use regex::Regex;
use std::process::{Command, Stdio};

pub fn fuzzy_match_account<'a>(
    accounts: &'a [AccountObj],
    header: &str,
) -> Result<&'a AccountObj, Error> {
    let mut fzf = Command::new("fzf")
        .arg("--header")
        .arg(header)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn `fzf`");

    let mut input = String::new();
    for account in accounts {
        input.push_str(&format!(
            "{}\t\t[nr: {}]\n",
            account.name, account.account_number
        ))
    }

    io::copy(&mut input.as_bytes(), fzf.stdin.as_mut().unwrap())
        .expect("Could not pipe stdin to fzf process");
    let output = fzf.wait_with_output().unwrap();

    let account_picked = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r".+\[nr: (?P<account_nr>\w+)\]")?;
    let caps = match re.captures(&account_picked) {
        Some(capture) => capture,
        None => return Err(Error::Message("couldn't find a capture")),
    };

    if let Some(account_nr) = caps.name("account_nr") {
        for account in accounts {
            if account.account_number == account_nr.as_str() {
                return Ok(account);
            }
        }
    }

    Err(Error::NoAccountFound("Could not an account"))
}

pub fn remove_account<'a>(
    accounts: &'a mut Vec<AccountObj>,
    account_id: &str,
) -> &'a mut Vec<AccountObj> {
    let mut index_to_delete: Option<usize> = None;
    for (i, account) in accounts.iter_mut().enumerate() {
        if account.account_id == *account_id {
            index_to_delete = Some(i);
            break;
        }
    }
    if let Some(i) = index_to_delete {
        accounts.remove(i);
    }
    accounts
}
