use serde::{Deserialize, Serialize};
use salvo::prelude::*;
use std::{io::ErrorKind, io::Error};

use crate::models::{menu_item::menu_items_from_role, menu_tree::menu_tree_from_id, role::role_from_id};

use super::{menu_item::MenuItem, menu_tree::MenuTree, role::Role};

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
pub struct MenuAction {
	pub internal_id: Option<i32>,
	pub id: Option<String>,
	pub uuid: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub help: Option<String>
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
	pub window: Option<MenuAction>,
	pub process: Option<MenuAction>,
	pub form: Option<MenuAction>,
	pub browser: Option<MenuAction>,
	pub workflow: Option<MenuAction>,
    // Tree menu childs
    pub children: Option<Vec<Menu>>
}

impl Default for Menu {
	fn default() -> Self {
		Self {
			uuid: None,
			internal_id: None,
			id: None,
			parent_id: Some(0),
			sequence: Some(0),
			name: None,
			description: None,
			is_summary: Some(false),
			is_sales_transaction: Some(false),
			is_read_only: None,
			// Supported References
			action: None,
			action_id: Some(0),
			action_uuid: None,
			window: None,
			process: None,
			form: None,
			browser: None,
			workflow: None,
			// Tree menu childs
			children: Some(Vec::new())
		}
	}
}

impl Menu {
	pub fn from_id(_id: Option<String>) -> Self {
		let mut menu: Menu = Menu::default();
		menu.id = _id;
		menu
	}

	pub fn from_menu_item(_menu_item: MenuItem) -> Self {
		let mut menu: Menu = Menu::default();

		menu.uuid = _menu_item.uuid;
		menu.internal_id = _menu_item.internal_id;
		menu.id = _menu_item.id;
		menu.parent_id = _menu_item.parent_id;
		menu.sequence = _menu_item.sequence;
		menu.name = _menu_item.name;
		menu.description = _menu_item.description;
		menu.is_summary = _menu_item.is_summary;
		menu.is_sales_transaction = _menu_item.is_sales_transaction;
		menu.is_read_only = _menu_item.is_read_only;
		// Supported References
		menu.action = _menu_item.action;
		menu.action_id = _menu_item.action_id;
		menu.action_uuid = _menu_item.action_uuid;
		menu.window = _menu_item.window;
		menu.process = _menu_item.process;
		menu.form = _menu_item.form;
		menu.browser = _menu_item.browser;
		menu.workflow = _menu_item.workflow;

		menu
	}
}

pub async fn allowed_menu(_language: Option<&String>, _client_id: Option<&String>, _role_id: Option<&String>, _dictionary_code: Option<&String>) -> Result<MenuListResponse, std::io::Error> {
	let _expected_role: Result<Role, String> = role_from_id(_role_id, _client_id, _dictionary_code).await;
	let _role: Role = match _expected_role {
        Ok(role) => role,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };

	let _menu_items: Result<Vec<MenuItem>, Error> = menu_items_from_role(_role.to_owned(), _language, _dictionary_code, None, None).await;
	let _menu_items: Vec<MenuItem> = match _menu_items {
        Ok(menu) => menu,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };

	if _role.tree_id.is_none() {
		log::error!("Tree ID not found, on role {:?} = {:?}", _role.name, _role.internal_id);
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Tree ID not found")
		)
	}
	if _role.tree_uuid.is_none() {
		log::error!("Tree UUID not found, on role {:?} = {:?}", _role.name, _role.internal_id);
		return Err(
			Error::new(ErrorKind::InvalidData.into(), "Tree UUID not found")
		)
	}

	let _tree_result: Result<MenuTree, Error> = menu_tree_from_id(_role.tree_uuid, _dictionary_code).await;
	let _tree: MenuTree = match _tree_result {
        Ok(tree) => tree,
        Err(error) => return Err(Error::new(ErrorKind::InvalidData.into(), error))
    };
    //  Merge tree with menu
    //  Main Menu
	let _tree_children: Option<Vec<MenuTree>> = _tree.children;
	let menus: Vec<Menu> = load_valid_children(_tree_children, _menu_items);
    Ok(MenuListResponse {
        menus: Some(menus)
    })
}

fn load_valid_children(_tree: Option<Vec<MenuTree>>, _allowed_menu_items: Vec<MenuItem>) -> Vec<Menu> {
	if _tree.is_none() {
		return Vec::new()
	}
	let mut menus: Vec<Menu> = Vec::new();
	let _tree: Vec<MenuTree> = _tree.unwrap();
	for _tree_value in _tree {
		let _allowed_item: Option<MenuItem> = _allowed_menu_items.to_owned().into_iter().find(|_item: &MenuItem| _item.internal_id.is_some() && _item.internal_id == _tree_value.node_id);
		if _allowed_item.is_some() {
			let mut allowed_item: MenuItem = _allowed_item.unwrap();
			// overwrite sequence null by that of the node
			allowed_item.sequence = _tree_value.sequence;

			let mut _loaded_menu: Option<Menu> = Some(Menu::from_menu_item(allowed_item));
			if _loaded_menu.is_some() {
				let mut _current_menu: Menu = _loaded_menu.unwrap();

				let _parent_id: i32 = match _tree_value.parent_id {
					Some(value) => value,
					None => 0
				};
				_current_menu.parent_id = Some(_parent_id);

				if _tree_value.children.is_some() {
					let mut children_loaded_menu: Vec<Menu> = load_valid_children(_tree_value.children, _allowed_menu_items.to_owned());

					// sort child nodes by sequence
					children_loaded_menu.sort_by(|a: &Menu, b: &Menu| a.sequence.cmp(&b.sequence));

					_current_menu.children = Some(children_loaded_menu);
				}

				// Verify if the node is summary and has children with action and not just more summaries
				if _current_menu.is_summary.unwrap_or(false) {
					let has_action_id: bool = has_action_in_childrens(&_current_menu);
					if !has_action_id {
						// If no action was found in the children, do not add the menu to the list
						continue;
					}
				}

				_loaded_menu = Some(_current_menu);
				menus.push(_loaded_menu.unwrap());
			}
		}
	}

	// sort root nodes by sequence
	menus.sort_by(|a: &Menu, b: &Menu| a.sequence.cmp(&b.sequence));
	menus
}

fn has_action_in_childrens(menu: &Menu) -> bool {
	if let Some(children) = &menu.children {
		for child in children {
			if child.action_id.is_some() && child.action_id.unwrap() > 0 {
				return true;
			}
			if has_action_in_childrens(child) {
				return true;
			}
		}
	}
	false
}
