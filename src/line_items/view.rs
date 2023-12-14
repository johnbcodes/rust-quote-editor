use crate::quotes::view::SwapFooter;
use crate::{layout::Flash, line_items::model::LineItemPresenter, quotes::model::QuotePresenter};

markup::define! {
    LineItem<'a>(line_item: &'a LineItemPresenter) {
        div[id = &line_item.dom_id()] {
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
                    form["hx-post" = "/line_items/delete",
                        "hx-target" = {format!("#{}", &line_item.dom_id())},
                        "hx-swap" = "delete"] {

                        input[id = "line_item_id",
                            name = "id",
                            "type" = "hidden",
                            value = &line_item.id()] {}
                        button[class = "button button-light", "type" = "submit"] {"Delete"}
                    }
                    a[class = "button button-light",
                        "hx-get" = {format!("/line_items/edit/{}", &line_item.id())},
                        "hx-target" = {format!("#{}", &line_item.dom_id())},
                        "hx-trigger" = "click"] { "Edit" }
                }
            }
        }
    }

    EditForm<'a>(line_item: &'a LineItemPresenter,
                 quote: &'a QuotePresenter,
                 error_message: Option<String>) {
        div[id = &line_item.dom_id()] {
            form[id = &line_item.dom_id(),
                "hx-post" = "/line_items/update",
                "hx-target" = {format!("#{}", &line_item.dom_id())},
                "hx-swap" = "outerHTML",
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
                        class = {format!("resize-none {}", form_input_class)},
                        placeholder = "Description (optional)"] { @line_item.description }
                }
                a[class = "button button-light",
                    "hx-get" = {format!("/line_items/{}", &line_item.id())},
                    "hx-target" = {format!("#{}", &line_item.dom_id())},
                    "hx-trigger" = "click",
                    "hx-swap" = "outerHTML"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Update item",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    NewForm<'a>(line_item: &'a LineItemPresenter,
                quote: &'a QuotePresenter,
                error_message: Option<String>) {
        div[id = &line_item.dom_id()] {
            @let line_item_new_dom_id = format!("#line_item_date_{}_line_items", &line_item.line_item_date_id);
            form[id = "form_new",
                "hx-post" = "/line_items/create",
                "hx-target" = line_item_new_dom_id,
                "hx-swap" = "beforeend",
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
                        class = {format!("resize-none {}", form_input_class)},
                        placeholder = "Description (optional)"] { @line_item.description }
                }
                a[class = "button button-light",
                    "_" = "on click remove #form_new"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Create item",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    Create<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter, message: &'a str) {
        @let line_item_new_dom_id = format!("line_item_date_{}_line_item_new", &line_item.line_item_date_id);
        @LineItem{ line_item }
        div[id = &line_item_new_dom_id, "hx-swap-oob"="innerHTML"]{}
        @Flash{ message }
        @SwapFooter{ quote }
    }

    Update<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter, message: &'a str) {
        @LineItem{ line_item }
        @Flash{ message }
        @SwapFooter{ quote }
    }

    Destroy<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @Flash{ message }
        @SwapFooter{ quote }
    }
}
