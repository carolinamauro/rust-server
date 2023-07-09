use teloxide::utils::command::BotCommands;

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
        description = "Buys tickets for given movie. Must enter username, movie name, cinema, date, time and seats. For an example: /buytickets <username> <moviename> <cinema> <date> <time> <seats separated by commas>",
        parse_with = "split"
    )]
    // aca abria que hacerlo escalable, varias peliculas, varios cines, etc. Tambien podríamos dar la opcion de recibir notificaciones o recordatorios dada una reserva
    BuyTickets {
        username: String,
        movie: String,
        cinema: i32,
        date: String,
        time: String,
        seats: String,
    },
    #[command(
        description = "Check available seats for a movie. For an example: /checkseats <movie name> <cinema> <date> <time>",
        parse_with = "split"
    )]
    CheckSeats {
        movie: String,
        cinema: i32,
        date: String,
        time: String,
    },
    #[command(
        description = "Check all the tickets you have bought",
    )]
    CheckReservations,
}
