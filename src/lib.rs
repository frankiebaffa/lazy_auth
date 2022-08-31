use {
    base64::encode,
    reqwest::{
        blocking::{
            ClientBuilder,
            Client,
        },
        Error as ReqError,
        Proxy,
    },
};
pub struct LAClient;
impl LAClient {
    fn get_client<'a>(proxy: Option<&'a str>) -> Result<Client, ReqError> {
        let cli = match proxy {
            Some(p) => {
                ClientBuilder::new().proxy(
                    Proxy::all(p)?
                )
            },
            None => ClientBuilder::new()
        }.build()?;
        Ok(cli)
    }
    /// Returns the URL for the pairing QRCode
    ///
    /// # Arguments
    ///
    /// - `app_name_ref` Your application name, something brief, but recognizable
    /// - `app_info` Typically the user's name
    /// - `secret_code_ref` A secret code that only you know
    /// - `proxy` A url or ip:port of the proxy you wish to connect to
    pub fn pair<'a>(
        app_name: &'a str,
        app_info: &'a str,
        secret_code: &'a str,
        proxy: Option<&'a str>,
    ) -> Result<Option<String>, ReqError> {
        let cli = Self::get_client(proxy)?;
        let response = cli.get(
            format!(
                concat!(
                    "https://www.authenticatorApi.com/pair.aspx",
                    "?AppName={}",
                    "&AppInfo={}",
                    "&SecretCode={}",
                ),
                app_name,
                app_info,
                secret_code
            )
        ).send()?;
        let text = response.text()?;
        let to_find = "src='";
        let left_idx = match text.find(&to_find) {
            Some(idx) => idx + to_find.len(),
            None => return Ok(None),
        };
        let left = &text[left_idx..];
        let right_idx = match left.find("'") {
            Some(idx) => idx,
            None => return Ok(None),
        };
        let url = &left[0..right_idx];
        let img_response = cli.get(url).send()?;
        let bytes = img_response.bytes()?;
        let encoded = encode(bytes);
        let base64str = format!("data:image/png;base64,{}", encoded);
        Ok(Some(base64str))
    }
    /// Returns whether or not the pin was validated
    ///
    /// # Arguments
    /// - `pin` The user's pin
    /// - `secret_code` The secret code used during pairing
    pub fn validate<'a>(
        pin: &'a str,
        secret_code: &'a str,
        proxy: Option<&'a str>,
    ) -> Result<bool, ReqError> {
        let cli = Self::get_client(proxy)?;
        let response = cli.get(
            format!(
                concat!(
                    "https://www.authenticatorApi.com/Validate.aspx",
                    "?Pin={}",
                    "&SecretCode={}",
                ),
                pin,
                secret_code,
            )
        ).send()?;
        let text = response.text()?;
        match text.as_str() {
            "True" => Ok(true),
            _ => Ok(false),
        }
    }
}
