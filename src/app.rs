use eframe::egui;
use log::{info, error};

use crate::components::key_value_entry::{KeyValuePair, KeyValueEntry};
use crate::components::multiline_text::MultilineTextInput;
use crate::components::text::TextInput;
use crate::method::Method;
use crate::requests::perform_request;
use std::sync::Mutex;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{mpsc, Arc},
};

pub struct AppState {
    url: Rc<RefCell<String>>,
    body: Rc<RefCell<String>>,
    method: Rc<RefCell<Method>>,
    response: String,
    tx: mpsc::Sender<String>,
    headers: Vec<KeyValuePair>,
    queryparams: Vec<KeyValuePair>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side-panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", &*self.method.borrow()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::GET, "GET");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::POST, "POST");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::PUT, "PUT");
                    ui.selectable_value(&mut *self.method.borrow_mut(), Method::DELETE, "DELETE");
                });

            TextInput::new("URL:".to_string(), self.url.clone()).show(ui);

            KeyValueEntry::new("Headers", &mut self.headers).show(ui);
            KeyValueEntry::new("QueryParams", &mut self.queryparams).show(ui);

            MultilineTextInput::new("body".to_string(), self.body.clone()).show(ui);

            if ui.button("Send").clicked() {
                info!("Send button clicked");
                let tx = self.tx.clone();
                let url = self.url.borrow().clone();
                let method = self.method.borrow().clone();
                let body = self.body.borrow().clone();

                let headers: Vec<(String, String)> = self
                    .headers
                    .iter()
                    .map(|k| (k.key.clone(), k.value.clone()))
                    .collect();
                let query_params: Vec<(String, String)> = self
                    .queryparams
                    .iter()
                    .map(|k| (k.key.clone(), k.value.clone()))
                    .collect();

                tokio::spawn(async move {
                    // Your async or long-running code here
                    //perform_request(url, method, body)
                    let result = perform_request(
                        url.as_str(),
                        method,
                        body.as_str(),
                        headers.clone(),
                        query_params.clone(),
                    )
                    .await;

                    match result {
                        Ok(response) => {
                            info!("Response: {}", response);
                            tx.send(response).unwrap();
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                        }
                    }
                });
            }
            //rest of app
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.response.clone())
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            });
        });

        // Poll the status of the async task here and update your UI accordingly
        let rx = self.rx.lock().unwrap();
        if let Ok(message) = rx.try_recv() {
            // Process the message here, update UI, etc.
            info!("Message received: {}", message);
            self.response = message;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            url: Rc::new(RefCell::new("https://httpbin.org/json".to_string())),
            body: Rc::new(RefCell::new(String::new())),
            method: Rc::new(RefCell::new(Method::GET)),
            headers: vec![],
            queryparams: vec![],
            tx,
            rx: Arc::new(Mutex::new(rx)),
            response: String::new(),
        }
    }
}
