use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountObj {
    pub account_id: String,
    pub account_number: String,
    pub owner_customer_id: String,
    pub name: String,
    pub account_type: String,
    pub available: f32,
    pub balance: f32,
    pub credit_limit: f32,
}

impl fmt::Display for AccountObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
             Account Id:\t{}\n\
             Account Nr:\t{}\n\
             Name:\t\t{}\n\
             Type:\t\t{}\n\
             Available:\t{}\n\
             Balance:\t{}",
            self.account_id,
            self.account_number,
            self.name,
            self.account_type,
            self.available,
            self.balance
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
            writeln!(f, "{}\n", object)?;
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
        writeln!(
            f,
            "\
             First name:\t{}\n\
             Last name:\t{}\n\
             Email:\t\t{}\n\
             Phone numbers:",
            self.first_name, self.last_name, self.email_address
        )?;

        for number in &self.phone_numbers {
            writeln!(f, "{:}", number)?;
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
    pub card_number: String,
    pub currency_amount: f32,
    pub currency_rate: f32,
    pub merchant_category_code: String,
    pub merchant_category_description: String,
    pub merchant_city: String,
    pub merchant_name: String,
    pub original_currency_code: String,
    pub purchase_date: String,
    pub transaction_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionObj {
    pub accounting_date: String,
    pub interest_date: String,
    pub other_account_number_specified: bool,
    pub other_account_number: Option<String>,
    pub amount: f32,
    pub text: String,
    pub transaction_type: String,
    pub transaction_type_code: i32,
    pub transaction_type_text: String,
    pub is_reservation: bool,
    pub reservation_type: Option<String>,
    pub card_details_specified: bool,
    pub card_details: Option<CardDetailsObj>,
    pub transaction_id: String,
}

impl fmt::Display for TransactionObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Accounting date:\t{}", self.accounting_date)?;
        writeln!(f, "Interest date:\t\t{}", self.interest_date)?;
        if self.other_account_number_specified {
            writeln!(f, "Other account number:\t{:?}", self.other_account_number)?;
        }
        writeln!(f, "Amount:\t\t\t{}", self.amount)?;
        writeln!(f, "Description:\t\t{}", self.text)?;
        writeln!(
            f,
            "Reserved:\t\t{}",
            if self.is_reservation { "Yes" } else { "No" }
        )?;
        writeln!(f, "Transaction id:\t\t{}", self.transaction_id)?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub from_account_id: String,
    pub to_account_id: String,
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
