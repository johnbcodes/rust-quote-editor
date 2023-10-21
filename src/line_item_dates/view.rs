use crate::{
    layout::Flash,
    line_item_dates::model::LineItemDatePresenter,
    line_items::{model::LineItemPresenter, view::LineItem},
    quotes::{model::QuotePresenter, view::Footer},
};
use convert_case::{Case, Casing};

markup::define! {
    LineItemDate<'a>(line_item_date: &'a LineItemDatePresenter, line_items: &'a Vec<LineItemPresenter>) {
        $"turbo-frame"[id = &line_item_date.dom_id()] {
            // line-item-date
            div[class = "mt-8 mb-1.5"] {
                $"turbo-frame"[id = &line_item_date.edit_dom_id()] {
                    // line-item-date__header
                    div[class= "flex items-center justify-between gap-2"] {
                        h2[class = "text-[1.5rem] font-bold"] {
                            @line_item_date.date_long_form()
                        }
                        div["data-turbo-confirm" = "Are you sure?", class = "flex gap-2"] {
                            form[method = "post", action = "/line_item_dates/delete"] {
                                input[id = "line_item_date_id",
                                    name = "id",
                                    "type" = "hidden",
                                    value = &line_item_date.id()] {}
                                button[class = "button button-light", "type" = "submit"] {"Delete"}
                            }
                            a[class = "button button-light", href = {format!("/line_item_dates/edit/{}", &line_item_date.id())}] { "Edit" }
                        }
                    }
                }
                // line-item-date__body
                div[class = "bg-white rounded-md mt-2 p-4 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]"] {
                    // line-item line-item--header
                    div[class = "flex flex-wrap items-start bg-light gap-2 mb-3 p-2 rounded-md"] {
                        // line-item__name
                        div[class = "flex-1 font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Article" }
                        // line-item__quantity
                        div[class = "display-[revert] flex-[0_0_7rem] font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Quantity" }
                        // line-item__price
                        div[class = "display-[revert] flex-[0_0_9rem] font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Price" }
                        // line-item__actions
                        div[class = "flex flex-[0_0_10rem] order-[revert] gap-2 font-bold text-[0.875rem] tracking-[1px] uppercase"] {}
                    }

                    $"turbo-frame"[id = {format!("line_item_date_{}_line_items", &line_item_date.id())}] {
                        @for line_item in *line_items {
                            @LineItem { line_item }
                        }
                    }

                    @let line_item_new_id = format!("line_item_date_{}_line_item_new", &line_item_date.id());
                    $"turbo-frame"[id = &line_item_new_id] {}

                    div[class = "p-4 text-center border-2 border-dashed border-[hsl(0,6%,93%)] rounded-md"] {
                        a[class = "button button-prime", "data-turbo-frame" = &line_item_new_id, href = {format!("/line_items/new/{}", &line_item_date.id())}] {
                            "Add item"
                        }
                    }
                }
            }
        }
    }

    Form<'a>(dom_id: &'a String, line_item_date: &'a LineItemDatePresenter, action: &'a str, error_message: Option<String>) {
        $"turbo-frame"[id = dom_id] {
            form[id = dom_id,
                action = {format!("/line_item_dates/{}", action.to_case(Case::Flat))},
                method = "post",
                class = "flex flex-wrap justify-between items-center gap-2 mt-8 mb-1.5",
                autocomplete = "off",
                "accept-charset" = "UTF-8"] {
                @let form_input_class = if error_message.is_some() { "form-input border-primary" } else { "form-input" };
                @if let Some(message) = error_message {
                    div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] { @message }
                }
                @if let Some(id) = &line_item_date.id {
                    input[id = "line_item_date_id",
                        name = "id",
                        "type" = "hidden",
                        value = id] {}
                }
                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &line_item_date.quote_id] {}
                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "line_item_date_date"] { "Date" }
                    input[id = "line_item_date_date",
                        name = "date",
                        class = form_input_class,
                        autofocus = "autofocus",
                        required,
                        "type" = "date",
                        value = line_item_date.date_short_form()] {}
                }
                a[class = "button button-light", href = {format!("/quotes/show/{}", &line_item_date.quote_id)}] { "Cancel" }
                @let button_text = format!("{} date", action.to_case(Case::Title));
                input[name = "commit",
                    "type" = "submit",
                    value = &button_text,
                    class = "button button-secondary",
                    "data-disable-with" = &button_text] {}
            }
        }
    }

    Create<'a>(line_item_date: &'a LineItemDatePresenter,
               line_items: &'a Vec<LineItemPresenter>,
               message: &'a str) {
        @markup::raw(hotwire_turbo::stream::prepend("line_item_dates", LineItemDate{ line_item_date, line_items }.to_string()))
        @markup::raw(hotwire_turbo::stream::update("line_item_date_new", ""))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
    }

    Update<'a>(line_item_date: &'a LineItemDatePresenter,
               line_items: &'a Vec<LineItemPresenter>,
               message: &'a str) {
        @markup::raw(hotwire_turbo::stream::replace(&line_item_date.dom_id(), LineItemDate{ line_item_date, line_items }.to_string()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
    }

    Destroy<'a>(line_item_date: &'a LineItemDatePresenter, quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::remove(line_item_date.dom_id()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
        @markup::raw(hotwire_turbo::stream::update(&quote.total_dom_id(), Footer{ quote }.to_string()))
    }
}
