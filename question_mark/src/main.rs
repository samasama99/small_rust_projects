#[derive(Clone)]
pub struct One {
    first_layer: Option<Two>,
}
#[derive(Clone)]
pub struct Two {
    second_layer: Option<Three>,
}
#[derive(Clone)]
pub struct Three {
    third_layer: Option<Four>,
}
#[derive(Clone)]
pub struct Four {
    fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer
            .clone()?
            .second_layer
            .clone()?
            .third_layer
            .clone()?
            .fourth_layer
            .clone()
    }
}

fn main() {
    let a = One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000),
                }),
            }),
        }),
    };

    // output: 1000
    println!(
        "{:?}",
        match a.get_fourth_layer() {
            Some(e) => e,
            None => 0,
        }
    )
}
