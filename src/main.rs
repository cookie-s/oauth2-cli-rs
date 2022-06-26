use oauth2::TokenResponse;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    RedirectUrl, TokenUrl,
};
use std::io::Write;
use std::{env, io};

fn main() {
    let client = BasicClient::new(
        env::var("OAUTH_CLIENT_ID")
            .map(ClientId::new)
            .expect("OAUTH_CLIENT_ID"),
        env::var("OAUTH_CLIENT_SECRET").map(ClientSecret::new).ok(),
        env::var("OAUTH_AUTH_URL")
            .map(|s| AuthUrl::new(s).expect("Invalid AuthUrl"))
            .expect("OAUTH_AUTH_URL"),
        env::var("OAUTH_TOKEN_URL")
            .map(|s| TokenUrl::new(s).expect("invalid TokenUrl"))
            .ok(),
    )
    .set_redirect_uri(
        env::var("OAUTH_REDIRECT_URI")
            .map(|s| RedirectUrl::new(s).expect("Invalid RedirectUri"))
            .expect("OAUTH_REDIRECT_URI"),
    );

    let code = {
        print!("Code: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("invalid input");
        buf.trim().into()
    };

    let resp = client
        .exchange_code(AuthorizationCode::new(code))
        .request(http_client)
        .expect("something wrong happend");

    let access_token = resp.access_token();

    println!("Access Token Secret: {}", access_token.secret());
}
