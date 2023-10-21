// noinspection ES6UnusedImports
import * as Turbo from "@hotwired/turbo"
import { Application } from "@hotwired/stimulus"

import RemovalsController from "./controllers/removals_controller"

window.Stimulus = Application.start()
Stimulus.register("removals", RemovalsController)

document.addEventListener("turbo:before-frame-render", (event) => {
    const inputs = event.detail.newFrame.querySelectorAll("input, select, textarea");
    inputs.forEach(input => {
        input.addEventListener(
            "invalid",
            _event => {
                input.classList.add("error");
            },
            false
        );
    });
})