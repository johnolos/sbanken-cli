pub struct Credentials {
    pub secret: String,
    pub client_id: String,
    pub customer_id: String,
}

impl Credentials {
    pub fn new(secret: String, client_id: String, customer_id: String) -> Credentials {
        Credentials {
            secret,
            client_id,
            customer_id,
        }
    }
}
