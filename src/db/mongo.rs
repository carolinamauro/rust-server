use dotenv;
use mongodb::{
    bson::{doc, Document, Bson, oid::ObjectId},
    options::{ClientOptions, ServerApi, ServerApiVersion, UpdateOptions},
    Client,
    error::Error as MongoError,
};
use tokio_stream::StreamExt;
use chrono::{NaiveDateTime};
use teloxide::types::ChatId;


async fn connect_to_db() ->  Result<Client, MongoError> {
    if let Ok(key) = dotenv::var("MONGO_URL") {
        let client = create_client(&key).await?;
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

async fn create_client(key: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(key).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.app_name = Some("Cinema Telegram Bot".to_string());
    client_options.server_api = Some(server_api);
    Client::with_options(client_options)
}

pub async fn start_db_connection() {
    let client = connect_to_db().await;
    if let Ok(_connected_client) = client{
        //EJEMPLOS
        /*search_movie_by_title(&_connected_client, "Toy Story").await;
        search_movie_by_title(&_connected_client, "Jumanji").await; 
        search_movie_by_date_range(&_connected_client, "2023-06-12 00:00:00".to_string(), "2023-06-12 00:00:00".to_string()).await;  
        make_seat_reservation(&_connected_client, mongodb::bson::oid::ObjectId::parse_str("648680d984b08c29dcf00537").unwrap(), ('A', 2), String::from("Franco FV"), &String::from("1")).await;      
        get_available_seats(&_connected_client, mongodb::bson::oid::ObjectId::parse_str("648680d984b08c29dcf00537").unwrap()).await;*/

    }
    else{
        println!("Error al conectarse con la base de datos");
    }
}

pub async fn search_movie_by_title(client:&Client, title: &str){

        let movies = client.database("cinemaData").collection("movies");

        let movie: Result<Option<Document>, MongoError> = movies
        .find_one(
            doc! {
                    "original_title": title,
            },
            None,
        ).await;
        println!("Movie: {:?} \n", movie);
    
}

pub async fn search_movie_by_id(client:&Client, id: ObjectId){

        let movies = client.database("cinemaData").collection("movies");

        let movie: Result<Option<Document>, MongoError> = movies
        .find_one(
            doc! {
                    "_id": id,
            },
            None,
        ).await;
        println!("Movie: {:?} \n", movie);
    
}

pub async fn search_movie_by_date_range(client:&Client, from: String, to: String){

        let movies = client.database("cinemaData").collection("movies");

        let from_date = NaiveDateTime::parse_from_str(&from, "%Y-%m-%d %H:%M:%S").unwrap();
        let to_date = NaiveDateTime::parse_from_str(&to, "%Y-%m-%d %H:%M:%S").unwrap();
        let bson_date_from = Bson::DateTime(mongodb::bson::DateTime::from_millis(from_date.timestamp_millis()));
        let bson_date_to = Bson::DateTime(mongodb::bson::DateTime::from_millis(to_date.timestamp_millis()));

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
                println!("Found document: {:?}\n", result);
            }
        } else if let Err(error) = movie {
            println!("Error executing the query: {}", error);
        }
    
}

pub async fn make_seat_reservation(mongoclient:&Client, movie_id: ObjectId, seat: (char, usize), name: String, chatid: &String){

        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");
        let movies:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("movies");

        if let Ok(None) = get_client(mongoclient, chatid).await {
            create_new_client(mongoclient, chatid, name).await;
        }
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
    
        // Perform the update operation
        if let Ok(_res) = movies.update_one(movie_filter, update, options).await{
            if let Ok(_res2) = clients.update_one(client_filter, update_2, options_2).await{
                println!("Created reservation {}", &reservation_id);
            }else{
                println!("Error creating reservation for client");
            }
        }else{
            println!("Error creating reservation for movie");
        }   
}

pub async fn create_new_client(mongoclient:&Client, chatid: &String, name:String ){

        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");

        let client = clients
        .insert_one(
            doc! {
                    "chatid": chatid,
                    "name": name
            },
            None,
        ).await;
        println!("Client: {:?} \n", client);
    
}

pub async fn get_client(mongoclient:&Client, chatid: &String)-> Result<Option<Document>, MongoError>{

        let clients:mongodb::Collection<Document> = mongoclient.database("cinemaData").collection("clients");

        clients
        .find_one(
            doc! {
                    "chatid": chatid.to_string(),
            },
            None,
        ).await
}
pub async fn get_available_seats(client:&Client, movie_id: ObjectId){

    let movies = client.database("cinemaData").collection("movies");

    let movie: Result<Option<Document>, MongoError> = movies
    .find_one(
        doc! {
                "_id": movie_id,
        },
        None,
    ).await;
    if let Ok(Some(found_movie)) = movie{
        if let Some(Bson::Array(reservations)) = found_movie.get("reservations"){

            let all_seats: Vec<String> = (1..=12)
            .flat_map(|col| ('A'..='F').map(move |row| format!("{}{}", row, col)))//ASSUMING SEATS GO FROM A1 TO F12
            .collect();

            let unavailable_seats: Vec<&str> = reservations.iter()
                .map(|seat| &seat.as_str().unwrap()[seat.as_str().unwrap().len() - 2 ..])
                .collect();

            let available_seats: Vec<&String> = all_seats.iter()
                .filter(|&seat| !unavailable_seats.contains(&seat.as_str()))
                .collect();

            println!("UNAVAILABLE {:?}", unavailable_seats);
            println!("{:?}", available_seats);
        }else{
            println!("All seats are available");
        }
    }else{
        println!("Error finding movie of id: {}", movie_id)
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