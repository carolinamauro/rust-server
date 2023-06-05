use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct SalaCine {
    //Formato clave podria ser {"NombrePelicula|FechayHora"} 
    //Vec seria los asientos
    cant_asientos: usize,
    peliculas: HashMap<String, Arc<Mutex<Vec<bool>>>>
}


impl SalaCine {
    pub fn new(cant_asientos:usize) -> Self {
        SalaCine {
            cant_asientos,
            peliculas: HashMap::new(),
        }
    }

    pub fn insertar_pelicula(&mut self, clave: String){
        let asientos = Arc::new(Mutex::new(vec![true; self.cant_asientos]));
        self.peliculas.insert(clave, asientos);
    }

    pub fn cancelar_pelicula(&mut self, clave: String){
        self.peliculas.remove(&clave);
    }

    pub fn get_asientos_disponibles(&self, asientos:MutexGuard<'_, Vec<bool>>) -> usize{
        asientos.iter().filter(|&&seat| seat).count()
    }

    pub fn get_cant_asientos_disponibles_pelicula(&self, clave: String) -> Result<usize, String>{
        if let Some(asientos_lock) = self.peliculas.get(&clave){
            if let Ok(asientos) = asientos_lock.lock(){
                Ok(self.get_asientos_disponibles(asientos))
            }else{
                Err("Error al buscar asientos disponibles".to_string())
            }
        }else{
            Err("No existe esa pelicula para esta sala".to_string())
        }
    }

    pub fn reservar_asientos_pelicula(&self, clave: String, cantidad: usize) -> Result<usize, String>{
        
        if let Some(asientos_lock) = self.peliculas.get(&clave){
            if let Ok(asientos) = asientos_lock.lock(){
                let mut asientos_clone = asientos.clone();
                if cantidad <= self.get_asientos_disponibles(asientos) {
                    let mut count = 0;
                    for asiento in asientos_clone.iter_mut() {
                        if count == cantidad {
                            break;
                        }
                        if *asiento {
                            *asiento = false;
                            count += 1;
                        }
                    }
                    Ok(cantidad)
                }else{
                    Err("No hay cantidad suficiente de asientos disponibles".to_string())
                }
            }else{
                Err("Error al conseguir lock de asientos".to_string())
            }
        }else{
            Err("No existe esa pelicula para esta sala".to_string())
        }
    }
}