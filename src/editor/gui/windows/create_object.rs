use macroquad::{
    prelude::*,
    ui::{hash, widgets, Ui},
};

use crate::map::Map;

use super::{ButtonParams, EditorAction, EditorContext, Window, WindowParams};

pub struct CreateObjectWindow {
    params: WindowParams,
    name: String,
    position: Vec2,
    size: Option<Vec2>,
    layer_id: String,
}

impl CreateObjectWindow {
    pub fn new(position: Vec2, layer_id: String) -> Self {
        let params = WindowParams {
            title: Some("Create Tileset".to_string()),
            size: vec2(350.0, 350.0),
            ..Default::default()
        };

        CreateObjectWindow {
            params,
            name: "Unnamed Object".to_string(),
            position,
            size: None,
            layer_id,
        }
    }
}

impl Window for CreateObjectWindow {
    fn get_params(&self) -> &WindowParams {
        &self.params
    }

    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams> {
        let mut res = Vec::new();

        let action = self.get_close_action().then(EditorAction::CreateObject {
            name: self.name.clone(),
            position: self.position,
            size: self.size,
            layer_id: self.layer_id.clone(),
        });

        res.push(ButtonParams {
            label: "Create",
            action: Some(action),
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Cancel",
            action: Some(self.get_close_action()),
            ..Default::default()
        });

        res
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        _size: Vec2,
        _map: &Map,
        _ctx: &EditorContext,
    ) -> Option<EditorAction> {
        let id = hash!("create_object_window");

        {
            let size = vec2(173.0, 25.0);

            widgets::InputText::new(hash!(id, "name_input"))
                .size(size)
                .ratio(1.0)
                .label("Name")
                .ui(ui, &mut self.name);
        }

        None
    }
}