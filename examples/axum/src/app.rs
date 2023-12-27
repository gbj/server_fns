use std::collections::HashMap;
use std::time::Duration;

use futures::channel::mpsc::unbounded;
use futures::StreamExt;
use server_fn_macro_default::server;
use server_fns::codec::TextStream;
use server_fns::request::browser::BrowserFormData;
use server_fns::ServerFn;
use server_fns::{
    codec::{ByteStream, MultipartData},
    ServerFnError,
};
use tachys::tachydom::view::keyed::keyed;
use tachys::{
    prelude::*,
    tachy_reaccy::spawn::spawn_local,
    tachydom::{
        html::{
            element::{Form, Input},
            event::SubmitEvent,
        },
        node_ref::NodeRef,
    },
};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

/* #[server]
pub async fn server_fn_1(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(1))
}

#[server(encoding = "Cbor")]
pub async fn server_fn_2(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(2))
}

#[server(input = GetUrl, output = Cbor)]
pub async fn server_fn_3(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(3))
}

#[server(input = GetUrl, output = Streaming, endpoint = "stream")]
pub async fn streaming() -> Result<ByteStream, ServerFnError> {
    use futures::StreamExt;
    use tokio::time;
    use tokio_stream::wrappers::IntervalStream;

    let mut count = 0;
    let stream = IntervalStream::new(time::interval(Duration::from_secs(1)))
        .map(move |_| {
            count += 1;
            count
        })
        .take(5);

    Ok(ByteStream::from(stream.map(|n| n.to_string())))
} */

#[server(input = MultipartFormData, output = StreamingText)]
pub async fn multipart_upload(data: MultipartData) -> Result<TextStream, ServerFnError> {
    use bytes::Bytes;
    use futures::channel::mpsc::unbounded;

    let mut data = data.into_data().unwrap();
    println!("\n\n{data:#?}\n\n");

    let (tx, rx) = unbounded();
    tokio::spawn(async move {
        while let Ok(Some(mut field)) = data.next_field().await {
            println!("\n[NEXT FIELD]\n");
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                println!("      [CHUNK] {len}");
                tx.unbounded_send(format!("{name}<>{len}\n"));
            }
        }
    });

    Ok(TextStream::from(rx))
}

pub fn my_app() -> impl RenderHtml<Dom> {
    let progress: RwSignal<HashMap<String, (u32, u32)>> = RwSignal::new(HashMap::new());
    let input_ref = NodeRef::<Input>::new();
    let form_ref = NodeRef::<Form>::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let form = form_ref.get().unwrap();
        let form_data = FormData::new_with_form(&form).unwrap();

        let files = input_ref.get().unwrap().files().unwrap();
        let len = files.length();
        let mut starting_files = HashMap::new();
        for idx in 0..len {
            let file = files.item(idx).unwrap();
            starting_files.insert(file.name(), (0, file.size() as u32));
            form_data.append_with_blob(&file.name(), &file);
        }
        progress.set(starting_files);

        spawn_local(async move {
            let mut stream = multipart_upload(form_data.into())
                .await
                .unwrap()
                .into_inner();
            while let Some(value) = stream.next().await {
                tachys::tachydom::log(&format!("value = {value:?}"));
                for line in value.unwrap().lines() {
                    let (name, bytes) = line.split_once("<>").unwrap();
                    let bytes = bytes.parse::<u32>().unwrap();
                    progress.update(|map| {
                        let entry = map.get_mut(name).unwrap();
                        entry.0 += bytes;
                    });
                }
            }
        })
    };

    let progress = move || {
        keyed(
            progress.get(),
            |(name, _)| name.to_owned(),
            move |(name, (current, max))| {
                view! {
                    <div>
                        <label>
                            {name.clone()}
                            <progress max=max.to_string() value=move || {
                                progress
                                    .with(|prog| prog.get(&name).map(|(curr, _)| *curr))
                                    .unwrap_or(0)
                                    .to_string()
                            }/>
                        </label>
                    </div>
                }
            },
        )
    };

    view! {
            <form on:submit=on_submit
                node_ref=form_ref
                method="post"
                enctype="multipart/form-data"
                action=MultipartUpload::url()
            >
                <input type="file" node_ref=input_ref multiple/>
                <button>"Upload"</button>
            </form>
            {progress}
    /*         <button on:click=|_| {
                spawn_local(async move {
                    let mut stream = streaming().await.unwrap().into_inner();
                    while let Some(value) = stream.next().await {
                        tachys::tachydom::log(&format!("{:?}", value));
                    }
                })
            }>
                "Streaming"
            </button>
            <button
                on:click=move |_| spawn_local(async move {
                    let new_count = server_fn_1(count.get()).await.expect("server fn failed");
                    set_count.set(new_count);
                })
            >
                "JSON " {move || count.get()}
            </button>
            <button
                on:click=move |_| spawn_local(async move {
                    let new_count = server_fn_2(count2.get()).await.expect("server fn failed");
                    set_count2.set(new_count);
                })
            >
                "CBOR " {move || count2.get()}
            </button>
            <button
                on:click=move |_| spawn_local(async move {
                    let new_count = server_fn_3(count2.get()).await.expect("server fn failed");
                    set_count3.set(new_count);
                })
            >
                "Get/CBOR " {move || count3.get()}
            </button> */
        }
}
