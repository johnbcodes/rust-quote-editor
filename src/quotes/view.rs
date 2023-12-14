use crate::{
    layout::Flash,
    line_item_dates::{model::LineItemDatePresenter, view::LineItemDate},
    line_items::model::LineItemPresenter,
    quotes::model::QuotePresenter,
};
use std::collections::HashMap;

markup::define! {
    Index(quotes: Vec<QuotePresenter>) {
        main[id = "container", class = "w-full px-4 py-0 mx-auto my-0 max-w-[60rem]"] {
            div[id = "header", class = "flex flex-wrap gap-3 justify-between mt-4 mb-8"] {
                h1[class = "text-header text-[2rem]/[1.1] box-border m-0 p-0 font-bold"] {"Quotes"}
                a[class = "button button-prime",
                    "hx-get" = "/quotes/new",
                    "hx-target" = "#quote_new",
                    "hx-trigger" = "click",
                    "hx-swap" = "outerHTML"] { "Add quote" }
            }

            div[id = "quote_new"] {}

            div[id = "quotes"] {
                div[id = "quotes_empty", class = "p-4 border-2 border-[hsl(0,6%,93%)] border-dashed text-center hidden only:[display:revert]"] {
                    p[class = "[font-size:1.125rem] text-header mb-6 font-bold"] {
                        "You don't have any quotes yet!"
                    }
                    a[class = "button button-prime",
                        "hx-get" = "/quotes/new",
                        "hx-target" = "quote_new",
                        "hx-trigger" = "click"] { "Add quote" }
                }
                @for quote in quotes {
                    @Quote { quote }
                }
            }
        }
    }

    Quote<'a>(quote: &'a QuotePresenter) {
        div[id = &quote.dom_id()] {
            div[class= "flex justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]"] {
                a[href = {format!("/quotes/show/{}", &quote.id())},
                    "hx-boost" = "true",
                    "hx-push-url" = "true",
                    "hx-history" = "false"] { @quote.name }
                div[class = "flex flex-auto grow-0 shrink-0 self-start gap-2"] {
                    form["hx-post" = "/quotes/delete",
                         "hx-target" = {format!("#{}", &quote.dom_id())},
                         "hx-swap" = "delete"] {
                        input[id = "quote_id",
                            name = "id",
                            "type" = "hidden",
                            value = &quote.id()] {}
                        button[class = "button button-light", "type" = "submit"] {"Delete"}
                    }
                    a[class = "button button-light",
                        "hx-get" = {format!("/quotes/edit/{}", &quote.id())},
                        "hx-target" = {format!("#{}", &quote.dom_id())},
                        "hx-trigger" = "click"] { "Edit" }
                }
            }
        }
    }

    Show<'a>(quote: &'a QuotePresenter,
             line_item_dates: &'a Vec<LineItemDatePresenter>,
             line_items: &'a HashMap<String, Vec<LineItemPresenter>>) {
        main[id = "container", class = "w-full px-4 py-0 mb-16 mx-auto my-0 max-w-[60rem]"] {
            a[href = "/quotes",
                "hx-boost" = "true",
                "hx-push-url" = "true",
                "hx-history" = "false"] { "← Back to quotes" }
            div[class = "flex flex-wrap gap-3 justify-between mt-4 mb-8"] {
                h1[class = "text-header text-[2rem]/[1.1] m-0 p-0 font-bold"] {
                    @quote.name
                }

                a[class = "button button-prime",
                    "hx-get" = {format!("/line_item_dates/new/{}", &quote.id())},
                    "hx-target" = "#line_item_date_new",
                    "hx-trigger" = "click",
                    "hx-swap" = "innerHTML"] { "New date" }
            }
            div[id = "line_item_date_new"] {}

            div[id = "line_item_dates"] {
                @for line_item_date in *line_item_dates {
                    @let empty = Vec::new();
                    @let line_items = line_items.get(&line_item_date.id()).unwrap_or(&empty);
                    @LineItemDate { line_item_date, line_items }
                }
            }
        }

        @InitialFooter { quote }
    }

    EditForm<'a>(quote: &'a QuotePresenter, error_message: Option<String>) {
        div[id = &quote.dom_id()] {
            form[id = format!("form_{}", &quote.id()),
                "hx-post" = "/quotes/update",
                "hx-target" = {format!("#{}", &quote.dom_id())},
                "hx-swap" = "outerHTML",
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
                a[class = "button button-light",
                    "hx-get" = {format!("/quotes/{}", &quote.id())},
                    "hx-target" = {format!("#{}", &quote.dom_id())},
                    "hx-trigger" = "click"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Update quote",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    NewForm<'a>(quote: &'a QuotePresenter, error_message: Option<String>) {
        div[id = &quote.dom_id()] {
            form[id = "form_new",
                "hx-post" = "/quotes/create",
                "hx-target" = "#quotes_empty",
                "hx-swap" = "afterend",
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
                a[class = "button button-light",
                    "_" = "on click remove #form_new"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Create quote",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    Footer<'a>(quote: &'a QuotePresenter) {
        footer[class = "fixed bottom-0 w-full py-4 text-[1.25rem] font-bold bg-white shadow-[2px_4px_10px_hsl(0,0%,0%,0.1)]"] {
            div[class = "flex items-center justify-between w-full px-4 mx-auto max-w-[60rem]"] {
                div { "Total:" }
                div { @quote.total.format() }
            }
        }
    }

    InitialFooter<'a>(quote: &'a QuotePresenter) {
        div[id = "quote_total_footer"] {
            @Footer{ quote }
        }
    }

    SwapFooter<'a>(quote: &'a QuotePresenter) {
        div[id = "quote_total_footer", "hx-swap-oob" = "true"] {
            @Footer{ quote }
        }
    }

    Create<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @Quote{ quote }
        div[id = "quote_new", "hx-swap-oob"="innerHTML"]{}
        @Flash{ message }
    }

    Update<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @Quote{ quote }
        @Flash{ message }
    }
}
