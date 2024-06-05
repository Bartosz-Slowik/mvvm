mod models;
mod viewmodel;
mod view;

use eframe::egui::{self, Context};
use viewmodel::ProductViewModel;
use view::ProductView;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let view_model = ProductViewModel::new();
        let product_view = ProductView::new(view_model);
        
        let options = eframe::NativeOptions {
            resizable: true,
            icon_data: Some(load_icon()),
            ..Default::default()};
        eframe::run_native(
            "Product App",
            options,
            Box::new(|_| Box::new(MyApp::new(product_view))),
        );
    });
    pub(crate) fn load_icon() -> eframe::IconData {
        let (icon_rgba, icon_width, icon_height) = {
            let icon = include_bytes!("./icon.png");
            let image = image::load_from_memory(icon)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        
        eframe::IconData {
            rgba: icon_rgba,
            width: icon_width,
            height: icon_height,
        }
    }

}

pub struct MyApp {
    product_view: ProductView,
}

impl MyApp {
    pub fn new(product_view: ProductView) -> Self {
        Self { product_view }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        self.product_view.ui(ctx);
    }
}