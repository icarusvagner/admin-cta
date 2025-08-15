use std::fmt::Display;

#[derive(Default, Debug, Clone)]
pub enum Themes {
    #[default]
    Light,
    Dark,
    Night,
    Dracula,
    Cupcake,
    Bumblebee,
    Garden,
    Wireframe,
    Dim,
    Silk,
    Totalnova,
    Rawsupreme,
    Holydonut,
    Frtowm,
}

pub const THEMES: [Themes; 14] = [
    Themes::Light,
    Themes::Dark,
    Themes::Night,
    Themes::Dracula,
    Themes::Cupcake,
    Themes::Bumblebee,
    Themes::Garden,
    Themes::Wireframe,
    Themes::Dim,
    Themes::Silk,
    Themes::Totalnova,
    Themes::Rawsupreme,
    Themes::Holydonut,
    Themes::Frtowm,
];

impl Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Themes::Light => write!(f, "light"),
            Themes::Dark => write!(f, "dark"),
            Themes::Night => write!(f, "night"),
            Themes::Dracula => write!(f, "dracula"),
            Themes::Cupcake => write!(f, "cupcake"),
            Themes::Bumblebee => write!(f, "bumblebee"),
            Themes::Garden => write!(f, "garden"),
            Themes::Wireframe => write!(f, "wireframe"),
            Themes::Dim => write!(f, "dim"),
            Themes::Silk => write!(f, "silk"),
            Themes::Totalnova => write!(f, "totalnova"),
            Themes::Rawsupreme => write!(f, "rawsupreme"),
            Themes::Holydonut => write!(f, "holydonut"),
            Themes::Frtowm => write!(f, "frtown"),
        }
    }
}

impl Themes {
    pub fn as_str(&self) -> &str {
        match self {
            Themes::Light => "light",
            Themes::Dark => "dark",
            Themes::Night => "night",
            Themes::Dracula => "cracula",
            Themes::Cupcake => "cupcake",
            Themes::Bumblebee => "bumblebee",
            Themes::Garden => "garden",
            Themes::Wireframe => "wireframe",
            Themes::Dim => "dim",
            Themes::Silk => "silk",
            Themes::Totalnova => "totalnova",
            Themes::Rawsupreme => "rawsupreme",
            Themes::Holydonut => "holydonut",
            Themes::Frtowm => "frtown",
        }
    }

    pub fn to_title(&self) -> &str {
        match self {
            Themes::Light => "Light",
            Themes::Dark => "Dark",
            Themes::Night => "Night",
            Themes::Dracula => "Dracula",
            Themes::Cupcake => "Cupcake",
            Themes::Bumblebee => "Bumblebee",
            Themes::Garden => "Garden",
            Themes::Wireframe => "Wireframe",
            Themes::Dim => "Dim",
            Themes::Silk => "Silk",
            Themes::Totalnova => "Total Nova",
            Themes::Rawsupreme => "Raw Supreme",
            Themes::Holydonut => "Holy Donut",
            Themes::Frtowm => "FR Town",
        }
    }
}
