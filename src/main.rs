use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveName,
    ReceiveNo {
        name: String,
    },
    ReceiveSymbol {
        name: String,
        no: u8,
    },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveName].endpoint(receive_name))
            .branch(dptree::case![State::ReceiveNo { name }].endpoint(receive_no))
            .branch(
                dptree::case![State::ReceiveSymbol { name, no }].endpoint(receive_symbol),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! Enter Your Fav Crypto?").await?;
    dialogue.update(State::ReceiveName).await?;
    Ok(())
}

async fn receive_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "How much you Holding?").await?;
            dialogue.update(State::ReceiveNo { name: text.into() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn receive_no(
    bot: Bot,
    dialogue: MyDialogue,
    name: String, // Available from `State::ReceiveAge`.
    msg: Message,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(no)) => {
            bot.send_message(msg.chat.id, "What's The Symbol of Your Crypto Holding").await?;
            dialogue.update(State::ReceiveSymbol { name, no }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}

async fn receive_symbol(
    bot: Bot,
    dialogue: MyDialogue,
    (name, no): (String, u8), // Available from `State::ReceiveLocation`.
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(symbol) => {
            let report = format!("Cyptocurrency: {name}\nCoins: {no}\nSymbol: {symbol}");
            bot.send_message(msg.chat.id, report).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
