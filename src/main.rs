extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;
use std::borrow::Borrow;

use sheets4::api::ValueRange;
use sheets4::Sheets;

static DUES_SHEET: &str = "1ntm3FVOFp2ZQ11EkzGLM_RCoFSOXQhwqdKzNHNI7tgQ";
static DONATIONS_SHEET: &str = "1aiODkmkKyFYaZFTTgzmpDtSyhH4FGKrH-krpOSufUSU";

async fn get_hub() -> Sheets {
    let secret = yup_oauth2::read_application_secret("secret.json")
        .await
        .expect("client secret could not be read");

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("token.json")
    .build()
    .await
    .unwrap();

    let sheets = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );
    return sheets;
}

async fn get_values(hub: &Sheets, sheet_id: &str) -> ValueRange {
    let result = hub
        .spreadsheets()
        .values_get(sheet_id, "A:H")
        .major_dimension("ROWS")
        .doit()
        .await;

    let response = match result {
        Ok(result) => result.1,
        Err(error) => panic!("Problem getting sheet: {:?}", error),
    };

    return response;
}

#[tokio::main]
async fn main() {
    let hub = get_hub().await;

    let donation_vals = get_values(hub.borrow(), DONATIONS_SHEET).await;
    let dues_vals = get_values(hub.borrow(), DUES_SHEET).await;

    println!("{:?}", donation_vals.values);
    println!("{:?}", dues_vals.values);
}
