use crate::{
    forms::css_for_field,
    layout::Flash,
    line_item_dates::{model::LineItemDatePresenter, view::LineItemDate},
    line_items::model::LineItemPresenter,
    quotes::model::{EditQuoteForm, NewQuoteForm, QuotePresenter},
};
use rocket::form::{Contextual, Form};
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

    EditForm<'a>(quote: &'a QuotePresenter) {
        div[id = &quote.dom_id()] {
            form[id = format!("form_{}", &quote.id()),
                "hx-post" = "/quotes/update",
                "hx-target" = {format!("#{}", &quote.dom_id())},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                div[class = "[flex:1]"] {
                    input[id = "quote_id",
                        name = "id",
                        "type" = "hidden",
                        value = &quote.id.clone().unwrap()] {}
                    label[class = "visually-hidden", "for" = "quote_name"] { "Name" }
                    input[id = "quote_name",
                        name = "name",
                        class = "form-input",
                        autofocus = "autofocus",
                        placeholder = "Name of your quote",
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

    EditFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, EditQuoteForm>>) {
        @let id = form.context.field_value("id").unwrap_or("");
        @let name = form.context.field_value("name").unwrap_or("");
        @let dom_id = format!("quote_{}", &id);
        div[id = &dom_id] {
            form[id = format!("form_{}", &id),
                "hx-post" = "/quotes/update",
                "hx-target" = {format!("#{}", &dom_id)},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                @let messages = form.context.errors().map(|item| item.to_string()).collect::<Vec<String>>();
                div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] {
                    @for message in messages {
                        p { @message }
                    }
                }

                div[class = "[flex:1]"] {
                    input[id = "quote_id",
                        name = "id",
                        "type" = "hidden",
                        value = &id] {}
                    label[class = "visually-hidden", "for" = "quote_name"] { "Name" }
                    input[id = "quote_name",
                        name = "name",
                        class = css_for_field(form, "name", "form-input", "border-primary"),
                        autofocus = "autofocus",
                        placeholder = "Name of your quote",
                        "type" = "text",
                        value = &name] {}
                }
                a[class = "button button-light",
                    "hx-get" = {format!("/quotes/{}", &id)},
                    "hx-target" = {format!("#{}", &dom_id)},
                    "hx-trigger" = "click"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Update quote",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    NewForm() {
        div[id = "quote_new"] {
            form[id = "form_new",
                "hx-post" = "/quotes/create",
                "hx-target" = "#quotes_empty",
                "hx-swap" = "afterend",
                class = "flex flex-wrap justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "quote_name"] { "Name" }
                    input[id = "quote_name",
                        name = "name",
                        class = "form-input",
                        autofocus = "autofocus",
                        placeholder = "Name of your quote",
                        "type" = "text"] {}
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

    NewFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, NewQuoteForm>>) {
        @let name = form.context.field_value("name").unwrap_or("");
        div[id = "quote_new"] {
            form[id = "form_new",
                "hx-post" = "/quotes/create",
                "hx-target" = "#quotes_empty",
                "hx-swap" = "afterend",
                class = "flex flex-wrap justify-between items-center gap-3 bg-white rounded-md mb-4 px-4 py-2 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                @let messages = form.context.errors().map(|item| item.to_string()).collect::<Vec<String>>();
                div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] {
                    @for message in messages {
                        p { @message }
                    }
                }

                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "quote_name"] { "Name" }
                    input[id = "quote_name",
                        name = "name",
                        class = "form-input border-primary",
                        autofocus = "autofocus",
                        placeholder = "Name of your quote",
                        "type" = "text",
                        value = &name] {}
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
