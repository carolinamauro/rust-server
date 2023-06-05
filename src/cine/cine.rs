
use std::collections::HashMap;
use super::sala_cine::SalaCine;

pub struct Cine {
    next_sala_id: i32,
    salas: HashMap<i32, SalaCine>
}


impl Cine {
    pub fn new() -> Self {
        Cine {
            next_sala_id: 1,
            salas: HashMap::new(),
        }
    }
    pub fn agregar_sala(&mut self, cantidad_asientos: usize){
        self.salas.insert(self.next_sala_id, SalaCine::new(cantidad_asientos));
        self.next_sala_id += 1;
    }

    pub fn agregar_pelicula(&mut self, sala_id:i32, clave:String){
        if let Some(sala) = self.salas.get_mut(&sala_id){
            sala.insertar_pelicula(clave);
        };
    }

    pub fn comprar_entradas(&self, sala_id:i32, clave: String, cantidad: usize) -> Result<usize, String>{
        if let Some(sala) = self.salas.get(&sala_id){
            sala.reservar_asientos_pelicula(clave, cantidad)
        }else{
            Err("Error al intentar comprar entradas".to_string())
        }
    }
}