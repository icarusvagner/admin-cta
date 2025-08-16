use serde::{Serialize, de::DeserializeOwned};
use web_sys::FormData;

use crate::{web_config, error::{AuthorizeErrors, Error}};

const REQUEST_DEV_URL: &str = env!("REQUEST_DEV_URL");

pub async fn request<B, T>(
	method: reqwasm::http::Method,
	url: String,
	body: B,
) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
	B: Serialize + std::fmt::Debug,
{
	let allow_body = matches!(
		method,
		reqwasm::http::Method::POST | reqwasm::http::Method::PUT
	);

	let url = format!("{}{}", &REQUEST_DEV_URL, url);

	let mut req = reqwasm::http::Request::new(&url)
		.method(method)
		.header("Content-Type", "application/json");

	if allow_body {
		let body_json = serde_json::to_string(&body).map_err(|ex| {
			Error::SerdeJson(format!("Serialize error: {ex}"))
		})?;
		req = req.body(body_json);
	}

	let response = req.send().await.map_err(|ex| {
		Error::Network(format!("Http request error{ex}"))
	})?;

	handle_response(response).await
}

pub async fn request_form<T>(
	method: reqwasm::http::Method,
	url: String,
	form_data: FormData,
) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
{
	let url = format!("{}{}", &web_config().DEV_REQ_URL, url);

	let req = reqwasm::http::Request::new(&url)
		.method(method)
		.body(form_data);

	let response = req.send().await.map_err(|ex| {
		Error::Network(format!("Http request error: {ex}"))
	})?;

	handle_response(response).await
}

async fn handle_response<T>(
	response: reqwasm::http::Response,
) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
{
	if response.ok() {
		let data: T = response
			.json()
			.await
			.map_err(|ex| Error::SerdeJson(ex.to_string()))?;
		Ok(data)
	} else {
		let status = response.status();
		let error_txt = response.text().await.unwrap_or_default();

		match status {
			400 => Err(Error::Network("Bad request".into())),
			401 => {
				let data: Result<AuthorizeErrors, _> =
					serde_json::from_str(&error_txt);
				if let Ok(data) = data {
					Err(Error::Network(format!(
						"Unauthorized request: {}",
						data.message
					)))
				} else {
					Err(Error::SerdeJson("Unauthorized Error 401".to_string()))
				}
			}
			403 => Err(Error::Network("Forbidden request".into())),
			404 => {
				let data: Result<AuthorizeErrors, _> =
					serde_json::from_str(&error_txt);
				Err(Error::Network(format!(
					"Not Found: {}",
					data.map(|d| d.message).unwrap_or_else(|_| error_txt),
				)))
			}
			500 => {
				let data: Result<AuthorizeErrors, _> =
					serde_json::from_str(&error_txt);
				Err(Error::Network(format!(
					"Internal server Error: {}",
					data.map(|d| d.message).unwrap_or_else(|_| error_txt),
				)))
			}
			_ => Err(Error::Network(format!(
				"Unexpected status code: {status}",
			))),
		}
	}
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
	B: Serialize + std::fmt::Debug,
{
	request(reqwasm::http::Method::POST, url, body).await
}

pub async fn request_form_post<T>(
	url: String,
	form_data: FormData,
) -> Result<T, Error>
where
	T: DeserializeOwned + 'static + std::fmt::Debug,
{
	request_form(reqwasm::http::Method::POST, url, form_data).await
}
