pub mod models;
pub mod controller;

// #[cfg(test)]
// mod tests {
//     use crate::{models::menu::Menu, controller::opensearch::{create_index_definition, IndexDocument, create, delete_index_definition, delete, find}};

//     #[tokio::test]
//     async fn populate() {
//         //  Create Index
//         create_index_definition(&Menu::default()).await.expect("error creating index");
//         //  Populate
//         for counter in 0..100 {
//             let mut _document = Menu::default();
//             _document.id = Some(counter);
//             _document.uuid = Some(format!("uuid-{}", counter));
//             _document.name = Some(format!("name-{}", counter));
//             _document.description = Some(format!("description-{:}", counter));
//             let _menu_document: &dyn IndexDocument = &_document;
//             match create(_menu_document).await {
//                 Ok(_) => {},
//                 Err(error) => log::warn!("{}", error)
//             }
//         }
//         //  Find
//         let mut _document = Menu::default();
//         let _menu_document: &dyn IndexDocument = &_document;
//         match find(_menu_document, "name-1".to_string(), 0, 10).await {
//             Ok(values) => {
//                 for value in values {
//                     let menu: Menu = serde_json::from_value(value).unwrap();
//                     log::info!("Finded Value: {:?}", menu);
//                 }
//             },
//             Err(error) => log::warn!("{}", error)
//         }
//         //  Delete
//         for counter in 0..100 {
//             let mut _document = Menu::default();
//             _document.id = Some(counter);
//             let _menu_document: &dyn IndexDocument = &_document;
//             match delete(_menu_document).await {
//                 Ok(_) => {},
//                 Err(error) => log::warn!("{}", error)
//             }
//         }
//         delete_index_definition(&Menu::default()).await.expect("error deleting index");
//     }
// }