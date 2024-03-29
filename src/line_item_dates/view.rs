use crate::{
    forms::css_for_field,
    layout::Flash,
    line_item_dates::model::{EditLineItemDateForm, LineItemDatePresenter, NewLineItemDateForm},
    line_items::{model::LineItemPresenter, view::LineItem},
    quotes::{model::QuotePresenter, view::SwapFooter},
};
use rocket::form::{Contextual, Form};

markup::define! {
    LineItemDate<'a>(line_item_date: &'a LineItemDatePresenter, line_items: &'a Vec<LineItemPresenter>) {
        div[id = &line_item_date.dom_id()] {
            div[class = "mt-8 mb-1.5"] {
                @LineItemDateInfo{ line_item_date }

                // line-item body
                div[class = "bg-white rounded-md mt-2 p-4 shadow-[1px_3px_6px_hsl(0,0%,0%,0.1)]"] {
                    // header
                    div[class = "flex flex-wrap items-start bg-light gap-2 mb-3 p-2 rounded-md"] {
                        // name
                        div[class = "flex-1 font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Article" }
                        // quantity
                        div[class = "display-[revert] flex-[0_0_7rem] font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Quantity" }
                        // price
                        div[class = "display-[revert] flex-[0_0_9rem] font-bold text-[0.875rem] tracking-[1px] uppercase"] { "Price" }
                        // actions
                        div[class = "flex flex-[0_0_10rem] order-[revert] gap-2 font-bold text-[0.875rem] tracking-[1px] uppercase"] {}
                    }

                    div[id = {format!("line_item_date_{}_line_items", &line_item_date.id())}] {
                        @for line_item in *line_items {
                            @LineItem { line_item }
                        }
                    }

                    @let line_item_new_id = format!("line_item_date_{}_line_item_new", &line_item_date.id());
                    div[id = &line_item_new_id] {}

                    div[class = "p-4 text-center border-2 border-dashed border-[hsl(0,6%,93%)] rounded-md"] {
                        @let target = format!("#line_item_date_{}_line_item_new", &line_item_date.id());
                        a[class = "button button-prime",
                            "hx-get" = {format!("/line_items/new/{}", &line_item_date.id())},
                            "hx-target" = &target,
                            "hx-trigger" = "click",
                            "hx-swap" = "innerHTML"] { "Add item" }
                    }
                }
            }
        }
    }

    LineItemDateInfo<'a>(line_item_date: &'a LineItemDatePresenter) {
        div[id = &line_item_date.edit_dom_id()] {
            div[class= "flex items-center justify-between gap-2"] {
                h2[class = "text-[1.5rem] font-bold"] {
                    @line_item_date.date_long_form()
                }
                div[class = "flex gap-2"] {
                    form["hx-post" = "/line_item_dates/delete",
                        "hx-target" = {format!("#{}", &line_item_date.dom_id())},
                        "hx-swap" = "delete"] {

                        input[id = "line_item_date_id",
                            name = "id",
                            "type" = "hidden",
                            value = &line_item_date.id()] {}
                        button[class = "button button-light", "hx-confirm" = "Are you sure?", "type" = "submit"] {"Delete"}
                    }
                    a[class = "button button-light",
                        "hx-get" = {format!("/line_item_dates/edit/{}", &line_item_date.id())},
                        "hx-target" = {format!("#{}", &line_item_date.edit_dom_id())},
                        "hx-trigger" = "click"] { "Edit" }
                }
            }
        }
    }

    EditForm<'a>(line_item_date: &'a LineItemDatePresenter) {
        div[id = line_item_date.edit_dom_id()] {
            form[id = {format!("form_{}", &line_item_date.edit_dom_id())},
                "hx-post" = "/line_item_dates/update",
                "hx-target" = {format!("#{}", &line_item_date.dom_id())},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap justify-between items-center gap-2 mt-8 mb-1.5",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                input[id = "id",
                    name = "id",
                    disabled,
                    "type" = "hidden",
                    value = &line_item_date.id.clone().unwrap()] {}
                input[id = "quote_id",
                    name = "quote_id",
                    disabled,
                    "type" = "hidden",
                    value = &line_item_date.quote_id] {}
                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "line_item_date_date"] { "Date" }
                    input[id = "line_item_date_date",
                        name = "date",
                        class = "form-input",
                        autofocus = "autofocus",
                        required,
                        "type" = "date",
                        value = line_item_date.date_short_form()] {}
                }
                a[class = "button button-light",
                    "hx-get" = {format!("/line_item_dates/{}", &line_item_date.id())},
                    "hx-target" = {format!("#{}", &line_item_date.edit_dom_id())},
                    "hx-trigger" = "click",
                    "hx-swap" = "outerHTML"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Update date",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    EditFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, EditLineItemDateForm>>) {
        @let context = &form.context;
        @let id = context.field_value("id").unwrap_or("");
        @let quote_id = context.field_value("quote_id").unwrap_or("");
        @let date = context.field_value("date").unwrap_or("");
        @let dom_id = format!("line_item_date_{}", &id);
        @let edit_dom_id = format!("edit_line_item_date_{}", &id);
        div[id = &dom_id] {
            form[id = {format!("form_{}", &edit_dom_id)},
                "hx-post" = "/line_item_dates/update",
                "hx-target" = {format!("#{}", &dom_id)},
                "hx-swap" = "outerHTML",
                class = "flex flex-wrap justify-between items-center gap-2 mt-8 mb-1.5",
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
                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "line_item_date_date"] { "Date" }
                    input[id = "line_item_date_date",
                        name = "date",
                        class = css_for_field(form, "date", "form-input", "border-primary"),
                        autofocus = "autofocus",
                        required,
                        "type" = "date",
                        value = &date] {}
                }
                a[class = "button button-light",
                    "hx-get" = {format!("/line_item_dates/{}", id)},
                    "hx-target" = {format!("#{}", edit_dom_id)},
                    "hx-trigger" = "click",
                    "hx-swap" = "outerHTML"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Update date",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    NewForm<'a>(quote_id: &'a str) {
        div[id = "line_item_date_new"] {
            form[id = "form_new",
                "hx-post" = "/line_item_dates/create",
                "hx-target" = "#line_item_dates",
                "hx-swap" = "afterbegin",
                class = "flex flex-wrap justify-between items-center gap-2 mt-8 mb-1.5",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &quote_id] {}
                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "line_item_date_date"] { "Date" }
                    input[id = "line_item_date_date",
                        name = "date",
                        class = "form-input",
                        autofocus = "autofocus",
                        required,
                        "type" = "date"] {}
                }
                a[class = "button button-light",
                    "_" = "on click remove #form_new"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Create date",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    NewFormWithErrors<'a, 'r>(form: &'a Form<Contextual<'r, NewLineItemDateForm>>) {
        @let quote_id = form.context.field_value("quote_id").unwrap_or("");
        @let date = form.context.field_value("date").unwrap_or("");
        div[id = "line_item_date_new"] {
            form[id = "form_new",
                "hx-post" = "/line_item_dates/create",
                "hx-target" = "#line_item_dates",
                "hx-swap" = "afterbegin",
                class = "flex flex-wrap justify-between items-center gap-2 mt-8 mb-1.5",
                autocomplete = "off",
                novalidate,
                "accept-charset" = "UTF-8"] {

                @let messages = form.context.errors().map(|item| item.to_string()).collect::<Vec<String>>();
                div[class = "w-full text-primary bg-primary-bg p-2 rounded-md"] {
                    @for message in messages {
                        p { @message }
                    }
                }

                input[id = "quote_id",
                    name = "quote_id",
                    "type" = "hidden",
                    value = &quote_id] {}
                div[class = "[flex:1]"] {
                    label[class = "visually-hidden", "for" = "line_item_date_date"] { "Date" }
                    input[id = "line_item_date_date",
                        name = "date",
                        class = css_for_field(form, "date", "form-input", "border-primary"),
                        autofocus = "autofocus",
                        required,
                        "type" = "date",
                        value = &date] {}
                }
                a[class = "button button-light",
                    "_" = "on click remove #form_new"] { "Cancel" }
                input[name = "commit",
                    "type" = "submit",
                    value = "Create date",
                    class = "button button-secondary",
                    "_" = "on click add { pointer-events: none }"] {}
            }
        }
    }

    Create<'a>(line_item_date: &'a LineItemDatePresenter,
               line_items: &'a Vec<LineItemPresenter>,
               message: &'a str) {
        @LineItemDate{ line_item_date, line_items }
        div[id = "line_item_date_new", "hx-swap-oob"="innerHTML"]{}
        @Flash{ message }
    }

    Update<'a>(line_item_date: &'a LineItemDatePresenter,
               line_items: &'a Vec<LineItemPresenter>,
               message: &'a str) {
        @LineItemDate{ line_item_date, line_items }
        @Flash{ message }
    }

    Destroy<'a>(quote: &'a QuotePresenter, message: &'a str) {
        @Flash{ message }
        @SwapFooter{ quote }
    }
}
