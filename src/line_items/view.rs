use crate::{
    forms::css_for_field,
    layout::Flash,
    line_items::model::{EditLineItemForm, LineItemPresenter, NewLineItemForm},
    quotes::{model::QuotePresenter, view::SwapFooter},
};
use rocket::form::{Contextual, Form};

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

    EditForm<'a>(line_item: &'a LineItemPresenter, quote: &'a QuotePresenter) {
        div[id = &line_item.dom_id()] {
            form[id = &line_item.dom_id(),
                "hx-post" = "/line_items/update",
                "hx-target" = {format!("#{}", &line_item.dom_id())},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                input[id = "id",
                    name = "id",
                    "type" = "hidden",
                    value = &line_item.id.clone().unwrap()] {}
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
                        class = "form-input",
                        autofocus = "autofocus",
                        placeholder = "Name of your item",
                        required,
                        "type" = "text",
                        value = &line_item.name] {}
                }
                div[class = "block flex-[0_0_7rem] mb-0"] {
                    input[id = "line_item_quantity",
                        name = "quantity",
                        class = "form-input",
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
                        class = "form-input",
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
                        class = "resize-none form-input",
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

    EditFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, EditLineItemForm>>) {
        @let context = &form.context;
        @let id = context.field_value("id").unwrap_or("");
        @let quote_id = context.field_value("quote_id").unwrap_or("");
        @let line_item_date_id = context.field_value("line_item_date_id").unwrap_or("");
        @let name = context.field_value("name").unwrap_or("");
        @let quantity = context.field_value("quantity").unwrap_or("");
        @let unit_price = context.field_value("unit_price").unwrap_or("");
        @let description = context.field_value("description").unwrap_or("");
        @let dom_id = format!("line_item_{}", &id);

        div[id = &dom_id] {
            form[id = &dom_id,
                "hx-post" = "/line_items/update",
                "hx-target" = {format!("#{}", &dom_id)},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                @let messages = context.errors().map(|item| item.to_string()).collect::<Vec<String>>();
                div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] {
                    @for message in messages {
                        p { @message }
                    }
                }

                input[id = "id",
                    name = "id",
                    "type" = "hidden",
                    value = &id] {}
                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &quote_id] {}
                input[id = "line_item_date_id",
                    name = "line_item_date_id",
                    "type" = "hidden",
                    value = &line_item_date_id] {}
                div[class = "flex-1 font-bold mb-0"] {
                    input[id = "line_item_name",
                        name = "name",
                        class = css_for_field(form, "name", "form-input", "border-primary"),
                        autofocus = "autofocus",
                        placeholder = "Name of your item",
                        "type" = "text",
                        value = &name] {}
                }
                div[class = "block flex-[0_0_7rem] mb-0"] {
                    input[id = "line_item_quantity",
                        name = "quantity",
                        class = css_for_field(form, "quantity", "form-input", "border-primary"),
                        placeholder = "1",
                        "type" = "number",
                        min = "1",
                        step = "1",
                        value = &quantity] {}
                }
                div[class = "block flex-[0_0_9rem] mb-0"] {
                    input[id = "line_item_price",
                        name = "unit_price",
                        class = css_for_field(form, "unit_price", "form-input", "border-primary"),
                        placeholder = "$100.00",
                        "type" = "number",
                        min = "0.01",
                        step = "0.01",
                        value = &unit_price] {}
                }
                div[class = "basis-full order-2 m-w-100 font-normal text-[0.875rem] text-[hsl(0,1%,44%)] mb-0"] {
                    textarea[id = "line_item_description",
                        name = "description",
                        class = css_for_field(form, "description", "resize-none form-input", "border-primary"),
                        placeholder = "Description (optional)"] { @description }
                }
                a[class = "button button-light",
                    "hx-get" = {format!("/line_items/{}", &id)},
                    "hx-target" = {format!("#{}", &dom_id)},
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
                quote: &'a QuotePresenter) {
        div[id = &line_item.dom_id()] {
            @let line_item_new_dom_id = format!("#line_item_date_{}_line_items", &line_item.line_item_date_id);
            form[id = "form_new",
                "hx-post" = "/line_items/create",
                "hx-target" = line_item_new_dom_id,
                "hx-swap" = "beforeend",
                class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

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
                        class = "form-input",
                        autofocus = "autofocus",
                        placeholder = "Name of your item",
                        "type" = "text",
                        value = &line_item.name] {}
                }
                div[class = "block flex-[0_0_7rem] mb-0"] {
                    input[id = "line_item_quantity",
                        name = "quantity",
                        class = "form-input",
                        placeholder = "1",
                        "type" = "number",
                        min = "1",
                        step = "1",
                        value = &line_item.quantity] {}
                }
                div[class = "block flex-[0_0_9rem] mb-0"] {
                    input[id = "line_item_price",
                        name = "unit_price",
                        class = "form-input",
                        placeholder = "$100.00",
                        "type" = "number",
                        min = "0.01",
                        step = "0.01",
                        value = &line_item.unit_price.to_string()] {}
                }
                div[class = "basis-full order-2 m-w-100 font-normal text-[0.875rem] text-[hsl(0,1%,44%)] mb-0"] {
                    textarea[id = "line_item_description",
                        name = "description",
                        class = "resize-none form-input",
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

    NewFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, NewLineItemForm>>) {
        @let context = &form.context;
        @let quote_id = context.field_value("quote_id").unwrap_or("");
        @let line_item_date_id = context.field_value("line_item_date_id").unwrap_or("");
        @let name = context.field_value("name").unwrap_or("");
        @let quantity = context.field_value("quantity").unwrap_or("");
        @let unit_price = context.field_value("unit_price").unwrap_or("");
        @let description = context.field_value("description").unwrap_or("");

        div[id = "line_item_new"] {
            @let line_item_new_dom_id = format!("#line_item_date_{}_line_items", &line_item_date_id);
            form[id = "form_new",
                "hx-post" = "/line_items/create",
                "hx-target" = line_item_new_dom_id,
                "hx-swap" = "beforeend",
                class = "flex flex-wrap items-start bg-white gap-2 mb-3 p-2 rounded-md",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                @let messages = context.errors().map(|item| item.to_string()).collect::<Vec<String>>();
                div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] {
                    @for message in messages {
                        p { @message }
                    }
                }

                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &quote_id] {}
                input[id = "line_item_date_id",
                    name = "line_item_date_id",
                    "type" = "hidden",
                    value = &line_item_date_id] {}
                div[class = "flex-1 font-bold mb-0"] {
                    input[id = "line_item_name",
                        name = "name",
                        class = css_for_field(form, "name", "form-input", "border-primary"),
                        autofocus = "autofocus",
                        placeholder = "Name of your item",
                        required,
                        "type" = "text",
                        value = &name] {}
                }
                div[class = "block flex-[0_0_7rem] mb-0"] {
                    input[id = "line_item_quantity",
                        name = "quantity",
                        class = css_for_field(form, "quantity", "form-input", "border-primary"),
                        placeholder = "1",
                        required,
                        "type" = "number",
                        min = "1",
                        step = "1",
                        value = &quantity] {}
                }
                div[class = "block flex-[0_0_9rem] mb-0"] {
                    input[id = "line_item_price",
                        name = "unit_price",
                        class = css_for_field(form, "unit_price", "form-input", "border-primary"),
                        placeholder = "$100.00",
                        required,
                        "type" = "number",
                        min = "0.01",
                        step = "0.01",
                        value = &unit_price] {}
                }
                div[class = "basis-full order-2 m-w-100 font-normal text-[0.875rem] text-[hsl(0,1%,44%)] mb-0"] {
                    textarea[id = "line_item_description",
                        name = "description",
                        class = css_for_field(form, "description", "resize-none form-input", "border-primary"),
                        placeholder = "Description (optional)"] { @description }
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
