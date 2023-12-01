use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-krabbels.css"/>

        // sets the document title
        <Title text="KRABBELS"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Board() -> impl IntoView {
    view! {
        <div class="grid gap-0.5 board lg:board-lg border-2 rounded border-green-600">
            <button class="tile bg-orange-600">MT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-orange-600">MT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-orange-600">MT</button>

            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile bg-orange-600">MT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">"★"</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-orange-600">MT</button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>

            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>

            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-blue-400">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-rose-200">MD</button>
            <button class="tile"></button>

            <button class="tile bg-orange-600">MT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LD</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-orange-600">MT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-cyan-200">LT</button>
            <button class="tile"></button>
            <button class="tile"></button>
            <button class="tile bg-orange-600">MT</button>

        </div>
    }
}

#[component]
fn Tile(letter: char) -> impl IntoView {
    let toggle = RwSignal::new(false);
    view! {
        <li class="inline-block">
            <button class="tile-bag lg:tile-bag-lg" class=("tile-bag-sel", move || toggle()) on:click=move |_| toggle.update(|t| *t = !*t)>
            {letter}
            </button>
        </li>
    }
}

#[derive(Clone, Copy, Debug)]
struct Letter {
    name: char,
    value: u8,
    id: u8,
}

#[component]
fn TileBag() -> impl IntoView {
    let mut bag = Vec::new();

    bag.extend((0..9).map(|_| {
        Letter {
            name: 'A',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'B',
            value: 3,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'C',
            value: 3,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..3).map(|_| {
        Letter {
            name: 'D',
            value: 2,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..15).map(|_| {
        Letter {
            name: 'E',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'F',
            value: 4,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'G',
            value: 2,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'H',
            value: 4,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..8).map(|_| {
        Letter {
            name: 'I',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'J',
            value: 8,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'K',
            value: 10,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..5).map(|_| {
        Letter {
            name: 'L',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'M',
            value: 2,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'N',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'O',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'P',
            value: 3,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'Q',
            value: 8,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'R',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'S',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'T',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..6).map(|_| {
        Letter {
            name: 'U',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: 'V',
            value: 4,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'W',
            value: 10,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'X',
            value: 10,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'Y',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..1).map(|_| {
        Letter {
            name: 'Z',
            value: 1,
            id: 0,
        }
        .clone()
    }));
    bag.extend((0..2).map(|_| {
        Letter {
            name: '.',
            value: 0,
            id: 0,
        }
        .clone()
    }));

    let mut rng = rand::thread_rng();
    // let mut n = 0;
    let letters_rack = (0..7)
        .map(|id| {
            let mut a = bag.choose(&mut rng).unwrap().clone();
            a.id = id + 1;
            a
        })
        .collect::<Vec<_>>();

    let rack = RwSignal::new(letters_rack);

    let selected = RwSignal::new(Vec::new());

    view! {

        <ul class="block">
            <p class="dark:text-white">Le sac contient:</p>
            {bag.iter()
                .map(|n| view! { <li class="inline-block dark:text-white">{n.name}</li>})
                .collect::<Vec<_>>()}
        </ul>

        <ul class="block">
            <For
                each=move || rack()
                key=|letter| letter.id
                children= move |letter| {
                    view! {
                        <li class="inline-block">
                            <button class="tile-bag lg:tile-bag-lg" class=("text-yellow-400 hover:text-purple-200", move || letter.value == 0)
                                on:click=move |_| {
                                    rack.update(|lett_vec| {
                                        lett_vec.retain(|lett_id| lett_id.id != letter.id)
                                        });
                                    selected.update(|lett_vec| {
                                        lett_vec.push(letter)
                                        })
                                    }
                            ><sup class="text-xs text-gray-400">{letter.id}</sup>{letter.name}<sub class="text-xs">{letter.value}</sub></button>

                        </li>
                    }
                }
             />
        </ul>
        <ul class="block">
            <For
                each=move || selected()
                key=|letter| letter.id
                children= move |letter| {
                    view! {
                        <li class="inline-block">
                            <button class="tile-bag bg-yellow-300 lg:tile-bag-lg" class=("text-yellow-300 hover:text-purple-200", move || letter.value == 0)
                                on:click=move |_| {
                                    selected.update(|lett_vec| {
                                        lett_vec.retain(|lett_id| lett_id.id != letter.id)
                                        });
                                    rack.update(|lett_vec| {
                                        lett_vec.push(letter)
                                        })
                                }
                              ><sup class="text-xs text-gray-400">{letter.id}</sup>{letter.name}<sub class="text-xs">{letter.value}</sub></button>
                         </li>

                    }
                }
            />
        </ul>
    }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {

        <main class="container mx-auto pt-2 lg:p-5 grid grid-rows-2 lg:grid-rows-1 lg:grid-cols-2">
            <div class="flex justify-center">
                <Board/>
            </div>

            <div class="pl-5">
                <h1 class="hidden lg:block p-5 text-4xl font-bold dark:text-yellow-100 text-center">"KRABBELS"</h1>
                <h2 class="text-xs text-center p-2 mb-5 border-b-2 border-black dark:border-white dark:text-white">"A study project to learn further RUST, LEPTOS framework and TAILDWIND css."</h2>
                // <button class="hidden lg:block p-3 bg-cyan-300 text-white rounded-full" on:click=on_click>"Click Me: " {count}</button>

                <TileBag/>


                // <h2>"Dynamic List"</h2>
                // <DynamicList initial_length=5/>

            </div>
        </main>

    }
}
