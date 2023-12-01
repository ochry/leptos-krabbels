
    let letters = vec!['D', 'I', 'V', 'O', 'C', 'U', 'L'];
    let togglers = (0..letters.len())
        .map(|_| RwSignal::new(false))
        .collect::<Vec<_>>();
    let zipped = togglers.clone().into_iter().zip(letters.clone());
    let valid = RwSignal::new(Vec::new());

    let togglers_true= togglers
        .clone()
        .into_iter()
        .filter(|x| x())
        .collect::<Vec<_>>;
        
    // let letter_sel = togglers[0].clone();

    let newletters = vec!['A', 'B', 'C', 'D', 'E', 'F', 'A'];

    let tiles = zipped
        .clone()
        .map(|(toggle, letter)| {
            view! {
                <li class="inline-block">
                    <button class="tile-bag lg:tile-bag-lg" class=("tile-bag-sel", move || toggle())
                    on:click=move |_| toggle.update(|t| *t = !*t)>
                    {toggle}<br/>{letter}
                    </button>
                </li>
            }
        })
        // .collect::<Vec<_>>();
        .collect_view();

    let tiles_sel = zipped
        .clone()
        .map(|(toggle, letter)| {
            view! {
                <li class="inline-grid">
                    <button class="tile-bag lg:tile-bag-lg" class=("inline order-last", move || toggle()) class=("hidden", move || !toggle())>
                    {toggle}<br/>{letter}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    let deser = valid();
    let newtiles = deser
        .clone()
        .into_iter()
        .map(|letter| {
            view! {<Tile letter=letter/>}
        })
        .collect::<Vec<_>>();

    view! {
    <ul class="block">{tiles}</ul>
    <ul class="block">{tiles_sel}</ul>
    <ul class="block">{newtiles}</ul>

    <button on:click=move |_| {}></button>


    // <button on:click=move |_| {valid.update(|vec| vec.push(letters[0]))}>Valider
    // </button>
