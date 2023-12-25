use reqwest;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("END_POINT").expect("MAILCOACH_API_TOKEN must be set.");  // Reemplaza con la URL de la API que deseas consumir

    let response = reqwest::get(url).await?;  // Realiza una solicitud GET a la API y espera la respuesta

    // Verifica si la solicitud fue exitosa (código de respuesta 200 OK)
    if response.status().is_success() {
        let body = response.text().await?;  // Lee el cuerpo de la respuesta como texto
        println!("Respuesta de la API: {}", body);
    } else {
        println!("La solicitud no fue exitosa. Código de estado: {}", response.status());
    }

    Ok(())
}