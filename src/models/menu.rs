use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use std::{io::ErrorKind, io::Error};

use crate::models::{menu_item::menu_items_from_role, menu_tree::menu_tree_from_id, role::role_from_id};

use super::{menu_item::MenuItem, menu_tree::MenuTree};

#[derive(Deserialize, Extractible, Debug, Clone)]
#[salvo(extract(default_source(from = "body")))]
pub struct MenuDocument {
    pub document: Option<Menu>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuResponse {
    pub menu: Option<Menu>
}

#[derive(Serialize, Debug, Clone)]
pub struct MenuListResponse {
    pub menus: Option<Vec<Menu>>
}

impl Default for MenuResponse {
    fn default() -> Self {
        MenuResponse { 
            menu: None 
        }
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Menu {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub parent_id: Option<i32>,
    pub sequence: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_summary: Option<bool>,
    pub is_sales_transaction: Option<bool>,
    pub is_read_only: Option<bool>,
    // Supported References
    pub action: Option<String>,
    pub action_id: Option<i32>,
    pub action_uuid: Option<String>,
    pub window: Option<Window>,
    pub process: Option<Process>,
    pub form: Option<Form>,
	pub browser: Option<Browser>,
    pub workflow: Option<Workflow>,
    // Tree menu childs
    pub children: Option<Vec<Menu>>
}

impl Default for Menu {
    fn default() -> Self {
        Self { 
            uuid: None, 
            internal_id: None,
            id: None, 
            parent_id: None, 
            sequence: None, 
            name: None, 
            description: None, 
            is_summary: None, 
            is_sales_transaction: None, 
            is_read_only: None, 
			// Supported References
            action: None,
			action_id: None,
			action_uuid: None,
            window: None, 
            process: None, 
            form: None, 
			browser: None,
			workflow: None,
			// Tree menu childs
			children: None
        }
    }
}

impl Menu {
    pub fn from_id(_id: Option<String>) -> Self {
        let mut menu = Menu::default();
        menu.id = _id;
        menu
    }

    pub fn from_menu_item(_menu_item: MenuItem) -> Self {
        let mut menu = Menu::default();
        menu.action = _menu_item.action;
        menu.action_id = _menu_item.action_id;
        menu.action_uuid = _menu_item.action_uuid;
        menu.description = _menu_item.description;
        menu.id = _menu_item.id;
        menu.is_read_only = _menu_item.is_read_only;
        menu.is_sales_transaction = _menu_item.is_sales_transaction;
        menu.is_summary = _menu_item.is_summary;
        menu.name = _menu_item.name;
        menu.parent_id = _menu_item.parent_id;
        menu.sequence = _menu_item.sequence;
        menu.uuid = _menu_item.uuid;
        menu.window = _menu_item.window;
        menu.workflow = _menu_item.workflow;
        menu.browser = _menu_item.browser;
        menu.process = _menu_item.process;
        menu.form = _menu_item.form;
        menu
    }
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Window {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Process {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Form {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Browser {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Deserialize, Serialize, Extractible, Debug, Clone)]
pub struct Workflow {
    pub uuid: Option<String>,
    pub internal_id: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

pub async fn allowed_menu(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>) -> Result<MenuListResponse, std::io::Error> {
    let _expected_role = role_from_id(_role_id, _client_id).await;
    let _role = match _expected_role {
        Ok(role) => role,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };

    let _menu_items = menu_items_from_role(_role.to_owned(), _language, None, None).await;
    let _menu_items = match _menu_items {
        Ok(menu) => menu,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };

    if _role.tree_id.is_none() {
        return Err(Error::new(ErrorKind::InvalidData.into(), "Tree ID not found"))
    }

    let _tree_result = menu_tree_from_id(_role.tree_uuid).await;
    let _tree = match _tree_result {
        Ok(tree) => tree,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };
    //  Merge tree with menu
    //  Main Menu
    let _tree_children = _tree.children;
    let menus = load_valid_children(_tree_children, _menu_items);
    // println!("Epale: {:?}", menus);
    Ok(MenuListResponse {
        menus: Some(menus)
    })
}

fn load_valid_children(_tree: Option<Vec<MenuTree>>, _allowed_menu_items: Vec<MenuItem>) -> Vec<Menu> {
    if _tree.is_none() {
        return Vec::new()
    }
    let mut menus = Vec::new();
    let _tree = _tree.unwrap();
    for _tree_value in _tree {
        let _allowed_item = _allowed_menu_items.to_owned().into_iter().find(|_item| _item.internal_id.is_some() && _item.internal_id == _tree_value.node_id);
        let mut _loaded_menu: Option<Menu> = None;
        if _allowed_item.is_some() {
            _loaded_menu = Some(Menu::from_menu_item(_allowed_item.unwrap()));
        }
        if _loaded_menu.is_some() && _tree_value.children.is_some() {
            let children_loaded_menu = load_valid_children(_tree_value.children, _allowed_menu_items.to_owned());
            let mut _current_menu = _loaded_menu.unwrap();
            _current_menu.children = Some(children_loaded_menu);
            _loaded_menu = Some(_current_menu);
        }
        if _loaded_menu.is_some() {
            menus.push(_loaded_menu.unwrap());
        }
    }
    menus
}