use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Muestra este texto de ayuda")]
    Help,
    // #[command(
    //     description = "Inicia la interacción y proporciona una breve descripción de sus funcionalidades."
    // )]
    // Start,
    #[command(description = "Realiza una búsqueda de la película solicitada.")]
    Search(String),
    #[command(description = "Muestra la lista de películas que están actualmente en cartelera")]
    CinemaListings,
    #[command(
        description = "Reserva entradas para la película seleccionada. Proporcionar el nombre de la película, el cine, la fecha, la hora y la cantidad de entradas a reservar. Ejemplo: /reserve <nombre de la película> <nombre del cine> <fecha> <hora> <cantidad de entradas>",
        parse_with = "split"
    )]
    // aca abria que hacerlo escalable, varias peliculas, varios cines, etc. Tabien podríamos dar la opcion de recibir notificaciones o recordatorios dada una reserva
    Reserve {
        movie: String,
        cinema: String,
        date: String,
        time: String,
        tickets: u8,
    },
    #[command(
        description = "Activa las notificaciones para recibir actualizaciones y recordatorios sobre una película en específico"
    )]
    Notify(String),
    #[command(description = "Desactiva las notificaciones de una película en específico")]
    DisableNotifications(String),
}
