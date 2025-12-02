use serde::Deserialize;

#[derive(PartialEq, Clone, Debug)]
pub struct Lista {
    pub id: usize,
    pub nombre: String,
    pub items: Option<Vec<Item>>,
    pub total: f32,
    pub modo_simple: bool,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Item {
    pub id: usize,
    pub id_lista: usize,
    pub nombre: String,
    pub unidad: String,
    pub cantidad_requerida: f32,
    pub cantidad_comprada: f32,
    pub precio: f32,
}

impl Default for Item {
    fn default() -> Item {
        Item {
            id: 0,
            id_lista: 0,
            nombre: String::from(""),
            unidad: String::from("unidad"),
            cantidad_requerida: 1.0,
            cantidad_comprada: 0.0,
            precio: 0.0,
        }
    }
}

#[derive(Deserialize)]
pub struct ItemForm {
    pub id: String,
    pub id_lista: String,
    pub nombre: String,
    pub unidad: String,
    pub cantidad_requerida: String,
    pub cantidad_comprada: Option<String>,
    pub precio: String,
}

impl ItemForm {
    pub fn into_item(self) -> Item {
        Item {
            id: self.id.parse().unwrap_or_default(),
            id_lista: self.id_lista.parse().unwrap_or_default(),
            nombre: self.nombre,
            unidad: self.unidad,
            cantidad_requerida: self.cantidad_requerida.parse().unwrap_or_default(),
            cantidad_comprada: self
                .cantidad_comprada
                .unwrap_or(String::from("0.0"))
                .parse()
                .unwrap_or_default(),
            precio: self.precio.parse().unwrap_or_default(),
        }
    }
}
