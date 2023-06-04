use super::handle_commands::command::Command;
use teloxide::{prelude::*, utils::command::BotCommands};

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        // Command::Start => {
        //     bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        // }
        Command::Search(movie) => {
            bot.send_message(msg.chat.id, format!("Buscando la película {}...", movie))
                .await?
        }
        Command::CinemaListings => {
            bot.send_message(msg.chat.id, "Mostrando las películas que están actualmente en cartelera...").await?
        }
        Command::Reserve {
            movie,
            cinema,
            date,
            time,
            tickets,
        } => {
            bot.send_message(msg.chat.id, format!("Reservando {} entradas para la película {} en el cine {} para el día {} a las {}...", tickets, movie, cinema, date, time))
                .await?
        }
        Command::Notify(movie) => {
            bot.send_message(msg.chat.id, format!("Activando las notificaciones para la película {}...", movie))
                .await?
        }
        Command::DisableNotifications(movie) => {
            bot.send_message(msg.chat.id, format!("Desactivando las notificaciones para la película {}...", movie))
                .await?
        }
    };

    Ok(())
}

pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
