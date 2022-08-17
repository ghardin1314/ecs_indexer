use crate::prelude::*;

pub fn load_providers(mut commands: Commands) {
    let api_key = env::var("API_KEY").expect("no api key provided");
    let provider_http =
        Provider::<Http>::try_from(format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key))
            .expect("Error connecting to Ethereum node");
    let provider_ws =
        Provider::<Ws>::connect(format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key))
            .compat_await()
            .expect("Error connecting to websocket node");

    commands.insert_resource(provider_http);
    commands.insert_resource(provider_ws);

    println!("Loaded providers");
}
