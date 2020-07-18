use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{IntervalService, Task};

struct ChordGenerator {
    rng: rand::rngs::StdRng,
    distribution: Uniform<usize>,
}

impl ChordGenerator {
    fn new() -> ChordGenerator {
        ChordGenerator {
            rng: rand::rngs::StdRng::seed_from_u64(10),
            distribution: Uniform::from(0..ChordDisplay::CHORDS.len()),
        }
    }

    fn next_index(&mut self) -> usize {
        self.distribution.sample(&mut self.rng)
    }
}

struct ChordDisplay {
    link: ComponentLink<Self>,
    chord_index: usize,
    timer_task: Option<Box<dyn Task>>,
    generator: ChordGenerator,
}

impl ChordDisplay {
    const CHORDS: &'static [&'static str] = &["A", "B", "C", "D", "E", "F", "G"];
    fn next_chord(&mut self) -> () {
        self.chord_index = self.generator.next_index();
    }
}

enum Msg {
    StartTimer,
    Tick,
}

impl Component for ChordDisplay {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            chord_index: 0,
            timer_task: None,
            generator: ChordGenerator::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimer => {
                let handle = IntervalService::spawn(
                    Duration::from_secs(2),
                    self.link.callback(|_| Msg::Tick),
                );
                self.timer_task = Some(Box::new(handle))
            }
            Msg::Tick => self.next_chord(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::StartTimer)>{ "Start chord changes" }</button>
                <h1>{ ChordDisplay::CHORDS[self.chord_index] }</h1>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<ChordDisplay>::new().mount_to_body();
}
