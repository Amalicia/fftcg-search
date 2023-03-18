use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Parser)]
#[clap(name = "fftcg-search", version, author)]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand
}

#[derive(Parser)]
enum SubCommand {
    SearchByName {
        #[clap(short = 'n', long)]
        name: String
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    count: u32,
    cards: Vec<Card>
}

#[derive(Debug, Serialize, Deserialize)]
struct Card {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Cost")]
    cost: String,
    #[serde(rename = "Power")]
    power: String,
    #[serde(rename = "Category_1")]
    cat_1: String,
    #[serde(rename = "Category_2")]
    cat_2: String,
    #[serde(rename = "Name_EN")]
    name: String,
    #[serde(rename = "Type_EN")]
    typ: String,
    #[serde(rename = "Job_EN")]
    job: String,
    #[serde(rename = "Text_EN")]
    text: String,
    #[serde(rename = "Set")]
    set: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let cards = get_cards();

    match opts.command {
        SubCommand::SearchByName {
            name
        } => {
            cards?.into_iter()
                .filter(|card| card.name.contains(&name))
                .for_each(print_card);
        }
    }

    // TODO: Figure out cloudflare workaround
    // let req = reqwest::get("https://104.18.38.130/en/get-cards")
    //     .await?;
    //
    // println!("{}", req.status());
    // println!("{}", req.text().await?);

    // let resp: Root = req.json().await?;

    // println!("{:#?}", resp);

    Ok(())
}

fn get_cards() -> Result<Vec<Card>> {
    let file_contents = fs::read_to_string("cards.json").expect("File read");
    let cards: Root = serde_json::from_str(file_contents.as_str())?;

    return Ok(cards.cards)
}

fn print_card(card: Card) {
    println!("{} ({})", card.name, card.code);
    println!("Cost: {}", card.cost);
    println!("Power: {}", card.power);
    println!("Categories: {}, {}", card.cat_1, card.cat_2);
    println!("Job: {}", card.job);
    println!("Effect: {}", format_text(card.text));
    println!("---------------------")
}

fn format_text(text: String) -> String {
    text.replace("[[br]]   ", "\n")
}
