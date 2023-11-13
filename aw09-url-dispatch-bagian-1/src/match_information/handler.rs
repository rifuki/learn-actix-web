use actix_web::{HttpRequest, Result, get};

#[get("/version/{v1}/{v2}")]
pub async fn version(req: HttpRequest) -> Result<String> {
    let v1 = req.match_info().get("v1").unwrap().parse::<u8>().unwrap();
    let v2 = req.match_info().query("v2").parse::<u8>().unwrap();

    let (v3, v4) = req.match_info().load::<(&str, &str)>().unwrap(); /* <- catch all values from params or path paramater */

    Ok(format!("v1: {}, v2: {}, v3: {}, v4: {}", v1, v2, v3, v4))
}