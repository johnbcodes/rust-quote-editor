markup::define! {
    Layout<Head: markup::Render, Body: markup::Render>(head: Head, body: Body) {
        @markup::doctype()
        html[lang = "en", class = "h-full overflow-y-auto"] {
            head {
                meta[name = "viewport", content = "width=device-width,initial-scale=1"] {}
                meta[charset = "utf-8"] {}
                @head
                link[rel = "stylesheet", href = {format!("/dist/{}", env!("STYLESHEET"))}] {}
                link[rel = "icon", href = {format!("/dist/{}", env!("FAVICON"))}] {}
                script["type" = "module", src = {format!("/dist/{}", env!("SCRIPT"))}] {}
            }
            body[class = "flex flex-col min-h-full bg-background text-body leading-[1.5]"] {
                header[class = "flex items-center px-4 py-2 mb-10 bg-white shadow-[2px_4px_10px_hsl(0,0%,0%,0.1)]"] {
                    div[class = "font-bold text-xl/[1.5] text-header"] { "ABC Corp" }
                    div[class = "font-bold ml-auto mr-3 text-header"] { "Accountant" }
                    a[class = "button button-dark", href = "#"] { "Sign out" }
                }
                div[id = "flash", class = "fixed top-20 left-1/2 -translate-x-1/2 flex flex-col items-center gap-3 m-w-full w-max px-4 py-0"] {}
                @body
            }
        }
    }

    Flash<'a>(message: &'a str) {
        div[id = "flash", "hx-swap-oob" = "innerHTML"] {
            div[class = "text-[0.875rem] text-white px-4 py-2 bg-dark rounded-full animate-appear-then-fade",
                "_" = "on animationend remove me"] {
                @message
            }
        }
    }
}
