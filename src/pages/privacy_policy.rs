use yew::prelude::*;

#[function_component(PrivacyPolicy)]
pub fn privacy_policy() -> Html {
	html! {
		<>
		<p>{"This page does not collect data outside of"}<a href="https://www.goatcounter.com/help/privacy"
												 rel="noopener noreferrer"
												 target="_blank">{"GoatCounter"}</a>{"which complies with GDPR."}</p>
		<p>{"Consenting to view discord content means you consent to"}<a href="https://discord.com/privacy"
															  rel="noopener noreferrer"
															  target="_blank">{"Discord's privacy policy"}</a></p>
		</>
	}
}