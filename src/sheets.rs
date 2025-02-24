use gcp_auth::{CustomServiceAccount, TokenProvider};
use sheets::{
    Client, ClientError, Response,
    types::{
        AppendValuesResponse, DateTimeRenderOption, Dimension, InsertDataOption, ValueInputOption,
        ValueRange, ValueRenderOption,
    },
};

pub fn get_account(credentials_path: &str) -> CustomServiceAccount {
    CustomServiceAccount::from_file(credentials_path).expect("Failed to get auth provider")
}

pub async fn get_client(account: &CustomServiceAccount) -> Client {
    let scopes = &[
        "https://www.googleapis.com/auth/drive",
        "https://www.googleapis.com/auth/drive.file",
        "https://www.googleapis.com/auth/spreadsheets",
    ];
    let token = account.token(scopes).await.expect("Failed to get token");

    Client::new("", "", "", token.as_str(), "")
}

pub async fn append(
    account: &CustomServiceAccount,
    spreadsheet_id: &str,
    worksheet_name: &str,
    values: Vec<String>,
) -> Result<Response<AppendValuesResponse>, ClientError> {
    let client = get_client(account).await;
    let sheets = client.spreadsheets();

    assert!(values.len() <= 26);
    let range = format!("'{worksheet_name}'!A:A");
    sheets
        .values_append(
            spreadsheet_id,
            &range,
            false,
            InsertDataOption::InsertRows,
            DateTimeRenderOption::Noop,
            ValueRenderOption::Noop,
            ValueInputOption::UserEntered,
            &ValueRange {
                major_dimension: Some(Dimension::Rows),
                range: range.clone(),
                values: vec![values],
            },
        )
        .await
}
