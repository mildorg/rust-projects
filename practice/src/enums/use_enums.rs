enum WebEvent {
    PageLoad,
    PageUpload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsTodoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsTodoWithNumbers;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUpload => println!("page uploaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("Click at x={} y={}.", x, y);
        }
    }
}

pub fn practice_enums() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_string());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUpload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    let y = Operations::Subtract;
    println!("x = {:?}, y = {:?}", x, y);
}
