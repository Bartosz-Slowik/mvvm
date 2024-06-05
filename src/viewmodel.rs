use crate::models::{Product, ShortProduct};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use tokio::task;
use bson::oid::ObjectId;

#[derive(Default)]
pub struct ProductViewModel {
    pub short_products: Arc<Mutex<Vec<ShortProduct>>>,
    pub product_detail: Arc<Mutex<Option<Product>>>,
    pub error: Arc<Mutex<Option<String>>>,
}

impl ProductViewModel {
    pub fn new() -> Self {
        Self {
            short_products: Arc::new(Mutex::new(Vec::new())),
            product_detail: Arc::new(Mutex::new(None)),
            error: Arc::new(Mutex::new(None)),
        }
    }

    pub fn fetch_short_products(&self) {
        let short_products_clone = Arc::clone(&self.short_products);
        let error_clone = Arc::clone(&self.error);

        task::spawn(async move {
            let client = Client::new();
            match client.get("http://localhost:8080/api/products").send().await {
                Ok(resp) => match resp.json::<Vec<ShortProduct>>().await {
                    Ok(short_products) => {
                        *short_products_clone.lock().unwrap() = short_products;
                    }
                    Err(_) => {
                        *error_clone.lock().unwrap() = Some("Failed to fetch short products".into());
                    }
                },
                Err(_) => {
                    *error_clone.lock().unwrap() = Some("Failed to fetch short products".into());
                }
            }
        });
    }

    pub fn fetch_product_detail(&self, product_id: ObjectId) {
        let product_detail_clone = Arc::clone(&self.product_detail);
        let error_clone = Arc::clone(&self.error);

        task::spawn(async move {
            let client = Client::new();
            let url = format!("http://localhost:8080/api/products/{}", product_id);
            match client.get(&url).send().await {
                Ok(resp) => match resp.json::<Product>().await {
                    Ok(product) => {
                        *product_detail_clone.lock().unwrap() = Some(product);
                    }
                    Err(_) => {
                        *error_clone.lock().unwrap() = Some("Failed to fetch product detail".into());
                    }
                },
                Err(_) => {
                    *error_clone.lock().unwrap() = Some("Failed to fetch product detail".into());
                }
            }
        });
    }

    pub fn create_product(&self, product: Product) {
        println!("asdasd");
        let short_products_clone = Arc::clone(&self.short_products);
        let error_clone = Arc::clone(&self.error);
        
        task::spawn(async move {
            let client = Client::new();
            match client.post("http://localhost:8080/api/products")
                .json(&product)
                .send()
                .await
            {
                Ok(resp) => match resp.text().await {
                    Ok(id) => {
                        println!("Product created with ID: {}", id);
                    }
                    Err(err) => {
                        println!("{}",err);
                        *error_clone.lock().unwrap() = Some("Failed to create product".into());
                    }
                },
                Err(err) => {
                    println!("{}",err);
                    *error_clone.lock().unwrap() = Some("Failed to create product".into());
                }
            }
        });
    }

    pub fn delete_product(&self, product_id: ObjectId) {
        let error_clone = Arc::clone(&self.error);

        task::spawn(async move {
            let client = Client::new();
            let url = format!("http://localhost:8080/api/products/{}", product_id);
            match client.delete(&url).send().await {
                Ok(_) => (),
                Err(_) => {
                    *error_clone.lock().unwrap() = Some("Failed to delete product".into());
                }
            }
        });
    }
    pub fn update_product(&self, product: Product) {
        let product_id = product._id.clone();
        let error_clone = Arc::clone(&self.error);
    
        task::spawn(async move {
            let client = Client::new();
            let url = format!("http://localhost:8080/api/products/{}", product_id);
            match client.put(&url)
                .json(&product)
                .send()
                .await
            {
                Ok(resp) => match resp.text().await {
                    Ok(_) => {
                        println!("Product updated with ID: {}", product_id);
                    }
                    Err(_) => {
                        *error_clone.lock().unwrap() = Some("Failed to update product".into());
                    }
                },
                Err(_) => {
                    *error_clone.lock().unwrap() = Some("Failed to update product".into());
                }
            }
        });
    }
}
