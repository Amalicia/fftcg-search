use clap::Parser;
use serde_json::Result;
use std::fs;
use fftcg_search::model::card::Card;
use fftcg_search::model::root::Root;

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
    println!("Categories: {}, {}", format_text(card.cat_1), format_text(card.cat_2));
    println!("Job: {}", card.job);
    println!("Effect: {}", format_text(card.text));
    println!("---------------------")
}

fn format_text(text: String) -> String {
    text.replace("[[br]]   ", "\n")
        .replace("[[i]]", "")
        .replace("[[/]]", "")
        .replace("&middot;", "-")
        .replace("[[s]]", "(S)")
}
