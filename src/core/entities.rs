use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountObj {
    pub account_number: String,
    pub customer_id: String,
    pub owner_customer_id: String,
    pub name: String,
    pub account_type: String,
    pub available: f32,
    pub balance: f32,
    pub credit_limit: f32,
    pub default_account: bool,
}

impl fmt::Display for AccountObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
             Account:\t{}\n\
             Name:\t\t{}\n\
             Type:\t\t{}\n\
             Available:\t{}\n\
             Balance:\t{}\n\
             Default:\t{}",
            self.account_number,
            self.name,
            self.account_type,
            self.available,
            self.balance,
            if self.default_account { "Yes" } else { "No" }
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleItems<T> {
    pub available_items: i32,
    pub items: Vec<T>,
    pub error_type: Option<String>,
    pub is_error: bool,
    pub error_message: Option<String>,
    pub trace_id: Option<String>,
}

impl<T: fmt::Display> fmt::Display for MultipleItems<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for object in &self.items {
            write!(f, "{}\n\n", object)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleItem<T> {
    pub item: T,
    pub error_type: Option<String>,
    pub error_message: Option<String>,
    pub is_error: bool,
    pub trace_id: Option<String>,
}

impl<T: fmt::Display> fmt::Display for SingleItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.item)
    }
}

pub type Account = SingleItem<AccountObj>;
pub type Accounts = MultipleItems<AccountObj>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressObj {
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub address_line4: Option<String>,
    pub country: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumberObj {
    pub country_code: String,
    pub number: String,
}

impl fmt::Display for PhoneNumberObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t\t+{} {}", self.country_code, self.number)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerObj {
    pub customer_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub date_of_birth: String,
    pub postal_address: AddressObj,
    pub street_address: AddressObj,
    pub phone_numbers: Vec<PhoneNumberObj>,
}

impl fmt::Display for CustomerObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
             First name:\t{}\n\
             Last name:\t{}\n\
             Email:\t\t{}\n\
             Phone numbers:\n",
            self.first_name, self.last_name, self.email_address
        )?;

        for number in &self.phone_numbers {
            write!(f, "{:}\n", number)?;
        }

        Ok(())
    }
}

pub type Customer = SingleItem<CustomerObj>;

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardDetailsObj {
    pub card_number: Option<String>,
    pub currency_amount: Option<f32>,
    pub currency_rate: Option<f32>,
    pub merchant_category_code: Option<String>,
    pub merchant_category_description: Option<String>,
    pub merchant_city: Option<String>,
    pub merchant_name: Option<String>,
    pub original_currency_code: Option<String>,
    pub purchase_date: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionObj {
    pub transaction_id: String,
    pub accounting_date: Option<String>,
    pub interest_date: Option<String>,
    pub other_account_number: Option<String>,
    pub amount: f32,
    pub text: Option<String>,
    pub transaction_type: Option<String>,
    pub transaction_type_code: Option<i32>,
    pub transaction_type_text: Option<String>,
    pub is_reservation: Option<bool>,
    pub card_details_specified: Option<bool>,
    pub card_details: Option<CardDetailsObj>,
}

impl fmt::Display for TransactionObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Transaction id:\t\t{}", self.transaction_id)?;
        if let Some(ref date) = self.accounting_date {
            writeln!(f, "Accounting date:\t{}", date)?;
        }
        if let Some(ref date) = self.interest_date {
            writeln!(f, "Interest date:\t\t{}", date)?;
        }
        if let Some(ref account_number) = self.other_account_number {
            writeln!(f, "Other account number:\t{}", account_number)?;
        }
        writeln!(f, "Amount:\t\t\t{}", self.amount)?;
        if let Some(ref text) = self.text {
            writeln!(f, "Description:\t\t{}", text)?;
        }
        if let Some(b) = self.is_reservation {
            writeln!(f, "Reserved:\t{}", if b { "Yes" } else { "No" })?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub from_account: String,
    pub to_account: String,
    pub message: String,
    pub amount: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    pub error_type: Option<String>,
    pub is_error: bool,
    pub error_message: Option<String>,
}

pub type Transactions = MultipleItems<TransactionObj>;
