mod random;
mod snake;

use js_sys::Function;
use snake::{Direction, SnakeGame};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, UnwrapThrowExt};
use web_sys::{window, Document, HtmlElement, KeyboardEvent};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(WIDTH, HEIGHT)));

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
    Closure::wrap(Box::new(|event: KeyboardEvent| GAME.with(|game| {
        let direction = match &event.key()[..] {
            "w" => Some(Direction::Up),
            "d" => Some(Direction::Right),
            "s" => Some(Direction::Down),
            "a" => Some(Direction::Left),
            _ => None,
        };

        if let Some(direction) = direction {
            game.borrow_mut().change_direction(direction);
        }
    })) as Box<dyn FnMut(KeyboardEvent)>);

    static DOCUMENT: Rc<RefCell<Document>> = Rc::new(RefCell::new(window().unwrap_throw().document().unwrap_throw()));
}

#[wasm_bindgen(start)]
pub fn main() {
    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    DOCUMENT.with(|document| {
        let document = document.borrow();
        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        root_container
            .style()
            .set_property("display", "inline-grid")
            .unwrap_throw();

        root_container
            .style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", HEIGHT, WIDTH),
            )
            .unwrap_throw();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");
                field_element.set_id(&format!("field-{}-{}", x, y));
                field_element.set_inner_text(" ");
                field_element.style().set_css_text(
                    "width: 1rem; height: 1rem; line-height: 1rem; text-indent: -0.2rem;",
                );
                root_container.append_child(&field_element).unwrap_throw();
            }
        }
    });
}

#[wasm_bindgen(js_name = render)]
pub fn render() {
    GAME.with(|game| {
        DOCUMENT.with(|document| {
            // Get the game and tick it.
            let mut game = game.borrow_mut();
            game.tick();

            // Get the document and update it.
            let document = document.borrow_mut();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let pos = (x, y);
                    let field_element = document
                        .get_element_by_id(&format!("field-{}-{}", x, y))
                        .unwrap_throw()
                        .dyn_into::<HtmlElement>()
                        .unwrap_throw();

                    field_element.set_inner_text({
                        if pos == game.food {
                            "🍎"
                        } else if game.snake.get(0) == Some(&pos) {
                            "❇️"
                        } else if game.snake.contains(&pos) {
                            "🟩"
                        } else {
                            " "
                        }
                    });
                }
            }
        });
    });
}
