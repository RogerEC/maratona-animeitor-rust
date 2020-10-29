// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
//#![allow(clippy::wildcard_imports)]

use maratona_animeitor_rust::data;
use seed::{prelude::*, *};
extern crate rand;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.


// Request::new(get_request_url())
//         .method(Method::Post)
//         .json(&shared::SendMessageRequestBody { text: new_message })?
//         .fetch()
//         .await?
//         .check_status()?
//         .json()
//         .await

async fn fetch_allruns() -> fetch::Result<data::RunsFile> {
    Request::new("/allruns")
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn fetch_contest() -> fetch::Result<data::ContestFile> {
    Request::new("/contest")
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Model::default()

    orders.skip().perform_cmd({
        async {
            let m = fetch_contest().await;
            Msg::FetchedContest(m)
        }
    });
    Model { 
        contest: data::ContestFile::dummy(),
        runs: data::RunsFile::empty(),
        current_run: 0,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
// type Model = Vec<i64>;
struct Model {
    contest : data::ContestFile,
    runs: data::RunsFile,
    current_run: usize,
}

// impl Model {
//     fn append(&mut self) {
//         self.items.push(self.items.len() as i64)
//     }
// }

// impl Default for Model {
//     fn default() -> Self {
//         Self { items : Vec::new() }
//     }

// }

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    // Append,
    // Shuffle,
    // Sort,
    // SortEnd,
    FetchedRuns(fetch::Result<data::RunsFile>),
    FetchedContest(fetch::Result<data::ContestFile>),
}

// fn shuffle(v: &mut  Vec<i64> ) {
//     use rand::thread_rng;
//     use rand::seq::SliceRandom;

//     let mut rng = thread_rng();
//     v.shuffle(&mut rng);
// }

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // Msg::Append => model.items.push(model.items.len() as i64),
        // Msg::Shuffle => {
        //     orders.perform_cmd(cmds::timeout(1000, || Msg::Sort));
        //     shuffle(&mut model.items)
        // },
        // Msg::Sort => {
        //     orders.perform_cmd(cmds::timeout(1000, || Msg::SortEnd));
        //     model.items.sort();
        // },
        // Msg::SortEnd => {
        //     log!("sort ended!")
        // },
        Msg::FetchedRuns(Ok(runs)) => {
            log!("fetched runs data!");
            model.runs = runs;
        },
        Msg::FetchedContest(Ok(contest)) => {
            log!("fetched contest data!");

            model.contest = contest;
            model.contest.reload_score().unwrap();
            orders.perform_cmd({
                async { Msg::FetchedRuns(fetch_allruns().await) }
            });
        },
        Msg::FetchedContest(Err(e)) => {
            log!("fetched contest error!", e)
        },
        Msg::FetchedRuns(Err(e)) => {
            log!("fetched runs error!", e)
        },

    }
}

fn make_style(e : & i64, offset : i64) -> seed::Style {
    style!{
        St::Position => "absolute",
        St::Top => px(100 - offset*50 + e*50),
        St::Transition => "1s ease top",
        St::BorderStyle => "solid",
        St::BorderWidth => px(1),
        St::Padding => px(5),
        St::BorderColor => if *e!=0 { CSSValue::Ignored } else { "red".into() },
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
// #[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.



fn view(model: &Model) -> Node<Msg> {
    let problem_letters = 
        vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
             "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    // let all_problems = vec!["A", "B", "C"];
    let n = model.contest.number_problems;
    log!(model.contest.number_problems);
    let all_problems = &problem_letters[..n];
    div![
        attrs!{"border" => 1},
        div![
            C!["run"],
            style!{ St::Position => "absolute", St::Top => px(10) },
            div![C!["cell", "titulo"], "Placar"],
            all_problems.iter().map( |p| div![C!["cell", "problema"], p])
        ],
        model.contest.score_board.iter().enumerate().map (|(idx, dev)| {
            let team = &model.contest.teams[&dev.clone()];
            let (solved, penalty) = team.score();
            div![
                C!["run"],
                attrs!{"key"=>dev},
                style!{
                    St::Position => "absolute",
                    St::Top => px(10 + (1+idx) * 90),
                    St::Transition => "1s ease top",
                },
                div![C!["cell", "colocacao"], team.placement],
                div![
                    C!["cell", "time"],
                    div![C!["nomeEscola"], &team.escola],
                    div![C!["nomeTime"], &team.name],
                ],
                div![
                    C!["cell", "problema"],
                    div![C!["cima"], solved],
                    div![C!["baixo"], penalty],
                ],
                all_problems.iter().map( |prob| {
                    match team.problems.get(*prob) {
                        None => div![C!["cell", "problema"], "-"],
                        Some(prob_v) => {
                            if prob_v.solved {
                                div![
                                    C!["cell", "problema", "verde"],
                                    div![C!["cima"], "+", prob_v.submissions],
                                    div![C!["baixo"], prob_v.penalty],
                                ]
                            }
                            else {
                                let color = if prob_v.wait {"amarelo"} else {"vermelho"};
                                div![
                                    C!["cell", "problema", color],
                                    div![C!["cima"], "X"],
                                    div![C!["baixo"], "(", prob_v.submissions, ")"],
                                ]
                            }
                        },
                    }
                })
            ]
        })
        // button!["+1", ev(Ev::Click, |_| Msg::Append),],
        // button!["shuffle", ev(Ev::Click, |_| Msg::Shuffle),],
        // button!["sort", ev(Ev::Click, |_| Msg::Sort),],
        // model.items.iter().enumerate().map( |(i,e)| 
        //     div![
        //         id![i],
        //         make_style(e, 0),
        //         i,
        //         "->",
        //         e
        //     ]
        // ),
        // div![
        //     id![1],
        //     "Up",
        //     make_style(model),
        // ],
        // div![
        //     id![2],
        //     "Down",
        //     make_style(&(model+1)),
        // ],
        // <div id=1 style=updown_style(self.value%2) >{ "Up" }</div>
        // <div id=2 style=updown_style((1+self.value)%2) >{ "Down" }</div>
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);


}
