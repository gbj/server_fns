use std::time::Duration;

use futures::channel::mpsc::unbounded;
use futures::StreamExt;
use server_fn_macro_default::server;
use server_fns::request::browser::BrowserFormData;
use server_fns::ServerFn;
use server_fns::{
    codec::{ByteStream, MultipartData},
    ServerFnError,
};
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
use web_sys::FormData;

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

#[server(input = MultipartFormData, output = Streaming)]
pub async fn multipart_upload(data: MultipartData) -> Result<ByteStream, ServerFnError> {
    use bytes::Bytes;
    use futures::channel::mpsc::unbounded;

    let mut data = data.into_data().unwrap();
    let (tx, rx) = unbounded();
    tokio::spawn(async move {
        while let Ok(Some(mut field)) = data.next_field().await {
            while let Ok(Some(chunk)) = field.chunk().await {
                tx.unbounded_send(Bytes::from(format!("uploaded {} bytes\n", chunk.len())));
            }
        }
    });

    Ok(ByteStream::from(rx))
}

pub fn my_app() -> impl RenderHtml<Dom> {
    let form_ref = NodeRef::<Form>::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let form = form_ref.get().unwrap();
        let form_data = FormData::new_with_form(&form).unwrap();
        spawn_local(async move {
            let mut stream = multipart_upload(form_data.into())
                .await
                .unwrap()
                .into_inner();
            while let Some(value) = stream.next().await {
                tachys::tachydom::log(&String::from_utf8_lossy(value.as_ref()));
            }
        })
    };

    view! {
            <form on:submit=on_submit node_ref=form_ref
                method="post"
                enctype="multipart/form-data"
                action=MultipartUpload::url()
            >
                <input type="file" name="file_to_upload"/>
                <button>"Upload"</button>
            </form>
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
