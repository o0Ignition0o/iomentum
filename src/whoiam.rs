
use yew::prelude::*;

pub struct WhoIAm {
    photo_source: &'static str,
    photo_alt: &'static str,
    name: &'static str,
    job: &'static str,
    offers: Vec<&'static str>
}

pub enum NoMsg {}

impl Component for WhoIAm {

    type Message = NoMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        WhoIAm {
            photo_source: "images/jeremy_lempereur.jpg",
            photo_alt: "Jérémy Lempereur",
            name: "Jérémy Lempereur",
            job: "Freelance Software Engineer",
            offers: vec!["Tech lead","Loves to adapt and get things done","Mozilla code contributor"]
         }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<WhoIAm> for WhoIAm {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div class="inner",>
			    	<a href="#", class="image",><img src={ self.photo_source }, alt={ self.photo_alt }, /></a>
                    <div class="content",>
			    		<h2 class="major",>{ self.name }</h2>
                        <ul class="offers", >
                            <li>{ self.job }</li>
                            { for self.offers.iter().map(|offer| display_offer(*offer)) }
                            <li style="margin-top: 30px;",>
                                <a href="#footer", class="button", style="background-color:#2e3141",>{ "Let's meet!" }</a>
                            </li>
                        </ul>
			    	</div>
                </div>
            </div>
            <div class="center",>
            </div>
        }
    }
}

fn display_offer(offer: &'static str) -> Html<WhoIAm> {
    html! {
            <li>{ offer }</li>
    }
}