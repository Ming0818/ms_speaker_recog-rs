# ms_speaker_recog-rs

A Wrapper around Microsoft Speaker Recognition API
[API Documentation](https://dev.projectoxford.ai/docs/services/563309b6778daf02acc0a508/operations/5645c3271984551c84ec6797)

```rust
extern crate ms_speaker_recog
use std::{time, thread};

fn main() {
	//0: Identification, 1: Verification
	let client = ms_speaker_recog::Client::new("my-key", 0);
	//user test
	let id = client.create_profile("en-us");
	let _ = client.create_enrollment(&id, include_bytes!("path to audio file"), false).unwrap();

	thread::sleep(time::Duration::from_secs(1));
	//can add up to 10 ids
	let mut ids: Vec<String> = Vec::new();
	ids.push(id)
	let status_id = client.identify(ids, include_bytes!("path to short audio file), false).unwrap();
	
	thread::sleep(time::Duration::from_secs(2));
	
	let res = client.get_operation_status_id(&status_id).unwrap();
	if res == id {
		println!("user test is talking")
	}
}
```
