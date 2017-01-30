use {Error, Profile, Verification};

use serde_json;

use hyper::client;
use hyper::header::Headers;
use hyper::status::StatusCode;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

header! { (ContentType, "Content-Type") => [String] }
header! { (SubscriptionKey, "Ocp-Apim-Subscription-Key") => [String] }

pub struct Client {
    key: String,
    mode: i32,
    base_url: String,
    client: client::Client
}

impl Client {
    pub fn new(key: &str, mode: i32) -> Client {
        let ssl = NativeTlsClient::new().unwrap();
        let con = HttpsConnector::new(ssl);
        let client = client::Client::with_connector(con);
        let base_url = "https://westus.api.cognitive.microsoft.com/spid/v1.0/";

        Client{
            key: key.into(),
            mode: mode,
            base_url: base_url.into(), 
            client: client
        }
    }

    pub fn create_enrollment(&self, id: &str, audio: &[u8], short: bool) -> Result<String, Error> {
        let params = format!("{}/enroll?shortAudio={}", id, short);
        self.request("multipart/form-data", &params, Some(audio))
    }

    pub fn create_profile(&self, locale: &str) -> Result<String, Error> {
        let b: String = "{\"locale\":\"".to_owned()+locale+"\"}";
        let res = try!(self.request("application/json", "", Some(b.as_bytes())));
        let v: serde_json::Value = serde_json::from_str(&res).unwrap();
        let id = if self.mode == 0 {"identificationProfileId"} else {"verificationProfileId"};
        Ok(v[id].as_str().unwrap().into())
    }

    pub fn identify(&self, ids: Vec<String>, audio: &[u8], short: bool) -> Result<String, Error> {
        let mut param = String::new();
        for id in ids {
            param = format!("{},{}", param, id);
        }
        let (_, l) = param.split_at(1);
        println!("{}", l);
        let params = format!("identify?identificationProfileIds={}&shortAudio={}",l, short);
        self.request("multipart/form-data", &params, Some(audio))
    }

    pub fn verify(&self, id: &str, audio: &[u8]) -> Result<Verification, Error> {
        let params = format!("verify?verificationProfileId={}", id);
        let res = try!(self.request("multipart/form-data", &params, Some(audio)));
        Ok(Verification::from_json(&res))
    }

    pub fn delete_profile(&self, id: &str) -> Result<String, Error> {
        self.request("", "", None)

    }

    pub fn get_profile(&self, id: &str) -> Result<Profile, Error> {
        let res = try!(self.request("application/json", id, None)); 
        Ok(Profile::from_json(&res))
    }

    pub fn get_all_profiles(&self) -> Result<Vec<Profile>, Error> {
       unimplemented!() 
    }

    pub fn reset_enrollments(&self, id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    fn request(&self, content_type: &str, params: &str, data: Option<&[u8]>) -> Result<String, Error> {
        let mut headers = Headers::new();
        headers.set(ContentType(content_type.into()));
        headers.set(SubscriptionKey(self.key.clone()));
        let mut mode = if self.mode == 0 {"identificationProfiles"} else {"verificationProfiles"};
        if params.contains("identify") || params.contains("verify") || params.contains("operation") {
            mode = ""
        }
        let url = format!("{}{}/{}", self.base_url, mode, params);

        let http = match data {
            Some(data) => {
                self.client.post(&url).body(data)
            }
            None => {
                self.client.get(&url)
            }
        };
        match http.headers(headers).send() {
            Ok(mut res) => {
                use std::io::Read;
                let mut s = String::new();
                let _ = res.read_to_string(&mut s).unwrap();
                match res.status {
                    StatusCode::Ok => {
                        Ok(s)
                    }
                    StatusCode::Accepted => {
                        let header = res.headers.get::<ContentType>().unwrap();
                        Ok(header.as_str().into())
                    }
                    _ => {Err(Error::from_json(&s))}
                }
            }
            Err(err) => {
                Err(Error::new("",&format!("{}",err)))
            }
        }
    }
}





