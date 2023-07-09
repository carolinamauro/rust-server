use dotenv::dotenv;
use grupo_gpt::{bot::init::run_bot};

#[tokio::main]
async fn main() {
    // loads env variables from .env file
    dotenv().ok();
    /* let mut db = DataBase::new();

    db.start_db_connection().await; */
    //EJEMPLOS
        //println!("{:?}", db.search_movie_by_title("Toy Story").await);
        //println!("{:?}", db.search_movie_by_date_range( "2023-07-02 00:00:00".to_string(), "2023-07-02 00:00:00".to_string()).await);  
        //let mut seats = vec!();
        //seats.push(('A', 8));
        //seats.push(('A', 6));
        //seats.push(('A', 7));
        //println!("{:?}", db.buy_tickets( mongodb::bson::oid::ObjectId::parse_str("648680d984b08c29dcf00537").unwrap(), seats, String::from("Lionel Messi"), &String::from("2")).await);
        //println!("{:?}", db.get_available_seats( mongodb::bson::oid::ObjectId::parse_str("648680d984b08c29dcf00537").unwrap()).await);
    run_bot().await;
}
