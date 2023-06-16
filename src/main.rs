use std::collections::HashMap;

use build_timestamp::build_time;
use lazy_static::lazy_static;
use wt_datamine_extractor_lib::bombs::bombs::Bomb;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;
use wt_sensor::Radar;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::widgets::missile_seeker_category::MissileSeekerCategory;

use crate::pages::privacy_policy::PrivacyPolicy;
use crate::pages::navbar::Header;

use crate::util::{console_log, get_document, make_missile_option_inputs};

pub mod table;
pub mod util;
pub mod live_calc;
pub mod comparison;
pub mod thermal_index;
pub mod shell_index;
pub mod missile_ballistics;
pub mod bombing_table;
pub mod battle_rating_statistics;
pub mod fm;
pub mod utils;
pub mod localhost;
pub mod radar;
pub mod blk_proto;
mod pages;

pub const GAME_VER: &str = include_str!("../wt_datamine_extractor/meta_index/version.txt");
pub const BATTLE_RATINGS_RAW: &str = include_str!("../wt_datamine_extractor/battle_rating/all.json");

build_time!("%Y-%m-%d %H:%M:%S");

lazy_static! {
    static ref BOMBS: Vec<Bomb> = {
		 serde_json::from_str::<Vec<Bomb>>(&include_str!("../wt_datamine_extractor/bombs/all.json")).unwrap()
    };
	static ref BOMB_MAP: HashMap<String, Bomb> = {
		HashMap::from_iter(
			BOMBS.clone()
		.into_iter()
		.map(|x|(x.name.clone(), x))
		)
	};
	 static ref THERMALS: Vec<Thermal> = {
		let json = include_str!("../wt_datamine_extractor/thermal_index/all.json");
		let mut thermals: Vec<Thermal> = serde_json::from_str(json).unwrap();
		thermals.sort_by_key(|d| d.name.clone());

		thermals
	};
	static ref MISSILES: Vec<Missile> = {
		let json = include_str!("../wt_datamine_extractor/missile_index/all.json");
		let mut missiles: Vec<Missile> = serde_json::from_str(json).unwrap();
		// Hide useless duplicates
		// Missiles with the default keyword are the stock-counterparts to
		missiles = missiles.into_iter().filter(|m|!m.name.contains("default")).collect();
		missiles.sort_by_key(|d| d.name.clone());

		missiles
	};

	static ref RADAR: Radar = {
		let json = include_str!("../wt_datamine_extractor/sensors/us_an_apg_66.txt");
		let  radar = Radar::from_str("us_an_apg_66".to_owned(), json);
		radar
	};
}


#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/")]
	Index,

	#[at("/table")]
	Table,


	#[at("/privacy_policy")]
	PrivacyPolicy,


	#[not_found]
	#[at("/404")]
	NotFound,
}

fn main() {
	yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {

	html! {
		<>
		<Header/>
        <BrowserRouter>
            <Switch<Route> render={route} />
        </BrowserRouter>
		<Footer/>
		</>
    }
}

fn route(routes: Route) -> Html {
	match routes {
		Route::Index => html! { {"index"} },


		Route::Table => html! {
			 <MissileSeekerCategory />
		},

		Route::PrivacyPolicy => html! {
            <PrivacyPolicy />
        },


		Route::NotFound => html! {
			<div style="width: fit-content; margin: 0 auto">
				<h1 style="width: fit-content; margin: 0 auto">{ "404 \n" }</h1>
				<h2>{"The requested page does not exist"}</h2>
			</div>
		},
	}
}

#[function_component(Footer)]
fn footer() -> Html {
	let updated = {format!("Game version: {}. Updated on: {}", GAME_VER, BUILD_TIME)};
	html! {
		<>
			<div id="footer">
				{updated}<br/>
				<p>
					<a href="privacy_policy">{"Privacy Policy"}</a>
				</p>
			</div>
		</>
	}
}