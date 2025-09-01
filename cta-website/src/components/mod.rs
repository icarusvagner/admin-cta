use crate::{error::{Error, Result}, server::auth::api_logout_req, types::request_types::{LogoffPayload, LogoffReturn}};

pub mod counter_btn;
pub mod loading;
pub mod navbar;
pub mod sidebar;
pub mod themes;
pub mod toaster;

pub async fn send_logoff_api(logoff: bool) -> Result<LogoffReturn> {
	let data = LogoffPayload { logoff };

	match api_logout_req(data).await {
	    Ok(res) => Ok(res),
	    Err(ex) => {
	                leptos::logging::log!("{}", ex.to_string());
	                Err(Error::Network(ex.to_string()))
	    }
	} 
}

