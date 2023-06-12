use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
	html! {
		<nav class="menu-container">
	//burger menu
	<input aria-label="Toggle menu" type="checkbox"/>
	<span></span>
	<span></span>
	<span></span>

	// logo
	// <a href="#" class="menu-logo"
	// 	<img src="" alt="M"/
	// 	</a>

	// menu items
	<div class="menu">
		<ul>
			<li>
				<a href="/">
					{"Home"}
				</a>
			</li>
			<li>
				<div class="dropdown">
					<button class="dropbtn">
							{"A-A missiles"}
						<i class="fa fa-caret-down"></i>
					</button>
					<div class="dropdown-content">
						<a href="table">
							{"Overview table"}
						</a>
						<a href="live_calc">
							{"Live calculation"}
						</a>
						<a href="compare">
							{"Compare missiles"}
						</a>
						<a href="documentation">
							{"Documentation"}
						</a>
						<a href="missile_ballistics">
							{"Missile ballistics simulator"}
						</a>
					</div>
				</div>
			</li>
			<li>
				<div class="dropdown">
					<button class="dropbtn">
						{"Anything vehicles"}
						<i class="fa fa-caret-down"></i>
					</button>
					<div class="dropdown-content">
						<a href="thermal_index">
							{"Thermal index"}
						</a>
						<a href="shell_index">
							{"Shell index"}
						</a>
						<a href="bombing_table">
						{"Bombing table"}
						</a>
					</div>
				</div>
			</li>
		</ul>
		<ul>
			<li class="nav_source_setting">
				<a href="https://github.com/Warthunder-Open-Source-Foundation/wt_data_sheets_wasm"
				   rel="noopener noreferrer" target="_blank">
					//The width and height is defaults for on-load layout shift, gets overridden in CSS
					<svg height="16" width="16" aria-hidden="true" viewBox="0 0 16 16" version="1.1"
						 data-view-component="true" id="github_logo">
						<path fill-rule="evenodd"
							  d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
					</svg>
					{"Source code"}
				</a>
			</li>
			<li>
				<svg  height="20" width="20" version="1.1" id="settings_icon"
					 xmlns="http://www.w3.org/2000/svg"
					 viewBox="0 0 472.615 472.615"
						//xml:space="preserve"
						>
						<polygon
								points="285.279,192.568 271.356,206.492 301.153,236.29 271.356,266.087 285.279,280.011 329,236.29"/>
					<polygon
							points="201.259,206.492 187.337,192.568 143.615,236.29 187.337,280.011 201.259,266.087 171.462,236.29 "/>
					<rect x="173.856" y="232.405" transform="matrix(0.3417 -0.9398 0.9398 0.3417 -72.2692 381.3245)"
						  width="124.435" height="19.692"/>
					<path d="M472.615,274.117V198.4l-55.335-9.255c-4.332-16.64-10.929-32.492-19.692-47.458L430.178,96l-53.563-53.563
							l-45.686,32.591c-14.966-8.763-30.917-15.36-47.557-19.692L274.215,0H198.4l-9.157,55.335
							c-16.64,4.332-32.591,10.929-47.557,19.692L96,42.437L42.437,96l32.591,45.686c-8.763,14.966-15.36,30.818-19.692,47.458L0,198.4
							v75.717l55.335,9.255c4.332,16.64,10.929,32.591,19.692,47.557l-32.591,45.686L96,430.178l45.686-32.689
							c14.966,8.862,30.917,15.458,47.557,19.791l9.157,55.335h75.815l9.157-55.335c16.64-4.332,32.591-10.929,47.557-19.791
							l45.686,32.689l53.563-53.563l-32.591-45.686c8.763-14.966,15.36-30.917,19.692-47.557L472.615,274.117z M236.308,364.29
							c-70.582,0-128-57.424-128-128c0-70.577,57.418-128,128-128c70.582,0,128,57.423,128,128
							C364.308,306.866,306.89,364.29,236.308,364.29z"/>

					</svg>
			</li>
		</ul>

	</div>
</nav>
	}
}