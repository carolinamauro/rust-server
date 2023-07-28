use teloxide::utils::command::BotCommands;

fn parse_check_seats(input: String) -> Result<(i32, String, String, String), teloxide::utils::command::ParseError> {
    let mut parts = input.split_whitespace();
    let cinema = parts.next().unwrap().parse().ok().unwrap();
    let date = parts.next().unwrap().to_string();
    let time = parts.next().unwrap().to_string();
    let movie = parts.collect::<Vec<&str>>().join(" ").to_string(); 

    Ok((cinema, date, time, movie))
}

fn parse_buy_tickets(input: String) -> Result<(String, i32, String, String, String, String), teloxide::utils::command::ParseError> {
    let mut parts = input.split_whitespace();

    let username = parts.next().unwrap().to_string();
    let cinema = parts.next().unwrap().parse().ok().unwrap();
    let date = parts.next().unwrap().to_string();
    let time = parts.next().unwrap().to_string();
    let seats = parts.next().unwrap().to_string();
    let movie = parts.collect::<Vec<&str>>().join(" ").to_string(); 

    Ok((username, cinema, date, time, seats, movie))
}
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Shows this help text")]
    Help,
    // #[command(
    //     description = "Inicia la interacción y proporciona una breve descripción de sus funcionalidades."
    // )]
    // Start,
    #[command(description = "Searches for given movie")]
    Search(String),
    #[command(description = "Shows movies that are showing today")]
    CinemaListings,
    #[command(
        description = "Buys tickets for given movie. Must enter username, cinema, date, time, seats and movie name. For an example: /buytickets <username> <cinema> <date> <time> <seats separated by commas> <movie name>",
        parse_with = parse_buy_tickets
    )]
    // aca abria que hacerlo escalable, varias peliculas, varios cines, etc. Tambien podríamos dar la opcion de recibir notificaciones o recordatorios dada una reserva
    BuyTickets {
        username: String,
        cinema: i32,
        date: String,
        time: String,
        seats: String,
        movie: String,
    },
    #[command(
        description = "Check available seats for a movie. For an example: /checkseats <cinema> <date> <time> <movie name>",
        parse_with = parse_check_seats
    )]
    CheckSeats {
        cinema: i32,
        date: String,
        time: String,
        movie: String,
    },
    #[command(
        description = "Check all the tickets you have bought",
    )]
    CheckReservations,
}
