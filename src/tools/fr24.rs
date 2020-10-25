use reqwest;
use regex::Regex;

async fn get_fr24_airport(code: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://www.flightradar24.com/data/airports/{}/routes", code);
    let result = reqwest::get(&url).await?.text().await?;

    let re = Regex::new(r"arrRoutes=(\[[^]]+\])").unwrap();
    let s = re.captures(&result).unwrap().get(1).unwrap();

    Ok(String::from(s.as_str()))
}

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} (airport)", args[0]);
        std::process::exit(1);
    }

    let res = get_fr24_airport(&args[1]).await;
    match res {
        Ok(json) => {
            print!("{}", json);
        },
        Err(_) => ()
    };
}
