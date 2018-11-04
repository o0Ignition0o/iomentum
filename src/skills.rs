
use yew::prelude::*;

pub struct Skills {
    se_title: &'static str,
    se: Vec<&'static str>,
    it_title: &'static str,
    it: Vec<&'static str>
}

pub enum NoMsg {}

impl Component for Skills {

    type Message = NoMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Skills {
            se_title: "Software Engineering",
            se: vec!["Technical Leadership", "Web Backend", "IoT", "Devops"],
            it_title: "IT consulting",
            it: vec!["Developer mentoring", "Agile methodology", "IT strategy", "GDPR compliance"]
         }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Skills> for Skills {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="inner",>
			    <div class="content",>
				    <h2 class="major",>{ "Skills" }</h2>
				    <div class="row",>
				    	<div class="col-6 col-12-medium",>
                            <ul class="skills",>
                                <li><h3>{ self.se_title }</h3></li>
                                { for self.se.iter().map(|skill| display_skill(*skill)) }
                            </ul>
				    	</div>
                        <div class="col-6 col-12-medium",>
                            <ul class="skills",>
                                <li><h3>{ self.it_title }</h3></li>
                                { for self.it.iter().map(|skill| display_skill(*skill)) }
                            </ul>
				    	</div>
					</div>
				</div>
			</div>
        }
    }
}

fn display_skill(skill: &'static str) -> Html<Skills> {
    html! {
            <li>{ skill }</li>
    }
}