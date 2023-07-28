use dotenv;
use mongodb::{
    bson::{doc, Document, Bson, oid::ObjectId},
    options::{ClientOptions, ServerApi, ServerApiVersion, UpdateOptions},
    Client,
    error::Error as MongoError,
    results::InsertOneResult,
};
use tokio_stream::StreamExt;
use chrono::{NaiveDateTime};
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct DataBase{
    can_buy : Arc<Mutex<()>>,
    mongoclient: Option<Client>
}

impl DataBase {
    pub fn new() -> Self {
        DataBase {
            can_buy : Arc::new(Mutex::new(())),
            mongoclient: None
        }
    }

 pub async fn connect_to_db(&self) ->  Result<Client, MongoError> {
    if let Ok(key) = dotenv::var("MONGO_URL") {
        let client = self.create_client(&key).await?;
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(client)
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get MONGO_URL from environment",
        )))
    }
}

async fn create_client(&self, key: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(key).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.app_name = Some("Cinema Telegram Bot".to_string());
    client_options.server_api = Some(server_api);
    Client::with_options(client_options)
}

pub async fn start_db_connection(&mut self) {
    let client = self.connect_to_db().await;
    if let Ok(connected_client) = client{
        self.mongoclient = Some(connected_client);
    }
    else{
        println!("Error connecting to data base");
    }
}

pub async fn search_movie_by_title(&self, title: &str) -> Result<Option<Document>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let movies = mongoclient.database("cinemaData").collection("movies");

        let movie: Result<Option<Document>, MongoError> = movies
        .find_one(
            doc! {
                    "original_title": remove_quotes(title),
            },
            None,
        ).await;
        movie
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    }
    
}

pub async fn search_movie_with_multiple_params(&self, title: &str, cinema: i32, datetimestring: String) -> Result<Option<Document>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let movies = mongoclient.database("cinemaData").collection("movies");

        if let Ok(datetime) = NaiveDateTime::parse_from_str(&datetimestring, "%Y-%m-%d %H:%M:%S"){
            let bson_datetime = Bson::DateTime(mongodb::bson::DateTime::from_millis(datetime.timestamp_millis()));

            let movie: Result<Option<Document>, MongoError> = movies
            .find_one(
                doc! {
                        "original_title": remove_quotes(title),
                        "cinema_id": cinema,
                        "show_time": bson_datetime
                },
                None,
            ).await;
            movie
        }else{
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Wrong date format",
            )))
        }
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    }
    
}

pub async fn search_movie_by_id(&self , id: ObjectId) -> Result<Option<Document>, MongoError>{
    
    if let Some(mongoclient) = &self.mongoclient {
        let movies = mongoclient.database("cinemaData").collection("movies");

        let movie: Result<Option<Document>, MongoError> = movies
        .find_one(
            doc! {
                    "_id": id,
            },
            None,
        ).await;
        movie
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    }
    
}

pub async fn search_movie_by_date_range(&self, from: String, to: String) -> Result<Vec<Document>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let movies = mongoclient.database("cinemaData").collection("movies");

        let from_date = NaiveDateTime::parse_from_str(&from, "%Y-%m-%d %H:%M:%S").unwrap();
        let to_date = NaiveDateTime::parse_from_str(&to, "%Y-%m-%d %H:%M:%S").unwrap();
        let bson_date_from = Bson::DateTime(mongodb::bson::DateTime::from_millis(from_date.timestamp_millis()));
        let bson_date_to = Bson::DateTime(mongodb::bson::DateTime::from_millis(to_date.timestamp_millis()));

        let mut results = vec!();

        let movie: Result<mongodb::Cursor<Document>, MongoError> = movies
        .find(
            doc! {
                "show_time": {
                    "$gte": bson_date_from,
                    "$lte": bson_date_to,
                },
            },
            None,
        ).await;
        if let Ok(mut cursor) = movie {
            // Convert the cursor into a stream
            while let Some(result) = cursor.try_next().await.unwrap() {
                results.push(result);
            }
            Ok(results)
        } else {
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error making query",
            )))
        }
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    }
    
}

pub async fn buy_tickets(&self, movie_id: ObjectId, seats: Vec<(char, usize)>, name: &String, chatid: &String) -> Result<Vec<String>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");
        let movies:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("movies");

        if let Ok(None) = self.get_client(chatid).await {
            self.create_new_client(chatid, name).await?;
        }

            let _res = self.can_buy.lock().await;
            // Perform the update operation
            if let Ok(true) = self.seats_are_available(movie_id, &seats).await{
                let mut reservations = vec!();
                for seat in seats {

                    let movie_filter = doc! {
                        "_id": movie_id,
                    };
                    let client_filter = doc! {
                        "chatid": chatid,
                    };
                    let reservation_id = movie_id.to_string() + &seat.0.to_string() + &seat.1.to_string();
                    let update = doc! {
                        "$push": {
                            "reservations": &reservation_id,
                        },
                    };
                    let update_2 = update.clone();
                
                    // Create the options to enable upsert (create the field if it doesn't exist)
                    let options = UpdateOptions::builder().upsert(true).build();
                    let options_2 = options.clone();
                    if let Ok(_res) = movies.update_one(movie_filter, update, options).await{
                        if let Ok(_res2) = clients.update_one(client_filter, update_2, options_2).await{
                            reservations.push(reservation_id);
                        }else{
                            break;
                        }
                    }else{
                        break;
                    }
                }
                Ok(reservations)
            }else{
                Err(MongoError::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "One or more seats are taken",
                )))
            }
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    }   
}

pub async fn create_new_client(&self, chatid: &String, name:&String ) -> Result<InsertOneResult, MongoError>{
    
    if let Some(mongoclient) = &self.mongoclient {
        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");

        let client = clients
        .insert_one(
            doc! {
                    "chatid": chatid,
                    "name": name
            },
            None,
        ).await;
        client
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    } 
    
}

pub async fn get_client(&self, chatid: &String)-> Result<Option<Document>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");

        clients
        .find_one(
            doc! {
                    "chatid": chatid.to_string(),
            },
            None,
        ).await

    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    } 
}
pub async fn get_available_seats(&self, movie_id: ObjectId) -> Result<Vec<String>, MongoError>{

    if let Some(mongoclient) = &self.mongoclient {
        let movies = mongoclient.database("cinemaData").collection("movies");

        let movie: Result<Option<Document>, MongoError> = movies
        .find_one(
            doc! {
                    "_id": movie_id,
            },
            None,
        ).await;
        if let Ok(Some(found_movie)) = movie{
            let all_seats: Vec<String> = (1..=12)
                .flat_map(|col| ('A'..='F').map(move |row| format!("{}{}", row, col)))//ASSUMING SEATS GO FROM A1 TO F12
                .collect();
            if let Some(Bson::Array(reservations)) = found_movie.get("reservations"){
                let unavailable_seats: Vec<&str> = reservations.iter()
                    .map(|seat| &seat.as_str().unwrap()[seat.as_str().unwrap().len() - 2 ..])
                    .collect();


                let available_seats: Vec<String> = all_seats
                    .into_iter()
                    .filter(|seat| !unavailable_seats.contains(&seat.as_str()))
                    .collect();

                Ok(available_seats)
            }else{
                Ok(all_seats)
            }
        }else{
            Err(MongoError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error finding movie",
            )))
        }
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not connected to database",
        )))
    } 

}

async fn seats_are_available(&self, movie_id: ObjectId, seats_to_buy: &Vec<(char, usize)>) -> Result<bool, MongoError>{
    if let Ok(available_seats) = self.get_available_seats(movie_id).await{
        Ok(seats_to_buy.iter().all(|seat| {
            let seat_string = seat.0.to_string() + &seat.1.to_string();
            available_seats.contains(&seat_string)
        }))
    }else{
        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error getting available seats",
        )))
    } 
}

}

fn remove_quotes(input: &str) -> &str {
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        &input[1..input.len() - 1]
    } else {
        input
    }
}
/* fn generate_random_date() -> NaiveDateTime {
    let start_date = NaiveDateTime::parse_from_str("2023-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end_date = NaiveDateTime::parse_from_str("2023-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let duration = end_date - start_date;
    let random_duration = Duration::minutes((rand::random::<i64>() % (duration.num_minutes() / 10)) * 10);
    let random_date = start_date + random_duration;

    // Set seconds to 0
    let random_date = random_date.with_second(0).unwrap();

    // Set minute to the nearest multiple of 10
    let minute = random_date.minute() / 10 * 10;
    let random_date = random_date.with_minute(minute).unwrap();

    random_date
}

pub async fn add_random_date_field(client:Client){


        let movies:Collection<Document> = client.database("cinemaData").collection("movies");

        let filter = doc! {};
        let options = FindOptions::default();
        let mut cursor = movies.find(filter, options).await.unwrap();

        // Iterate over the documents and update each one with a random date field
        while let Ok(Some(mut document)) = cursor.try_next().await {
            document.remove("run_date");
            // Generate a random date
            let random_date = generate_random_date();
            let bson_date = Bson::DateTime(mongodb::bson::DateTime::from_millis(random_date.timestamp_millis()));

            // Add the random date field to the document
            println!("{:?}",bson_date);
            let update_doc = doc! {
                "$set": {
                    "show_time": bson_date,
                },
            };

            // Update the document in the collection
            let filter = doc! {"_id": document.get_object_id("_id").unwrap()};
            match movies.update_one(filter, update_doc, None).await{
                Ok(res) => println!("{:?}", res),
                Err(res) => println!("{:?}", res),
            }
        }
    }   
} */