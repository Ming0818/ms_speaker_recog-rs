use serde_json;

#[derive(Debug)]
pub struct Error {
    pub code: String,
    pub message: String
}

impl Error {
    pub fn new(code: &str, message: &str) -> Error {
        Error{code:code.into(), message:message.into()}
    }
    pub fn from_json(json: &str) -> Error {
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        Error{
            code: v["error"]["code"].as_str().unwrap().into(),
            message: v["error"]["message"].as_str().unwrap().into()
        }
    }
}

pub struct Verification {
    pub result: String,
    pub confidence: String,
    pub phrase: String
}

impl Verification {
    pub fn acceped(&self)-> bool {
        self.result == "Accept"
    }
    pub fn from_json(json: &str) -> Verification {
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        Verification{
            result: v["result"].as_str().unwrap().into(),
            confidence: v["confidence"].as_str().unwrap().into(),
            phrase: v["phrase"].as_str().unwrap().into()
        }
    }
}

pub struct Profile {
    id: String,
    locale: String,
    enrollment_speech_time: f64,
    remainig_enrollment_speech_time: f64,
    //TODO: change String to Dateformat
    created_date_time: String,
    last_action_date_time: String,
    enrollment_status: String
}

impl Profile {
    pub fn from_json(json: &str) -> Profile {
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        Profile{
            id: to_string(&v,"identificationProfileId"),
            locale: to_string(&v, "locale"),
            enrollment_speech_time: v["enrollmentSpeechTime"].as_f64().unwrap(),
            remainig_enrollment_speech_time: v["remainigEnrollmentSpeechTime"].as_f64().unwrap(),
            created_date_time: to_string(&v, "createdDateTime"),
            last_action_date_time: to_string(&v, "lastActionDateTime"),
            enrollment_status: to_string(&v, "enrollmentStatus")
        }
    }
}
fn to_string(v: &serde_json::Value, key: &str) -> String {
    v[key].as_str().unwrap().into()
}


