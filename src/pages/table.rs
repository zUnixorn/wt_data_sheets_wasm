use yew::{function_component, html, Html};
use crate::MissileSeekerCategory;

#[function_component(Table)]
pub fn table() -> Html {
	html!( // language=html
		<>
		<MissileSeekerCategory />

<div class="top_header">
	<div>
		<p>{"Parameters used for ballistic simulation to calculate flight range."}<a
				href="documentation.html">{"Simulation formula and documentation"}</a></p>
		<button class="rounded_button" id="reset_values" style="display: none">{"Reset values"}</button>
	</div>
	<div class="side_by_side">
		//Altitude

		<div class="container">

			<div class="range-slider">
				<input class="rs-label" id="alt" type="number" min="0" max="10000" value="1000" step="100"/>
				<input id="alt_slider" class="rs-range" type="range" min="0" max="10000" value="1000" step="100"/>

			</div>

			<div class="box-minmax">
				<span>{"meters altitude"}</span>
			</div>
		</div>
		//Speed
		<div class="container">

			<div class="range-slider">
				<input class="rs-label" id="vel" type="number" min="0" max="1000" value="343" step="10"/>
				<input id="vel_slider" class="rs-range" type="range" min="0" max="1000" value="343" step="10"/>

			</div>

			<div class="box-minmax">
				<span>{"m/s launch velocity"}</span>
			</div>
		</div>
	</div>
</div>
<table class="main_table missile_table" id="ir_table">
	<thead>
	<tr class="top-tr" id="ir_index">
		<td>
			<a href="documentation.html#name">{"Name"}</a></td>
		<td class="sort" data-type="range">
			<a class="calculated" href="documentation.html#range">{"Range"}</a>
		</td>
		<td class="sort" data-type="twr">
			{"TWR"}
		</td>
		<td class="sort" data-type="max_speed">
			<a href="documentation.html#maxspeed">{"Max Speed"}</a>
		</td>
		<td class="sort" data-type="delta_v">
			<a class="calculated" href="documentation.html#deltav">{"DeltaV"}</a>
		</td>
		<td class="sort" data-type="launch_g">
			<a href="documentation.html#launchg">{"LaunchG"}</a>
		</td>
		<td class="sort" data-type="flight_g">
			<a href="documentation.html#flightg">{"FlightG"}</a>
		</td>
		<td class="sort" data-type="rear_aspect">
			<a href="documentation.html#rearaspect">{"Rear aspect"}</a>
		</td>
		<td class="sort" data-type="all_aspect">
			<a href="documentation.html#allaspect">{"All aspect"}</a>
		</td>
		<td class="sort" data-type="ir_decoys">
			<a href="documentation.html#irdecoys">{"IR decoys"}</a>
		</td>
		<td class="sort" data-type="ircm">
			<a href="documentation.html#ircm">{"IRCM"}</a>
		</td>
		<td class="sort" data-type="ircm">
			<a href="documentation.html#sun_rockets_misc">{"Sun & misc."}</a>
		</td>
		<td class="sort" data-type="ircm">
			<a href="documentation.html#dircm">{"DIRCM"}</a>
		</td>
		<td class="sort" data-type="ircm">
			<a href="documentation.html#afterburner_plume">{"Afterburner"}</a>
		</td>
		<td class="sort" data-type="fov">
			<a href="documentation.html#fov">{"Fov"}</a>
		</td>
		<td class="sort" data-type="gate">
			<a href="documentation.html#fov">{"Gate"}</a>
		</td>
		<td class="sort" data-type="launch_fov">
			<a href="documentation.html#launchfov">{"LaunchFov"}</a>
		</td>
		<td class="sort" data-type="flight_fov">
			<a href="documentation.html#flightfov">{"FlightFov"}</a>
		</td>
		<td class="sort" data-type="warm_up_time">
			<a href="documentation.html#warmuptime">{"WarmUpTime"}</a>
		</td>
		<td class="sort" data-type="work_time">
			<a href="documentation.html#worktime">{"WorkTime"}</a>
		</td>
		<td class="sort" data-type="time_out">
			<a href="documentation.html#timeout">{"Time out"}</a>
		</td>
		<td>
			<a href="documentation.html#uncage">{"Uncage"}</a>
		</td>
		<td>
			<a href="documentation.html#allow_radar_slave">{"Radar slave"}</a>
		</td>
		<td>
			{"Datalink"}
		</td>
		<td>
			{"INS"}
		</td>
	</tr>
	</thead>
</table>
<table class="sarh_table missile_table" id="sarh_table">
	<thead>
	<tr class="top-tr" id="sarh_index">
		<td>
			<a href="documentation.html#name">{"Name"}</a></td>
		<td class="sort" data-type="range">
			<a class="calculated" href="documentation.html#range">{"Range"}</a>
		</td>
		<td class="sort" data-type="twr">
			{"TWR"}
		</td>
		<td class="sort" data-type="max_speed">
			<a href="documentation.html#maxspeed">{"Max Speed"}</a>
		</td>
		<td class="sort" data-type="delta_v">
			<a class="calculated" href="documentation.html#deltav">{"DeltaV"}</a>
		</td>
		<td class="sort" data-type="launch_g">
			<a href="documentation.html#launchg">{"LaunchG"}</a>
		</td>
		<td class="sort" data-type="flight_g">
			<a href="documentation.html#flightg">{"FlightG"}</a>
		</td>
		<td class="sort" data-type="launch_fov">
			<a href="documentation.html#launchfov">{"LaunchFov"}</a>
		</td>
		<td class="sort" data-type="flight_fov">
			<a href="documentation.html#flightfov">{"FlightFov"}</a>
		</td>
		<td class="sort" data-type="warm_up_time">
			<a href="documentation.html#warmuptime">{"WarmUpTime"}</a>
		</td>
		<td class="sort" data-type="work_time">
			<a href="documentation.html#worktime">{"WorkTime"}</a>
		</td>
		<td>
			<a href="documentation.html#uncage">{"Uncage"}</a>
		</td>
		<td>
			{"Datalink"}
		</td>
		<td>
			{"INS"}
		</td>
	</tr>
	</thead>
</table>
<table class="arh_table missile_table" id="arh_table">
	<thead>
	<tr class="top-tr" id="arh_index">
		<td>
			<a href="documentation.html#name">{"Name"}</a></td>
		<td class="sort" data-type="range">
			<a class="calculated" href="documentation.html#range">{"Range"}</a>
		</td>
		<td class="sort" data-type="twr">
			{"TWR"}
		</td>
		<td class="sort" data-type="max_speed">
			<a href="documentation.html#maxspeed">{"Max Speed"}</a></td>
		<td class="sort" data-type="delta_v">
			<a class="calculated" href="documentation.html#deltav">{"DeltaV"}</a>
		</td>
		<td class="sort" data-type="launch_g">
			<a href="documentation.html#launchg">{"LaunchG"}</a>
		</td>
		<td class="sort" data-type="flight_g">
			<a href="documentation.html#flightg">{"FlightG"}</a>
		</td>
		<td class="sort" data-type="launch_fov">
			<a href="documentation.html#launchfov">{"LaunchFov"}</a>
		</td>
		<td class="sort" data-type="flight_fov">
			<a href="documentation.html#flightfov">{"FlightFov"}</a>
		</td>
		<td class="sort" data-type="warm_up_time">
			<a href="documentation.html#warmuptime">{"WarmUpTime"}</a>
		</td>
		<td class="sort" data-type="work_time">
			<a href="documentation.html#worktime">{"WorkTime"}</a>
		</td>
		<td>
			<a href="documentation.html#uncage">{"Uncage"}</a>
		</td>
		<td>
			{"Datalink"}
		</td>
		<td>
			{"INS"}
		</td>
	</tr>
	</thead>
</table>
		</>
	)
}