use crate::Msg;
use seed::*;
use seed::prelude::*;
use super::*;
use about_content::about_body_content;
pub fn body_content(model: &Model) -> El<Msg> {
    if model.current_page == Page::Sobre {
        about_body_content()
    }else if model.current_page == Page::Fotos {
        div![attrs!{At::Class => "row"},
            div![attrs!{At::Class => "column"},
                img![attrs!{At::Src => "fotos_rep/piscina0.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/piscina1.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/piscina2.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/piscina3.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/piscina4.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random7.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random8.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random16.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random22.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random23.jpg"}, style!["width" => "100%"]],
            ],
            div![attrs!{At::Class => "column"},
                img![attrs!{At::Src => "fotos_rep/sala0.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/sala1.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/sala2.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random5.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random9.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random10.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random11.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random17.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random20.jpg"}, style!["width" => "100%"]],
            ],
            div![attrs!{At::Class => "column"},
                img![attrs!{At::Src => "fotos_rep/cozinha0.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/cozinha1.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/cozinha2.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/cozinha3.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random12.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random13.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random18.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random21.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random24.jpg"}, style!["width" => "100%"]],
            ],
            div![attrs!{At::Class => "column"},
                img![attrs!{At::Src => "fotos_rep/random0.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random1.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random2.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random3.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random4.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random14.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random15.jpg"}, style!["width" => "100%"]],
                img![attrs!{At::Src => "fotos_rep/random19.jpg"}, style!["width" => "100%"]],
            ]
        ]
    }else {
        seed::empty()
    }
}


// mente aberta // livres de preconceitos // dona helena nossa mãe // ARU