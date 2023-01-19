wit_bindgen_rust::export!("../wit/spin-timer.wit");

struct SpinTimer;

impl spin_timer::SpinTimer for SpinTimer {
    fn handle_timer_request() {
        let text = spin_sdk::config::get("message").unwrap();
        println!("{text}");
    }
}

// /// A simple Spin HTTP component.
// #[http_component]
// fn goodbye(req: Request) -> Result<Response> {
//     println!("{:?}", req.headers());
//     Ok(http::Response::builder()
//         .status(200)
//         .header("foo", "bar")
//         .body(Some("Hello, Fermyon".into()))?)
// }


