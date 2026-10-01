use thinkthen::{Description, Engine, Question};

thinkthen::choices! {
    enum Team {
        Billing => "billing",
        Shipping => "shipping",
        Account => "account",
    }
}

let tt = Engine::from_env()?;
let question = "Which team owns this?";
let billing =
    Description::text("Invoices, fees, and refunds.")?;
let shipping = Description::text("Parcels and delivery.")?;
let account = Description::text("Logins and passwords.")?;
let team_question = Question::choose::<Team>(question)?
    .option(Team::Billing, Some(billing))?
    .option(Team::Shipping, Some(shipping))?
    .option(Team::Account, Some(account))?
    .build()?;
let parcel = "My parcel went to the wrong address.";
let team = tt.choose(&team_question, parcel)?.into_value();
assert_eq!(team, Some(Team::Shipping));
