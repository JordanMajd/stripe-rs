fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // Define a verified bank account to charge
    let bank_id = "ba_1GxbKJHEjXW7vVNGve1q6DUL".parse::<stripe::BankAccountId>().expect("expected bank account to be valid");
    // Define customer who owns bank account 
    let customer_id = "cus_HWedOre34KrFEt".parse::<stripe::CustomerId>().expect("expected customer to be valid");

    // Define the charge
    let mut params = stripe::CreateCharge::new();
    params.amount = Some(1000);
    params.source = Some(stripe::ChargeSourceParams::BankAccount(bank_id));
    params.currency = Some(stripe::Currency::USD);
    params.customer = Some(customer_id);

    // Create the charge
    let charge = stripe::Charge::create(&client, params).unwrap();

    // Output in a ~prettyprint format
    println!(
        "Charge {{
    id: {:?},
    amount: {:?},
    created: {:?},
    status: {:?},
    ..
}}",
        charge.id, charge.amount, charge.created, charge.status
    );
}
