use crate::{
    layout,
    line_item_dates::{model::LineItemDatePresenter, view::LineItemDate},
    line_items::model::LineItemPresenter,
    quotes::model::QuotePresenter,
};
use convert_case::{Case, Casing};
use std::collections::HashMap;

markup::define! {
    Index<'a>(quotes: &'a Vec<QuotePresenter>) {
        main[id = "container", class = "w-full px-4 py-0 mx-auto my-0 max-w-[60rem]"] {
            div[id = "header", class = "flex flex-wrap gap-3 justify-between mt-4 mb-8"] {
                h1[class = "text-header text-[2rem]/[1.1] box-border m-0 p-0 font-bold"] {"Quotes"}
                a[class = "button button-prime", "data-turbo-frame" = "quote_new", href = "/quotes/new"] { "Add quote" }
            }

            $"turbo-frame"[id = "quote_new"] {}

            $"turbo-frame"[id = "quotes"] {
                div[class = "p-4 border-2 border-[hsl(0,6%,93%)] border-dashed text-center hidden only:[display:revert]"] {
                    p[class = "[font-size:1.125rem] text-header mb-6 font-bold"] {
                        "You don't have any quotes yet!"
                    }
                    a[class = "button button-prime", "data-turbo-frame" = "quote_new", href = "/quotes/new"] {
                        "Add quote"
                    }
                }
                @for quote in *quotes {
                    @Quote { quote }
                }
            }
        }
    }

    Quote<'a>(quote: &'a QuotePresenter) {
        $"turbo-frame"[id = &quote.dom_id()] {
            div[class= "flex justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]"] {
                a["data-turbo-frame" = "_top", href = {format!("/quotes/show/{}", &quote.id())}] { @quote.name }
                div[class = "flex flex-auto grow-0 shrink-0 self-start gap-2"] {
                    form[method = "post", action = "/quotes/delete"] {
                        input[id = "quote_id",
                            name = "id",
                            "type" = "hidden",
                            value = &quote.id()] {}
                        button[class = "button button-light", "type" = "submit"] {"Delete"}
                    }
                    a[class = "button button-light", href = {format!("/quotes/edit/{}", &quote.id())}] { "Edit" }
                }
            }
        }
    }

    Show<'a>(quote: &'a QuotePresenter,
             line_item_dates: &'a Vec<LineItemDatePresenter>,
             line_items: &'a HashMap<String, Vec<LineItemPresenter>>) {
        main[id = "container", class = "w-full px-4 py-0 mb-16 mx-auto my-0 max-w-[60rem]"] {
            a[href = "/quotes"] { "← Back to quotes" }
            div[class = "flex flex-wrap gap-3 justify-between mt-4 mb-8"] {
                h1[class = "text-header text-[2rem]/[1.1] m-0 p-0 font-bold"] {
                    @quote.name
                }

                a[class = "button button-prime",
                    "data-turbo-frame" = "line_item_date_new",
                    href = {format!("/line_item_dates/new/{}", &quote.id())}] {
                    "New date"
                }
            }
            $"turbo-frame"[id = "line_item_date_new"] {}

            $"turbo-frame"[id = "line_item_dates"] {
                @for line_item_date in *line_item_dates {
                    @let empty = Vec::new();
                    @let line_items = line_items.get(&line_item_date.id()).unwrap_or(&empty);
                    @LineItemDate { line_item_date, line_items }
                }
            }
        }

        @Footer { quote }
    }

    Form<'a>(quote: &'a QuotePresenter, action: &'a str, error_message: Option<String>) {
        $"turbo-frame"[id = &quote.dom_id()] {
            form[id = &quote.dom_id(),
                action = {format!("/quotes/{}", action)},
                method = "post",
                class = "flex flex-wrap justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]",
                autocomplete = "off",
                "accept-charset" = "UTF-8"] {
                @let form_input_class = if error_message.is_some() { "form-input border-primary" } else { "form-input" };
                @if let Some(message) = error_message {
                    div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] { @message }
                }
                div[class = "[flex:1]"] {
                    @if let Some(id) = &quote.id {
                        input[id = "quote_id",
                            name = "id",
                            "type" = "hidden",
                            value = id] {}
                    }
                    label[class = "visually-hidden", "for" = "quote_name"] { "Name" }
                    input[id = "quote_name",
                        name = "name",
                        class = form_input_class,
                        autofocus = "autofocus",
                        placeholder = "Name of your quote",
                        required,
                        "type" = "text",
                        value = &quote.name] {}
                }
                a[class = "button button-light", href="/quotes"] { "Cancel" }
                @let button_text = format!("{} quote", action.to_case(Case::Title));
                input[name = "commit",
                    "type" = "submit",
                    value = &button_text,
                    class = "button button-secondary",
                    "data-disable-with" = &button_text] {}
            }
        }
    }

    Footer<'a>(quote: &'a QuotePresenter) {
        $"turbo-frame"[id = &quote.total_dom_id()] {
            footer[class = "fixed bottom-0 w-full py-4 text-[1.25rem] font-bold bg-white shadow-[2px_4px_10px_hsl(0,0%,0%,0.1)]"] {
                div[class = "flex items-center justify-between w-full px-4 mx-auto max-w-[60rem]"] {
                    div { "Total:" }
                    div { @quote.total.format() }
                }
            }
        }
    }

    Create<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::prepend("quotes", Quote{ quote }.to_string()))
        @markup::raw(hotwire_turbo::stream::update("quote_new", ""))
        @markup::raw(hotwire_turbo::stream::prepend("flash", layout::Flash{ message: Some(message) }.to_string()))
    }

    Update<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::replace(&quote.dom_id(), Quote{ quote }.to_string()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", layout::Flash{ message: Some(message) }.to_string()))
    }

    Destroy<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::remove(quote.dom_id()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", layout::Flash{ message: Some(message) }.to_string()))
    }
}
