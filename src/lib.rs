use log::info;
use modinfo::settings::{Cracks, Cracksanity, Keysy, LogicMode, NiceItems, PedestalSetting, RaviosShop, WeatherVanes};
use modinfo::Settings;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, FormData, HtmlElement, HtmlFormElement, MessageEvent, Worker};

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {
    wasm_logger::init(wasm_logger::Config::default());

    // This is not strictly needed but makes debugging a lot easier.
    // Should not be used in productive deployments.
    set_panic_hook();

    // Here, we create our worker. In a larger app, multiple callbacks should be able to interact
    // with the code in the worker. Therefore, we wrap it in `Rc<RefCell>` following the interior
    // mutability pattern. In this example, it would not be needed, but we include the wrapping
    // anyway as example.
    let worker = Worker::new("worker.js").unwrap();
    console::log_1(&"Created a new worker from within Wasm".into());

    // Pass the worker to the function which sets up the `onchange` callback.
    setup_input_onchange_callback(worker);
}

fn setup_input_onchange_callback(worker: Worker) {
    // If our `onmessage` callback should stay valid after exiting from the `onchange` closure,
    // we need to either forget it (so it is not destroyed) or store it somewhere.
    // To avoid leaking memory every time we want to receive a response from the worker, we
    // move a handle into the `onchange` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used, and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"callback triggered".into());

        // Hide form, Show Waiting screen...
        let document = web_sys::window().unwrap().document().unwrap();
        document.get_element_by_id("settings-form").unwrap().set_attribute("style", "display: none").unwrap();
        document.get_element_by_id("waiting-screen").unwrap().set_attribute("style", "display: block").unwrap();

        // Parse settings from the form. Web Workers cannot access the window, document, or DOM,
        // so we determine the settings here and send them to the Worker as parameters.
        let worker_request = get_worker_request().expect("Error fetching settings");
        //let rom = get_rom().expect("Error fetching ROM");

        // console::log_1(&format!("rom size: {}", rom.len()).into());
        // console::log_1(&format!("rom: {rom:?}").into());

        let json = serde_json::to_string(&worker_request)
            .expect(&format!("Failed to serialize Settings as JSON: {:?}", worker_request));

        // Access worker behind shared handle, following the interior mutability pattern.
        persistent_callback_handle = get_on_msg_callback();

        // Since the worker returns the message asynchronously, we attach a callback to be
        // triggered when the worker returns.
        worker.set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));

        // Send the actual message to the Worker
        let _ = worker.post_message(&json.into());

        false
    }) as Box<dyn FnMut() -> bool>);

    let document = web_sys::window().unwrap().document().unwrap();

    // Attach the closure as `onsubmit` callback to the input field.
    let form_element = document.get_element_by_id("settings-form").expect("#settings-form should exist");

    form_element.dyn_ref::<HtmlElement>().unwrap().set_onsubmit(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct WorkerRequest {
    rom: Vec<u8>,
    seed: u32,
    settings: Settings,
}

// fn get_rom() -> Result<Vec<u8>, String> {
//     let document = web_sys::window().unwrap().document().unwrap();
//     let element = document.get_element_by_id("rom").expect("Couldn't load ROM");
//     let html_input_element = element.dyn_ref::<HtmlInputElement>().unwrap();
//
//     let file_string = html_input_element.into_js_result().unwrap();
//     let file_string = file_string.as_string().unwrap();
//
//     Ok(file_string.as_bytes().into())
// }

fn get_worker_request() -> Result<WorkerRequest, String> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.forms().get_with_name("settings-form").expect("element: settings-form");
    let html_form_element = element.dyn_ref::<HtmlFormElement>().expect("html_form_element");
    let form_data = FormData::new_with_form(html_form_element).expect("form_data");

    // Seed Number
    let seed = u32::from_str(&form_data.get("seed").as_string().unwrap_or("0".to_owned()))
        .expect("Seed must be a number between 0 and 4294967295.");

    // Select drop downs
    let ped_requirement = PedestalSetting::try_from(form_data.get("ped_requirement").as_string().unwrap())?;
    let logic_mode = LogicMode::try_from(form_data.get("logic_mode").as_string().unwrap())?;
    let nice_items = NiceItems::try_from(form_data.get("nice_items").as_string().unwrap())?;
    let cracks = Cracks::try_from(form_data.get("cracks").as_string().unwrap())?;
    let cracksanity = Cracksanity::try_from(form_data.get("cracksanity").as_string().unwrap())?;
    let weather_vanes = WeatherVanes::try_from(form_data.get("weather_vanes").as_string().unwrap())?;
    let keysy = Keysy::try_from(form_data.get("keysy").as_string().unwrap())?;

    // Checkboxes
    let dark_rooms_lampless = parse_bool(&form_data, "dark_rooms_lampless");
    let dungeon_prize_shuffle = parse_bool(&form_data, "dungeon_prize_shuffle");
    let maiamai_madness = parse_bool(&form_data, "maiamai_madness");
    let super_items = parse_bool(&form_data, "super_items");
    let lamp_and_net_as_weapons = parse_bool(&form_data, "lamp_and_net_as_weapons");
    let bow_of_light_in_castle = parse_bool(&form_data, "bow_of_light_in_castle");
    let no_progression_enemies = !parse_bool(&form_data, "progression_enemies"); // fixme
    let swordless_mode = !parse_bool(&form_data, "swordless_mode");
    let start_with_merge = !parse_bool(&form_data, "start_with_merge");
    let start_with_pouch = !parse_bool(&form_data, "start_with_pouch");
    let bell_in_shop = !parse_bool(&form_data, "bell_in_shop");
    let sword_in_shop = !parse_bool(&form_data, "sword_in_shop");
    let boots_in_shop = !parse_bool(&form_data, "boots_in_shop");
    let assured_weapon = !parse_bool(&form_data, "assured_weapon");
    let chest_size_matches_contents = !parse_bool(&form_data, "chest_size_matches_contents");
    let minigames_excluded = !parse_bool(&form_data, "minigames_excluded");
    let skip_big_bomb_flower = !parse_bool(&form_data, "skip_big_bomb_flower");
    let purple_potion_bottles = !parse_bool(&form_data, "purple_potion_bottles");
    let night_mode = !parse_bool(&form_data, "night_mode");

    // Ranges
    let lc_requirement = parse_int(&form_data, "lc_requirement");
    let maiamai_limit = parse_int(&form_data, "maiamai_limit");
    let treacherous_tower_floors = parse_int(&form_data, "treacherous_tower_floors");

    // ROM
    // let rom = form_data.get("rom").as_string().unwrap_or("".to_owned()).as_bytes().into();

    // let rom = web_sys::File::from(form_data.get("rom"));

    // ;
    //
    // let reader = rom.stream().get_reader().dyn_into::<ReadableStreamDefaultReader>().expect_throw("Reader is reader");

    // let _ = rom.array_buffer().then(&Closure::wrap(Box::new(move |js_value: JsValue| {
    //     // todo
    //
    //
    // }) as Box<dyn FnMut(_)>));

    Ok(WorkerRequest {
        rom: vec![], // TODO
        seed,
        settings: Settings {
            dev_mode: false,
            lc_requirement,
            yuganon_requirement: lc_requirement,
            ped_requirement,
            logic_mode,
            dark_rooms_lampless,
            dungeon_prize_shuffle,
            maiamai_limit,
            maiamai_madness,
            nice_items,
            super_items,
            lamp_and_net_as_weapons,
            cracks,
            cracksanity,
            weather_vanes,
            ravios_shop: RaviosShop::Open,
            bow_of_light_in_castle,
            no_progression_enemies,
            keysy,
            progressive_bow_of_light: false,
            swordless_mode,
            start_with_merge,
            start_with_pouch,
            bell_in_shop,
            sword_in_shop,
            boots_in_shop,
            assured_weapon,
            chest_size_matches_contents,
            minigames_excluded,
            skip_big_bomb_flower,
            trials_door: Default::default(),
            treacherous_tower_floors,
            purple_potion_bottles,
            night_mode,
            user_exclusions: Default::default(),
        },
    })
}

fn parse_int(form_data: &FormData, name: &str) -> u8 {
    let js_value = form_data.get(name);
    let str_value = js_value.as_string().expect(&format!("Couldn't convert JsValue to String: {js_value:?}"));

    let val = u8::from_str(&str_value).expect(&format!("Couldn't parse u8 for name={name}, val={str_value}"));
    // log::debug!("{val:?}");
    val
}

fn parse_bool(form_data: &FormData, name: &str) -> bool {
    form_data.get(name).is_null()
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::wrap(Box::new(move |event: MessageEvent| {
        event.prevent_default();

        console::log_2(&"Received response: ".into(), &event.data());

        // let result = match event.data().as_bool().unwrap() {
        //     true => "even",
        //     false => "odd",
        // };
        //
        // let document = web_sys::window().unwrap().document().unwrap();
        // document
        //     .get_element_by_id("resultField")
        //     .expect("#resultField should exist")
        //     .dyn_ref::<HtmlElement>()
        //     .expect("#resultField should be a HtmlInputElement")
        //     .set_inner_text(result);
    }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn generate_seed(request: String) {
    wasm_logger::init(wasm_logger::Config::default());

    let worker_request = serde_json::from_str::<WorkerRequest>(&request).expect("Failed to deserialize settings");

    info!("{worker_request:?}");

    let seed = if worker_request.seed == 0 { rand::random() } else { worker_request.seed };

    info!("Generating Seed: {}", seed);

    // let settings = parse_settings().expect("Error Parsing Settings");
    // let rom = vec![];

    // Load User Config
    // let user_config: &UserConfig = &randomizer::system::System::load_config().unwrap_or_else(|error| {
    //     panic!("Failed to parse configuration file: config.json\n\
    //             Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
    //             Full Error: {}\n", error);
    // });

    // info!("{settings:?}");

    info!("Skipping actual seed generation");
    let _ = match randomizer::generate_seed(seed, worker_request.settings, worker_request.rom) {
        Ok(p) => Ok(p),
        Err(err) => Err(format!("{err:?}")),
    };
}

/// Set a hook to log a panic stack trace in JS.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
