use crate::egui::Color32;
use crate::models::Product;
use crate::viewmodel::ProductViewModel;
use bson::oid::ObjectId;
use eframe::egui::{self, CentralPanel, Context, TextEdit};
use async_std::task;

#[derive(Default)]
enum Menu {
    #[default]
    Products,
    AddProduct,
    ProductDetail,
}

#[derive(Default)]
pub struct ProductView {
    view_model: ProductViewModel,
    name: String,
    description: String,
    price: String,
    quantity: String,
    status: String,
    menu: Menu,
}

impl ProductView {
    pub fn new(view_model: ProductViewModel) -> Self {
        Self {
            view_model,
            ..Default::default()
        }
    }
    pub fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input().pointer.any_down() {
                *self.view_model.error.lock().unwrap() = None;
            }
            if let Some(error) = &*self.view_model.error.lock().unwrap() {
                ui.label(error);
            }
            match self.menu {
                Menu::Products => {
                    self.product_list_ui(ui);
                    
                }
                Menu::AddProduct => {
                    self.add_product_ui(ui);
                }
                Menu::ProductDetail => {
                    self.product_detail_ui(ui);
                }
            }
        });
    }
    pub fn product_detail_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(product) = &mut *self.view_model.product_detail.lock().unwrap() {
            let mut price = product.price.to_string();
            let mut quantity = product.quantity.to_string();

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut product.name);
            });

            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_singleline(&mut product.description);
            });

            ui.horizontal(|ui| {
                ui.label("Price:");
                ui.text_edit_singleline(&mut price);
            });

            ui.horizontal(|ui| {
                ui.label("Quantity:");
                ui.text_edit_singleline(&mut quantity);
            });

            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.text_edit_singleline(&mut product.status);
            });

            if ui.button("Save").clicked() {
                product.price = price.parse().unwrap_or(0);
                product.quantity = quantity.parse().unwrap_or(0);
                let update_handle = self.view_model.update_product(product.clone());
            
                // Wait for the update to complete before fetching the products
                task::block_on(async {
                    update_handle.await;
                    self.menu = Menu::Products;
                    self.view_model.fetch_short_products();
                });
            }
            if ui.button("Delete").clicked() {
                self.view_model.delete_product(product._id);
                self.menu = Menu::Products;
                self.view_model.fetch_short_products();
            }
        }

        if ui.button("Back").clicked() {
            *self.view_model.product_detail.lock().unwrap() = None;
            self.menu = Menu::Products;
        }
    }
    pub fn product_list_ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(egui::Button::new("Add Product").fill(Color32::from_rgb(128, 0, 0)))
            .clicked()
        {
            self.menu = Menu::AddProduct;
        }
        if ui.button("Fetch Products").clicked() {
            self.view_model.fetch_short_products();
        }
        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let short_products = self.view_model.short_products.lock().unwrap();
                if short_products.is_empty() {
                    ui.label("Products will be here");
                } else {
                    for short_product in short_products.iter() {
                        if ui
                            .button(format!("{} - ${}", short_product.name, short_product.price))
                            .clicked()
                        {
                            self.view_model
                                .fetch_product_detail(short_product._id);
                            self.menu = Menu::ProductDetail;
                        }
                    }
                }
            });
    }

    pub fn add_product_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Add Product");
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.add(TextEdit::singleline(&mut self.name));
        });
        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.add(TextEdit::singleline(&mut self.description));
        });
        ui.horizontal(|ui| {
            ui.label("Price:");
            ui.add(TextEdit::singleline(&mut self.price));
        });
        ui.horizontal(|ui| {
            ui.label("Quantity:");
            ui.add(TextEdit::singleline(&mut self.quantity));
        });
        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.add(TextEdit::singleline(&mut self.status));
        });
        if ui.button("Add").clicked() {
            let price = self.price.parse::<u32>().unwrap_or(0);
            let quantity = self.quantity.parse::<u32>().unwrap_or(0);
            let new_product = Product {
                _id: ObjectId::new(),
                name: self.name.clone(),
                description: self.description.clone(),
                price,
                quantity,
                status: self.status.clone(),
            };
            self.view_model.create_product(new_product);
        }
        if ui.button("Back").clicked() {
            self.menu = Menu::Products;
        }
    }
}
