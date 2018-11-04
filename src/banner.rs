
use yew::prelude::*;

pub struct Banner {
    logo_icon_classes: &'static str,
    company_name: &'static str,
    motto: &'static str
}

pub enum NoMsg {}

impl Component for Banner {

    type Message = NoMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Banner {
            logo_icon_classes: "icon fa-power-off",
            company_name: "IOMentum",
            motto: "Technology to innovate"
         }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Banner> for Banner {
    fn view(&self) -> Html<Self> {
        html! {
			<div class="inner",>
			    <div class="logo",><span class={ self.logo_icon_classes },></span></div>
			    <h2 class="brand",>{ self.company_name }</h2>
				<p> { self.motto }</p>
			</div>
        }
    }
}