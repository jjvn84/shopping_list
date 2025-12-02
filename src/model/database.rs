use crate::model::{Item, Lista};
use rusqlite::params;
use rusqlite_migration::{M, Migrations};
#[cfg(target_os = "android")]
use std::path::PathBuf;

pub trait DBConnector {
    // Operaciones con listas
    fn create_new_list(&self, nombre: String) -> Result<(), anyhow::Error>;
    fn update_list(
        &self,
        id: usize,
        nombre: String,
        modo_simple: usize,
    ) -> Result<(), rusqlite::Error>;
    fn get_list_of_lists(&self) -> Result<Vec<Lista>, anyhow::Error>;
    fn get_list(&self, id_lista: usize) -> Result<Lista, anyhow::Error>;
    fn delete_list(&self, id_lista: usize) -> Result<(), anyhow::Error>;
    // Operaciones con items
    fn create_new_list_item(&self, id_lista: usize, item: Item) -> Result<(), anyhow::Error>;
    fn update_list_item(&self, item: Item) -> Result<(), anyhow::Error>;
    fn delete_item(&self, id: usize) -> Result<(), anyhow::Error>;
    fn clear_list_items(&self, id_lista: usize) -> Result<(), anyhow::Error>;
}

pub struct SQLiteConnector {
    connection: rusqlite::Connection,
}

impl SQLiteConnector {
    pub fn new() -> Self {
        let migrations_slice: &[M<'_>] = &[
            M::up(
                "CREATE TABLE IF NOT EXISTS listas (
                id INTEGER PRIMARY KEY,
                nombre TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                id_lista INTEGER,
                nombre TEXT NOT NULL,
                unidad TEXT,
                cantidad_requerida FLOAT,
                cantidad_comprada FLOAT,
                precio FLOAT
                );",
            ),
            M::up("ALTER TABLE listas ADD COLUMN modo_simple INTEGER DEFAULT 0;"),
        ];
        let migrations: Migrations<'_> = Migrations::from_slice(migrations_slice);

        let db_path = SQLiteConnector::get_db_path();

        // Open the database from the persisted "hotdog.db" file
        let mut conn = rusqlite::Connection::open(format!("{db_path}/shopping_list.db"))
            .expect("Failed to open database");

        // Apply some PRAGMA, often better to do it outside of migrations
        conn.pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
            .unwrap();

        // Update the database schema, atomically
        migrations.to_latest(&mut conn).unwrap();

        // Return the connection
        SQLiteConnector { connection: conn }
    }

    #[allow(unused)]
    #[cfg(target_os = "linux")]
    fn get_db_path() -> String {
        String::from("assets")
    }

    #[cfg(target_os = "android")]
    fn get_db_path() -> String {
        use jni::JNIEnv;
        use jni::objects::{JObject, JString};
        let (tx, rx) = std::sync::mpsc::channel();

        fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> anyhow::Result<PathBuf> {
            let files_dir = env
                .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
                .l()?;
            let files_dir: JString<'_> = env
                .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
                .l()?
                .into();
            let files_dir: String = env.get_string(&files_dir)?.into();
            Ok(PathBuf::from(files_dir))
        }

        dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
            tx.send(run(env, activity)).unwrap()
        });

        let temp_path = rx.recv().unwrap().unwrap();
        return String::from(temp_path.to_str().unwrap());
    }
}
impl DBConnector for SQLiteConnector {
    // Operaciones con listas
    fn create_new_list(&self, nombre: String) -> Result<(), anyhow::Error> {
        if !nombre.trim().is_empty() {
            self.connection.execute(
                "INSERT INTO listas (nombre, modo_simple) VALUES (?1, 0)",
                [nombre],
            )?;
        }
        Ok(())
    }

    fn update_list(
        &self,
        id: usize,
        nombre: String,
        modo_simple: usize,
    ) -> Result<(), rusqlite::Error> {
        if !nombre.trim().is_empty() {
            self.connection.execute(
                "UPDATE listas SET nombre=?1, modo_simple=?2 WHERE id = ?3;",
                params![nombre, modo_simple, id],
            )?;
        }

        Ok(())
    }

    fn get_list_of_lists(&self) -> Result<Vec<Lista>, anyhow::Error> {
        let result = self
            .connection
            .prepare("SELECT id, nombre, modo_simple FROM listas ORDER BY nombre;")
            .unwrap()
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get::<usize, usize>(2)?))
            })
            .unwrap()
            .map(|r| {
                let row = r.unwrap();
                Lista {
                    id: row.0,
                    nombre: row.1,
                    items: None,
                    total: 0.0,
                    modo_simple: row.2 == 1,
                }
            })
            .collect();
        Ok(result)
    }

    fn get_list(&self, id_lista: usize) -> Result<Lista, anyhow::Error> {
        let result = self
            .connection
            .prepare("SELECT id, nombre, modo_simple FROM listas WHERE id = (?1);")
            .unwrap()
            .query_row([id_lista], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get::<usize, usize>(2)?))
            })
            .unwrap();
        let mut final_list = Lista {
            id: result.0,
            nombre: result.1,
            items: None,
            total: 0.0,
            modo_simple: result.2 == 1,
        };
        let mut result: Vec<Item> = self.connection.prepare("SELECT id, nombre, unidad, cantidad_requerida, cantidad_comprada, precio FROM items WHERE id_lista = (?1);")
            .unwrap()
            .query_map([id_lista], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)))
            .unwrap()
            .map(|r| {
                let row = r.unwrap();
                Item {
                    id: row.0,
                    id_lista,
                    nombre: row.1,
                    unidad: row.2,
                    cantidad_requerida: row.3,
                    cantidad_comprada: row.4,
                    precio: row.5,
                }
            })
            .collect()
            ;
        result.sort_by(|a, b| {
            if a.cantidad_comprada < 0.001 && b.cantidad_comprada < 0.001
                || a.cantidad_comprada >= 0.001 && b.cantidad_comprada >= 0.001
            {
                return a.nombre.cmp(&b.nombre);
            }
            if a.cantidad_comprada < 0.001 && b.cantidad_comprada >= 0.001 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        final_list.items = Some(result);
        final_list.total = final_list
            .clone()
            .items
            .unwrap()
            .iter()
            .fold(0.0, |acc, item| acc + item.cantidad_comprada * item.precio);
        Ok(final_list)
    }

    fn delete_list(&self, id_lista: usize) -> Result<(), anyhow::Error> {
        self.connection
            .execute("DELETE FROM items WHERE id_lista = ?1;", [id_lista])?;
        self.connection
            .execute("DELETE FROM listas WHERE id = ?1;", [id_lista])?;
        Ok(())
    }

    // Operaciones con items
    fn create_new_list_item(&self, id_lista: usize, item: Item) -> Result<(), anyhow::Error> {
        if !item.nombre.trim().is_empty() {
            self.connection.execute("INSERT INTO items (id_lista, nombre, unidad, cantidad_requerida, cantidad_comprada, precio) VALUES (?1, ?2, ?3, ?4, ?5, ?6);", params![id_lista, item.nombre, item.unidad, item.cantidad_requerida, item.cantidad_comprada, item.precio])?;
        }
        Ok(())
    }

    fn update_list_item(&self, item: Item) -> Result<(), anyhow::Error> {
        if !item.nombre.trim().is_empty() {
            self.connection.execute("UPDATE items SET nombre=?1, unidad=?2, cantidad_requerida=?3, cantidad_comprada=?4, precio=?5 WHERE id = ?6;", params![item.nombre, item.unidad, item.cantidad_requerida, item.cantidad_comprada, item.precio, item.id])?;
        }
        Ok(())
    }

    fn delete_item(&self, id: usize) -> Result<(), anyhow::Error> {
        self.connection
            .execute("DELETE FROM items WHERE id = ?1;", [id])?;
        Ok(())
    }

    fn clear_list_items(&self, id_lista: usize) -> Result<(), anyhow::Error> {
        self.connection.execute(
            "UPDATE items SET cantidad_comprada=?1 WHERE id_lista = ?2",
            params![0.0, id_lista],
        )?;
        Ok(())
    }
}
