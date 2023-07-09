use super::handle_commands::command::Command;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::db::mongo::DataBase;
use chrono::{DateTime, Utc};
use mongodb::bson::{Bson, oid::ObjectId};
use lazy_static::lazy_static;
use std::sync::Arc;
use::tokio::sync::RwLock;

lazy_static! {
    static ref DB: Arc<RwLock<DataBase>> = Arc::new(RwLock::new(DataBase::new()));
}

fn parse_seats(seats:String)-> Vec<(char, usize)>{
    seats
        .split(',')
        .map(|s| {
            let mut chars = s.trim().chars();
            let char_value = chars.next().unwrap_or('\0');
            let num_value = chars.collect::<String>().parse().unwrap_or(0);
            (char_value, num_value)
        })
        .collect()
}

fn bson_array_to_string_vec(bson_array: &Bson) -> Option<Vec<String>> {
    if let Bson::Array(array) = bson_array {
        let strings: Vec<String> = array
            .iter()
            .filter_map(|element| {
                if let Bson::String(string_value) = element {
                    Some(string_value.clone())
                } else {
                    None
                }
            })
            .collect();
        Some(strings)
    } else {
        None
    }
}

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
            let db_lock = DB.read().await;
            if let Ok(Some(res)) = db_lock.search_movie_by_title(&movie).await{
                if let Bson::DateTime(bson_date_time) = res.get("show_time").unwrap() {
                    // Convert the BsonDateTime to a DateTime<Utc>
                    let time : DateTime<Utc>= bson_date_time.to_system_time().into();
                
                    // Use the datetime as needed
                    bot.send_message(msg.chat.id, format!("Title: {}\n Overview: {}\n Date: {} \n Date: {} \n Cinema: {} \n Room: {}", res.get("original_title").unwrap(), res.get("overview").unwrap(), time.date_naive(), time.time(), res.get("cinema_id").unwrap(), res.get("room_id").unwrap()))
                    .await?
                } else {
                    bot.send_message(msg.chat.id, format!("No show-time found for movie"))
                    .await?
                }
            }else{
                bot.send_message(msg.chat.id, format!("Movie not found"))
                    .await?
            }
        }
        Command::CinemaListings => {
            let db_lock = DB.read().await;
            let current_date = Utc::now().date_naive();
            let date_string = current_date.format("%Y-%m-%d").to_string();
            let date_string2 = date_string.clone();
            if let Ok(vec) = db_lock.search_movie_by_date_range( date_string + " 00:00:00", date_string2 + " 23:59:59").await{
                for movie in vec {
                    if let Bson::DateTime(bson_date_time) = movie.get("show_time").unwrap() {
                        // Convert the BsonDateTime to a DateTime<Utc>
                        let time : DateTime<Utc>= bson_date_time.to_system_time().into();
                    
                        // Use the datetime as needed
                        bot.send_message(msg.chat.id, format!("Title: {}\n Date: {} \n Date: {} \n Cinema: {}", movie.get("original_title").unwrap(), time.date_naive(), time.time(), movie.get("cinema_id").unwrap()))
                        .await?;
                    } else {
                        bot.send_message(msg.chat.id, format!("No show-time for movie: {}", movie.get("original_title").unwrap()))
                        .await?;
                    }
                }
                bot.send_message(msg.chat.id, format!("End of movies showing today."))
                    .await?
            }else{
                bot.send_message(msg.chat.id, format!("There are no movies showing today."))
                    .await?
            }
        }
        Command::BuyTickets {
            username,
            movie,
            cinema,
            date,
            time,
            seats,
        } => {
            let db_lock = DB.read().await;
            let date_clone = date.clone();
            if let Ok(Some(res)) = db_lock.search_movie_with_multiple_params(&movie, cinema, date + " " + &time).await{
                let vec_seats = parse_seats(seats);
                let vec_seats_clone = vec_seats.clone();
                if let Ok(_vec) = db_lock.buy_tickets( res.get("_id").unwrap().as_object_id().unwrap(), vec_seats, &username, &msg.chat.id.to_string()).await{
                    bot.send_message(msg.chat.id, format!("{} has bought the seats {:?} for movie {} at cinema {} which will be showing the {} at {}.",username, vec_seats_clone, movie, cinema, date_clone, time))
                    .await?
                }else{
                    bot.send_message(msg.chat.id, "One or more seats are taken")
                    .await?
                }
            }else{
                bot.send_message(msg.chat.id, format!("Movie not found. Check date and time format"))
                    .await?
            }
        }
        Command::CheckSeats{
            movie,
            cinema,
            date,
            time,
        } => {

            let db_lock = DB.read().await;
            if let Ok(Some(res)) = db_lock.search_movie_with_multiple_params(&movie, cinema, date + " " + &time).await{
                if let Ok(vec) = db_lock.get_available_seats(res.get("_id").unwrap().as_object_id().unwrap()).await{
                    bot.send_message(msg.chat.id, format!("{:?}", vec))
                    .await?
                }else{
                    bot.send_message(msg.chat.id, "Error when getting available seats")
                    .await?
                }
            }else{
                bot.send_message(msg.chat.id, format!("Movie not found. Check date and time format"))
                    .await?
            }
        }
        Command::CheckReservations => {

            let db_lock = DB.read().await;
            if let Ok(Some(client)) = db_lock.get_client(&msg.chat.id.to_string()).await{
                if let Some(reservations) = client.get("reservations"){
                    let vec = bson_array_to_string_vec(reservations).unwrap();
                    bot.send_message(msg.chat.id, "The following are all the tickets you have bought:")
                        .await?;
                    for reservation in vec{
                        let len = reservation.len();
                        let (movieid, seat) = reservation.split_at(len - 2);
                        if let Ok(Some(movie)) = db_lock.search_movie_by_id(ObjectId::parse_str(movieid.to_string()).unwrap()).await{
                            if let Bson::DateTime(bson_date_time) = movie.get("show_time").unwrap() {
                                // Convert the BsonDateTime to a DateTime<Utc>
                                let time : DateTime<Utc>= bson_date_time.to_system_time().into();
                            
                                // Use the datetime as needed
                                bot.send_message(msg.chat.id, format!("Title: {}\n Date: {} \n Date: {} \n Cinema: {} \n Room: {} \n Seat: {}", movie.get("original_title").unwrap(), time.date_naive(), time.time(), movie.get("cinema_id").unwrap(), movie.get("room_id").unwrap(), seat))
                                .await?;
                            } else {
                                bot.send_message(msg.chat.id, format!("No show-time found for movie"))
                                .await?;
                            }
                        }
                    }
                    bot.send_message(msg.chat.id, format!("End of reservations"))
                        .await?
                }else{
                    bot.send_message(msg.chat.id, format!("No reservations found"))
                    .await?
                }
            }else{
                bot.send_message(msg.chat.id, format!("No client registered"))
                    .await?
            }
        }
    };

    Ok(())
}

pub async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    {
        let mut db_mut = DB.write().await;
        db_mut.start_db_connection().await;
    }

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
