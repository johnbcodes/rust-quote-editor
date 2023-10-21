use crate::quotes::view::Footer;
use crate::{layout::Flash, line_items::model::LineItemPresenter, quotes::model::QuotePresenter};
use convert_case::{Case, Casing};

markup::define! {
    LineItem<'a>(line_item: &'a LineItemPresenter) {
        $"turbo-frame"[id = &line_item.dom_id()] {
            div[class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md"] {
                div[class = "flex-1 font-bold mb-0"] {
                    @line_item.name
                    div[class = "basis-full m-w-100 font-normal text-[0.875rem] text-[hsl(0,1%,44%)]"] {
                        @line_item.description
                    }
                }
                div[class = "display-[revert] flex-[0_0_7rem] mb-0"] {
                    @line_item.quantity
                }
                div[class = "display-[revert] flex-[0_0_9rem] mb-0"] {
                    @line_item.unit_price.format()
                }
                div[class = "flex flex-[0_0_10rem] order-[revert] gap-2"] {
                    form[method = "post", action = "/line_items/delete"] {
                        input[id = "line_item_id",
                            name = "id",
                            "type" = "hidden",
                            value = &line_item.id()] {}
                        button[class = "button button-light", "type" = "submit"] {"Delete"}
                    }
                    a[class = "button button-light", href = {format!("/line_items/edit/{}", &line_item.id())}] { "Edit" }
                }
            }
        }
    }

    Form<'a>(line_item: &'a LineItemPresenter,
             quote: &'a QuotePresenter,
             action: &'a str,
             error_message: Option<String>) {
        $"turbo-frame"[id = &line_item.dom_id()] {
            form[id = &line_item.dom_id(),
                action = {format!("/line_items/{}", action.to_case(Case::Flat))},
                method = "post",
                class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md",
                autocomplete = "off",
                "accept-charset" = "UTF-8"] {
                @let form_input_class = if error_message.is_some() { "form-input border-primary" } else { "form-input" };
                @if let Some(message) = error_message {
                    div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] { @message }
                }
                @if let Some(id) = &line_item.id {
                    input[id = "line_item_id",
                        name = "id",
                        "type" = "hidden",
                        value = id] {}
                }
                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &quote.id] {}
                input[id = "line_item_date_id",
                    name = "line_item_date_id",
                    "type" = "hidden",
                    value = &line_item.line_item_date_id] {}
                div[class = "flex-1 font-bold mb-0"] {
                    input[id = "line_item_name",
                        name = "name",
                        class = form_input_class,
                        autofocus = "autofocus",
                        placeholder = "Name of your item",
                        required,
                        "type" = "text",
                        value = &line_item.name] {}
                }
                div[class = "block flex-[0_0_7rem] mb-0"] {
                    input[id = "line_item_quantity",
                        name = "quantity",
                        class = form_input_class,
                        placeholder = "1",
                        required,
                        "type" = "number",
                        min = "1",
                        step = "1",
                        value = &line_item.quantity] {}
                }
                div[class = "block flex-[0_0_9rem] mb-0"] {
                    input[id = "line_item_price",
                        name = "unit_price",
                        class = form_input_class,
                        placeholder = "$100.00",
                        required,
                        "type" = "number",
                        min = "0.01",
                        step = "0.01",
                        value = &line_item.unit_price.to_string()] {}
                }
                div[class = "basis-full order-2 m-w-100 font-normal text-[0.875rem] text-[hsl(0,1%,44%)] mb-0"] {
                    textarea[id = "line_item_description",
                        name = "description",
                        class = form_input_class,
                        placeholder = "Description (optional)"] { @line_item.description }
                }
                a[class = "button button-light", href = {format!("/quotes/show/{}", &quote.id())}] { "Cancel" }
                @let button_text = format!("{} item", action.to_case(Case::Title));
                input[name = "commit",
                    "type" = "submit",
                    value = &button_text,
                    class = "button button-secondary",
                    "data-disable-with" = &button_text] {}
            }
        }
    }

    Create<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter, message: &'a str) {
        @let line_items_dom_id = format!("line_item_date_{}_line_items", &line_item.line_item_date_id);
        @let line_item_new_dom_id = format!("line_item_date_{}_line_item_new", &line_item.line_item_date_id);
        @markup::raw(hotwire_turbo::stream::append(&line_items_dom_id, LineItem{ line_item }.to_string()))
        @markup::raw(hotwire_turbo::stream::update(&line_item_new_dom_id, ""))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
        @markup::raw(hotwire_turbo::stream::update(&quote.total_dom_id(), Footer{ quote }.to_string()))
    }

    Update<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::replace(&line_item.dom_id(), LineItem{ line_item }.to_string()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
        @markup::raw(hotwire_turbo::stream::update(&quote.total_dom_id(), Footer{ quote }.to_string()))
    }

    Destroy<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter, message: &'a str) {
        @markup::raw(hotwire_turbo::stream::remove(line_item.dom_id()))
        @markup::raw(hotwire_turbo::stream::prepend("flash", Flash{ message: Some(message) }.to_string()))
        @markup::raw(hotwire_turbo::stream::update(&quote.total_dom_id(), Footer{ quote }.to_string()))
    }
}
