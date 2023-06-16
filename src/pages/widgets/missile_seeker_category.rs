use yew::{function_component, Html, html};

#[function_component(MissileSeekerCategory)]
pub fn missile_seeker_category() -> Html {
	// language=html
	html!(
		<dialog id="dialog_missiles">
	<p class="missile_category_table">{"Missile category"} </p>
	<div class="missile_category_box">
		<button class="missile_class_btn" data-type="ir">
			<img alt="Infrared Homing IR"
				 src="https://raw.githubusercontent.com/gszabi99/War-Thunder-Datamine/master/atlases.vromfs.bin_u/gameuiskin/missile_type_b_air_to_air.png"
				 class="stacked"/>
			<p class="missile_class_btn_text stacked">{"IR"}</p>
		</button>
		<button class="missile_class_btn" data-type="sarh">
			<img alt="Semi Active Radar Homing SARH"
				 src="https://raw.githubusercontent.com/gszabi99/War-Thunder-Datamine/master/atlases.vromfs.bin_u/gameuiskin/missile_air_to_air_midrange.png"
				 class="stacked"/>
			<p class="missile_class_btn_text stacked">{"SARH"}</p>
		</button>
		<button class="missile_class_btn" data-type="arh">
			<img alt="Active Radar Homing ARH"
				 src="https://raw.githubusercontent.com/gszabi99/War-Thunder-Datamine/master/atlases.vromfs.bin_u/gameuiskin/missile_type_b_air_to_air_midrange.png"
				 class="stacked"/>
			<p class="missile_class_btn_text stacked">{"ARH"}</p>
		</button>
	</div>
	<form class="missile_category_table" method="dialog">
		<button>{"Show all"}</button>
	</form>
</dialog>
	)
}