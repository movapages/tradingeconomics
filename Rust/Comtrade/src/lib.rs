use std::error::Error;

pub fn get_comtrade_categories(
    base_url: String,
    client_key: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/categories");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    println!("-----------------------Get detailed information about comtrade main categories----------------------");
    println!("{:#?}", resp);
    Ok(())
}

pub fn get_comtrade_countries(
    base_url: String,
    client_key: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/countries");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    println!("-----------------------Get detailed information about comtrade countries----------------------");
    println!("{:#?}", resp);
    Ok(())
}

pub fn get_comtrade_by_country(
    base_url: String,
    client_key: String,
) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/country/sweden");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    Ok(resp)
}

pub fn get_comtrade_by_country_page(
    base_url: String,
    client_key: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/country");
    let params = String::from("/sweden");
    let page = String::from("/2");
    let url = format!("{}{}{}{}?c={}", base_url, path, params, page, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    println!("-----------------------Get comtrade data by specific country and page----------------------");
    println!("{:#?}", resp);
    Ok(())
}

pub fn get_comtrade_between_two_countries(
    base_url: String,
    client_key: String,
) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/country/mexico/sweden/1");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    Ok(resp)
}

pub fn get_comtrade_historical(
    base_url: String,
    client_key: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path = String::from("/comtrade/historical/PRTESP24031");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    println!("-----------------------Get comtrade historical data by symbol----------------------");
    println!("{:#?}", resp);
    Ok(())
}

pub fn get_crude_oil_historical(
    base_url: String,
    client_key: String,
) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let path = String::from("/markets/historical/WTICOUSD:CUR");
    let url = format!("{}{}?c={}", base_url, path, client_key);
    let resp = reqwest::blocking::get(url)?.json::<serde_json::Value>()?;
    Ok(resp)
}
